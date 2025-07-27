// Global Bind Group
@group(0) @binding(0) var<uniform> camera_view_uniforms: CameraViewUniforms;

// Sky Object Bind Group
@group(1) @binding(0) var<uniform> moon_uniforms: MoonUniforms;
@group(1) @binding(1) var<uniform> sun_uniforms: SunUniforms;
@group(1) @binding(2) var texture_sampler: sampler;
@group(1) @binding(3) var texture: texture_2d_array<f32>;

struct CameraViewUniforms {
    view_projection_matrix: mat4x4<f32>,
    view_projection_matrix_inverse: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    world_position: vec4<f32>,
};

struct MoonUniforms {
    tint: vec4<f32>,
    rotation: vec4<f32>,
    size: f32,
    phase_index: u32,
    padding1: u32,
    padding2: u32,
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

    let object_direction = rotate_vector(moon_uniforms.rotation, vec3<f32>(0.0, -1.0, 0.0));

    let cos_angle = dot(world_direction, object_direction);
    let angle = acos(clamp(cos_angle, -1.0, 1.0));
    let radius = moon_uniforms.size;

    if angle > radius {
        discard;
    }

    let world_up = vec3(0.0, 1.0, 0.0);

    let projected_up = normalize(world_up - object_direction * dot(world_up, object_direction));
    var fallback = world_up;

     if abs(object_direction.y) > 0.999 {
            // Near poles, use X axis as fallback
            fallback = vec3<f32>(1.0, 0.0, 0.0);
     };

    let right = normalize(cross(fallback, object_direction));
    let true_up = cross(object_direction, right);

    let local = world_direction - object_direction * dot(world_direction, object_direction);
    let uv = vec2(
        dot(local, right), // right
        dot(local, true_up)  // up
    ) / radius * 0.5 + vec2(0.5);

    let base_color_sample = textureSample(texture, texture_sampler, uv, moon_uniforms.phase_index);
    let tint = moon_uniforms.tint.rgb;
    let tint_strength = moon_uniforms.tint.a;

    let base_color_tinted = base_color_sample.rgb + tint * tint_strength * base_color_sample.a;
    let base_color_final = clamp(base_color_tinted, vec3(0.0), vec3(1.0));
    let alpha_final = base_color_sample.a;

    return vec4(base_color_sample);
}
