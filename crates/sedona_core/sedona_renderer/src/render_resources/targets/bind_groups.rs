use crate::render_resources::layouts::BindGroupLayouts;
use crate::render_resources::targets::buffers::RenderTargetBuffers;
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, Device};

pub struct RenderTargetBindGroups {
    hdr_a_bind_group: BindGroup,
    hdr_b_bind_group: BindGroup,
    frame_buffers_a_bind_group: BindGroup,
    frame_buffers_b_bind_group: BindGroup,
    shadow_map_bind_group: BindGroup,
    luminance_downsample_bind_groups: Vec<BindGroup>,
    luminance_sample_a_bind_group: BindGroup,
    luminance_sample_b_bind_group: BindGroup,
    adaptation_buffer_a_bind_group: BindGroup,
    adaptation_buffer_b_bind_group: BindGroup,
    bloom_buffer_a_bind_group: BindGroup,
    bloom_buffer_b_bind_group: BindGroup,
}

impl RenderTargetBindGroups {
    pub fn new(buffers: &RenderTargetBuffers, layouts: &BindGroupLayouts, device: &Device) -> Self {
        let [
            hdr_a_bind_group,
            hdr_b_bind_group,
            frame_buffers_a_bind_group,
            frame_buffers_b_bind_group,
            luminance_sample_a_bind_group,
            luminance_sample_b_bind_group,
            adaptation_buffer_a_bind_group,
            adaptation_buffer_b_bind_group,
            bloom_buffer_a_bind_group,
            bloom_buffer_b_bind_group,
        ] = Self::create_frame_buffer_bind_groups(buffers, layouts, device);

        let shadow_map_bind_group = Self::create_shadow_map_bind_group(buffers, layouts, device);

        let luminance_downsample_bind_groups =
            Self::create_luminance_downsample_bind_groups(buffers, layouts, device);

        Self {
            hdr_a_bind_group,
            hdr_b_bind_group,
            frame_buffers_a_bind_group,
            frame_buffers_b_bind_group,
            shadow_map_bind_group,
            luminance_downsample_bind_groups,
            luminance_sample_a_bind_group,
            luminance_sample_b_bind_group,
            adaptation_buffer_a_bind_group,
            adaptation_buffer_b_bind_group,
            bloom_buffer_a_bind_group,
            bloom_buffer_b_bind_group,
        }
    }

    pub fn create_frame_buffer_bind_groups(
        buffers: &RenderTargetBuffers,
        layouts: &BindGroupLayouts,
        device: &Device,
    ) -> [BindGroup; 10] {
        let hdr_buffer_a_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("hdr_frame_buffer_a"),
            layout: layouts.blit(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&buffers.filter_sampler()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&buffers.hdr_buffer_view_a()),
                },
            ],
        });

        let hdr_buffer_b_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("hdr_frame_buffer_b"),
            layout: layouts.blit(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&buffers.filter_sampler()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&buffers.hdr_buffer_view_b()),
                },
            ],
        });

        let frame_buffers_a_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("frame_buffers_a"),
            layout: layouts.frame_buffers(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&buffers.filter_sampler()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&buffers.compare_sampler()),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(&buffers.hdr_buffer_view_a()),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::TextureView(&buffers.depth_buffer_view()),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: BindingResource::TextureView(&buffers.normal_buffer_view()),
                },
            ],
        });

        let frame_buffers_b_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("frame_buffers_b"),
            layout: layouts.frame_buffers(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&buffers.filter_sampler()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&buffers.compare_sampler()),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(&buffers.hdr_buffer_view_b()),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::TextureView(&buffers.depth_buffer_view()),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: BindingResource::TextureView(&buffers.normal_buffer_view()),
                },
            ],
        });

        let luminance_sample_a_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("luminance_sample_a"),
            layout: layouts.luminance_sample(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(
                        &buffers.luminance_mip_views().last().unwrap(),
                    ),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&buffers.adaptation_buffer_view_a()),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(&buffers.adaptation_buffer_view_b()),
                },
            ],
        });

        let luminance_sample_b_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("luminance_sample_b"),
            layout: layouts.luminance_sample(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(
                        &buffers.luminance_mip_views().last().unwrap(),
                    ),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&buffers.adaptation_buffer_view_b()),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(&buffers.adaptation_buffer_view_a()),
                },
            ],
        });

        let adaptation_buffer_a_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("adaptation_frame_buffer_a"),
            layout: layouts.blit(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&buffers.filter_sampler()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&buffers.adaptation_buffer_view_a()),
                },
            ],
        });

        let adaptation_buffer_b_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("adaptation_frame_buffer_b"),
            layout: layouts.blit(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&buffers.filter_sampler()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&buffers.adaptation_buffer_view_b()),
                },
            ],
        });

        let bloom_buffer_a_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("bloom_frame_buffer_a"),
            layout: layouts.blit(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&buffers.filter_sampler()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&buffers.bloom_buffer_view_a()),
                },
            ],
        });

        let bloom_buffer_b_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("bloom_frame_buffer_b"),
            layout: layouts.blit(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&buffers.filter_sampler()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&buffers.bloom_buffer_view_b()),
                },
            ],
        });

        [
            hdr_buffer_a_bind_group,
            hdr_buffer_b_bind_group,
            frame_buffers_a_bind_group,
            frame_buffers_b_bind_group,
            luminance_sample_a_bind_group,
            luminance_sample_b_bind_group,
            adaptation_buffer_a_bind_group,
            adaptation_buffer_b_bind_group,
            bloom_buffer_a_bind_group,
            bloom_buffer_b_bind_group,
        ]
    }

    pub fn create_shadow_map_bind_group(
        buffers: &RenderTargetBuffers,
        layouts: &BindGroupLayouts,
        device: &Device,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("shadow_map"),
            layout: layouts.shadow_map(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&buffers.compare_sampler()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&buffers.shadow_map_view()),
                },
            ],
        })
    }

    pub fn create_luminance_downsample_bind_groups(
        buffers: &RenderTargetBuffers,
        layouts: &BindGroupLayouts,
        device: &Device,
    ) -> Vec<BindGroup> {
        let mut luminance_downsample_bind_groups = Vec::new();
        for mip in 0..(buffers.luminance_mip_views().len() - 1) {
            let source_view = &buffers.luminance_mip_views()[mip];
            let destination_view = &buffers.luminance_mip_views()[mip + 1];

            let bind_group = device.create_bind_group(&BindGroupDescriptor {
                layout: layouts.luminance_downsample(),
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(source_view),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::TextureView(&destination_view),
                    },
                ],
                label: Some(&format!("luminance_downsample_{}", mip)),
            });

            luminance_downsample_bind_groups.push(bind_group);
        }

        luminance_downsample_bind_groups
    }

    pub fn resize(
        &mut self,
        buffers: &RenderTargetBuffers,
        layouts: &BindGroupLayouts,
        device: &Device,
    ) {
        let [
            hdr_a_bind_group,
            hdr_b_bind_group,
            frame_buffers_a_bind_group,
            frame_buffers_b_bind_group,
            luminance_sample_a_bind_group,
            luminance_sample_b_bind_group,
            adaptation_buffer_a_bind_group,
            adaptation_buffer_b_bind_group,
            bloom_buffer_a_bind_group,
            bloom_buffer_b_bind_group,
        ] = Self::create_frame_buffer_bind_groups(buffers, layouts, device);
        let luminance_downsample_bind_groups =
            Self::create_luminance_downsample_bind_groups(buffers, layouts, device);

        self.hdr_a_bind_group = hdr_a_bind_group;
        self.hdr_b_bind_group = hdr_b_bind_group;
        self.frame_buffers_a_bind_group = frame_buffers_a_bind_group;
        self.frame_buffers_b_bind_group = frame_buffers_b_bind_group;
        self.luminance_downsample_bind_groups = luminance_downsample_bind_groups;
        self.luminance_sample_a_bind_group = luminance_sample_a_bind_group;
        self.luminance_sample_b_bind_group = luminance_sample_b_bind_group;
        self.adaptation_buffer_a_bind_group = adaptation_buffer_a_bind_group;
        self.adaptation_buffer_b_bind_group = adaptation_buffer_b_bind_group;
        self.bloom_buffer_a_bind_group = bloom_buffer_a_bind_group;
        self.bloom_buffer_b_bind_group = bloom_buffer_b_bind_group;
    }

    pub fn hdr_a_bind_group(&self) -> &BindGroup {
        &self.hdr_a_bind_group
    }

    pub fn hdr_b_bind_group(&self) -> &BindGroup {
        &self.hdr_b_bind_group
    }

    pub fn frame_buffers_a_bind_group(&self) -> &BindGroup {
        &self.frame_buffers_a_bind_group
    }

    pub fn frame_buffers_b_bind_group(&self) -> &BindGroup {
        &self.frame_buffers_b_bind_group
    }

    pub fn shadow_map_bind_group(&self) -> &BindGroup {
        &self.shadow_map_bind_group
    }

    pub fn luminance_downsample_bind_groups(&self) -> &[BindGroup] {
        &self.luminance_downsample_bind_groups
    }

    pub fn get_luminance_downsample_bind_group(&self, index: usize) -> &BindGroup {
        &self.luminance_downsample_bind_groups[index]
    }

    pub fn luminance_sample_a_bind_group(&self) -> &BindGroup {
        &self.luminance_sample_a_bind_group
    }

    pub fn luminance_sample_b_bind_group(&self) -> &BindGroup {
        &self.luminance_sample_b_bind_group
    }

    pub fn adaptation_buffer_a_bind_group(&self) -> &BindGroup {
        &self.adaptation_buffer_a_bind_group
    }

    pub fn adaptation_buffer_b_bind_group(&self) -> &BindGroup {
        &self.adaptation_buffer_b_bind_group
    }

    pub fn bloom_buffer_a_bind_group(&self) -> &BindGroup {
        &self.bloom_buffer_a_bind_group
    }

    pub fn bloom_buffer_b_bind_group(&self) -> &BindGroup {
        &self.bloom_buffer_b_bind_group
    }
}
