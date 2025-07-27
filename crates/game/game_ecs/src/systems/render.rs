use crate::GameResources;
use crate::components::{
    CameraComponent, CameraProjection, NodeCameraComponent, NodeComponent, TransformComponent,
    WeatherComponent,
};
use crate::utils::view_uniforms::update_view_uniforms;
use crate::world::{Entity, EntityType, Query, World};
use game_settings::ACTIVE_CAMERA;
use sedona_ecs::system;
use sedona_settings::{value_as, SettingsValue};

#[system(group=window_resized)]
pub fn render_window_resized(resources: &mut GameResources) {
    let (width, height) = resources.window.size();
    resources.renderer.resize(width, height);
}

#[system(group=post_update)]
pub fn render_update(
    world: &mut World,
    resources: &mut GameResources,
    cameras: Query<(&CameraComponent, &TransformComponent)>,
    node_cameras: Query<(&NodeCameraComponent, &NodeComponent)>,
) {
    let active_camera_id = match resources.variables.get(ACTIVE_CAMERA) {
        Some(SettingsValue::EntityId(id)) => *id,
        _ => return,
    };

    if let Some((camera, transform)) = world.with_query(cameras).get(active_camera_id) {
        let transform_matrix = transform.to_matrix();
        update_view_uniforms(resources, camera, transform_matrix);
        return;
    }

    if let Some((node_camera, node)) = world.with_query(node_cameras).get(active_camera_id) {
        if let Some(camera) = &node_camera.camera {
            let transform_matrix = node.global_transform;
            update_view_uniforms(resources, camera, transform_matrix);
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
