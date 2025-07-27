use crate::utils::defaults::{
    BASE_COLOR_DEFAULT, BASE_COLOR_TEXTURE_FORMAT, EMISSIVE_DEFAULT, EMISSIVE_TEXTURE_FORMAT,
    METALLIC_ROUGHNESS_DEFAULT, METALLIC_ROUGHNESS_TEXTURE_FORMAT, NORMAL_DEFAULT,
    NORMAL_TEXTURE_FORMAT, OCCLUSION_DEFAULT, OCCLUSION_TEXTURE_FORMAT,
};
use wgpu::TexelCopyBufferLayout;
use wgpu::{
    Device, Extent3d, Queue, TextureDescriptor, TextureDimension, TextureUsages, TextureView,
};

pub struct FallbackTextures {
    pub base_color: TextureView,
    pub metallic_roughness: TextureView,
    pub normal: TextureView,
    pub emissive: TextureView,
    pub occlusion: TextureView,
}

macro_rules! create_fallback_textures {
    (
        device: $device:expr,
        queue: $queue:expr,
        extent: $extent:expr,
        $( ($format_const:ident, $color_const:ident, $field_name:ident) ),* $(,)?
    ) => {
        {
            $(
                let $field_name = {
                    let label = concat!("texture_fallback_", stringify!($field_name));
                    let texture = $device.create_texture(&TextureDescriptor {
                        label: Some(label),
                        size: $extent,
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: TextureDimension::D2,
                        format: $format_const,
                        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                        view_formats: &[],
                    });

                    let pixel: [u8; 4] = $color_const;

                    $queue.write_texture(
                        texture.as_image_copy(),
                        &pixel,
                        TexelCopyBufferLayout {
                            offset: 0,
                            bytes_per_row: Some(4),
                            rows_per_image: Some(1),
                        },
                        $extent,
                    );

                    texture.create_view(&Default::default())
                };
            )*

            FallbackTextures {
                $(
                    $field_name,
                )*
            }
        }
    };
}

impl FallbackTextures {
    pub fn new(device: &Device, queue: &Queue) -> Self {
        let texture_extent = Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };

        create_fallback_textures! {
            device: device,
            queue: queue,
            extent: texture_extent,
            (BASE_COLOR_TEXTURE_FORMAT, BASE_COLOR_DEFAULT, base_color),
            (METALLIC_ROUGHNESS_TEXTURE_FORMAT, METALLIC_ROUGHNESS_DEFAULT, metallic_roughness),
            (NORMAL_TEXTURE_FORMAT, NORMAL_DEFAULT, normal),
            (EMISSIVE_TEXTURE_FORMAT, EMISSIVE_DEFAULT, emissive),
            (OCCLUSION_TEXTURE_FORMAT, OCCLUSION_DEFAULT, occlusion),
        }
    }
}
