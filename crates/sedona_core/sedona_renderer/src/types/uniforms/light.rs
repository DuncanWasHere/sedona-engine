use bytemuck::{Pod, Zeroable};
use glam::Vec4;
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Uniform, Zeroable)]
pub struct LightStorageUniforms {
    pub color: Vec4,
    pub position: Vec4,
    pub angle: Vec4,
    pub strength: f32,
    pub range: f32,
    pub light_type: u32,
    pub cutoff: f32,
}
