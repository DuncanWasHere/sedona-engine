@group(0) @binding(0) var<uniform> view_uniforms: ViewUniforms;

@group(1) @binding(0) var texture_sampler: sampler;
@group(1) @binding(1) var environment_texture: texture_cube<f32>;

struct ViewUniforms {
    view_projection_matrix: mat4x4<f32>,
    view_projection_matrix_inverse: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    view_position: vec4<f32>,
};

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) clip_position: vec4<f32>,
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
  output.clip_position = output.position;

  return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
  let world_direction = normalize((view_uniforms.view_projection_matrix_inverse * input.clip_position).xyz);
  let sky_color = textureSample(environment_texture, texture_sampler, world_direction);

  return sky_color;
}
