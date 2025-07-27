@group(0) @binding(0) var<uniform> screen_uniforms: ScreenUniforms;
@group(0) @binding(1) var<uniform> blur_uniforms: BlurUniforms;

@group(1) @binding(0) var texture_sampler: sampler;
@group(1) @binding(1) var screen_color: texture_2d<f32>;

struct ScreenUniforms {
    width: f32,
    height: f32,
    _padding0: u32,
    _padding1: u32,
};

struct BlurUniforms {
    direction: vec2<f32>,  // (1, 0) for horizontal, (0, 1) for vertical
    radius: f32,
    _padding0: u32,
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

    let direction = normalize(blur_uniforms.direction);
    let texel_size = 1.0 / resolution;

    let radius = blur_uniforms.radius; // this can be < 1.0
    let sigma = radius * 0.5;

    let sample_count = 9u; // tweakable (should be odd)
    let half_count = f32(i32(sample_count) / 2);

    var color = vec3<f32>(0.0);
    var weight_sum = 0.0;

    for (var i: u32 = 0u; i < sample_count; i = i + 1u) {
        let offset = f32(i) - half_count;
        let sample_offset = direction * texel_size * offset * radius;
        let sample_uv = uv + sample_offset;

        let weight = exp(-0.5 * (offset * offset) / (sigma * sigma));
        let sample_color = textureSample(screen_color, texture_sampler, sample_uv).rgb;

        color += sample_color * weight;
        weight_sum += weight;
    }

    color /= weight_sum;
    return vec4<f32>(color, 1.0);
}

