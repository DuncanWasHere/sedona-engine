use bytemuck::{Pod, Zeroable};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct SsrUniforms {
    max_steps: u32,
    stride: f32,
    thickness: f32,
    reflection_intensity: f32,
}

impl Default for SsrUniforms {
    fn default() -> Self {
        Self {
            max_steps: 64,
            stride: 1.0,
            thickness: 0.2,
            reflection_intensity: 1.0,
        }
    }
}
