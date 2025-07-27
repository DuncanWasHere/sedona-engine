use crate::types::Vertex;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode, vertex_attr_array};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Deserialize, Serialize, Zeroable)]
pub struct SkinnedVertex {
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub tangent: [f32; 4],
    pub color: [f32; 4],
    pub tex_coord: [f32; 2],
    pub weight: f32,
}

impl SkinnedVertex {
    const ATTRIBUTES: [VertexAttribute; 6] = vertex_attr_array![
        0 => Float32x4,
        1 => Float32x4,
        2 => Float32x4,
        3 => Float32x4,
        4 => Float32x2,
        5 => Float32,
    ];

    pub fn new(
        position: [f32; 3],
        normal: [f32; 3],
        tangent: [f32; 3],
        color: [f32; 3],
        tex_coord: [f32; 2],
        weight: f32,
    ) -> Self {
        Self {
            position: [position[0], position[1], position[2], 1.0],
            normal: [normal[0], normal[1], normal[2], 1.0],
            tangent: [tangent[0], tangent[1], tangent[2], 1.0],
            color: [color[0], color[1], color[2], 1.0],
            tex_coord: [tex_coord[0], tex_coord[1]],
            weight,
        }
    }
}

impl Vertex for SkinnedVertex {
    fn descriptor<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: size_of::<SkinnedVertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
