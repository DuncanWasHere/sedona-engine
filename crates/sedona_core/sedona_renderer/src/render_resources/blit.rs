use crate::render_resources::layouts::BindGroupLayouts;
use crate::render_resources::shaders::Shaders;
use crate::utils::pipeline::PipelineBuilder;
use wgpu::{
    BindGroup, Color, CommandEncoder, Device, LoadOp, Operations, PipelineLayout,
    PipelineLayoutDescriptor, RenderPassDescriptor, RenderPipeline, ShaderModule, StoreOp,
    TextureFormat, TextureView,
};

pub struct BlitRenderManager {
    pipeline_layout: PipelineLayout,
    bgra8_unorm_srgb_pipeline: RenderPipeline,
    rgba8_unorm_srgb_pipeline: RenderPipeline,
    rgba16_float_pipeline: RenderPipeline,
    r16_float_pipeline: RenderPipeline,
}

impl BlitRenderManager {
    pub fn new(layouts: &BindGroupLayouts, shaders: &Shaders, device: &Device) -> Self {
        let shader = shaders.blit();

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("blit"),
            bind_group_layouts: &[layouts.blit()],
            push_constant_ranges: &[],
        });

        let bgra8_unorm_srgb_pipeline = Self::create_blit_pipeline(
            &pipeline_layout,
            shader,
            TextureFormat::Bgra8UnormSrgb,
            "fs_rgba",
            device,
        );

        let rgba8_unorm_srgb_pipeline = Self::create_blit_pipeline(
            &pipeline_layout,
            shader,
            TextureFormat::Rgba8UnormSrgb,
            "fs_rgba",
            device,
        );

        let rgba16_float_pipeline = Self::create_blit_pipeline(
            &pipeline_layout,
            shader,
            TextureFormat::Rgba16Float,
            "fs_rgba",
            device,
        );

        let r16_float_pipeline = Self::create_blit_pipeline(
            &pipeline_layout,
            shader,
            TextureFormat::R16Float,
            "fs_luminance",
            device,
        );

        Self {
            pipeline_layout,
            bgra8_unorm_srgb_pipeline,
            rgba8_unorm_srgb_pipeline,
            rgba16_float_pipeline,
            r16_float_pipeline,
        }
    }

    pub fn blit_to_texture_from_bind_group(
        &self,
        source_texture_bind_group: &BindGroup,
        destination_texture_view: &TextureView,
        destination_format: TextureFormat,
        encoder: &mut CommandEncoder,
    ) {
        let pipeline = match destination_format {
            TextureFormat::Rgba8UnormSrgb => &self.rgba8_unorm_srgb_pipeline,
            TextureFormat::Rgba16Float => &self.rgba16_float_pipeline,
            TextureFormat::R16Float => &self.r16_float_pipeline,
            _ => {
                eprintln!(
                    "Blit pipeline not implemented for destination texture format: {:?}",
                    destination_format
                );
                return;
            }
        };

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("render_pass_blit"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: destination_texture_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::WHITE),
                    store: StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, source_texture_bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }

    fn create_blit_pipeline(
        layout: &PipelineLayout,
        shader: &ShaderModule,
        format: TextureFormat,
        entry_point: &str,
        device: &Device,
    ) -> RenderPipeline {
        PipelineBuilder::new("blit", layout, shader, device)
            .target_format(format)
            .fragment_entry_point(entry_point)
            .no_depth()
            .build()
    }
}
