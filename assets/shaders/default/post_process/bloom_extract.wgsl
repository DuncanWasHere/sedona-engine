@group(0) @binding(0) var<uniform> screen_uniforms: ScreenUniforms;
@group(0) @binding(1) var<uniform> bloom_uniforms: BloomUniforms;

@group(1) @binding(0) var texture_sampler: sampler;
@group(1) @binding(1) var screen_color: texture_2d<f32>;

struct ScreenUniforms {
    width: f32,
    height: f32,
    _padding0: u32,
    _padding1: u32,
};

struct BloomUniforms {
    threshold: f32,
    soft_knee: f32,
    intensity: f32,
    radius: f32,
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
    let resolution = vec2(screen_uniforms.width, screen_uniforms.height);
    let uv = coord.xy / resolution;

    let threshold = bloom_uniforms.threshold;
    let soft_knee = bloom_uniforms.soft_knee;
    let intensity = bloom_uniforms.intensity;

    // Sample HDR screen color
    let color = textureSample(screen_color, texture_sampler, uv);
    let brightness = max(max(color.r, color.g), color.b);

    // Apply soft threshold (approximated from Unity bloom model)
    let knee = threshold * soft_knee;
    let softness = clamp((brightness - threshold + knee) / (2.0 * knee), 0.0, 1.0);
    let contribution = max(brightness - threshold, 0.0) + knee * softness;

    let bloom = color.rgb * contribution * intensity;

    return vec4<f32>(bloom, 1.0);
}
