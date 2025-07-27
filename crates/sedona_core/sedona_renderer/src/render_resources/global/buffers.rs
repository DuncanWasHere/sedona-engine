use crate::types::uniform_buffer_object::UniformBufferObject;
use crate::types::uniforms::{
    CameraViewUniforms, LightStorageUniforms, LightViewUniforms, LightingUniforms, Uniform,
};
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Queue};

pub struct GlobalBuffers {
    pub camera_view_ubo: UniformBufferObject<CameraViewUniforms>,
    pub light_view_ubo: UniformBufferObject<LightViewUniforms>,
    pub lighting_ubo: UniformBufferObject<LightingUniforms>,

    light_storage_buffer: Buffer,
    lights: Vec<Option<u64>>,
}

impl GlobalBuffers {
    pub fn new(device: &Device) -> Self {
        let camera_view_ubo = UniformBufferObject::new(device);
        let light_view_ubo = UniformBufferObject::new(device);
        let lighting_ubo = UniformBufferObject::new(device);

        let light_storage_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("buffer_lights_storage"),
            size: LightStorageUniforms::SIZE * MAX_LIGHTS,
            mapped_at_creation: false,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        let lights = Vec::with_capacity(MAX_LIGHTS as usize);

        Self {
            camera_view_ubo,
            light_view_ubo,
            lighting_ubo,
            light_storage_buffer,
            lights,
        }
    }

    pub fn create_light(&mut self, id: u64, data: &LightStorageUniforms, queue: &Queue) {
        if self.lights.iter().any(|&entry| entry == Some(id)) {
            return;
        }

        let slot = if let Some(index) = self.lights.iter().position(|entry| entry.is_none()) {
            index
        } else {
            return; // Max capacity reached
        };

        self.lights[slot] = Some(id);

        let offset = (slot * LightStorageUniforms::SIZE as usize) as u64;
        queue.write_buffer(&self.light_storage_buffer, offset, bytemuck::bytes_of(data));
    }

    pub fn destroy_light(&mut self, id: u64, queue: &Queue) {
        if let Some(index) = self.lights.iter().position(|&entry| entry == Some(id)) {
            self.lights[index] = None;

            let zero_data = [0u8; LightStorageUniforms::SIZE as usize];
            let offset = (index * LightStorageUniforms::SIZE as usize) as u64;
            queue.write_buffer(&self.light_storage_buffer, offset, &zero_data);
        }
    }

    pub fn clear_lights(&mut self, queue: &Queue) {
        for entry in &mut self.lights {
            *entry = None;
        }

        let zero_data = vec![0u8; (LightStorageUniforms::SIZE * MAX_LIGHTS) as usize];
        queue.write_buffer(&self.light_storage_buffer, 0, &zero_data);
    }

    pub fn light_storage_buffer(&self) -> &Buffer {
        &self.light_storage_buffer
    }
}

pub const MAX_LIGHTS: u64 = 16;
