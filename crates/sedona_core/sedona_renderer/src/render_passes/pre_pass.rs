use crate::render_resources::render_resources::RenderResources;
use crate::types::render_object::RenderObject;
use wgpu::{
    Color, CommandEncoder, IndexFormat, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp,
};

pub fn render_pre_pass(
    objects: &[&RenderObject],
    encoder: &mut CommandEncoder,
    resources: &RenderResources,
) {
    let pre_pass_render_manager = &resources.pre_pass;

    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("pre_pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: resources.targets.buffers.normal_buffer_view(),
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color::TRANSPARENT),
                store: StoreOp::Store,
            },
            depth_slice: None,
        })],
        depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
            view: resources.targets.buffers.depth_buffer_view(),
            depth_ops: Some(Operations {
                load: LoadOp::Clear(1.0),
                store: StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        ..Default::default()
    });

    let pipeline = pre_pass_render_manager.pipeline();
    render_pass.set_pipeline(pipeline);

    render_pass.set_bind_group(
        0,
        resources.globals.bind_groups.camera_view_bind_group(),
        &[],
    );

    for object in objects {
        let node = resources.objects.get_node(object.node).unwrap();
        let model_bind_group = &node.bind_group;
        render_pass.set_bind_group(1, model_bind_group, &[]);

        let vertex_buffer = &object.vbo.vertex_buffer;
        let index_buffer = &object.vbo.index_buffer;
        let index_count = object.vbo.index_count;

        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);
        render_pass.draw_indexed(0..index_count, 0, 0..1);
    }
}
