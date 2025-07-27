use crate::render_resources::global::buffers::GlobalBuffers;
use crate::render_resources::layouts::BindGroupLayouts;
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, Device};

pub struct GlobalBindGroups {
    camera_view_bind_group: BindGroup,
    lighting_bind_group: BindGroup,
}

impl GlobalBindGroups {
    pub fn new(buffers: &GlobalBuffers, layouts: &BindGroupLayouts, device: &Device) -> Self {
        let camera_view_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("camera_view"),
            layout: layouts.camera_view(),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffers.camera_view_ubo.as_entire_binding(),
            }],
        });

        let lighting_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("lighting"),
            layout: layouts.lighting(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffers.light_view_ubo.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: buffers.lighting_ubo.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: buffers.light_storage_buffer().as_entire_binding(),
                },
            ],
        });

        Self {
            camera_view_bind_group,
            lighting_bind_group,
        }
    }

    pub fn camera_view_bind_group(&self) -> &BindGroup {
        &self.camera_view_bind_group
    }

    pub fn lighting_bind_group(&self) -> &BindGroup {
        &self.lighting_bind_group
    }
}
