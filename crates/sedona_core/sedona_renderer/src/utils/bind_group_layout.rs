use wgpu::{ShaderStages, StorageTextureAccess, TextureFormat};

#[derive(Clone, Debug)]
pub enum BindGroupLayoutEntryToken {
    BufferUniform {
        stages: ShaderStages,
    },
    BufferStorage {
        stages: ShaderStages,
        read_only: bool,
    },

    SamplerFilter,
    SamplerNoFilter,
    SamplerCompare,

    Texture2d,
    Texture2dArray,
    Texture3d,
    TextureCube,

    TextureDepth,
    TextureDepthArray,

    TextureStorage {
        format: TextureFormat,
        access: StorageTextureAccess,
    },
}

#[macro_export]
macro_rules! create_bind_group_layout {
    ($tokens:expr, $device:expr, $compute:expr, $label:expr) => {{
        use wgpu::*;
        use $crate::utils::bind_group_layout::BindGroupLayoutEntryToken::*;

        let default_shader_stage = if $compute {
            ShaderStages::COMPUTE
        } else {
            ShaderStages::FRAGMENT
        };

        let entries: Vec<BindGroupLayoutEntry> = $tokens
            .iter()
            .enumerate()
            .map(|(i, token)| {
                let (ty, visibility) = match token {
                    BufferUniform { stages } => (
                        BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            min_binding_size: None,
                            has_dynamic_offset: false,
                        },
                        *stages,
                    ),
                    BufferStorage { stages, read_only } => (
                        BindingType::Buffer {
                            ty: BufferBindingType::Storage {
                                read_only: *read_only,
                            },
                            min_binding_size: None,
                            has_dynamic_offset: false,
                        },
                        *stages,
                    ),

                    SamplerFilter => (
                        BindingType::Sampler(SamplerBindingType::Filtering),
                        default_shader_stage,
                    ),
                    SamplerNoFilter => (
                        BindingType::Sampler(SamplerBindingType::NonFiltering),
                        default_shader_stage,
                    ),
                    SamplerCompare => (
                        BindingType::Sampler(SamplerBindingType::Comparison),
                        default_shader_stage,
                    ),

                    Texture2d => (
                        BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2,
                            sample_type: TextureSampleType::Float { filterable: true },
                        },
                        default_shader_stage,
                    ),
                    Texture2dArray => (
                        BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2Array,
                            sample_type: TextureSampleType::Float { filterable: true },
                        },
                        default_shader_stage,
                    ),
                    Texture3d => (
                        BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D3,
                            sample_type: TextureSampleType::Float { filterable: true },
                        },
                        default_shader_stage,
                    ),
                    TextureCube => (
                        BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::Cube,
                            sample_type: TextureSampleType::Float { filterable: true },
                        },
                        default_shader_stage,
                    ),
                    TextureDepth => (
                        BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2,
                            sample_type: TextureSampleType::Depth,
                        },
                        default_shader_stage,
                    ),
                    TextureDepthArray => (
                        BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2Array,
                            sample_type: TextureSampleType::Depth,
                        },
                        default_shader_stage,
                    ),
                    TextureStorage { format, access } => (
                        BindingType::StorageTexture {
                            view_dimension: TextureViewDimension::D2,
                            format: *format,
                            access: *access,
                        },
                        default_shader_stage,
                    ),
                };

                BindGroupLayoutEntry {
                    binding: i as u32,
                    visibility,
                    ty,
                    count: None,
                }
            })
            .collect();

        $device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some($label),
            entries: &entries,
        })
    }};
}
