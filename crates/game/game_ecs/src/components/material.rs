use sedona_ecs::component;
use sedona_renderer::types::ShaderFlags;

#[component]
pub struct MaterialData {
    pub shader_flags: ShaderFlags,
    pub base_color_texture: Option<u64>,
    pub metallic_roughness_texture: Option<u64>,
    pub normal_texture: Option<u64>,
    pub emissive_texture: Option<u64>,
    pub occlusion_texture: Option<u64>,
}
