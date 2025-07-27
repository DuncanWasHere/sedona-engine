use bytemuck::{Pod, Zeroable};
use sedona_renderer_macros::Uniform;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Uniform, Zeroable)]
pub struct LensFlareUniforms {
    pub intensity: f32,
    pub ghost_count: u32,
    pub halo_size: f32,
    pub chromatic_offset: f32,
}

impl Default for LensFlareUniforms {
    fn default() -> Self {
        Self {
            intensity: 0.5,
            ghost_count: 2,
            halo_size: 0.25,
            chromatic_offset: 0.01,
        }
    }
}
