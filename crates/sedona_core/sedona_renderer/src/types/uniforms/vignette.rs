use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct VignetteUniforms {
    center: Vec2,
    radius: f32,
    softness: f32,
    color: Vec3,
    intensity: f32,
}

impl Default for VignetteUniforms {
    fn default() -> Self {
        Self {
            center: Vec2::new(0.5, 0.5),
            radius: 0.75,
            softness: 0.4,
            color: Vec3::ZERO,
            intensity: 1.0,
        }
    }
}
