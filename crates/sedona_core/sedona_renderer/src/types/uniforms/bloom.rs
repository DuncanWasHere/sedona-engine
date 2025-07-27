use bytemuck::{Pod, Zeroable};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct BloomUniforms {
    pub threshold: f32,
    pub soft_knee: f32,
    pub intensity: f32,
    pub radius: f32,
}

impl Default for BloomUniforms {
    fn default() -> Self {
        Self {
            threshold: 1.0,
            soft_knee: 0.9,
            intensity: 0.8,
            radius: 1.0,
        }
    }
}
