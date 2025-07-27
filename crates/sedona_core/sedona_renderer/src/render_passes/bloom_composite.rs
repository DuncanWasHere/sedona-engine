use crate::render_resources::render_resources::RenderResources;
use wgpu::{
    Color, CommandEncoder, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    StoreOp,
};

pub fn render_bloom_composite_pass(encoder: &mut CommandEncoder, resources: &RenderResources) {
    let (target_view, hdr_buffer_bind_group) = resources.targets.begin_post_process_pass();
    let bloom_buffer_bind_group = resources.targets.bloom_buffer_bind_group();

    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("bloom_composite_pass"),
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

    render_pass.set_pipeline(resources.post_process.pipelines.bloom_composite_pipeline());

    render_pass.set_bind_group(
        0,
        resources.post_process.bind_groups.lens_flare_bind_group(),
        &[],
    );
    render_pass.set_bind_group(1, hdr_buffer_bind_group, &[]);
    render_pass.set_bind_group(2, bloom_buffer_bind_group, &[]);
    render_pass.draw(0..3, 0..1);
}
