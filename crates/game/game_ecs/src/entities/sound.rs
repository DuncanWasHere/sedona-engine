use crate::components::{SoundData, StringId};
use sedona_ecs::entity;

#[entity]
pub struct SoundEntity {
    pub string_id: StringId,
    pub sound_data: SoundData,
}
