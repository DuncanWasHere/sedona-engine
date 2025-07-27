use crate::GameResources;
use crate::components::{CameraComponent, CameraProjection};
use crate::utils::vector_math::vec3_center_point;
use game_settings::SUN_DIRECTION;
use glam::{Mat3, Mat4, Vec3, Vec4};
use sedona_renderer::types::{CameraViewUniforms, LightViewUniforms, SHADOW_CASCADE_COUNT};
use sedona_settings::SettingsValue;

pub fn update_view_uniforms(
    resources: &mut GameResources,
    camera: &CameraComponent,
    transform_matrix: Mat4,
) {
    let queue = &resources.renderer.queue.borrow();

    let view_matrix = transform_matrix.inverse();
    let view_projection_matrix = camera.projection_matrix * view_matrix;
    let transform_no_pos = Mat4::from_mat3(Mat3::from_mat4(transform_matrix));
    let view_no_pos = transform_no_pos.inverse();
    let direction_projection_matrix = (camera.projection_matrix * view_no_pos).inverse();

    let camera_view_uniforms = CameraViewUniforms {
        view_projection_matrix,
        view_matrix,
        direction_projection_matrix,
        view_position: transform_matrix.w_axis,
    };

    resources
        .renderer
        .resources
        .globals
        .buffers
        .camera_view_ubo
        .set(camera_view_uniforms, queue);

    resources
        .renderer
        .resources
        .objects
        .sort(transform_matrix.w_axis.truncate());

    let shadow_map_resolution = resources.renderer.resources.settings.shadow_map_resolution;

    let sun_direction = match resources.variables.get(SUN_DIRECTION) {
        Some(SettingsValue::Vec3(value)) => *value,
        _ => Vec3::ZERO,
    };

    let light_view_uniforms = create_light_view_uniforms(
        &camera_view_uniforms,
        camera,
        sun_direction,
        shadow_map_resolution,
    );

    resources
        .renderer
        .resources
        .globals
        .buffers
        .light_view_ubo
        .set(light_view_uniforms, queue);

    resources
        .renderer
        .resources
        .shadow
        .update_light_cascade_buffers(light_view_uniforms, queue);

    resources
        .renderer
        .resources
        .post_process
        .update(resources.input_state.dt(), queue);
}

fn create_light_view_uniforms(
    camera_view_uniforms: &CameraViewUniforms,
    camera: &CameraComponent,
    sun_direction: Vec3,
    resolution: u32,
) -> LightViewUniforms {
    let camera_view = camera_view_uniforms.view_matrix;
    let cascade_splits = compute_cascade_splits(camera.clip_near, camera.clip_far, 0.75);

    let mut light_view_projections = [Mat4::IDENTITY; SHADOW_CASCADE_COUNT];

    for i in 0..SHADOW_CASCADE_COUNT {
        let cascade_near = cascade_splits[i];
        let cascade_far = cascade_splits[i + 1];

        // Get the corners of the camera view's frustum within the cascade range, in world space.
        let (world_space_corners) =
            compute_world_space_corners(camera_view, camera, cascade_near, cascade_far);

        // The light always faces the world space point at the center of the frustum slice.
        let world_space_center = vec3_center_point(&world_space_corners);

        // Light view matrix will be invalid when up direction is parallel to light direction (noon).
        let up = if sun_direction.abs_diff_eq(Vec3::Y, 0.01) {
            Vec3::Z
        } else {
            Vec3::Y
        };

        // Light direction is independent of camera, so light position must be calculated from both.
        let light_position = world_space_center + sun_direction;
        let light_view = Mat4::look_at_rh(light_position, world_space_center, up);

        // Light projection bounds must be calculated from frustum corners transformed into light space.
        let mut light_space_corners = [Vec3::ZERO; 8];
        for (i, &corner) in world_space_corners.iter().enumerate() {
            light_space_corners[i] = light_view.transform_point3(corner);
        }

        // Calculate the actual AABB bounds.
        let mut min = light_space_corners[0];
        let mut max = light_space_corners[0];
        for corner in &light_space_corners[1..] {
            min = min.min(*corner);
            max = max.max(*corner);
        }

        let light_space_center = (min + max) * 0.5;
        let light_space_extent_x = max.x - min.x;
        let light_space_extent_y = max.y - min.y;

        // Texel snapping is needed to stabilize shadow maps.
        let world_units_per_texel =
            (light_space_extent_x.max(light_space_extent_y)) / resolution as f32;

        let snapped_x =
            (light_space_center.x / world_units_per_texel).floor() * world_units_per_texel;
        let snapped_y =
            (light_space_center.y / world_units_per_texel).floor() * world_units_per_texel;
        let light_space_center_snapped = Vec3::new(snapped_x, snapped_y, light_space_center.z);

        let world_space_center_snapped = light_view
            .inverse()
            .transform_point3(light_space_center_snapped);
        let light_position_snapped = world_space_center_snapped + sun_direction;
        let light_view_snapped =
            Mat4::look_at_rh(light_position_snapped, world_space_center_snapped, up);

        for (i, &corner) in world_space_corners.iter().enumerate() {
            light_space_corners[i] = light_view_snapped.transform_point3(corner);
        }

        min = light_space_corners[0];
        max = light_space_corners[0];
        for corner in &light_space_corners[1..] {
            min = min.min(*corner);
            max = max.max(*corner);
        }

        let z_mult = 2.5;

        if min.z < 0.0 {
            min.z *= z_mult
        } else {
            min.z /= z_mult
        }

        if max.z < 0.0 {
            max.z /= z_mult
        } else {
            max.z *= z_mult
        }

        let light_projection = Mat4::orthographic_rh(min.x, max.x, min.y, max.y, -1000.0, 1000.0);

        light_view_projections[i] = light_projection * light_view_snapped;
    }

    LightViewUniforms {
        light_view_projections,
        cascade_splits,
    }
}

fn compute_cascade_splits(near: f32, far: f32, lambda: f32) -> Vec4 {
    let mut splits = [0.0; 4];
    splits[0] = near;

    for i in 1..=SHADOW_CASCADE_COUNT {
        let p = i as f32 / 3 as f32;
        let log = near * (far / near).powf(p);
        let uniform = near + (far - near) * p;
        splits[i] = lambda * log + (1.0 - lambda) * uniform;
    }

    Vec4::from_array(splits)
}

fn compute_world_space_corners(
    camera_view: Mat4,
    camera: &CameraComponent,
    z_near: f32,
    z_far: f32,
) -> ([Vec3; 8]) {
    let (aspect_ratio, fov_y) = match camera.projection {
        CameraProjection::Perspective {
            aspect_ratio,
            fov_y,
        } => (aspect_ratio, fov_y),
        _ => panic!("Frustum world corners can only be computed for perspective camera"),
    };
    let camera_projection = Mat4::perspective_rh(fov_y, aspect_ratio, z_near, z_far);
    let inverse_camera_view_projection = (camera_projection * camera_view).inverse();

    let ndc_planes = [
        (-1.0, -1.0), // Bottom-left
        (1.0, -1.0),  // Bottom-right
        (1.0, 1.0),   // Top-right
        (-1.0, 1.0),  // Top-left
    ];

    let mut world_space_corners = [Vec3::ZERO; 8];

    for (i, &(x, y)) in ndc_planes.iter().enumerate() {
        let ndc_near = Vec4::new(x, y, 0.0, 1.0);
        let ndc_far = Vec4::new(x, y, 1.0, 1.0);

        let world_space_near = inverse_camera_view_projection * ndc_near;
        let world_space_far = inverse_camera_view_projection * ndc_far;

        world_space_corners[i] = (world_space_near / world_space_near.w).truncate();

        world_space_corners[i + 4] = (world_space_far / world_space_far.w).truncate();
    }

    world_space_corners
}
