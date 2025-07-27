use crate::utils::shader::create_shader_from_path;
use sedona_settings::*;
use wgpu::{Device, ShaderModule};

pub struct Shaders {
    adaptation: ShaderModule,
    blit: ShaderModule,
    blit_depth: ShaderModule,
    bloom_composite: ShaderModule,
    bloom_extract: ShaderModule,
    blur: ShaderModule,
    cloud: ShaderModule,
    // depth_of_field: ShaderModule,
    // lens_flare: ShaderModule,
    luminance_downsample: ShaderModule,
    moon: ShaderModule,
    pre_pass: ShaderModule,
    // screen_space_ao: ShaderModule,
    // screen_space_reflection: ShaderModule,
    // screen_space_shadow: ShaderModule,
    shadow: ShaderModule,
    sky_box: ShaderModule,
    sky_gradient: ShaderModule,
    sky_pbr: ShaderModule,
    star_map: ShaderModule,
    sun: ShaderModule,
    tone_map: ShaderModule,
}

macro_rules! load_shader {
    ($config:expr, $key:ident, $default:ident, $shader_name:expr, $device:expr) => {{
        let path = match $config.get($key) {
            Some(SettingsValue::String(value)) => value,
            _ => $default,
        };
        create_shader_from_path($shader_name, path, $device)
    }};
}

impl Shaders {
    pub fn new(config: &Settings, device: &Device) -> Self {
        let adaptation = load_shader!(
            config,
            ADAPTATION_SHADER_PATH,
            DEFAULT_ADAPTATION_SHADER_PATH,
            "adaptation",
            device
        );
        let blit = load_shader!(
            config,
            BLIT_SHADER_PATH,
            DEFAULT_BLIT_SHADER_PATH,
            "blit",
            device
        );
        let blit_depth = load_shader!(
            config,
            BLIT_DEPTH_SHADER_PATH,
            DEFAULT_BLIT_DEPTH_SHADER_PATH,
            "blit_depth",
            device
        );
        let bloom_composite = load_shader!(
            config,
            BLOOM_COMPOSITE_SHADER_PATH,
            DEFAULT_BLOOM_COMPOSITE_SHADER_PATH,
            "bloom_composite",
            device
        );
        let bloom_extract = load_shader!(
            config,
            BLOOM_EXTRACT_SHADER_PATH,
            DEFAULT_BLOOM_EXTRACT_SHADER_PATH,
            "bloom_extract",
            device
        );
        let blur = load_shader!(
            config,
            BLUR_SHADER_PATH,
            DEFAULT_BLUR_SHADER_PATH,
            "blur",
            device
        );
        let cloud = load_shader!(
            config,
            CLOUD_SHADER_PATH,
            DEFAULT_CLOUD_SHADER_PATH,
            "cloud",
            device
        );
        // let depth_of_field = load_shader!(config, DEPTH_OF_FIELD_SHADER_PATH, DEFAULT_DEPTH_OF_FIELD_SHADER_PATH, "depth_of_field", device);
        // let lens_flare = load_shader!(config, LENS_FLARE_SHADER_PATH, DEFAULT_LENS_FLARE_SHADER_PATH, "lens_flare", device);
        let luminance_downsample = load_shader!(
            config,
            LUMINANCE_DOWNSAMPLE_SHADER_PATH,
            DEFAULT_LUMINANCE_DOWNSAMPLE_SHADER_PATH,
            "luminance_downsample",
            device
        );
        let moon = load_shader!(
            config,
            MOON_SHADER_PATH,
            DEFAULT_MOON_SHADER_PATH,
            "moon",
            device
        );
        let pre_pass = load_shader!(
            config,
            PRE_PASS_SHADER_PATH,
            DEFAULT_PRE_PASS_SHADER_PATH,
            "pre_pass",
            device
        );
        // let screen_space_ao = load_shader!(config, SCREEN_SPACE_AO_SHADER_PATH, DEFAULT_SCREEN_SPACE_AO_SHADER_PATH, "screen_space_ao", device);
        // let screen_space_reflection = load_shader!(config, SCREEN_SPACE_REFLECTION_SHADER_PATH, DEFAULT_SCREEN_SPACE_REFLECTION_SHADER_PATH, "screen_space_reflection", device);
        // let screen_space_shadow = load_shader!(config, SCREEN_SPACE_SHADOW_SHADER_PATH, DEFAULT_SCREEN_SPACE_SHADOW_SHADER_PATH, "screen_space_shadow", device);
        let shadow = load_shader!(
            config,
            SHADOW_SHADER_PATH,
            DEFAULT_SHADOW_SHADER_PATH,
            "shadow",
            device
        );
        let sky_box = load_shader!(
            config,
            SKY_BOX_SHADER_PATH,
            DEFAULT_SKY_BOX_SHADER_PATH,
            "sky_box",
            device
        );
        let sky_gradient = load_shader!(
            config,
            SKY_GRADIENT_SHADER_PATH,
            DEFAULT_SKY_GRADIENT_SHADER_PATH,
            "sky_gradient",
            device
        );
        let sky_pbr = load_shader!(
            config,
            SKY_PBR_SHADER_PATH,
            DEFAULT_SKY_PBR_SHADER_PATH,
            "sky_pbr",
            device
        );
        let star_map = load_shader!(
            config,
            STAR_MAP_SHADER_PATH,
            DEFAULT_STAR_MAP_SHADER_PATH,
            "star_map",
            device
        );
        let sun = load_shader!(
            config,
            SUN_SHADER_PATH,
            DEFAULT_SUN_SHADER_PATH,
            "sun",
            device
        );
        let tone_map = load_shader!(
            config,
            TONE_MAP_SHADER_PATH,
            DEFAULT_TONE_MAP_SHADER_PATH,
            "tone_map",
            device
        );

        Self {
            adaptation,
            blit,
            blit_depth,
            bloom_composite,
            bloom_extract,
            blur,
            cloud,
            // depth_of_field,
            // lens_flare,
            luminance_downsample,
            moon,
            pre_pass,
            // screen_space_ao,
            // screen_space_reflection,
            // screen_space_shadow,
            shadow,
            sky_box,
            sky_gradient,
            sky_pbr,
            star_map,
            sun,
            tone_map,
        }
    }

    pub fn adaptation(&self) -> &ShaderModule {
        &self.adaptation
    }

    pub fn blit(&self) -> &ShaderModule {
        &self.blit
    }

    pub fn blit_depth(&self) -> &ShaderModule {
        &self.blit_depth
    }

    pub fn bloom_composite(&self) -> &ShaderModule {
        &self.bloom_composite
    }

    pub fn bloom_extract(&self) -> &ShaderModule {
        &self.bloom_extract
    }

    pub fn blur(&self) -> &ShaderModule {
        &self.blur
    }

    pub fn cloud(&self) -> &ShaderModule {
        &self.cloud
    }

    pub fn luminance_downsample(&self) -> &ShaderModule {
        &self.luminance_downsample
    }

    pub fn moon(&self) -> &ShaderModule {
        &self.moon
    }

    pub fn pre_pass(&self) -> &ShaderModule {
        &self.pre_pass
    }

    pub fn shadow(&self) -> &ShaderModule {
        &self.shadow
    }

    pub fn sky_box(&self) -> &ShaderModule {
        &self.sky_box
    }

    pub fn sky_gradient(&self) -> &ShaderModule {
        &self.sky_gradient
    }

    pub fn sky_pbr(&self) -> &ShaderModule {
        &self.sky_pbr
    }

    pub fn star_map(&self) -> &ShaderModule {
        &self.star_map
    }

    pub fn sun(&self) -> &ShaderModule {
        &self.sun
    }

    pub fn tone_map(&self) -> &ShaderModule {
        &self.tone_map
    }
}

pub const DEFAULT_ADAPTATION_SHADER_PATH: &str =
    "assets/shaders/default/post_process/adaptation.wgsl";
pub const DEFAULT_BLIT_SHADER_PATH: &str = "assets/shaders/default/blit/blit.wgsl";
pub const DEFAULT_BLIT_DEPTH_SHADER_PATH: &str = "assets/shaders/default/blit/blit_depth.wgsl";
pub const DEFAULT_BLOOM_COMPOSITE_SHADER_PATH: &str =
    "assets/shaders/default/post_process/bloom_composite.wgsl";
pub const DEFAULT_BLOOM_EXTRACT_SHADER_PATH: &str =
    "assets/shaders/default/post_process/bloom_extract.wgsl";
pub const DEFAULT_BLUR_SHADER_PATH: &str = "assets/shaders/default/post_process/blur.wgsl";
pub const DEFAULT_CLOUD_SHADER_PATH: &str = "assets/shaders/default/sky/cloud.wgsl";
pub const DEFAULT_DEPTH_OF_FIELD_SHADER_PATH: &str =
    "assets/shaders/default/post_process/depth_of_field.wgsl";
pub const DEFAULT_LENS_FLARE_SHADER_PATH: &str =
    "assets/shaders/default/post_process/lens_flare.wgsl";
pub const DEFAULT_LUMINANCE_DOWNSAMPLE_SHADER_PATH: &str =
    "assets/shaders/default/post_process/luminance_downsample.wgsl";
pub const DEFAULT_MOON_SHADER_PATH: &str = "assets/shaders/default/sky/moon.wgsl";
pub const DEFAULT_PRE_PASS_SHADER_PATH: &str = "assets/shaders/default/pre_pass.wgsl";
pub const DEFAULT_SCREEN_SPACE_AO_SHADER_PATH: &str =
    "assets/shaders/default/post_process/screen_space_ao.wgsl";
pub const DEFAULT_SCREEN_SPACE_REFLECTION_SHADER_PATH: &str =
    "assets/shaders/default/post_process/screen_space_reflection.wgsl";
pub const DEFAULT_SCREEN_SPACE_SHADOW_SHADER_PATH: &str =
    "assets/shaders/default/post_process/screen_space_shadow.wgsl";
pub const DEFAULT_SHADOW_SHADER_PATH: &str = "assets/shaders/default/shadow.wgsl";
pub const DEFAULT_SKY_BOX_SHADER_PATH: &str = "assets/shaders/default/sky/sky_box.wgsl";
pub const DEFAULT_SKY_GRADIENT_SHADER_PATH: &str = "assets/shaders/default/sky/sky_gradient.wgsl";
pub const DEFAULT_SKY_PBR_SHADER_PATH: &str = "assets/shaders/default/sky/sky_pbr.wgsl";
pub const DEFAULT_STAR_MAP_SHADER_PATH: &str = "assets/shaders/default/sky/star_map.wgsl";
pub const DEFAULT_SUN_SHADER_PATH: &str = "assets/shaders/default/sky/sun.wgsl";
pub const DEFAULT_TONE_MAP_SHADER_PATH: &str = "assets/shaders/default/post_process/tone_map.wgsl";
