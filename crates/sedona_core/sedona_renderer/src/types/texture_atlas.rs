use crate::types::Pixels;
use wgpu::{
    Device, Extent3d, Origin3d, Queue, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture,
    TextureAspect, TextureDescriptor, TextureDimension, TextureUsages, TextureView,
    TextureViewDescriptor,
};

pub struct TextureAtlas {
    pub texture: Texture,
    pub view: TextureView,
    pub regions: Vec<AtlasRegion>,
}

pub struct AtlasRegion {
    pub uv_offset: (f32, f32),
    pub uv_extent: (f32, f32),
    pub pixel_offset: (u32, u32),
    pub pixel_extent: (u32, u32),
}

impl AtlasRegion {
    pub fn new(
        images: &[impl Pixels],
        device: &Device,
        queue: &Queue,
        padding: u32,
    ) -> TextureAtlas {
        assert!(
            !images.is_empty(),
            "Cannot create atlas from empty image list"
        );

        let (tile_width, tile_height) = images[0].dimensions();
        let expected_format = images[0].format();
        let expected_bpp = images[0].bytes_per_pixel();

        let count = images.len();
        let columns = (count as f32).sqrt().ceil() as u32;
        let rows = ((count as u32 + columns - 1) / columns);

        let atlas_width = columns * (tile_width + padding);
        let atlas_height = rows * (tile_height + padding);

        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: atlas_width,
                height: atlas_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: expected_format,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let mut regions = Vec::with_capacity(images.len());

        for (i, image) in images.iter().enumerate() {
            let (w, h) = image.dimensions();
            if w != tile_width || h != tile_height {
                panic!("Image at index {} has mismatched size", i);
            }
            if image.format() != expected_format {
                panic!("Image at index {} has mismatched format", i);
            }
            if image.bytes_per_pixel() != expected_bpp {
                panic!("Image at index {} has mismatched bytes-per-pixel", i);
            }

            let col = (i as u32) % columns;
            let row = (i as u32) / columns;

            let x_offset = col * (tile_width + padding);
            let y_offset = row * (tile_height + padding);

            queue.write_texture(
                TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: Origin3d {
                        x: x_offset,
                        y: y_offset,
                        z: 0,
                    },
                    aspect: TextureAspect::All,
                },
                image.raw_data(),
                TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(expected_bpp * tile_width),
                    rows_per_image: Some(tile_height),
                },
                Extent3d {
                    width: tile_width,
                    height: tile_height,
                    depth_or_array_layers: 1,
                },
            );

            regions.push(AtlasRegion {
                uv_offset: (
                    x_offset as f32 / atlas_width as f32,
                    y_offset as f32 / atlas_height as f32,
                ),
                uv_extent: (
                    tile_width as f32 / atlas_width as f32,
                    tile_height as f32 / atlas_height as f32,
                ),
                pixel_offset: (x_offset, y_offset),
                pixel_extent: (tile_width, tile_height),
            });
        }

        let view = texture.create_view(&TextureViewDescriptor::default());

        TextureAtlas {
            texture,
            view,
            regions,
        }
    }
}
