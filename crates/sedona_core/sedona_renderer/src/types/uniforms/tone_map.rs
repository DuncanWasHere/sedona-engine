use bytemuck::{Pod, Zeroable};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct ToneMapUniforms {
    exposure: f32,
    gamma: f32,
    tone_map_operator: u32,
    apply_gamma: u32,
}

impl Default for ToneMapUniforms {
    fn default() -> Self {
        Self {
            exposure: 1.0,
            gamma: 2.2,
            tone_map_operator: 0,
            apply_gamma: 1,
        }
    }
}
