use crate::GameResources;
use glam::{Vec3, Vec4};
use gltf::image::Format;
use gltf::material::AlphaMode;
use gltf::{Primitive, Texture};
use sedona_renderer::TextureFormat;
use sedona_renderer::renderer::Renderer;
use sedona_renderer::types::{MaterialUniforms, Rgba8Pixels, ShaderFlags};

pub(crate) fn process_material(
    g_mesh: &Primitive,
    g_images: &[gltf::image::Data],
    resources: &mut GameResources,
) -> usize {
    let g_material = g_mesh.material();
    let g_pbr = g_material.pbr_metallic_roughness();

    // Textures
    let base_color_texture = if let Some(info) = g_pbr.base_color_texture() {
        Some(process_texture(
            info.texture(),
            TextureFormat::Rgba8UnormSrgb,
            g_images,
            &mut resources.renderer,
        ))
    } else {
        None
    };

    let metallic_roughness_texture = if let Some(info) = g_pbr.metallic_roughness_texture() {
        Some(process_texture(
            info.texture(),
            TextureFormat::Rgba8Unorm,
            g_images,
            &mut resources.renderer,
        ))
    } else {
        None
    };

    let normal_texture = if let Some(normal) = g_material.normal_texture() {
        Some(process_texture(
            normal.texture(),
            TextureFormat::Rgba8Unorm,
            g_images,
            &mut resources.renderer,
        ))
    } else {
        None
    };

    let emissive_texture = if let Some(info) = g_material.emissive_texture() {
        Some(process_texture(
            info.texture(),
            TextureFormat::Rgba8UnormSrgb,
            g_images,
            &mut resources.renderer,
        ))
    } else {
        None
    };

    let occlusion_texture = if let Some(occlusion) = g_material.occlusion_texture() {
        Some(process_texture(
            occlusion.texture(),
            TextureFormat::Rgba8Unorm,
            g_images,
            &mut resources.renderer,
        ))
    } else {
        None
    };

    let textures = [
        base_color_texture,
        metallic_roughness_texture,
        normal_texture,
        emissive_texture,
        occlusion_texture,
    ];

    // Shader
    let shader = if g_material.unlit() {
        "assets/shaders/default/material/unlit.wgsl"
    } else {
        "assets/shaders/default/material/pbr.wgsl"
    };

    // Shader Flags
    let mut shader_flags = ShaderFlags::NONE;

    match g_material.alpha_mode() {
        AlphaMode::Blend => {
            shader_flags |= ShaderFlags::BLEND_ALPHA | ShaderFlags::DEPTH_TEST;
        }
        _ => {
            shader_flags |= ShaderFlags::DEPTH_TEST | ShaderFlags::DEPTH_WRITE;
        }
    };
    if g_material.double_sided() {
        shader_flags |= ShaderFlags::DOUBLE_SIDED;
    }

    // Material Uniforms
    let uniforms = MaterialUniforms {
        base_color_factor: Vec4::from_array(g_pbr.base_color_factor()),
        emissive_factor: Vec3::from_array(g_material.emissive_factor()),
        emissive_multiplier: g_material.emissive_strength().unwrap_or(1.0),
        metallic_factor: g_pbr.metallic_factor(),
        roughness_factor: g_pbr.roughness_factor(),
        transmission_factor: g_material
            .transmission()
            .map(|t| t.transmission_factor())
            .unwrap_or(1.0),
        occlusion_strength: g_material
            .occlusion_texture()
            .map(|o| o.strength())
            .unwrap_or(1.0),
        alpha_multiplier: 1.0,
        alpha_cutoff: g_material.alpha_cutoff().unwrap_or(0.5),
        normal_scale: g_material
            .normal_texture()
            .map(|n| n.scale())
            .unwrap_or(1.0),
        ior: g_material.ior().unwrap_or(0.0),
    };

    resources
        .renderer
        .create_render_material(uniforms, &textures, shader, shader_flags)
        .unwrap()
}

fn process_texture(
    g_texture: Texture,
    _unused_format: TextureFormat,
    g_images: &[gltf::image::Data],
    renderer: &mut Renderer,
) -> u64 {
    let image_data = &g_images[g_texture.source().index()];
    let width = image_data.width;
    let height = image_data.height;

    let (converted_pixels, format) = match image_data.format {
        Format::R8G8B8 => {
            let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
            for chunk in image_data.pixels.chunks_exact(3) {
                rgba.extend_from_slice(&[chunk[0], chunk[1], chunk[2], 255]);
            }
            (rgba, TextureFormat::Rgba8UnormSrgb)
        }
        Format::R8G8B8A8 => (image_data.pixels.clone(), TextureFormat::Rgba8UnormSrgb),
        Format::R8 => {
            let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
            for &l in &image_data.pixels {
                rgba.extend_from_slice(&[l, l, l, 255]);
            }
            (rgba, TextureFormat::Rgba8UnormSrgb)
        }
        Format::R16G16B16 => {
            let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
            for chunk in image_data.pixels.chunks_exact(6) {
                let r = u16::from_le_bytes([chunk[0], chunk[1]]) >> 8;
                let g = u16::from_le_bytes([chunk[2], chunk[3]]) >> 8;
                let b = u16::from_le_bytes([chunk[4], chunk[5]]) >> 8;
                rgba.extend_from_slice(&[r as u8, g as u8, b as u8, 255]);
            }
            (rgba, TextureFormat::Rgba8UnormSrgb)
        }
        _ => {
            panic!("Unsupported GLTF image format: {:?}", image_data.format);
        }
    };
    let pixels = Rgba8Pixels {
        data: converted_pixels,
        width,
        height,
        format,
    };
    renderer.create_material_texture(pixels)
}
