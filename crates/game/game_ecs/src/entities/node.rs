use crate::components::{Children, MeshComponents, NodeCameraComponent, NodeComponent};
use sedona_ecs::entity;

#[entity(serialize=false)]
pub struct NodeEntity {
    pub node: NodeComponent,
    pub meshes: MeshComponents,
    pub camera: NodeCameraComponent,
    pub children: Children,
}
