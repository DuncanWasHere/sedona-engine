use crate::render_resources::layouts::BindGroupLayouts;
use crate::render_resources::post_process::bind_groups::PostProcessBindGroups;
use crate::render_resources::post_process::buffers::PostProcessBuffers;
use crate::render_resources::post_process::pipelines::PostProcessPipelines;
use crate::render_resources::settings::RenderSettings;
use crate::render_resources::shaders::Shaders;
use wgpu::{Device, Queue};

pub struct PostProcessRenderManager {
    pub buffers: PostProcessBuffers,
    pub bind_groups: PostProcessBindGroups,
    pub pipelines: PostProcessPipelines,
}

impl PostProcessRenderManager {
    pub fn new(
        settings: &RenderSettings,
        layouts: &BindGroupLayouts,
        shaders: &Shaders,
        device: &Device,
    ) -> Self {
        let buffers = PostProcessBuffers::new(settings, device);
        let bind_groups = PostProcessBindGroups::new(&buffers, layouts, device);
        let pipelines = PostProcessPipelines::new(settings, layouts, shaders, device);

        Self {
            buffers,
            bind_groups,
            pipelines,
        }
    }

    pub fn update(&mut self, dt: f32, queue: &Queue) {
        self.buffers.adaptation_ubo.write_field("dt", &dt, queue);
    }

    pub fn resize(&mut self, width: u32, height: u32, queue: &Queue) {
        let screen_data = self.buffers.screen_ubo.data_mut();
        screen_data.width = width as f32;
        screen_data.height = height as f32;
        self.buffers.screen_ubo.write(queue);
    }
}
