// Global Bind Group
@group(0) @binding(0) var<uniform> camera_view_uniforms: CameraViewUniforms;

// Sky Object Bind Group
@group(1) @binding(0) var<uniform> cloud_uniforms: CloudUniforms;
@group(1) @binding(1) var<uniform> sun_uniforms: SunUniforms;
@group(1) @binding(2) var texture_sampler: sampler;
@group(1) @binding(3) var texture: texture_2d<f32>;

struct CameraViewUniforms {
    view_projection_matrix: mat4x4<f32>,
    view_projection_matrix_inverse: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    world_position: vec4<f32>,
};

struct CloudUniforms {
    tint: vec4<f32>,
    u_offset: f32,
    v_offset: f32,
    _padding0: u32,
    _padding1: u32,
    _padding3: vec4<u32>,
};

struct SunUniforms {
    tint: vec4<f32>,
    rotation: vec4<f32>,
    size: f32,
    padding0: u32,
    padding1: u32,
    padding2: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) clip_position: vec4<f32>,
};

const PI: f32 = 3.14159;

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
    // Reconstruct normalized world direction from clip space
    let world_direction = normalize((camera_view_uniforms.view_projection_matrix_inverse * input.clip_position).xyz);

    // Discard pixels that are pointing below the horizon
    if (world_direction.y < 0.0) {
        discard;
    }

    // Convert direction to approximate UV coordinates (longitude/latitude mapping)
    let uv = vec2(
        0.5 + 0.5 * atan2(world_direction.z, world_direction.x) / PI,
        0.5 - 0.5 * world_direction.y
    );

    let offset_uv = fract(uv + vec2(cloud_uniforms.u_offset, cloud_uniforms.v_offset));

    let tex = textureSample(texture, texture_sampler, offset_uv);

    // Do your cloud shading logic
    return vec4(0.0, 0.0, 0.0, 0.0);
}
