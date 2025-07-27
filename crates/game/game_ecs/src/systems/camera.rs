use crate::components::{
    CameraComponent, CameraProjection, NodeCameraComponent, NodeComponent, StringId,
    TransformComponent,
};
use crate::entities::CameraEntity;
use crate::world::{Entity, EntityType, Query, World, WorldCreate};
use crate::{GameEventHandlers, GameResources};
use game_settings::{ACTIVE_CAMERA, ACTIVE_PLAYER, FOV};
use glam::Mat4;
use sedona_ecs::system;
use sedona_settings::{SHADOW_MAP_RESOLUTION, SettingsValue, value_as};

#[system(group=startup)]
pub fn camera_startup(
    world: &mut World,
    resources: &mut GameResources,
    event_handlers: &mut GameEventHandlers,
) {
    // const DEFAULT_CLIP_NEAR: f32 = 0.1;
    // const DEFAULT_CLIP_FAR: f32 = 1000.0;
    //
    // let fov: f32 = value_as(resources.config.get(FOV)).unwrap_or(72.0);
    // let fov_rad = fov.to_radians();
    //
    // let (width, height) = resources.window.size();
    // let aspect_ratio = width as f32 / height as f32;
    //
    // let projection_matrix =
    //     Mat4::perspective_rh(fov_rad, aspect_ratio, DEFAULT_CLIP_NEAR, DEFAULT_CLIP_FAR);
    //
    // let default_camera = world.create(CameraEntity {
    //     string_id: StringId(Some(String::from("camera_default"))),
    //     transform: TransformComponent::with_position(glam::vec3(0.0, 25.0, 0.0)),
    //     camera_data: CameraComponent {
    //         name: None,
    //         clip_near: DEFAULT_CLIP_NEAR,
    //         clip_far: DEFAULT_CLIP_FAR,
    //         projection: CameraProjection::Perspective {
    //             fov_y: fov_rad,
    //             aspect_ratio,
    //         },
    //         projection_matrix,
    //     },
    // });
    //
    // resources.variables.insert(
    //     ACTIVE_CAMERA,
    //     SettingsValue::EntityId(default_camera.id()),
    // );
}

#[system(group=window_resized)]
pub fn camera_window_resized(
    world: &mut World,
    resources: &mut GameResources,
    cameras: Query<&mut CameraComponent>,
    node_cameras: Query<&mut NodeCameraComponent>,
) {
    let active_camera_id = match resources.variables.get(ACTIVE_CAMERA) {
        Some(SettingsValue::EntityId(id)) => *id,
        _ => return,
    };

    let (width, height) = resources.window.size();
    let aspect_ratio = width as f32 / height as f32;

    if let Some(camera) = world.with_query_mut(cameras).get_mut(active_camera_id) {
        update_projection_matrix(camera, aspect_ratio);
    } else if let Some(camera) = world.with_query_mut(node_cameras).get_mut(active_camera_id) {
        if let Some(camera) = &mut camera.camera {
            update_projection_matrix(camera, aspect_ratio);
        } else {
            log::warn!(
                "Node {active_camera_id} is set as the active camera but has no camera attached",
            );
        }
    } else {
        log::warn!(
            "Entity {active_camera_id} is set as the active camera but has no camera component",
        );
    }
}

pub fn update_projection_matrix(camera: &mut CameraComponent, new_aspect_ratio: f32) {
    match &mut camera.projection {
        CameraProjection::Perspective {
            aspect_ratio,
            fov_y,
        } => {
            *aspect_ratio = new_aspect_ratio;
            camera.projection_matrix =
                Mat4::perspective_rh(*fov_y, *aspect_ratio, camera.clip_near, camera.clip_far);
        }
        CameraProjection::Orthographic { zoom_x, zoom_y } => {
            *zoom_x = *zoom_y * new_aspect_ratio;
        }
    }
}
