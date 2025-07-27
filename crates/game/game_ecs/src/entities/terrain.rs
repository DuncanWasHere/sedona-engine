use crate::components::MeshComponent;
use sedona_ecs::entity;

#[entity]
pub struct TerrainEntity {
    pub mesh: MeshComponent,
}
