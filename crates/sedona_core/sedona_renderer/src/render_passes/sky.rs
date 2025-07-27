use crate::render_resources::render_resources::RenderResources;
use crate::types::SkyMode;
use wgpu::{
    Color, CommandEncoder, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp,
};

pub fn render_sky_pass(encoder: &mut CommandEncoder, resources: &RenderResources) {
    let global_render_manager = &resources.globals;
    let sky_render_manager = &resources.sky;

    let sky_pipeline = match sky_render_manager.sky_mode {
        SkyMode::None => return,
        SkyMode::Gradient => sky_render_manager.pipelines.sky_gradient_pipeline(),
        SkyMode::Texture => sky_render_manager.pipelines.sky_box_pipeline(),
        SkyMode::Pbr => sky_render_manager.pipelines.sky_pbr_pipeline(),
    };

    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("sky_pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: resources.targets.hdr_buffer_view(),
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color::BLACK),
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

    let camera_view_bind_group = global_render_manager.bind_groups.camera_view_bind_group();
    let sky_bind_group = sky_render_manager.bind_groups.sky_bind_group();

    if let Some(star_map_bind_group) = sky_render_manager.bind_groups.star_map_bind_group() {
        let pipeline = sky_render_manager.pipelines.star_map_pipeline();

        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, camera_view_bind_group, &[]);
        render_pass.set_bind_group(1, star_map_bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }

    render_pass.set_pipeline(sky_pipeline);
    render_pass.set_bind_group(0, camera_view_bind_group, &[]);
    render_pass.set_bind_group(1, sky_bind_group, &[]);
    render_pass.draw(0..3, 0..1);

    let sun_bind_group = sky_render_manager.bind_groups.sun_bind_group();
    let pipeline = sky_render_manager.pipelines.sun_pipeline();

    render_pass.set_pipeline(pipeline);
    render_pass.set_bind_group(0, camera_view_bind_group, &[]);
    render_pass.set_bind_group(1, sun_bind_group, &[]);
    render_pass.draw(0..3, 0..1);

    if let Some(moon_bind_group) = sky_render_manager.bind_groups.moon_bind_group() {
        let pipeline = sky_render_manager.pipelines.moon_pipeline();

        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, camera_view_bind_group, &[]);
        render_pass.set_bind_group(1, moon_bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }

    if let Some(cloud_bind_group) = sky_render_manager.bind_groups.cloud_bind_group() {
        let pipeline = sky_render_manager.pipelines.cloud_pipeline();
        render_pass.set_pipeline(pipeline);

        render_pass.set_bind_group(0, camera_view_bind_group, &[]);
        render_pass.set_bind_group(1, cloud_bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
