use bytemuck::{Pod, Zeroable};
use glam::Vec4;
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Uniform, Zeroable)]
pub struct LightingUniforms {
    pub directional_angle: Vec4,
    pub directional_color: Vec4,
    pub ambient_color: Vec4,
    pub fog_color: Vec4,
    pub fog_start: f32,
    pub fog_end: f32,
    pub fog_exponent: f32,
    pub ambient_strength: f32,
    pub directional_strength: f32,
    pub contrast: f32,
    pub gamma: f32,
    pub saturation: f32,
}
