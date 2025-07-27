use crate::render_resources::layouts::BindGroupLayouts;
use crate::render_resources::post_process::buffers::PostProcessBuffers;
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, Device};

pub struct PostProcessBindGroups {
    adaptation_bind_group: BindGroup,
    bloom_bind_group: BindGroup,
    blur_bind_group: BindGroup,
    lens_flare_bind_group: BindGroup,
    tone_map_bind_group: BindGroup,
}

impl PostProcessBindGroups {
    pub fn new(buffers: &PostProcessBuffers, layouts: &BindGroupLayouts, device: &Device) -> Self {
        let adaptation_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("adaptation"),
            layout: layouts.adaptation(),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffers.adaptation_ubo.as_entire_binding(),
            }],
        });

        let bloom_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("bloom"),
            layout: layouts.bloom(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffers.screen_ubo.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: buffers.bloom_ubo.as_entire_binding(),
                },
            ],
        });

        let blur_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("blur"),
            layout: layouts.blur(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffers.screen_ubo.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: buffers.blur_ubo.as_entire_binding(),
                },
            ],
        });

        let lens_flare_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("lens_flare"),
            layout: layouts.lens_flare(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffers.screen_ubo.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: buffers.lens_flare_ubo.as_entire_binding(),
                },
            ],
        });

        let tone_map_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("tone_map"),
            layout: layouts.tone_map(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffers.tone_map_ubo.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: buffers.color_grade_ubo.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: buffers.vignette_ubo.as_entire_binding(),
                },
            ],
        });

        Self {
            adaptation_bind_group,
            bloom_bind_group,
            blur_bind_group,
            lens_flare_bind_group,
            tone_map_bind_group,
        }
    }

    pub fn adaptation_bind_group(&self) -> &BindGroup {
        &self.adaptation_bind_group
    }

    pub fn bloom_bind_group(&self) -> &BindGroup {
        &self.bloom_bind_group
    }

    pub fn blur_bind_group(&self) -> &BindGroup {
        &self.blur_bind_group
    }

    pub fn lens_flare_bind_group(&self) -> &BindGroup {
        &self.lens_flare_bind_group
    }

    pub fn tone_map_bind_group(&self) -> &BindGroup {
        &self.tone_map_bind_group
    }
}
