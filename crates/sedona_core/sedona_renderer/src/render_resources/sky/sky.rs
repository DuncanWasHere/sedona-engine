use crate::render_resources::layouts::BindGroupLayouts;
use crate::render_resources::settings::RenderSettings;
use crate::render_resources::shaders::Shaders;
use crate::render_resources::sky::bind_groups::SkyBindGroups;
use crate::render_resources::sky::buffers::SkyBuffers;
use crate::render_resources::sky::pipelines::SkyPipelines;
use crate::types::SkyMode;
use crate::types::uniforms::{CloudUniforms, MoonUniforms, SkyGradientUniforms, SkyObjectUniforms};
use crate::utils::{
    create_cube_map_from_dir, create_texture_2d_from_path, create_texture_array_from_paths,
};
use wgpu::{Device, Queue};

pub struct SkyRenderManager {
    pub sky_mode: SkyMode,
    pub buffers: SkyBuffers,
    pub bind_groups: SkyBindGroups,
    pub pipelines: SkyPipelines,
}

impl SkyRenderManager {
    pub fn new(
        settings: &RenderSettings,
        layouts: &BindGroupLayouts,
        shaders: &Shaders,
        device: &Device,
    ) -> Self {
        let buffers = SkyBuffers::new(settings, device);
        let bind_groups = SkyBindGroups::new(settings.sky_pbr, &buffers, layouts, device);
        let pipelines = SkyPipelines::new(layouts, shaders, device);

        let sky_mode = if settings.sky_pbr {
            SkyMode::Pbr
        } else {
            SkyMode::Gradient
        };

        Self {
            sky_mode,
            buffers,
            bind_groups,
            pipelines,
        }
    }

    pub fn set_moon_textures(
        &mut self,
        texture_paths: &[String],
        layouts: &BindGroupLayouts,
        device: &Device,
        queue: &Queue,
    ) {
        let (texture, texture_view) =
            create_texture_array_from_paths(texture_paths, device, queue).unwrap();

        self.buffers.moon_texture_array = Some(texture_view);
        self.bind_groups
            .update_moon_textures(&self.buffers, layouts, device);
    }

    pub fn set_star_map_texture(
        &mut self,
        texture_path: &str,
        layouts: &BindGroupLayouts,
        device: &Device,
        queue: &Queue,
    ) {
        let (_, texture_view) = create_texture_2d_from_path(texture_path, device, queue).unwrap();
        self.buffers.star_map_texture = Some(texture_view);
        self.bind_groups
            .update_star_map_texture(&self.buffers, layouts, device);
    }

    pub fn set_cloud_texture(
        &mut self,
        texture_path: &str,
        layouts: &BindGroupLayouts,
        device: &Device,
        queue: &Queue,
    ) {
        let (texture, texture_view) =
            create_texture_2d_from_path(texture_path, device, queue).unwrap();
        self.buffers.cloud_texture = Some(texture_view);
        self.bind_groups
            .update_cloud_texture(&self.buffers, layouts, device);
    }

    pub fn set_sky_texture(
        &mut self,
        path: &str,
        layouts: &BindGroupLayouts,
        device: &Device,
        queue: &Queue,
    ) {
        let (texture, texture_view) = create_cube_map_from_dir(path, device, queue).unwrap();
        self.buffers.sky_box_texture = Some(texture_view);

        if self.sky_mode == SkyMode::Texture {
            self.bind_groups
                .update_sky_box_texture(&self.buffers, layouts, device);
        }
    }

    pub fn set_sky_mode(&mut self, mode: SkyMode, layouts: &BindGroupLayouts, device: &Device) {
        if self
            .bind_groups
            .set_sky_mode(mode, &self.buffers, layouts, device)
            .is_ok()
        {
            self.sky_mode = mode;
        }
    }

    pub fn update_sky_gradient(&mut self, data: SkyGradientUniforms, queue: &Queue) {
        self.buffers.sky_gradient_ubo.set(data, queue);
    }

    pub fn update_sun(&mut self, data: SkyObjectUniforms, queue: &Queue) {
        self.buffers.sun_ubo.set(data, queue);
    }

    pub fn update_moon(&mut self, data: MoonUniforms, queue: &Queue) {
        self.buffers.moon_ubo.set(data, queue);
    }

    pub fn update_stars(&mut self, data: SkyObjectUniforms, queue: &Queue) {
        self.buffers.star_ubo.set(data, queue);
    }

    pub fn update_clouds(&mut self, data: CloudUniforms, queue: &Queue) {
        self.buffers.cloud_ubo.set(data, queue);
    }
}
