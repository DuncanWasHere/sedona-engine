@group(0) @binding(0) var<uniform> view_uniforms: ViewUniforms;

@group(1) @binding(0) var<uniform> model_uniforms: ModelUniforms;

struct ViewUniforms {
    view_projection_matrix: mat4x4<f32>,
    view_projection_matrix_inverse: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    view_position: vec4<f32>,
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

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let world_pos = model_uniforms.model_matrix * input.local_position;
    let world_normal = normalize((model_uniforms.model_normal_matrix * input.local_normal).xyz);

    out.position = view_uniforms.view_projection_matrix * world_pos;
    out.normal = world_normal;

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let encoded_normal = normalize(input.normal) * 0.5 + vec3<f32>(0.5);
    return vec4<f32>(encoded_normal, 1.0);
}
