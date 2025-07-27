use image::{DynamicImage, GenericImageView, ImageResult};
use std::path::Path;

pub fn load_image_from_file(path: &str) -> ImageResult<(DynamicImage, u32, u32)> {
    let image = image::open(Path::new(path))?;
    let (width, height) = image.dimensions();

    Ok((image, width, height))
}

pub fn load_rgba8_from_file(path: &str) -> ImageResult<(Vec<u8>, u32, u32)> {
    let (image, width, height) = load_image_from_file(path)?;
    let rgba = image.to_rgba8();
    let data = rgba.into_raw();

    Ok((data, width, height))
}
