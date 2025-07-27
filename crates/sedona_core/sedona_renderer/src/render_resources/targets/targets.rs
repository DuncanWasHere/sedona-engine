use crate::render_resources::layouts::BindGroupLayouts;
use crate::render_resources::settings::RenderSettings;
use crate::render_resources::targets::bind_groups::RenderTargetBindGroups;
use crate::render_resources::targets::buffers::RenderTargetBuffers;
use std::sync::atomic::{AtomicU8, Ordering};
use wgpu::{BindGroup, Device, TextureView};

pub struct RenderTargets {
    pub buffers: RenderTargetBuffers,
    pub bind_groups: RenderTargetBindGroups,
    current_hdr_target: AtomicU8,
    current_adaptation_target: AtomicU8,
    current_bloom_target: AtomicU8,
}

impl RenderTargets {
    pub fn new(settings: &RenderSettings, layouts: &BindGroupLayouts, device: &Device) -> Self {
        let buffers = RenderTargetBuffers::new(settings, device);
        let bind_groups = RenderTargetBindGroups::new(&buffers, layouts, device);

        Self {
            buffers,
            bind_groups,
            current_hdr_target: AtomicU8::new(0),
            current_adaptation_target: AtomicU8::new(0),
            current_bloom_target: AtomicU8::new(0),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32, layouts: &BindGroupLayouts, device: &Device) {
        self.buffers.resize(width, height, device);
        self.bind_groups.resize(&self.buffers, layouts, device);
    }

    pub fn begin_post_process_pass(&self) -> (&TextureView, &BindGroup) {
        let old_target = self.current_hdr_target.fetch_xor(1, Ordering::Relaxed);

        if old_target == 0 {
            (
                self.buffers.hdr_buffer_view_b(),
                self.bind_groups.hdr_a_bind_group(),
            )
        } else {
            (
                self.buffers.hdr_buffer_view_a(),
                self.bind_groups.hdr_b_bind_group(),
            )
        }
    }

    pub fn begin_luminance_sample_pass(&self) -> &BindGroup {
        let old_target = self
            .current_adaptation_target
            .fetch_xor(1, Ordering::Relaxed);

        if old_target == 0 {
            self.bind_groups.luminance_sample_a_bind_group()
        } else {
            self.bind_groups.luminance_sample_b_bind_group()
        }
    }

    pub fn begin_bloom_pass(&self) -> (&TextureView, &BindGroup) {
        let old_target = self.current_bloom_target.fetch_xor(1, Ordering::Relaxed);

        if old_target == 0 {
            (
                self.buffers.bloom_buffer_view_b(),
                self.bind_groups.bloom_buffer_a_bind_group(),
            )
        } else {
            (
                self.buffers.bloom_buffer_view_a(),
                self.bind_groups.bloom_buffer_b_bind_group(),
            )
        }
    }

    pub fn hdr_buffer_bind_group(&self) -> &BindGroup {
        if self.current_hdr_target.load(Ordering::Relaxed) == 0 {
            self.bind_groups.hdr_a_bind_group()
        } else {
            self.bind_groups.hdr_b_bind_group()
        }
    }

    pub fn frame_buffers_bind_group(&self) -> &BindGroup {
        if self.current_hdr_target.load(Ordering::Relaxed) == 0 {
            self.bind_groups.frame_buffers_a_bind_group()
        } else {
            self.bind_groups.frame_buffers_b_bind_group()
        }
    }

    pub fn hdr_buffer_view(&self) -> &TextureView {
        if self.current_hdr_target.load(Ordering::Relaxed) == 0 {
            self.buffers.hdr_buffer_view_a()
        } else {
            self.buffers.hdr_buffer_view_b()
        }
    }

    pub fn adaptation_buffer_bind_group(&self) -> &BindGroup {
        if self.current_adaptation_target.load(Ordering::Relaxed) == 0 {
            self.bind_groups.adaptation_buffer_a_bind_group()
        } else {
            self.bind_groups.adaptation_buffer_b_bind_group()
        }
    }

    pub fn bloom_buffer_bind_group(&self) -> &BindGroup {
        if self.current_bloom_target.load(Ordering::Relaxed) == 0 {
            self.bind_groups.bloom_buffer_a_bind_group()
        } else {
            self.bind_groups.bloom_buffer_b_bind_group()
        }
    }
}
