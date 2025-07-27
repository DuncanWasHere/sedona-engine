use std::fs;
use std::path::Path;
use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource};

pub fn create_shader_from_path(label: &str, path: &str, device: &Device) -> ShaderModule {
    let shader_path = Path::new(path);
    let source = fs::read_to_string(shader_path)
        .unwrap_or_else(|err| panic!("Failed to read shader file '{}': {}", path, err));

    create_shader_from_source(label, &source, device)
}

pub fn create_shader_from_source(label: &str, source: &str, device: &Device) -> ShaderModule {
    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some(label),
        source: ShaderSource::Wgsl(source.into()),
    });

    shader
}
