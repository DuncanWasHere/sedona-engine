@group(0) @binding(0) var tex_sampler: sampler;
@group(0) @binding(1) var tex: texture_2d<f32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_idx: u32) -> VertexOutput {
    let uv = vec2(f32(vertex_idx & 2u), f32((vertex_idx << 1u) & 2u));
    let pos = vec4(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, 0.0, 1.0);
    return VertexOutput(pos, uv);
}

@fragment
fn fs_rgba(vout: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(tex, tex_sampler, vout.tex_coords);
}

@fragment
fn fs_luminance(vout: VertexOutput) -> @location(0) f32 {
    // Calculate luminance here instead of just sampling
    let color = textureSample(tex, tex_sampler, vout.tex_coords);
    let luminance = dot(color.rgb, vec3(0.2126, 0.7152, 0.0722)); // Rec. 709
    return log2(luminance);
}
