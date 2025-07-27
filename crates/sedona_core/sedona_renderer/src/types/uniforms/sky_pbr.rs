use bytemuck::{Pod, Zeroable};
use glam::{Vec3, Vec4};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct SkyPbrUniforms {
    pub rayleigh: Vec4,
    pub mie: Vec4,
    pub sun_direction: Vec4,
    pub mie_anisotropy: f32,
    pub turbidity: f32,
    pub rayleigh_factor: f32,
    pub mie_factor: f32,
}

impl Default for SkyPbrUniforms {
    fn default() -> Self {
        Self {
            rayleigh: glam::vec4(0.65, 0.85, 1.0, 1.0),
            mie: glam::vec4(1.0, 0.95, 0.85, 0.15),
            sun_direction: Vec3::NEG_Y.extend(0.0),
            mie_anisotropy: 0.76,
            turbidity: 2.0,
            rayleigh_factor: 1.0,
            mie_factor: 1.0,
        }
    }
}
