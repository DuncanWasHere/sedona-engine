// Global Bind Group
@group(0) @binding(0) var<uniform> camera_view_uniforms: CameraViewUniforms;

// Sky Object Bind Group
@group(1) @binding(0) var<uniform> sky_object_uniforms: SkyObjectUniforms;
@group(1) @binding(1) var texture_sampler: sampler;
@group(1) @binding(2) var texture: texture_2d<f32>;

struct CameraViewUniforms {
    view_projection_matrix: mat4x4<f32>,
    view_projection_matrix_inverse: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    world_position: vec4<f32>,
};

struct SkyObjectUniforms {
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

fn rotate_vector(q: vec4<f32>, v: vec3<f32>) -> vec3<f32> {
    let q_xyz = q.xyz;
    let t = 2.0 * cross(q_xyz, v);
    return v + q.w * t + cross(q_xyz, t);
}

fn direction_to_uv(dir: vec3<f32>) -> vec2<f32> {
    let theta = atan2(dir.z, dir.x); // longitude
    let phi = acos(clamp(dir.y, -1.0, 1.0)); // latitude

    let u = (theta / (2.0 * PI)) + 0.5;
    let v = phi / PI;

    return vec2(u, v);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let ndc = vec4<f32>(input.clip_position.xy, 0.0, 1.0);
    let world_direction = normalize((camera_view_uniforms.view_projection_matrix_inverse * ndc).xyz);

    let rotated_direction = rotate_vector(sky_object_uniforms.rotation, world_direction);

    let uv = direction_to_uv(rotated_direction);
    let star_color = textureSample(texture, texture_sampler, uv);

    return star_color * sky_object_uniforms.tint.a;
}
