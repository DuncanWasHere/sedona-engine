use crate::utils::load_pixels_from_file;
use std::hash::{DefaultHasher, Hash, Hasher};
use wgpu::TextureFormat;

pub trait Pixels {
    fn compute_hash(&self) -> u64;
    fn raw_data(&self) -> &[u8];
    fn format(&self) -> TextureFormat;
    fn bytes_per_pixel(&self) -> u32;
    fn dimensions(&self) -> (u32, u32);
}

pub struct Rgba8Pixels {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
}

impl Rgba8Pixels {
    pub fn from_image_path(path: &str, srgb: bool) -> Option<Self> {
        let format = if srgb {
            TextureFormat::Rgba8UnormSrgb
        } else {
            TextureFormat::Rgba8Unorm
        };

        if let Some((data, width, height)) = load_pixels_from_file(path, format) {
            Some(Self {
                data,
                width,
                height,
                format,
            })
        } else {
            None
        }
    }

    pub fn from_image_paths(paths: &[String], srgb: bool) -> Option<Vec<Self>> {
        paths
            .iter()
            .map(|path| Rgba8Pixels::from_image_path(path, srgb))
            .collect()
    }
}

impl Pixels for Rgba8Pixels {
    fn compute_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.data.hash(&mut hasher);
        self.width.hash(&mut hasher);
        self.height.hash(&mut hasher);
        hasher.finish()
    }

    fn raw_data(&self) -> &[u8] {
        &self.data
    }

    fn format(&self) -> TextureFormat {
        self.format
    }

    fn bytes_per_pixel(&self) -> u32 {
        4
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
