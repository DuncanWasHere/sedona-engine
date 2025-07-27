use crate::utils::defaults::DEFAULT_COLOR_TARGET_FORMAT;

use crate::render_resources::{BindGroupLayouts, RenderSettings};
use crate::types::{
    MaterialUniforms, Pixels, RenderMaterial, RendererError, ShaderFlags, ShaderKey, StaticVertex,
    Vertex,
};
use crate::utils::{create_shader_from_path, create_texture, FallbackTextures};
use slab::Slab;
use std::collections::HashMap;
use std::hash::Hash;
use wgpu::*;

pub struct RenderMaterials {
    materials: Slab<RenderMaterial>,
    pipeline_layout: PipelineLayout,
    pipelines: HashMap<ShaderKey, RenderPipeline>,
    sampler: Sampler,
    fallback_textures: FallbackTextures,
    textures: HashMap<u64, Texture>,
    texture_views: HashMap<u64, TextureView>,
    shaders: HashMap<String, ShaderModule>,
}

impl RenderMaterials {
    pub fn new(
        settings: &RenderSettings,
        layouts: &BindGroupLayouts,
        device: &Device,
        queue: &Queue,
    ) -> RenderMaterials {
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("material"),
            bind_group_layouts: &[
                layouts.camera_view(),
                layouts.lighting(),
                layouts.shadow_map(),
                layouts.material(),
                layouts.model(),
            ],
            push_constant_ranges: &[],
        });

        let filter_mode = if settings.filter_textures {
            FilterMode::Linear
        } else {
            FilterMode::Nearest
        };

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("sampler_material"),
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter: filter_mode,
            min_filter: filter_mode,
            mipmap_filter: filter_mode,
            ..Default::default()
        });

        let fallback_textures = FallbackTextures::new(device, queue);

        Self {
            materials: Slab::new(),
            pipeline_layout,
            sampler,
            fallback_textures,
            textures: HashMap::new(),
            texture_views: HashMap::new(),
            pipelines: HashMap::new(),
            shaders: HashMap::new(),
        }
    }

    pub fn insert_material(
        &mut self,
        uniform_data: MaterialUniforms,
        textures: &[Option<u64>; 5],
        shader_path: &str,
        shader_flags: ShaderFlags,
        layouts: &BindGroupLayouts,
        device: &Device,
    ) -> Result<usize, RendererError> {
        if !self.shaders.contains_key(shader_path) {
            let shader = create_shader_from_path("material", shader_path, device);
            self.shaders.insert(shader_path.to_string(), shader);
        }

        let shader_key = ShaderKey::new(shader_path, shader_flags);
        self.create_pipeline(shader_key.clone(), device);
        let layout = layouts.material();

        let texture_views: [&TextureView; 5] = [
            textures[0]
                .and_then(|id| self.texture_views.get(&id))
                .unwrap_or(&self.fallback_textures.base_color),
            textures[1]
                .and_then(|id| self.texture_views.get(&id))
                .unwrap_or(&self.fallback_textures.metallic_roughness),
            textures[2]
                .and_then(|id| self.texture_views.get(&id))
                .unwrap_or(&self.fallback_textures.normal),
            textures[3]
                .and_then(|id| self.texture_views.get(&id))
                .unwrap_or(&self.fallback_textures.emissive),
            textures[4]
                .and_then(|id| self.texture_views.get(&id))
                .unwrap_or(&self.fallback_textures.occlusion),
        ];

        let material = RenderMaterial::new(
            uniform_data,
            shader_key,
            &texture_views,
            &self.sampler,
            layout,
            device,
        );
        Ok(self.materials.insert(material))
    }

    pub(crate) fn create_pipeline(&mut self, key: ShaderKey, device: &Device) {
        if !self.pipelines.contains_key(&key) {
            let shader = self.shaders.get(&key.path).unwrap();

            let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("pipeline_material"),
                layout: Some(&self.pipeline_layout),
                vertex: VertexState {
                    module: shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[StaticVertex::descriptor()],
                },
                fragment: Some(FragmentState {
                    module: shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(ColorTargetState {
                        format: DEFAULT_COLOR_TARGET_FORMAT,
                        blend: if key.flags.contains(ShaderFlags::BLEND_ALPHA) {
                            Some(BlendState::ALPHA_BLENDING)
                        } else {
                            Some(BlendState {
                                color: BlendComponent::OVER,
                                alpha: BlendComponent::OVER,
                            })
                        },
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    cull_mode: if key.flags.contains(ShaderFlags::CULL_DISABLED)
                        || key.flags.contains(ShaderFlags::DOUBLE_SIDED)
                    {
                        Some(Face::Back)
                    } else {
                        Some(Face::Back)
                    },
                    polygon_mode: if key.flags.contains(ShaderFlags::WIREFRAME) {
                        PolygonMode::Line
                    } else {
                        PolygonMode::Fill
                    },
                    unclipped_depth: false,
                    conservative: false,
                    ..Default::default()
                },
                depth_stencil: Some(DepthStencilState {
                    format: TextureFormat::Depth32Float,
                    depth_write_enabled: false,
                    depth_compare: CompareFunction::LessEqual,
                    stencil: StencilState::default(),
                    bias: DepthBiasState::default(),
                }),
                multisample: MultisampleState::default(),
                multiview: None,
                cache: None,
            });

            self.pipelines.insert(key, pipeline);
        }
    }

    pub fn create_texture(&mut self, pixels: impl Pixels, device: &Device, queue: &Queue) -> u64 {
        let hash = pixels.compute_hash();
        if self.textures.contains_key(&hash) {
            return hash;
        }

        let texture = create_texture(&[pixels], TextureDimension::D2, device, queue);
        let texture_view = texture.create_view(&TextureViewDescriptor::default());

        self.textures.insert(hash, texture);
        self.texture_views.insert(hash, texture_view);

        hash
    }

    pub fn get_material(&self, index: usize) -> Result<&RenderMaterial, RendererError> {
        self.materials
            .get(index)
            .ok_or(RendererError::InvalidMaterialIndex { index })
    }

    pub fn get_material_mut(&mut self, index: usize) -> Result<&mut RenderMaterial, RendererError> {
        self.materials
            .get_mut(index)
            .ok_or(RendererError::InvalidMaterialIndex { index })
    }

    pub fn get_pipeline(&self, key: &ShaderKey) -> &RenderPipeline {
        self.pipelines.get(key).unwrap()
    }

    pub fn get_texture_view(&self, id: u64) -> &TextureView {
        self.texture_views.get(&id).unwrap()
    }

    pub fn sampler(&self) -> &Sampler {
        &self.sampler
    }

    pub fn base_color_fallback(&self) -> &TextureView {
        &self.fallback_textures.base_color
    }

    pub fn metallic_roughness_fallback(&self) -> &TextureView {
        &self.fallback_textures.metallic_roughness
    }

    pub fn normal_fallback(&self) -> &TextureView {
        &self.fallback_textures.normal
    }

    pub fn emissive_fallback(&self) -> &TextureView {
        &self.fallback_textures.emissive
    }

    pub fn occlusion_fallback(&self) -> &TextureView {
        &self.fallback_textures.occlusion
    }
}
