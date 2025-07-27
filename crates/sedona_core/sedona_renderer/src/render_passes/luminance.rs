use crate::render_resources::render_resources::RenderResources;
use wgpu::{CommandEncoder, ComputePassDescriptor, TextureFormat};

pub fn compute_average_luminance(encoder: &mut CommandEncoder, resources: &RenderResources) {
    // Blit HDR buffer texture to first luminance mip level view.
    resources.blit.blit_to_texture_from_bind_group(
        resources.targets.hdr_buffer_bind_group(),
        resources.targets.buffers.get_luminance_mip_view(0),
        TextureFormat::R16Float,
        encoder,
    );

    // Downsample luminance texture to get average luminance.
    let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
        label: Some("compute_luminance"),
        timestamp_writes: None,
    });

    compute_pass.set_pipeline(
        resources
            .post_process
            .pipelines
            .luminance_downsample_pipeline(),
    );

    let base_width = resources.settings.resolution_width;
    let base_height = resources.settings.resolution_height;

    for (mip, bind_group) in resources
        .targets
        .bind_groups
        .luminance_downsample_bind_groups()
        .iter()
        .enumerate()
    {
        let mip_width = (base_width >> (mip + 1)).max(1);
        let mip_height = (base_height >> (mip + 1)).max(1);

        let workgroup_x = mip_width.div_ceil(8);
        let workgroup_y = mip_height.div_ceil(8);

        compute_pass.set_bind_group(0, bind_group, &[]);
        compute_pass.dispatch_workgroups(workgroup_x, workgroup_y, 1);
    }
}
