use bytemuck::{Pod, Zeroable};
use glam::Mat4;
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Uniform, Zeroable)]
pub struct ModelUniforms {
    pub model_matrix: Mat4,
    pub model_normal_matrix: Mat4,
}

impl ModelUniforms {
    pub const SIZE: u64 = size_of::<Self>() as u64;
}
