use bytemuck::{Pod, Zeroable};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct SsaoUniforms {
    radius: f32,
    bias: f32,
    intensity: f32,
    sample_count: u32,
}

impl Default for SsaoUniforms {
    fn default() -> Self {
        Self {
            radius: 0.5,
            bias: 0.025,
            intensity: 1.0,
            sample_count: 16,
        }
    }
}
