use bytemuck::{Pod, Zeroable};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct DepthOfFieldUniforms {
    focal_distance: f32,
    focal_range: f32,
    blur_strength: f32,
    aperture: f32,
}

impl Default for DepthOfFieldUniforms {
    fn default() -> Self {
        Self {
            focal_distance: 10.0,
            focal_range: 5.0,
            blur_strength: 1.5,
            aperture: 1.2,
        }
    }
}
