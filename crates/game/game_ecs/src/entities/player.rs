use crate::components::{ModelPath, NodeEntityRef, PlayerComponent, StringId, TransformComponent};
use sedona_ecs::entity;

#[entity]
pub struct PlayerEntity {
    pub string_id: StringId,
    pub player_data: PlayerComponent,
    pub transform: TransformComponent,
    pub model_path: ModelPath,
    #[serde(skip)]
    pub model: NodeEntityRef,
}
