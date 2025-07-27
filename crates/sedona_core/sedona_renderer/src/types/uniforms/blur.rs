use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct BlurUniforms {
    pub direction: Vec2,
    pub radius: f32,
    pub _padding0: u32,
}

impl Default for BlurUniforms {
    fn default() -> Self {
        Self {
            direction: Vec2::new(1.0, 0.0),
            radius: 0.9,
            _padding0: 0,
        }
    }
}
