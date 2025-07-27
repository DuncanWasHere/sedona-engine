@group(0) @binding(0) var<uniform> view_uniforms: ViewUniforms;

@group(1) @binding(0) var<uniform> light_uniforms: LightUniforms;
@group(1) @binding(1) var<uniform> lighting_uniforms: LightingUniforms;
@group(1) @binding(2) var<storage, read> lights: array<Light>;

@group(2) @binding(0) var shadow_sampler: sampler_comparison;
@group(2) @binding(1) var shadow_textures: texture_depth_2d_array;

@group(3) @binding(0) var <uniform> material_uniforms: MaterialUniforms;
@group(3) @binding(1) var texture_sampler: sampler;
@group(3) @binding(2) var base_color_texture: texture_2d<f32>;
@group(3) @binding(3) var metallic_roughness_texture: texture_2d<f32>;
@group(3) @binding(4) var normal_texture: texture_2d<f32>;
@group(3) @binding(5) var emissive_texture: texture_2d<f32>;
@group(3) @binding(6) var occlusion_texture: texture_2d<f32>;

@group(4) @binding(0) var<uniform> model_uniforms: ModelUniforms;

struct ViewUniforms {
    view_projection_matrix: mat4x4<f32>,
    view_projection_matrix_inverse: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    view_position: vec4<f32>,
};

struct LightUniforms {
    light_view_projections: array<mat4x4<f32>, 3>,
    cascade_splits: vec4<f32>,
};

struct LightingUniforms {
    directional_angle: vec4<f32>,
    directional_color: vec4<f32>,
    ambient_color: vec4<f32>,
    fog_color: vec4<f32>,
    fog_start: f32,
    fog_end: f32,
    fog_strength: f32,
    ambient_strength: f32,
    directional_strength: f32,
    contrast: f32,
    gamma: f32,
    saturation: f32,
};

struct Light {
    light_color: vec4<f32>,
    light_position: vec4<f32>,
    light_intensity: f32,
    light_falloff: f32,
    light_type: u32, // 1 = point, 2 = spot
    padding0: f32,
    light_spot_direction: vec4<f32>,
    light_spot_cutoff: f32,
    padding1: f32,
    padding2: f32,
    padding3: f32,
};

struct ModelUniforms {
    model_matrix: mat4x4<f32>,
    model_normal_matrix: mat4x4<f32>,
};

struct MaterialUniforms {
    base_color_factor: vec4<f32>,
    emissive_factor: vec3<f32>,
    emissive_multiplier: f32,
    metallic_factor: f32,
    roughness_factor: f32,
    transmission_factor: f32,
    occlusion_strength: f32,
    alpha_multiplier: f32,
    alpha_cutoff: f32,
    normal_scale: f32,
    ior: f32,
};

struct VertexInput {
    @location(0) local_position: vec4<f32>,
    @location(1) local_normal: vec4<f32>,
    @location(2) tangent: vec4<f32>,
    @location(3) color: vec4<f32>,
    @location(4) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec4<f32>,
    @location(2) tangent: vec4<f32>,
    @location(3) color: vec4<f32>,
    @location(4) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let world_position = model_uniforms.model_matrix * input.local_position;

    output.clip_position = view_uniforms.view_projection_matrix * world_position;
    output.world_position = world_position;
    output.world_normal = normalize((model_uniforms.model_normal_matrix * input.local_normal).xyz).xyzx;
    output.tangent = input.tangent;
    output.color = input.color;
    output.tex_coords = input.tex_coords;

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let base_color_sample = textureSample(base_color_texture, texture_sampler, input.tex_coords).rgb;
    let base_color_final = base_color_sample * input.color.rgb * material_uniforms.base_color_factor.rgb;

    let alpha_sample = textureSample(base_color_texture, texture_sampler, input.tex_coords).a;
    let final_alpha = alpha_sample * material_uniforms.alpha_multiplier;

    return vec4<f32>(base_color_final, final_alpha);
}
