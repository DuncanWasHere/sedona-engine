@group(0) @binding(0) var texture_sampler: sampler;
@group(1) @binding(0) var depth_sampler: sampler_comparison;
@group(1) @binding(1) var depth_tex: texture_depth_2d_array;
@group(2) @binding(0) var<uniform> layer_index: u32;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_idx: u32) -> VertexOutput {
    let uv = vec2<f32>(f32(vertex_idx & 2u), f32((vertex_idx << 1u) & 2u));
    let pos = vec4(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, 0.0, 1.0);
    return VertexOutput(pos, uv);
}

@fragment
fn fs_main(vout: VertexOutput) -> @location(0) vec4<f32> {
    // Read raw depth (0.0 to 1.0) and visualize it as grayscale
    let d = textureSample(depth_tex, texture_sampler, vout.tex_coords, i32(layer_index));
    return vec4<f32>(d, d, d, 1.0);
}
