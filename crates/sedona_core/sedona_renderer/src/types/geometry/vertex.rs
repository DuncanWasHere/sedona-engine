use wgpu::VertexBufferLayout;

pub trait Vertex: bytemuck::Pod + bytemuck::Zeroable + 'static {
    fn descriptor<'a>() -> VertexBufferLayout<'a>;
}
