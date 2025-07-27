use crate::components::{Children, StringId};
use sedona_ecs::entity;

#[entity]
pub struct SceneEntity {
    pub string_id: StringId,
    pub children: Children,
}
