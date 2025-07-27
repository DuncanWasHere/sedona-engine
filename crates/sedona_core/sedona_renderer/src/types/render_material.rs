use crate::types::{MaterialUniforms, ShaderKey, UniformBufferObject};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, Device,
    Sampler, TextureView,
};

#[derive(Clone, Debug)]
pub struct RenderMaterial {
    pub ubo: UniformBufferObject<MaterialUniforms>,
    pub bind_group: BindGroup,
    pub shader_key: ShaderKey,
    references: u64,
}

impl RenderMaterial {
    pub fn new(
        uniform_data: MaterialUniforms,
        shader_key: ShaderKey,
        textures: &[&TextureView],
        sampler: &Sampler,
        layout: &BindGroupLayout,
        device: &Device,
    ) -> Self {
        let ubo = UniformBufferObject::with_data(uniform_data, device);

        let mut entries = Vec::with_capacity(textures.len() + 2);

        entries.push(BindGroupEntry {
            binding: 0,
            resource: ubo.as_entire_binding(),
        });

        entries.push(BindGroupEntry {
            binding: 1,
            resource: BindingResource::Sampler(sampler),
        });

        for (i, texture) in textures.iter().enumerate() {
            entries.push(BindGroupEntry {
                binding: (i + 2) as u32,
                resource: BindingResource::TextureView(texture),
            })
        }

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("material"),
            layout,
            entries: &entries,
        });

        Self {
            ubo,
            bind_group,
            shader_key,
            references: 0,
        }
    }

    pub fn add_reference(&mut self) {
        self.references += 1;
    }

    pub fn remove_reference(&mut self) {
        self.references -= 1;
    }

    pub fn has_references(&self) -> bool {
        self.references > 0
    }
}
