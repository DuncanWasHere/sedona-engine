use crate::components::{LightData, StringId, TransformComponent};
use sedona_ecs::entity;

#[entity]
pub struct LightEntity {
    pub string_id: StringId,
    pub light_data: LightData,
    pub transform: TransformComponent,
}
