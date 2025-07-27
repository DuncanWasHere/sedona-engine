use bytemuck::{Pod, Zeroable};
use glam::Vec4;
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct CloudUniforms {
    pub tint: Vec4,
    pub u_offset: f32,
    pub v_offset: f32,
    pub speed: f32,
    pub _padding1: u32,
    pub _padding2: [u32; 4],
}

impl CloudUniforms {
    pub fn new(tint: Vec4, u_offset: f32, v_offset: f32) -> Self {
        Self {
            tint,
            u_offset,
            v_offset,
            ..Default::default()
        }
    }
}

impl Default for CloudUniforms {
    fn default() -> Self {
        Self {
            tint: Vec4::new(1.0, 1.0, 1.0, 1.0),
            u_offset: 0.0,
            v_offset: 0.0,
            speed: 0.0,
            _padding1: 0,
            _padding2: [0; 4],
        }
    }
}
