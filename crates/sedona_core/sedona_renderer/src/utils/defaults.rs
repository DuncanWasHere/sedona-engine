use wgpu::{AddressMode, FilterMode, SamplerDescriptor, TextureFormat};

pub const CLOUD_LAYER_COUNT: usize = 4;

pub const DEFAULT_SAMPLE_COUNT: u32 = 1;

pub const DEFAULT_SAMPLER_DESC: SamplerDescriptor<'static> = SamplerDescriptor {
    label: Some("sampler_default"),
    address_mode_u: AddressMode::Repeat,
    address_mode_v: AddressMode::Repeat,
    address_mode_w: AddressMode::Repeat,
    mag_filter: FilterMode::Linear,
    min_filter: FilterMode::Linear,
    mipmap_filter: FilterMode::Linear,
    lod_min_clamp: 0.0,
    lod_max_clamp: f32::MAX,
    compare: None,
    anisotropy_clamp: 1,
    border_color: None,
};

pub const DEFAULT_COLOR_TARGET_FORMAT: TextureFormat = TextureFormat::Rgba16Float;
pub const DEFAULT_DEPTH_TARGET_FORMAT: TextureFormat = TextureFormat::Depth32Float;

pub const BASE_COLOR_TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba8UnormSrgb;
pub const BASE_COLOR_DEFAULT: [u8; 4] = [255, 255, 255, 255];

pub const METALLIC_ROUGHNESS_TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;
pub const METALLIC_ROUGHNESS_DEFAULT: [u8; 4] = [0, 0, 0, 255];

pub const NORMAL_TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;
pub const NORMAL_DEFAULT: [u8; 4] = [128, 128, 255, 255];

pub const EMISSIVE_TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba8UnormSrgb;
pub const EMISSIVE_DEFAULT: [u8; 4] = [0, 0, 0, 255];

pub const OCCLUSION_TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;
pub const OCCLUSION_DEFAULT: [u8; 4] = [255, 255, 255, 255];
