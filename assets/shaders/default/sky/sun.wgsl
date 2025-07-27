// Global Bind Group
@group(0) @binding(0) var<uniform> camera_view_uniforms: CameraViewUniforms;

// Sky Object Bind Group
@group(1) @binding(0) var<uniform> sun_uniforms: SunUniforms;

struct CameraViewUniforms {
    view_projection_matrix: mat4x4<f32>,
    view_projection_matrix_inverse: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    world_position: vec4<f32>,
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

// Rotate a vector using a quaternion
fn rotate_vector(q: vec4<f32>, v: vec3<f32>) -> vec3<f32> {
    let q_xyz = q.xyz;
    let t = 2.0 * cross(q_xyz, v);
    return v + q.w * t + cross(q_xyz, t);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let world_direction = normalize((camera_view_uniforms.view_projection_matrix_inverse * input.clip_position).xyz);
    let object_direction = rotate_vector(sun_uniforms.rotation, vec3<f32>(0.0, -1.0, 0.0)); // sun forward

    let cos_angle = dot(world_direction, object_direction);
    let angle = acos(clamp(cos_angle, -1.0, 1.0));

    if angle > sun_uniforms.size {
        discard;
    }

    // Optional soft edge
    let distance = angle / sun_uniforms.size;
    let fade = smoothstep(1.0, 0.9, distance);

    return vec4(sun_uniforms.tint.rgb, sun_uniforms.tint.a * fade);
}
