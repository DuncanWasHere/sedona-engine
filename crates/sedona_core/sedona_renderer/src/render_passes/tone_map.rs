use crate::render_resources::render_resources::RenderResources;
use wgpu::{
    Color, CommandEncoder, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    StoreOp, TextureView,
};

pub fn render_tone_map_pass(
    encoder: &mut CommandEncoder,
    surface_view: &TextureView,
    resources: &RenderResources,
) {
    // HDR Buffer View + Avg Luminance -> Adaptation, Tone Map, Color Grade, Vignette -> SDR Surface View
    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("tone_map_pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: surface_view,
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

    render_pass.set_pipeline(resources.post_process.pipelines.tone_map_pipeline());

    render_pass.set_bind_group(
        0,
        resources.post_process.bind_groups.tone_map_bind_group(),
        &[],
    );
    render_pass.set_bind_group(1, resources.targets.hdr_buffer_bind_group(), &[]);
    render_pass.set_bind_group(2, resources.targets.adaptation_buffer_bind_group(), &[]);
    render_pass.draw(0..3, 0..1);
}
