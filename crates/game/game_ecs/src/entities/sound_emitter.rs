use crate::components::{SoundEntityRef, StringId, TransformComponent};
use sedona_ecs::entity;

#[entity]
pub struct SoundEmitterEntity {
    pub string_id: StringId,
    pub sound: SoundEntityRef,
    pub transform: TransformComponent,
}
