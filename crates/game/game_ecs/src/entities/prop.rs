use crate::components::{ModelPath, NodeEntityRef, StringId, TransformComponent};
use sedona_ecs::entity;

#[entity]
pub struct PropEntity {
    pub string_id: StringId,
    pub transform: TransformComponent,
    pub model_path: ModelPath,
    #[serde(skip)]
    pub model: NodeEntityRef,
}
