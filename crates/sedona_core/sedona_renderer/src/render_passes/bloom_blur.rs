use crate::render_resources::render_resources::RenderResources;
use wgpu::{
    Color, CommandEncoder, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    StoreOp,
};

pub fn render_bloom_blur_pass(encoder: &mut CommandEncoder, resources: &RenderResources) {
    let (target_view, bloom_buffer_bind_group) = resources.targets.begin_bloom_pass();

    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("bloom_blur_pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: target_view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color::TRANSPARENT),
                store: StoreOp::Store,
            },
            depth_slice: None,
        })],
        depth_stencil_attachment: None,
        ..Default::default()
    });

    render_pass.set_pipeline(resources.post_process.pipelines.blur_pipeline());

    render_pass.set_bind_group(0, resources.post_process.bind_groups.blur_bind_group(), &[]);
    render_pass.set_bind_group(1, bloom_buffer_bind_group, &[]);
    render_pass.draw(0..3, 0..1);
}
