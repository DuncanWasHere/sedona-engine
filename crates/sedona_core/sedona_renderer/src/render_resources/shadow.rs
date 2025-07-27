use crate::render_resources::layouts::BindGroupLayouts;
use crate::render_resources::settings::RenderSettings;
use crate::render_resources::shaders::Shaders;
use crate::types::uniforms::LightViewUniforms;
use crate::types::{StaticVertex, Vertex};
use crate::utils::pipeline::PipelineBuilder;
use bytemuck::bytes_of;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, Buffer, BufferDescriptor, BufferUsages,
    CompareFunction, DepthBiasState, Device, Face, PipelineLayout, PipelineLayoutDescriptor, Queue,
    RenderPipeline,
};

pub struct ShadowRenderManager {
    light_view_cascade_buffers: Vec<Buffer>,
    light_view_cascade_bind_groups: Vec<BindGroup>,
    pipeline_layout: PipelineLayout,
    pipeline: RenderPipeline,
}

impl ShadowRenderManager {
    pub fn new(
        settings: &RenderSettings,
        layouts: &BindGroupLayouts,
        shaders: &Shaders,
        device: &Device,
    ) -> Self {
        let mut light_view_cascade_buffers = Vec::with_capacity(settings.shadow_map_cascade_count);
        let mut light_view_cascade_bind_groups =
            Vec::with_capacity(settings.shadow_map_cascade_count);

        for i in 0..settings.shadow_map_cascade_count {
            let light_view_cascade_buffer = device.create_buffer(&BufferDescriptor {
                label: Some("light_view_cascade"),
                size: 64,
                mapped_at_creation: false,
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            });

            let light_view_cascade_bind_group = device.create_bind_group(&BindGroupDescriptor {
                label: Some("light_view_cascade"),
                layout: layouts.light_view(),
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: light_view_cascade_buffer.as_entire_binding(),
                }],
            });

            light_view_cascade_buffers.push(light_view_cascade_buffer);
            light_view_cascade_bind_groups.push(light_view_cascade_bind_group);
        }

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("shadow"),
            bind_group_layouts: &[layouts.light_view(), layouts.model()],
            push_constant_ranges: &[],
        });

        let pipeline = PipelineBuilder::new("shadow", &pipeline_layout, shaders.shadow(), device)
            .vertex_buffers(&[StaticVertex::descriptor()])
            .cull_mode(Some(Face::Back))
            .no_fragment()
            .depth_bias_state(DepthBiasState {
                constant: 4,
                slope_scale: 8.0,
                clamp: 2.0,
            })
            .depth_compare(CompareFunction::LessEqual)
            .build();

        Self {
            light_view_cascade_buffers,
            light_view_cascade_bind_groups,
            pipeline,
            pipeline_layout,
        }
    }

    pub fn update_light_cascade_buffers(&mut self, data: LightViewUniforms, queue: &Queue) {
        for (i, matrix) in data.light_view_projections.iter().enumerate() {
            queue.write_buffer(&self.light_view_cascade_buffers[i], 0, bytes_of(matrix));
        }
    }

    pub fn get_light_view_cascade_bind_group(&self, index: usize) -> &BindGroup {
        &self.light_view_cascade_bind_groups[index]
    }

    pub fn pipeline_layout(&self) -> &PipelineLayout {
        &self.pipeline_layout
    }

    pub fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }
}
