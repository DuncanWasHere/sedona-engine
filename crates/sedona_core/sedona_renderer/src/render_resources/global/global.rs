use crate::render_resources::global::bind_groups::GlobalBindGroups;
use crate::render_resources::global::buffers::GlobalBuffers;
use crate::render_resources::layouts::BindGroupLayouts;
use wgpu::Device;

pub struct RenderGlobals {
    pub buffers: GlobalBuffers,
    pub bind_groups: GlobalBindGroups,
}

impl RenderGlobals {
    pub fn new(layouts: &BindGroupLayouts, device: &Device) -> Self {
        let buffers = GlobalBuffers::new(device);
        let bind_groups = GlobalBindGroups::new(&buffers, layouts, device);

        Self {
            buffers,
            bind_groups,
        }
    }
}
