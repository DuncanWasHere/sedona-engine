use crate::render_resources::render_resources::RenderResources;
use wgpu::*;

pub fn render_object_pass(encoder: &mut CommandEncoder, resources: &RenderResources) {
    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("object_pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: resources.targets.hdr_buffer_view(),
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Load,
                store: StoreOp::Store,
            },
            depth_slice: None,
        })],
        depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
            view: resources.targets.buffers.depth_buffer_view(),
            depth_ops: Some(Operations {
                load: LoadOp::Load,
                store: StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        ..Default::default()
    });

    let camera_view_bind_group = resources.globals.bind_groups.camera_view_bind_group();
    render_pass.set_bind_group(0, camera_view_bind_group, &[]);

    let lighting_bind_group = resources.globals.bind_groups.lighting_bind_group();
    render_pass.set_bind_group(1, lighting_bind_group, &[]);

    let shadow_map_bind_group = resources.targets.bind_groups.shadow_map_bind_group();
    render_pass.set_bind_group(2, shadow_map_bind_group, &[]);

    for object in resources.objects.iter() {
        let material = resources.materials.get_material(object.material).unwrap();
        let node = resources.objects.get_node(object.node).unwrap();
        let pipeline = resources.materials.get_pipeline(&material.shader_key);

        render_pass.set_pipeline(pipeline);

        let material_bind_group = &material.bind_group;
        render_pass.set_bind_group(3, material_bind_group, &[]);

        let model_bind_group = &node.bind_group;
        render_pass.set_bind_group(4, model_bind_group, &[]);

        let vertex_buffer = &object.vbo.vertex_buffer;
        let index_buffer = &object.vbo.index_buffer;
        let index_count = object.vbo.index_count;

        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);
        render_pass.draw_indexed(0..index_count, 0, 0..1);
    }
}
