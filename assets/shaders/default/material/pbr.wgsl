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
    output.world_normal = normalize(model_uniforms.model_normal_matrix * input.local_normal);
    output.tangent = input.tangent;
    output.color = input.color;
    output.tex_coords = input.tex_coords;

    return output;
}

fn geometry_schlick_ggx(n_dot_v: f32, k: f32) -> f32 {
    return n_dot_v / (n_dot_v * (1.0 - k) + k);
}

fn compute_cascade_index(fragment_view_position: vec4<f32>) -> u32 {
    let view_z = -fragment_view_position.z / fragment_view_position.w;

    // light_uniforms.cascade_splits = vec4(near, split1, split2, split3)
    // skip cascade_splits[0] (near) as you mentioned
    if (view_z < light_uniforms.cascade_splits[1]) {
        return 0u;
    } else if (view_z < light_uniforms.cascade_splits[2]) {
        return 1u;
    } else {
        return 2u;
    }
}

fn compute_light_position(cascade_index: u32, fragment_world_position: vec4<f32>) -> vec4<f32> {
    return light_uniforms.light_view_projections[cascade_index] * fragment_world_position;
}

fn sample_shadow(cascade_index: u32, light_position: vec4<f32>, world_normal: vec4<f32>) -> f32 {
    let normal_bias = 0.0005;
    let slope_scale = 0.01;

    let angle = max(dot(world_normal, lighting_uniforms.directional_angle), 0.0);
    let slope_bias = slope_scale * (1.0 - clamp(angle, 0.0, 0.95));

    let bias = normal_bias + slope_bias;

    let light_ndc = light_position.xyz / light_position.w;
    let uv = light_ndc.xy * vec2f(0.5, -0.5) + 0.5;
    let compare_depth = light_ndc.z - bias;

    var shadow_factor = 1.0;
    if (all(uv >= vec2(0.0)) && all(uv <= vec2(1.0))) {
        shadow_factor = textureSampleCompare(shadow_textures, shadow_sampler, uv, cascade_index, compare_depth);
    }

    return shadow_factor;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let fragment_world_position = input.world_position;
    let view_direction_world = normalize(view_uniforms.view_position.xyz - fragment_world_position.xyz);

    // Compute TBN matrix from vertex data
    let surface_normal_world = normalize(input.world_normal.xyz);
    let tangent_world = normalize(input.tangent.xyz);
    let bitangent_world = normalize(cross(surface_normal_world, tangent_world) * input.tangent.w);

    // Sample and reconstruct normal from normal map
    let normal_sample_tangent = textureSample(normal_texture, texture_sampler, input.tex_coords).rgb * 2.0 - vec3<f32>(1.0);
    let normal_world = normalize(
        normal_sample_tangent.x * tangent_world +
        normal_sample_tangent.y * bitangent_world +
        normal_sample_tangent.z * surface_normal_world
    );

    // Base color with vertex color and factor
    let base_color_sample = textureSample(base_color_texture, texture_sampler, input.tex_coords).rgb;
    let base_color_final = base_color_sample * input.color.rgb * material_uniforms.base_color_factor.rgb;

    // Metallic and roughness from texture
    let metallic_roughness_sample = textureSample(metallic_roughness_texture, texture_sampler, input.tex_coords).rgb;
    let metallic_value = clamp(metallic_roughness_sample.b * material_uniforms.metallic_factor, 0.0, 1.0);
    let roughness_value = clamp(metallic_roughness_sample.g * material_uniforms.roughness_factor, 0.2, 1.0);

    // Fresnel reflectance at normal incidence
    let reflectance_default = vec3<f32>(0.04);
    let fresnel_reflectance = mix(reflectance_default, base_color_final, metallic_value);

    // Directional light
    let light_direction = normalize(lighting_uniforms.directional_angle.xyz);
    let light_color = lighting_uniforms.directional_color.rgb;
    let light_intensity = lighting_uniforms.directional_strength;

    let light_dot = max(dot(normal_world, light_direction), 0.0);
    let half_vector = normalize(view_direction_world + light_direction);
    let normal_dot_half = max(dot(normal_world, half_vector), 0.0);

    // Cook-Torrance BRDF components
    let alpha_roughness = roughness_value * roughness_value;

    // Distribution function (GGX)
    let normal_dot_half_squared = normal_dot_half * normal_dot_half;
    let roughness_squared = alpha_roughness * alpha_roughness;
    let denominator_distribution = (normal_dot_half_squared * (roughness_squared - 1.0) + 1.0);
    let distribution_ggx = roughness_squared / (3.14159 * denominator_distribution * denominator_distribution);

    // Smith Schlick-GGX
    let k_visibility = (alpha_roughness + 1.0) * (alpha_roughness + 1.0) / 8.0;
    let geometry = geometry_schlick_ggx(max(dot(normal_world, view_direction_world), 0.0), k_visibility) *
                   geometry_schlick_ggx(light_dot, k_visibility);

    // Fresnel term (Schlick approximation)
    let fresnel_schlick = fresnel_reflectance + (vec3<f32>(1.0) - fresnel_reflectance) *
                          pow(1.0 - max(dot(half_vector, view_direction_world), 0.0), 5.0);

    let fragment_view_position = view_uniforms.view_matrix * fragment_world_position;
    let cascade_index = compute_cascade_index(fragment_view_position);
    let fragment_light_position = compute_light_position(cascade_index, fragment_world_position);

    let shadow_factor = sample_shadow(cascade_index, fragment_light_position, input.world_normal);

    // Specular
    let specular_term = (distribution_ggx * geometry * fresnel_schlick) /
                        (4.0 * max(dot(normal_world, view_direction_world), 0.0) * light_dot + 0.001);

    // Diffuse term (Lambert)
    let diffuse_color = base_color_final * (1.0 - metallic_value);
    let diffuse_term = diffuse_color / 3.14159;

    let ambient_term = lighting_uniforms.ambient_color.rgb * lighting_uniforms.ambient_strength * diffuse_color;

    let shadow_hardness = 0.4; // tweakable
    let ambient_shadow_factor = mix(shadow_hardness, 1.0, shadow_factor);

    // Final light contribution
    let directional_light = light_color * light_intensity * light_dot *
                            (diffuse_term + specular_term) * shadow_factor;
    let light_contribution = directional_light + ambient_term * ambient_shadow_factor;

    // Emissive
    let emissive_color_sample = textureSample(emissive_texture, texture_sampler, input.tex_coords).rgb;
    let emissive_final = emissive_color_sample * material_uniforms.emissive_factor * material_uniforms.emissive_multiplier;

    // Occlusion
    let occlusion_value = textureSample(occlusion_texture, texture_sampler, input.tex_coords).r;
    let occlusion_final = mix(1.0, occlusion_value, material_uniforms.occlusion_strength);

    let final_color = light_contribution * occlusion_final + emissive_final;

    // Alpha
    let alpha_sample = textureSample(base_color_texture, texture_sampler, input.tex_coords).a;
    let final_alpha = alpha_sample * material_uniforms.alpha_multiplier;

    // Fog
    let view_distance = length(view_uniforms.view_position.xyz - fragment_world_position.xyz);
    let fog_blend = clamp(
        (view_distance - lighting_uniforms.fog_start) /
        (lighting_uniforms.fog_end - lighting_uniforms.fog_start),
        0.0, 1.0
    );
    let fog_factor = pow(fog_blend, lighting_uniforms.fog_strength);

    let lit_fog_color = lighting_uniforms.fog_color.rgb * lighting_uniforms.directional_strength;
    let fogged_color = mix(final_color, lit_fog_color, fog_factor);


    return vec4(fogged_color, final_alpha);
}
