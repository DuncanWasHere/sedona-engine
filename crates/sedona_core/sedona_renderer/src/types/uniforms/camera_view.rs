use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec4};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Uniform, Zeroable)]
pub struct CameraViewUniforms {
    pub view_projection_matrix: Mat4,
    pub direction_projection_matrix: Mat4,
    pub view_matrix: Mat4,
    pub view_position: Vec4,
}
