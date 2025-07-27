@group(0) @binding(0) var<uniform> screen_uniforms: ScreenUniforms;
@group(0) @binding(1) var<uniform> lens_flare_uniforms: LensFlareUniforms;

@group(1) @binding(0) var texture_sampler: sampler;
@group(1) @binding(1) var hdr_texture: texture_2d<f32>;

@group(2) @binding(0) var bloom_sampler: sampler;
@group(2) @binding(1) var bloom_texture: texture_2d<f32>;

struct ScreenUniforms {
    width: f32,
    height: f32,
    _padding0: u32,
    _padding1: u32,
};

struct LensFlareUniforms {
    intensity: f32,
    ghost_count: u32,
    halo_size: f32,
    chromatic_offset: f32,
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

    let scene_color = textureSample(hdr_texture, texture_sampler, uv);
    let bloom_color = textureSample(bloom_texture, texture_sampler, uv).rgb;

    // Additively blend bloom
    let final_color = scene_color.rgb + bloom_color;

    return vec4<f32>(final_color, scene_color.a);
}
