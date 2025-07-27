@group(0) @binding(0) var<uniform> camera_view_uniforms: CameraViewUniforms;

@group(1) @binding(0) var<uniform> post_process_uniforms: PostProcessUniforms;
@group(1) @binding(1) var<uniform> ssr_uniforms: SsrUniforms;

@group(2) @binding(0) var texture_sampler: sampler;
@group(2) @binding(1) var screen_color: texture_2d<f32>;
@group(2) @binding(2) var screen_normal: texture_2d<f32>;
@group(2) @binding(3) var screen_depth: texture_2d<f32>;

struct CameraViewUniforms {
    view_projection_matrix: mat4x4<f32>,
    view_projection_matrix_inverse: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    world_position: vec4<f32>,
};

struct PostProcessUniforms {
    screen_size: vec2<f32>,
    _padding0: f32,
    _padding1: f32,
};

struct SsrUniforms {
    max_steps: u32,
    stride: f32,
    thickness: f32,
    reflection_intensity: f32,
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
    let resolution = post_process_uniforms.screen_size;
    let uv = coord.xy / resolution;

    // === Reconstruct view-space position and normal ===
    let depth = textureSample(screen_depth, texture_sampler, uv).r;
    let view_pos = reconstruct_view_pos(uv, depth);
    let normal = normalize(textureSample(screen_normal, texture_sampler, uv).xyz * 2.0 - vec3<f32>(1.0));

    let view_dir = normalize(-view_pos); // from fragment toward camera
    let reflect_dir = normalize(reflect(view_dir, normal));

    // === SSR raymarch ===
    let max_steps = ssr_uniforms.max_steps;
    let stride = ssr_uniforms.stride;
    let thickness = ssr_uniforms.thickness;
    let reflection_intensity = ssr_uniforms.reflection_intensity;

    var hit_color = vec3<f32>(0.0);
    var hit = false;

    var t = stride;
    for (var i = 0u; i < max_steps; i = i + 1u) {
        let sample_pos = view_pos + reflect_dir * t;
        let sample_clip = camera_view_uniforms.view_projection_matrix * vec4<f32>(sample_pos, 1.0);
        let sample_ndc = sample_clip.xyz / sample_clip.w;
        let sample_uv = sample_ndc.xy * 0.5 + vec2<f32>(0.5);

        // Outside screen bounds — skip
        if (any(sample_uv < vec2<f32>(0.0)) || any(sample_uv > vec2<f32>(1.0))) {
            break;
        }

        let sample_depth = textureSample(screen_depth, texture_sampler, sample_uv).r;
        let sample_view_pos = reconstruct_view_pos(sample_uv, sample_depth);

        let distance = sample_pos.z - sample_view_pos.z;
        if (abs(distance) < thickness) {
            // Hit: sample reflected color
            hit_color = textureSample(screen_color, texture_sampler, sample_uv).rgb;
            hit = true;
            break;
        }

        t += stride;
    }

    // Original scene color
    let base_color = textureSample(screen_color, texture_sampler, uv).rgb;

    // Blend reflection if we hit
    let final_color = mix(base_color, hit_color, reflection_intensity * f32(hit));
    return vec4<f32>(final_color, 1.0);
}

fn reconstruct_view_pos(uv: vec2<f32>, depth: f32) -> vec3<f32> {
    let ndc = vec4<f32>(uv * 2.0 - vec2<f32>(1.0), depth * 2.0 - 1.0, 1.0);
    let view_pos = camera_view_uniforms.view_projection_matrix_inverse * ndc;
    return view_pos.xyz / view_pos.w;
}
