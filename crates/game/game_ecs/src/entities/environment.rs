use crate::components::{EnvironmentComponent, StringId, Weathers};
use sedona_ecs::entity;

#[entity]
pub struct EnvironmentEntity {
    pub string_id: StringId,
    pub environment_data: EnvironmentComponent,
    pub weathers: Weathers,
}
