use crate::types::VertexBufferObject;

#[derive(Clone, Debug)]
pub struct RenderObject {
    pub vbo: VertexBufferObject,
    pub material: usize,
    pub node: usize,
}

impl RenderObject {
    pub fn new(vbo: VertexBufferObject, material: usize, node: usize) -> Self {
        Self {
            vbo,
            material,
            node,
        }
    }
}
