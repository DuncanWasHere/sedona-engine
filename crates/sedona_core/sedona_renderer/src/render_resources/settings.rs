use sedona_settings::*;
use wgpu::TextureFormat;

pub struct RenderSettings {
    pub surface_format: TextureFormat,
    pub filter_textures: bool,
    pub window_width: u32,
    pub window_height: u32,
    pub auto_resolution: bool,
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub shadow_map_cascade_count: usize,
    pub shadow_map_resolution: u32,
    pub sky_pbr: bool,
    pub initial_render_object_array_capacity: usize,
}

impl RenderSettings {
    pub fn new(
        window_width: u32,
        window_height: u32,
        surface_format: TextureFormat,
        config: &Settings,
    ) -> Self {
        let filter_textures = value_as(config.get(FILTER_TEXTURES)).unwrap_or(true);
        let auto_resolution = value_as(config.get(AUTO_RESOLUTION)).unwrap_or(true);

        let (resolution_width, resolution_height) = if auto_resolution {
            (window_width, window_height)
        } else {
            let width = value_as(config.get(RESOLUTION_WIDTH)).unwrap_or(window_width);
            let height = value_as(config.get(RESOLUTION_HEIGHT)).unwrap_or(window_height);
            (width, height)
        };

        let shadow_map_cascade_count = value_as(config.get(SHADOW_MAP_CASCADE_COUNT)).unwrap_or(3);
        let shadow_map_resolution = value_as(config.get(SHADOW_MAP_RESOLUTION)).unwrap_or(2048);

        let sky_pbr = value_as(config.get(SKY_PBR)).unwrap_or_default();

        let initial_render_object_array_capacity =
            value_as(config.get(INITIAL_RENDER_OBJECT_ARRAY_CAPACITY)).unwrap_or(1024);

        Self {
            surface_format,
            filter_textures,
            window_width,
            window_height,
            auto_resolution,
            resolution_width,
            resolution_height,
            shadow_map_cascade_count,
            shadow_map_resolution,
            sky_pbr,
            initial_render_object_array_capacity,
        }
    }
}
