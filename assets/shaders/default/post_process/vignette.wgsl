@group(0) @binding(0) var<uniform> post_process_uniforms: PostProcessUniforms;
@group(0) @binding(1) var<uniform> vignette_uniforms: VignetteUniforms;

@group(1) @binding(0) var texture_sampler: sampler;
@group(1) @binding(1) var screen_color: texture_2d<f32>;
@group(1) @binding(2) var screen_normal: texture_2d<f32>;
@group(1) @binding(3) var screen_depth: texture_2d<f32>;

struct PostProcessUniforms {
    screen_size: vec2<f32>,
    _padding0: f32,
    _padding1: f32,
};

struct VignetteUniforms {
    center: vec2<f32>,
    radius: f32,
    softness: f32,
    color: vec3<f32>,
    intensity: f32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;

    let clip_position = array(
        vec2<f32>(-1, 3),
        vec2<f32>(-1,-1),
        vec2<f32>( 3,-1),
    );

    output.position = vec4<f32>(clip_position[vertex_index], 1, 1);

    return output;
}

@fragment
fn fs_main(@builtin(position) coord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution = post_process_uniforms.screen_size;
    let uv = coord.xy / resolution;

    let center = vignette_uniforms.center;
    let radius = vignette_uniforms.radius;
    let softness = vignette_uniforms.softness;
    let vignette_color = vignette_uniforms.color;
    let intensity = vignette_uniforms.intensity;

    let dist = distance(uv, center);
    let vignette = clamp((radius - dist) / softness, 0.0, 1.0);

    let color = textureSample(screen_color, texture_sampler, uv);

    let result_color = mix(vignette_color, color.rgb, vignette);
    let final_color = mix(color.rgb, result_color, intensity);

    return vec4<f32>(final_color, color.a);
}
