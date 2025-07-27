use crate::render_resources::settings::RenderSettings;
use crate::types::uniform_buffer_object::UniformBufferObject;
use crate::types::uniforms::{
    AdaptationUniforms, BloomUniforms, BlurUniforms, ColorGradeUniforms, DepthOfFieldUniforms,
    LensFlareUniforms, ScreenUniforms, SsaoUniforms, SsrUniforms, SssUniforms, ToneMapUniforms,
    VignetteUniforms,
};
use wgpu::Device;

pub struct PostProcessBuffers {
    pub adaptation_ubo: UniformBufferObject<AdaptationUniforms>,
    pub bloom_ubo: UniformBufferObject<BloomUniforms>,
    pub blur_ubo: UniformBufferObject<BlurUniforms>,
    pub color_grade_ubo: UniformBufferObject<ColorGradeUniforms>,
    pub depth_of_field_ubo: UniformBufferObject<DepthOfFieldUniforms>,
    pub lens_flare_ubo: UniformBufferObject<LensFlareUniforms>,
    pub screen_ubo: UniformBufferObject<ScreenUniforms>,
    pub ssao_ubo: UniformBufferObject<SsaoUniforms>,
    pub ssr_ubo: UniformBufferObject<SsrUniforms>,
    pub sss_ubo: UniformBufferObject<SssUniforms>,
    pub tone_map_ubo: UniformBufferObject<ToneMapUniforms>,
    pub vignette_ubo: UniformBufferObject<VignetteUniforms>,
}

impl PostProcessBuffers {
    pub fn new(settings: &RenderSettings, device: &Device) -> Self {
        let screen_data = ScreenUniforms::new(
            settings.resolution_width as f32,
            settings.resolution_height as f32,
        );

        let adaptation_ubo = UniformBufferObject::with_default_data(device);
        let bloom_ubo = UniformBufferObject::with_default_data(device);
        let blur_ubo = UniformBufferObject::with_default_data(device);
        let color_grade_ubo = UniformBufferObject::with_default_data(device);
        let depth_of_field_ubo = UniformBufferObject::with_default_data(device);
        let lens_flare_ubo = UniformBufferObject::with_default_data(device);
        let screen_ubo = UniformBufferObject::with_data(screen_data, device);
        let ssao_ubo = UniformBufferObject::with_default_data(device);
        let ssr_ubo = UniformBufferObject::with_default_data(device);
        let sss_ubo = UniformBufferObject::with_default_data(device);
        let tone_map_ubo = UniformBufferObject::with_default_data(device);
        let vignette_ubo = UniformBufferObject::with_default_data(device);

        Self {
            adaptation_ubo,
            bloom_ubo,
            blur_ubo,
            color_grade_ubo,
            depth_of_field_ubo,
            lens_flare_ubo,
            screen_ubo,
            ssao_ubo,
            ssr_ubo,
            sss_ubo,
            tone_map_ubo,
            vignette_ubo,
        }
    }
}
