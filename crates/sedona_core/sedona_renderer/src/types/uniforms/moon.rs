use bytemuck::{Pod, Zeroable};
use glam::{Quat, Vec4};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct MoonUniforms {
    pub tint: Vec4,
    pub rotation: Quat,
    pub size: f32,
    pub phase_index: u32,
    pub _padding1: u32,
    pub _padding2: u32,
}

impl Default for MoonUniforms {
    fn default() -> Self {
        Self {
            tint: Vec4::new(1.0, 1.0, 1.0, 1.0),
            rotation: Quat::IDENTITY,
            size: 1.0,
            phase_index: 0,
            _padding1: 0,
            _padding2: 0,
        }
    }
}
