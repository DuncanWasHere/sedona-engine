@group(0) @binding(0) var<uniform> light_view_uniforms: LightViewUniforms;
@group(1) @binding(0) var<uniform> model_uniforms: ModelUniforms;

struct LightViewUniforms {
    light_view_projection_matrix: mat4x4<f32>,
};

struct ModelUniforms {
    model_matrix: mat4x4<f32>,
    model_normal_matrix: mat4x4<f32>,
};

struct VertexInput {
    @location(0) local_position: vec4<f32>,
    @location(1) local_normal: vec4<f32>,
    @location(2) tangent: vec4<f32>,
    @location(3) color: vec4<f32>,
    @location(4) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> @builtin(position) vec4<f32> {
    return light_view_uniforms.light_view_projection_matrix * model_uniforms.model_matrix * input.local_position;
}
