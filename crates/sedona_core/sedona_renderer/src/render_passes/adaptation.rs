use crate::render_resources::render_resources::RenderResources;
use wgpu::{CommandEncoder, ComputePassDescriptor};

pub fn compute_adapted_luminance(encoder: &mut CommandEncoder, resources: &RenderResources) {
    // Downsample luminance texture to get average luminance.
    let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
        label: Some("compute_adapted_luminance"),
        timestamp_writes: None,
    });

    compute_pass.set_pipeline(resources.post_process.pipelines.adaptation_pipeline());

    compute_pass.set_bind_group(
        0,
        resources.post_process.bind_groups.adaptation_bind_group(),
        &[],
    );
    compute_pass.set_bind_group(1, resources.targets.begin_luminance_sample_pass(), &[]);
    compute_pass.dispatch_workgroups(1, 1, 1);
}
