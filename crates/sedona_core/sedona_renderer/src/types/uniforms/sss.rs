use bytemuck::{Pod, Zeroable};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct SssUniforms {
    max_distance: f32,
    sample_radius: f32,
    shadow_softness: f32,
    shadow_intensity: f32,
}

impl Default for SssUniforms {
    fn default() -> Self {
        Self {
            max_distance: 10.0,
            sample_radius: 0.25,
            shadow_softness: 0.5,
            shadow_intensity: 1.0,
        }
    }
}
