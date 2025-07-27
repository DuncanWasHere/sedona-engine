use crate::render_resources::settings::RenderSettings;
use crate::types::uniform_buffer_object::UniformBufferObject;
use crate::types::uniforms::{
    CloudUniforms, MoonUniforms, SkyGradientUniforms, SkyObjectUniforms, SkyPbrUniforms,
};
use wgpu::{AddressMode, Device, FilterMode, Sampler, TextureView};

pub struct SkyBuffers {
    pub sky_gradient_ubo: UniformBufferObject<SkyGradientUniforms>,
    pub sky_pbr_ubo: UniformBufferObject<SkyPbrUniforms>,
    pub sun_ubo: UniformBufferObject<SkyObjectUniforms>,
    pub moon_ubo: UniformBufferObject<MoonUniforms>,
    pub star_ubo: UniformBufferObject<SkyObjectUniforms>,
    pub cloud_ubo: UniformBufferObject<CloudUniforms>,

    pub clamp_sampler: Sampler,
    pub tile_sampler: Sampler,

    pub sky_box_texture: Option<TextureView>,
    pub sun_disc_texture: Option<TextureView>,
    pub moon_texture_array: Option<TextureView>,
    pub star_map_texture: Option<TextureView>,
    pub cloud_texture: Option<TextureView>,
}

impl SkyBuffers {
    pub fn new(settings: &RenderSettings, device: &Device) -> Self {
        let sky_gradient_ubo = UniformBufferObject::with_default_data(device);
        let sky_pbr_ubo = UniformBufferObject::with_default_data(device);
        let sun_ubo = UniformBufferObject::with_default_data(device);
        let moon_ubo = UniformBufferObject::with_default_data(device);
        let star_ubo = UniformBufferObject::with_default_data(device);
        let cloud_ubo = UniformBufferObject::with_default_data(device);

        let filter_mode = if settings.filter_textures {
            FilterMode::Linear
        } else {
            FilterMode::Nearest
        };

        let clamp_sampler = device.create_sampler(&wgpu::wgt::SamplerDescriptor {
            label: Some("sampler_sky_clamped"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            min_filter: filter_mode,
            mag_filter: filter_mode,
            mipmap_filter: filter_mode,
            ..Default::default()
        });

        let tile_sampler = device.create_sampler(&wgpu::wgt::SamplerDescriptor {
            label: Some("sampler_sky_tiled"),
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            min_filter: filter_mode,
            mag_filter: filter_mode,
            mipmap_filter: filter_mode,
            ..Default::default()
        });

        Self {
            sky_gradient_ubo,
            sky_pbr_ubo,
            sun_ubo,
            moon_ubo,
            star_ubo,
            cloud_ubo,
            clamp_sampler,
            tile_sampler,
            sky_box_texture: None,
            sun_disc_texture: None,
            moon_texture_array: None,
            star_map_texture: None,
            cloud_texture: None,
        }
    }
}
