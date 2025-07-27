use bytemuck::{Pod, Zeroable};
use glam::{Vec3, Vec4};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct SkyGradientUniforms {
    pub horizon_color: Vec4,
    pub sky_lower_color: Vec4,
    pub sky_upper_color: Vec4,
    pub sun_direction: Vec3,
    pub turbidity: f32,
    pub mie: Vec3,
    pub mie_anisotropy: f32,
}

impl Default for SkyGradientUniforms {
    fn default() -> Self {
        Self {
            horizon_color: glam::vec4(0.529, 0.808, 0.922, 1.0),
            sky_lower_color: glam::vec4(0.38, 0.678, 0.89, 1.0),
            sky_upper_color: glam::vec4(0.2, 0.4, 0.8, 1.0),
            sun_direction: Vec3::NEG_Z,
            turbidity: 2.0,
            mie: glam::vec3(1.0, 1.0, 1.0),
            mie_anisotropy: 0.78,
        }
    }
}
