use crate::render_resources::render_resources::RenderResources;
use crate::types::render_object::RenderObject;
use wgpu::{
    CommandEncoder, IndexFormat, LoadOp, Operations, RenderPassDepthStencilAttachment,
    RenderPassDescriptor, StoreOp,
};

pub fn render_shadow_pass(
    objects: &[&RenderObject],
    encoder: &mut CommandEncoder,
    resources: &RenderResources,
) {
    let shadow_render_manager = &resources.shadow;

    for i in 0..resources.settings.shadow_map_cascade_count {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("shadow_pass"),
            color_attachments: &[],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: resources.targets.buffers.get_shadow_map_cascade_view(i),
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });

        render_pass.set_pipeline(shadow_render_manager.pipeline());

        let light_view_bind_group = shadow_render_manager.get_light_view_cascade_bind_group(i);
        render_pass.set_bind_group(0, light_view_bind_group, &[]);

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
}
