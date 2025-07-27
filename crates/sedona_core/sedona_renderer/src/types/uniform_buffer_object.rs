use crate::types::uniforms::uniform::Uniform;
use bytemuck::{Pod, Zeroable};
use std::marker::PhantomData;
use wgpu::util::DeviceExt;
use wgpu::{BindingResource, Buffer, BufferUsages, Device, Queue};

#[derive(Debug, Clone, PartialEq)]
pub struct UniformBufferObject<T: Pod + Uniform + Zeroable> {
    buffer: Buffer,
    data: T,
    _marker: PhantomData<T>,
}

impl<T: Pod + Uniform + Zeroable> UniformBufferObject<T> {
    pub fn new(device: &Device) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(T::name()),
            contents: bytemuck::bytes_of(&T::zeroed()),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        Self {
            buffer,
            data: T::zeroed(),
            _marker: PhantomData,
        }
    }

    pub fn with_data(data: T, device: &Device) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(T::name()),
            contents: bytemuck::bytes_of(&data),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        Self {
            buffer,
            data,
            _marker: PhantomData,
        }
    }

    pub fn write_field<V: Pod>(&mut self, name: &str, value: &V, queue: &Queue) {
        let offset = match T::get_field_offset(name) {
            Some(off) => off,
            None => panic!("Field '{}' not found in {}", name, T::name()),
        };

        let expected_size = T::get_field_size(name).unwrap();
        let actual_size = size_of::<V>();

        if expected_size != actual_size {
            panic!(
                "Field '{}' of {} size mismatch: expected {}, got {}",
                name,
                T::name(),
                expected_size,
                actual_size
            );
        }

        unsafe {
            let data_ptr = &mut self.data as *mut T as *mut u8;
            let dest_ptr = data_ptr.add(offset) as *mut V;
            std::ptr::write_unaligned(dest_ptr, *value);
        }

        queue.write_buffer(&self.buffer, offset as u64, bytemuck::bytes_of(value));
    }

    pub fn set(&mut self, data: T, queue: &Queue) {
        self.data = data;
        self.write(queue);
    }

    pub fn write(&self, queue: &Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&self.data));
    }

    pub fn as_entire_binding(&self) -> BindingResource {
        self.buffer.as_entire_binding()
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
}

impl<T: Default + Pod + Uniform + Zeroable> UniformBufferObject<T> {
    pub fn with_default_data(device: &Device) -> Self {
        let data = T::default();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(T::name()),
            contents: bytemuck::bytes_of(&data),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        Self {
            buffer,
            data,
            _marker: PhantomData,
        }
    }
}
