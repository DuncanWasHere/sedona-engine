use crate::types::Vertex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferUsages, Device};

#[derive(Clone, Debug)]
pub struct VertexBufferObject {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub index_count: u32,
}

impl VertexBufferObject {
    pub fn new<V: Vertex>(vertices: &[V], indices: &[u32], device: &Device) -> Self {
        Self {
            vertex_buffer: device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices),
                usage: BufferUsages::VERTEX,
            }),
            index_buffer: device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
                usage: BufferUsages::INDEX,
            }),
            index_count: indices.len() as u32,
        }
    }

    pub fn resize<V: Vertex>(&mut self, vertices: &[V], indices: &[u32], device: &Device) {
        self.vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        });

        self.index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });

        self.index_count = indices.len() as u32;
    }

    pub fn is_empty(&self) -> bool {
        self.index_count == 0
    }
}
