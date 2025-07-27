use bytemuck::{Pod, Zeroable};
use glam::{Vec3, Vec4};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct ColorGradeUniforms {
    temperature: f32,
    tint: f32,
    contrast: f32,
    saturation: f32,
    shadows: Vec4,
    midtones: Vec4,
    highlights: Vec3,
    brightness: f32,
}

impl Default for ColorGradeUniforms {
    fn default() -> Self {
        Self {
            temperature: 0.0,
            tint: 0.0,
            contrast: 1.0,
            saturation: 1.0,
            shadows: Vec4::new(1.0, 1.0, 1.0, 1.0),
            midtones: Vec4::new(1.0, 1.0, 1.0, 1.0),
            highlights: Vec3::new(1.0, 1.0, 1.0),
            brightness: 1.0,
        }
    }
}
