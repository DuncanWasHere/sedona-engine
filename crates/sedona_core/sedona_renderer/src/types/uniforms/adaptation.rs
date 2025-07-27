use bytemuck::{Pod, Zeroable};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct AdaptationUniforms {
    pub adaptation_speed: f32,
    pub min_log_luminance: f32,
    pub max_log_luminance: f32,
    pub dt: f32,
}

impl Default for AdaptationUniforms {
    fn default() -> Self {
        Self {
            adaptation_speed: 1.0,
            min_log_luminance: -2.0,
            max_log_luminance: 2.0,
            dt: 1.0 / 60.0,
        }
    }
}
