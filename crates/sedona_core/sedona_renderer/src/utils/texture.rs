use crate::types::{Pixels, Rgba8Pixels};
use log::{debug, warn};
use std::path::Path;
use wgpu::*;

pub fn load_pixels_from_file(path: &str, format: TextureFormat) -> Option<(Vec<u8>, u32, u32)> {
    match format {
        TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => {
            match sedona_io::load_rgba8_from_file(path) {
                Ok(result) => Some(result),
                Err(err) => {
                    warn!("Failed to load image '{}': {}", path, err);
                    None
                }
            }
        }
        _ => {
            warn!("Image format {:?} is not supported", format);
            None
        }
    }
}

pub fn create_texture(
    images: &[impl Pixels],
    dimension: TextureDimension,
    device: &Device,
    queue: &Queue,
) -> Texture {
    assert!(
        !images.is_empty(),
        "create_texture_2d cannot be used to create an empty texture"
    );

    let expected_format = images[0].format();
    let expected_bpp = images[0].bytes_per_pixel();
    let (width, height) = images[0].dimensions();

    let texture = device.create_texture(&TextureDescriptor {
        label: None,
        size: Extent3d {
            width,
            height,
            depth_or_array_layers: images.len() as u32,
        },
        format: expected_format,
        mip_level_count: 1,
        sample_count: 1,
        dimension,
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        view_formats: &[],
    });

    for (i, pixels) in images.iter().enumerate() {
        let (w, h) = pixels.dimensions();
        if w != width || h != height {
            panic!(
                "All images must have the same dimensions. Mismatch at index {}",
                i
            );
        }
        if pixels.format() != expected_format {
            panic!(
                "All images must have the same format. Mismatch at index {}",
                i
            );
        }
        if pixels.bytes_per_pixel() != expected_bpp {
            panic!(
                "All images must have the same bytes per pixel. Mismatch at index {}",
                i
            );
        }

        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d {
                    x: 0,
                    y: 0,
                    z: i as u32,
                },
                aspect: TextureAspect::All,
            },
            pixels.raw_data(),
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(expected_bpp * width),
                rows_per_image: Some(height),
            },
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }

    texture
}

pub fn create_empty_texture(
    width: u32,
    height: u32,
    layers: u32,
    format: TextureFormat,
    device: &Device,
) -> Texture {
    device.create_texture(&TextureDescriptor {
        label: None,
        size: Extent3d {
            width,
            height,
            depth_or_array_layers: layers,
        },
        format,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        view_formats: &[],
    })
}

pub fn create_texture_2d_from_path(
    path: &str,
    device: &Device,
    queue: &Queue,
) -> Option<(Texture, TextureView)> {
    if let Some(image) = Rgba8Pixels::from_image_path(path, true) {
        let texture = create_texture(&[image], TextureDimension::D2, device, queue);

        let texture_view = texture.create_view(&TextureViewDescriptor::default());

        Some((texture, texture_view))
    } else {
        None
    }
}

pub fn create_texture_array_from_paths(
    paths: &[String],
    device: &Device,
    queue: &Queue,
) -> Option<(Texture, TextureView)> {
    if let Some(images) = Rgba8Pixels::from_image_paths(&paths, true) {
        let texture = create_texture(images.as_slice(), TextureDimension::D2, device, queue);

        let texture_view = texture.create_view(&TextureViewDescriptor {
            dimension: Some(TextureViewDimension::D2Array),
            ..Default::default()
        });

        Some((texture, texture_view))
    } else {
        None
    }
}

pub fn create_cube_map_from_dir(
    path: &str,
    device: &Device,
    queue: &Queue,
) -> Option<(Texture, TextureView)> {
    let face_paths = [
        format!("{}/px.png", path),
        format!("{}/nx.png", path),
        format!("{}/py.png", path),
        format!("{}/ny.png", path),
        format!("{}/pz.png", path),
        format!("{}/nz.png", path),
    ];

    if let Some(images) = Rgba8Pixels::from_image_paths(&face_paths, true) {
        let texture = create_texture(images.as_slice(), TextureDimension::D2, device, queue);

        let texture_view = texture.create_view(&TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..Default::default()
        });

        Some((texture, texture_view))
    } else {
        None
    }
}
