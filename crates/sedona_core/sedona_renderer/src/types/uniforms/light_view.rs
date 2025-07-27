use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec4};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Uniform, Zeroable)]
pub struct LightViewUniforms {
    pub light_view_projections: [Mat4; SHADOW_CASCADE_COUNT],
    pub cascade_splits: Vec4,
}

pub const SHADOW_CASCADE_COUNT: usize = 3;
