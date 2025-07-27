use bytemuck::{Pod, Zeroable};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct ScreenUniforms {
    pub width: f32,
    pub height: f32,
    pub _padding0: u32,
    pub _padding1: u32,
}

impl ScreenUniforms {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            _padding0: 0,
            _padding1: 0,
        }
    }
}
