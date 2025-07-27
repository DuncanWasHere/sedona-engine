use bytemuck::{Pod, Zeroable};
use glam::{Vec3, Vec4};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct MaterialUniforms {
    pub base_color_factor: Vec4,

    pub emissive_factor: Vec3,
    pub emissive_multiplier: f32,

    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub transmission_factor: f32,
    pub occlusion_strength: f32,

    pub alpha_multiplier: f32,
    pub alpha_cutoff: f32,
    pub normal_scale: f32,
    pub ior: f32,
}

impl Default for MaterialUniforms {
    fn default() -> Self {
        Self {
            base_color_factor: Vec4::ONE,
            emissive_factor: Vec3::ZERO,
            emissive_multiplier: 1.0,
            metallic_factor: 0.0,
            roughness_factor: 0.5,
            transmission_factor: 0.0,
            occlusion_strength: 1.0,
            alpha_multiplier: 1.0,
            alpha_cutoff: 0.5,
            normal_scale: 1.0,
            ior: 1.5,
        }
    }
}
