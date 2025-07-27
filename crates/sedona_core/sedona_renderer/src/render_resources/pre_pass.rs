use crate::render_resources::layouts::BindGroupLayouts;
use crate::render_resources::shaders::Shaders;
use crate::types::{StaticVertex, Vertex};
use crate::utils::pipeline::PipelineBuilder;
use wgpu::{Device, Face, PipelineLayout, PipelineLayoutDescriptor, RenderPipeline};

pub struct PrePassRenderManager {
    pipeline_layout: PipelineLayout,
    pipeline: RenderPipeline,
}

impl PrePassRenderManager {
    pub fn new(layouts: &BindGroupLayouts, shaders: &Shaders, device: &Device) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("pre_pass"),
            bind_group_layouts: &[layouts.camera_view(), layouts.model()],
            push_constant_ranges: &[],
        });

        let pipeline =
            PipelineBuilder::new("pre_pass", &pipeline_layout, shaders.pre_pass(), device)
                .vertex_buffers(&[StaticVertex::descriptor()])
                .cull_mode(Some(Face::Back))
                .build();

        Self {
            pipeline_layout,
            pipeline,
        }
    }

    pub fn pipeline_layout(&self) -> &PipelineLayout {
        &self.pipeline_layout
    }

    pub fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }
}
