use crate::render_resources::settings::RenderSettings;
use crate::utils::defaults::{DEFAULT_COLOR_TARGET_FORMAT, DEFAULT_DEPTH_TARGET_FORMAT};
use wgpu::wgt::SamplerDescriptor;
use wgpu::{
    AddressMode, CompareFunction, Device, Extent3d, FilterMode, Sampler, TextureDescriptor,
    TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
    TextureViewDimension,
};

pub struct RenderTargetBuffers {
    filter_sampler: Sampler,
    compare_sampler: Sampler,

    hdr_buffer_view_a: TextureView,
    hdr_buffer_view_b: TextureView,
    depth_buffer_view: TextureView,
    normal_buffer_view: TextureView,
    shadow_map_view: TextureView,
    shadow_map_cascade_views: Vec<TextureView>,
    luminance_mip_views: Vec<TextureView>,
    adaptation_buffer_view_a: TextureView,
    adaptation_buffer_view_b: TextureView,
    bloom_buffer_view_a: TextureView,
    bloom_buffer_view_b: TextureView,
}

impl RenderTargetBuffers {
    pub fn new(settings: &RenderSettings, device: &Device) -> Self {
        let width = settings.resolution_width;
        let height = settings.resolution_height;
        let shadow_map_resolution = settings.shadow_map_resolution;
        let shadow_map_cascade_count = settings.shadow_map_cascade_count;

        let filter_mode = if settings.auto_resolution {
            FilterMode::Linear
        } else {
            FilterMode::Nearest
        };

        let filter_sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("sampler_frame_buffer_filter"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: filter_mode,
            min_filter: filter_mode,
            mipmap_filter: filter_mode,
            lod_min_clamp: 0.0,
            lod_max_clamp: f32::MAX,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        });

        let compare_sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("sampler_frame_buffer_compare"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: filter_mode,
            min_filter: filter_mode,
            mipmap_filter: FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: Some(CompareFunction::Less),
            anisotropy_clamp: 1,
            border_color: None,
        });

        let [
            hdr_buffer_view_a,
            hdr_buffer_view_b,
            depth_buffer_view,
            normal_buffer_view,
            bloom_buffer_view_a,
            bloom_buffer_view_b,
        ] = Self::create_frame_buffer_views(width, height, device);
        let (shadow_map_view, shadow_map_cascade_views) =
            Self::create_shadow_map_views(shadow_map_resolution, shadow_map_cascade_count, device);
        let luminance_mip_views = Self::create_luminance_mip_views(width, height, device);

        let adaptation_texture_descriptor = TextureDescriptor {
            label: Some("frame_buffer_adaptation"),
            size: Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::R16Float,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_DST
                | TextureUsages::STORAGE_BINDING,
            view_formats: &[TextureFormat::R16Float],
        };

        let adaptation_buffer_texture_a = device.create_texture(&adaptation_texture_descriptor);
        let adaptation_buffer_view_a = adaptation_buffer_texture_a.create_view(&Default::default());

        let adaptation_buffer_texture_b = device.create_texture(&adaptation_texture_descriptor);
        let adaptation_buffer_view_b = adaptation_buffer_texture_b.create_view(&Default::default());

        Self {
            filter_sampler,
            compare_sampler,
            hdr_buffer_view_a,
            hdr_buffer_view_b,
            depth_buffer_view,
            normal_buffer_view,
            shadow_map_view,
            shadow_map_cascade_views,
            luminance_mip_views,
            adaptation_buffer_view_a,
            adaptation_buffer_view_b,
            bloom_buffer_view_a,
            bloom_buffer_view_b,
        }
    }

    pub fn create_frame_buffer_views(width: u32, height: u32, device: &Device) -> [TextureView; 6] {
        let color_texture_descriptor = TextureDescriptor {
            label: Some("frame_buffer_color"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: DEFAULT_COLOR_TARGET_FORMAT,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_DST,
            view_formats: &[
                DEFAULT_COLOR_TARGET_FORMAT,
                DEFAULT_COLOR_TARGET_FORMAT.add_srgb_suffix(),
            ],
        };

        let depth_texture_descriptor = TextureDescriptor {
            label: Some("frame_buffer_depth"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: DEFAULT_DEPTH_TARGET_FORMAT,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_DST,
            view_formats: &[DEFAULT_DEPTH_TARGET_FORMAT],
        };

        let back_buffer_texture_a = device.create_texture(&color_texture_descriptor);
        let back_buffer_view_a = back_buffer_texture_a.create_view(&Default::default());

        let back_buffer_texture_b = device.create_texture(&color_texture_descriptor);
        let back_buffer_view_b = back_buffer_texture_b.create_view(&Default::default());

        let depth_buffer_texture = device.create_texture(&depth_texture_descriptor);
        let depth_buffer_view = depth_buffer_texture.create_view(&Default::default());

        let normal_buffer_texture = device.create_texture(&color_texture_descriptor);
        let normal_buffer_view = normal_buffer_texture.create_view(&Default::default());

        let bloom_buffer_texture_a = device.create_texture(&color_texture_descriptor);
        let bloom_buffer_view_a = bloom_buffer_texture_a.create_view(&Default::default());

        let bloom_buffer_texture_b = device.create_texture(&color_texture_descriptor);
        let bloom_buffer_view_b = bloom_buffer_texture_b.create_view(&Default::default());

        [
            back_buffer_view_a,
            back_buffer_view_b,
            depth_buffer_view,
            normal_buffer_view,
            bloom_buffer_view_a,
            bloom_buffer_view_b,
        ]
    }

    pub fn create_shadow_map_views(
        resolution: u32,
        cascade_count: usize,
        device: &Device,
    ) -> (TextureView, Vec<TextureView>) {
        let shadow_map_texture_descriptor = TextureDescriptor {
            label: Some("frame_buffer_shadow_map"),
            size: Extent3d {
                width: resolution,
                height: resolution,
                depth_or_array_layers: cascade_count as u32,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: DEFAULT_DEPTH_TARGET_FORMAT,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_DST,
            view_formats: &[DEFAULT_DEPTH_TARGET_FORMAT],
        };

        let shadow_map_texture = device.create_texture(&shadow_map_texture_descriptor);
        let shadow_map_view = shadow_map_texture.create_view(&TextureViewDescriptor {
            dimension: Some(TextureViewDimension::D2Array),
            base_array_layer: 0,
            array_layer_count: Some(cascade_count as u32),
            ..Default::default()
        });

        let shadow_map_cascade_views: Vec<TextureView> = (0..cascade_count)
            .map(|cascade| {
                shadow_map_texture.create_view(&TextureViewDescriptor {
                    dimension: Some(TextureViewDimension::D2),
                    base_array_layer: cascade as u32,
                    array_layer_count: Some(1),
                    ..Default::default()
                })
            })
            .collect();

        (shadow_map_view, shadow_map_cascade_views)
    }

    pub fn create_luminance_mip_views(
        width: u32,
        height: u32,
        device: &Device,
    ) -> Vec<TextureView> {
        let mip_level_count = (width.max(height) as f32).log2().floor() as u32 + 1;

        let luminance_texture_descriptor = TextureDescriptor {
            label: Some("frame_buffer_luminance"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::R16Float,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_DST
                | TextureUsages::STORAGE_BINDING,
            view_formats: &[TextureFormat::R16Float],
        };

        let luminance_texture = device.create_texture(&luminance_texture_descriptor);

        (0..mip_level_count)
            .map(|mip| {
                luminance_texture.create_view(&TextureViewDescriptor {
                    base_mip_level: mip,
                    mip_level_count: Some(1),
                    ..Default::default()
                })
            })
            .collect::<Vec<_>>()
    }

    pub fn resize(&mut self, width: u32, height: u32, device: &Device) {
        let [
            hdr_buffer_view_a,
            hdr_buffer_view_b,
            depth_buffer_view,
            normal_buffer_view,
            bloom_buffer_view_a,
            bloom_buffer_view_b,
        ] = Self::create_frame_buffer_views(width, height, device);
        let luminance_mip_views = Self::create_luminance_mip_views(width, height, device);

        self.hdr_buffer_view_a = hdr_buffer_view_a;
        self.hdr_buffer_view_b = hdr_buffer_view_b;
        self.depth_buffer_view = depth_buffer_view;
        self.normal_buffer_view = normal_buffer_view;
        self.luminance_mip_views = luminance_mip_views;
        self.bloom_buffer_view_a = bloom_buffer_view_a;
        self.bloom_buffer_view_b = bloom_buffer_view_b;
    }

    pub fn filter_sampler(&self) -> &Sampler {
        &self.filter_sampler
    }

    pub fn compare_sampler(&self) -> &Sampler {
        &self.compare_sampler
    }

    pub fn hdr_buffer_view_a(&self) -> &TextureView {
        &self.hdr_buffer_view_a
    }

    pub fn hdr_buffer_view_b(&self) -> &TextureView {
        &self.hdr_buffer_view_b
    }

    pub fn depth_buffer_view(&self) -> &TextureView {
        &self.depth_buffer_view
    }

    pub fn normal_buffer_view(&self) -> &TextureView {
        &self.normal_buffer_view
    }

    pub fn shadow_map_view(&self) -> &TextureView {
        &self.shadow_map_view
    }

    pub fn shadow_map_cascade_views(&self) -> &[TextureView] {
        &self.shadow_map_cascade_views
    }

    pub fn get_shadow_map_cascade_view(&self, index: usize) -> &TextureView {
        &self.shadow_map_cascade_views[index]
    }

    pub fn luminance_mip_views(&self) -> &[TextureView] {
        &self.luminance_mip_views
    }

    pub fn get_luminance_mip_view(&self, index: usize) -> &TextureView {
        &self.luminance_mip_views[index]
    }

    pub fn adaptation_buffer_view_a(&self) -> &TextureView {
        &self.adaptation_buffer_view_a
    }

    pub fn adaptation_buffer_view_b(&self) -> &TextureView {
        &self.adaptation_buffer_view_b
    }

    pub fn bloom_buffer_view_a(&self) -> &TextureView {
        &self.bloom_buffer_view_a
    }

    pub fn bloom_buffer_view_b(&self) -> &TextureView {
        &self.bloom_buffer_view_b
    }
}
