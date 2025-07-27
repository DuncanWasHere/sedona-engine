use crate::create_bind_group_layout;
use wgpu::{BindGroupLayout, Device};

pub struct BindGroupLayouts {
    adaptation: BindGroupLayout,
    blit: BindGroupLayout,
    bloom: BindGroupLayout,
    blur: BindGroupLayout,
    camera_view: BindGroupLayout,
    cloud: BindGroupLayout,
    frame_buffers: BindGroupLayout,
    lens_flare: BindGroupLayout,
    lighting: BindGroupLayout,
    light_view: BindGroupLayout,
    luminance_downsample: BindGroupLayout,
    luminance_sample: BindGroupLayout,
    material: BindGroupLayout,
    model: BindGroupLayout,
    moon: BindGroupLayout,
    screen: BindGroupLayout,
    shadow_map: BindGroupLayout,
    sky_box: BindGroupLayout,
    sky_gradient: BindGroupLayout,
    sky_pbr: BindGroupLayout,
    star_map: BindGroupLayout,
    sun: BindGroupLayout,
    tone_map: BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(device: &Device) -> Self {
        let adaptation = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::COMPUTE
                } // Adaptation UBO
            ],
            device,
            true,
            "adaptation"
        );

        let blit = create_bind_group_layout!(
            &[
                SamplerFilter,
                Texture2d, // Source Texture
            ],
            device,
            false,
            "blit"
        );

        let bloom = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Screen UBO
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Bloom UBO
            ],
            device,
            false,
            "bloom"
        );

        let blur = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Screen UBO
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Blur UBO
            ],
            device,
            false,
            "blur"
        );

        let camera_view = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::VERTEX_FRAGMENT
                }, // Camera View UBO
            ],
            device,
            false,
            "camera_view"
        );

        let cloud = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Cloud UBO
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Sun UBO
                SamplerFilter,
                Texture2d, // Cloud Texture
            ],
            device,
            false,
            "cloud"
        );

        let frame_buffers = create_bind_group_layout!(
            &[
                SamplerFilter,
                SamplerCompare,
                Texture2d,    // HDR Ping-Pong Buffer Texture
                TextureDepth, // Depth Buffer Texture
                Texture2d,    // Normal Buffer Texture
            ],
            device,
            false,
            "frame_buffers"
        );

        let lens_flare = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Screen UBO
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Lens Flare UBO
            ],
            device,
            false,
            "bloom"
        );

        let lighting = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Shadow Cascades UBO
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Lighting UBO
                BufferStorage {
                    stages: ShaderStages::FRAGMENT,
                    read_only: true
                }, // Light Storage Buffer
            ],
            device,
            false,
            "lighting"
        );

        let light_view = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::VERTEX
                } // Light View UBO
            ],
            device,
            false,
            "light_view"
        );

        let luminance_downsample = create_bind_group_layout!(
            &[
                TextureStorage {
                    format: TextureFormat::R16Float,
                    access: StorageTextureAccess::ReadOnly
                }, // Source Mip Level View
                TextureStorage {
                    format: TextureFormat::R16Float,
                    access: StorageTextureAccess::WriteOnly,
                }, // Destination Mip Level View
            ],
            device,
            true,
            "luminance_downsample"
        );

        let luminance_sample = create_bind_group_layout!(
            &[
                TextureStorage {
                    format: TextureFormat::R16Float,
                    access: StorageTextureAccess::ReadOnly
                }, // Current Average Luminance View
                TextureStorage {
                    format: TextureFormat::R16Float,
                    access: StorageTextureAccess::ReadOnly,
                }, // Previous Adapted Luminance View
                TextureStorage {
                    format: TextureFormat::R16Float,
                    access: StorageTextureAccess::WriteOnly,
                }, // Next Adaptated Luminance View
            ],
            device,
            true,
            "luminance_sample"
        );

        let material = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Material UBO
                SamplerFilter,
                Texture2d, // Base Color Texture
                Texture2d, // Metallic Roughness Texture
                Texture2d, // Normal Texture
                Texture2d, // Emissive Texture
                Texture2d, // Occlusion Texture
            ],
            device,
            false,
            "material"
        );

        let model = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::VERTEX
                }, // Model UBO
            ],
            device,
            false,
            "model"
        );

        let moon = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Moon UBO
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Sun UBO
                SamplerFilter,
                Texture2dArray, // Moon Texture
            ],
            device,
            false,
            "moon"
        );

        let screen = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Screen UBO
            ],
            device,
            false,
            "screen"
        );

        let shadow_map = create_bind_group_layout!(
            &[
                SamplerCompare,
                TextureDepthArray, // Shadow Map Texture
            ],
            device,
            false,
            "shadow_map"
        );

        let sky_box = create_bind_group_layout!(
            &[
                SamplerFilter,
                TextureCube, // Sky Box Texture
            ],
            device,
            false,
            "sky_box"
        );

        let sky_gradient = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Sky Gradient UBO
            ],
            device,
            false,
            "sky_gradient"
        );

        let sky_pbr = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Sky PBR UBO
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Sun UBO
            ],
            device,
            false,
            "sky_pbr"
        );

        let star_map = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Star UBO
                SamplerFilter,
                Texture2d, // Star Map Texture
            ],
            device,
            false,
            "star_map"
        );

        let sun = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Sun UBO
            ],
            device,
            false,
            "sun"
        );

        let tone_map = create_bind_group_layout!(
            &[
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Tone Map UBO
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Color Grade UBO
                BufferUniform {
                    stages: ShaderStages::FRAGMENT
                }, // Vignette UBO
            ],
            device,
            false,
            "tone_map"
        );

        Self {
            adaptation,
            blit,
            bloom,
            blur,
            camera_view,
            cloud,
            lens_flare,
            luminance_downsample,
            luminance_sample,
            frame_buffers,
            lighting,
            light_view,
            material,
            model,
            moon,
            screen,
            shadow_map,
            sky_box,
            sky_gradient,
            sky_pbr,
            star_map,
            sun,
            tone_map,
        }
    }

    pub fn adaptation(&self) -> &BindGroupLayout {
        &self.adaptation
    }

    pub fn blit(&self) -> &BindGroupLayout {
        &self.blit
    }

    pub fn bloom(&self) -> &BindGroupLayout {
        &self.bloom
    }

    pub fn blur(&self) -> &BindGroupLayout {
        &self.blur
    }

    pub fn camera_view(&self) -> &BindGroupLayout {
        &self.camera_view
    }

    pub fn cloud(&self) -> &BindGroupLayout {
        &self.cloud
    }

    pub fn frame_buffers(&self) -> &BindGroupLayout {
        &self.frame_buffers
    }

    pub fn lens_flare(&self) -> &BindGroupLayout {
        &self.lens_flare
    }

    pub fn lighting(&self) -> &BindGroupLayout {
        &self.lighting
    }

    pub fn light_view(&self) -> &BindGroupLayout {
        &self.light_view
    }

    pub fn luminance_downsample(&self) -> &BindGroupLayout {
        &self.luminance_downsample
    }

    pub fn luminance_sample(&self) -> &BindGroupLayout {
        &self.luminance_sample
    }

    pub fn material(&self) -> &BindGroupLayout {
        &self.material
    }

    pub fn model(&self) -> &BindGroupLayout {
        &self.model
    }

    pub fn moon(&self) -> &BindGroupLayout {
        &self.moon
    }

    pub fn screen(&self) -> &BindGroupLayout {
        &self.screen
    }

    pub fn shadow_map(&self) -> &BindGroupLayout {
        &self.shadow_map
    }

    pub fn sky_box(&self) -> &BindGroupLayout {
        &self.sky_box
    }

    pub fn sky_gradient(&self) -> &BindGroupLayout {
        &self.sky_gradient
    }

    pub fn sky_pbr(&self) -> &BindGroupLayout {
        &self.sky_pbr
    }

    pub fn star_map(&self) -> &BindGroupLayout {
        &self.star_map
    }

    pub fn sun(&self) -> &BindGroupLayout {
        &self.sun
    }

    pub fn tone_map(&self) -> &BindGroupLayout {
        &self.tone_map
    }
}
