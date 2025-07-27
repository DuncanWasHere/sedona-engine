use crate::components::{CameraComponent, StringId, TransformComponent};
use sedona_ecs::entity;

#[entity]
pub struct CameraEntity {
    pub string_id: StringId,
    pub camera_data: CameraComponent,
    pub transform: TransformComponent,
}
