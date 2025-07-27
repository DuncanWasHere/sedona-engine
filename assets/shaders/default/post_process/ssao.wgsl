@group(0) @binding(0) var<uniform> camera_view_uniforms: CameraViewUniforms;

@group(1) @binding(0) var<uniform> post_process_uniforms: PostProcessUniforms;
@group(1) @binding(1) var<uniform> ssao_uniforms: SsaoUniforms;

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

struct SsaoUniforms {
    radius: f32,
    bias: f32,
    intensity: f32,
    sample_count: u32,
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
    let position = reconstruct_view_pos(uv, depth);
    let normal = normalize(textureSample(screen_normal, texture_sampler, uv).xyz * 2.0 - vec3<f32>(1.0));

    let radius = ssao_uniforms.radius;
    let bias = ssao_uniforms.bias;
    let intensity = ssao_uniforms.intensity;
    let sample_count = ssao_uniforms.sample_count;

    var occlusion = 0.0;

    for (var i = 0u; i < sample_count; i = i + 1u) {
        let sample_vec = kernel[i]; // view-space sample vector
        let sample_pos = position + sample_vec * radius;

        // Project sample position to clip space
        let sample_clip = camera_view_uniforms.view_projection_matrix * vec4<f32>(sample_pos, 1.0);
        let sample_ndc = sample_clip.xyz / sample_clip.w;
        let sample_uv = sample_ndc.xy * 0.5 + vec2<f32>(0.5);

        // Depth test
        if (all(sample_uv >= vec2<f32>(0.0)) && all(sample_uv <= vec2<f32>(1.0))) {
            let sample_depth = textureSample(screen_depth, texture_sampler, sample_uv).r;
            let sample_view = reconstruct_view_pos(sample_uv, sample_depth);

            let range_check = step(bias, position.z - sample_view.z);
            let angle_check = step(0.0, dot(normal, normalize(sample_view - position)));

            occlusion += range_check * angle_check;
        }
    }

    let ao = 1.0 - (occlusion / f32(sample_count)) * intensity;
    return vec4<f32>(vec3<f32>(ao), 1.0);
}

fn reconstruct_view_pos(uv: vec2<f32>, depth: f32) -> vec3<f32> {
    let ndc = vec4<f32>(uv * 2.0 - vec2<f32>(1.0), depth * 2.0 - 1.0, 1.0);
    let view_pos = camera_view_uniforms.view_projection_matrix_inverse * ndc;
    return (view_pos.xyz / view_pos.w);
}

// Generate a few fixed sample vectors in view-space hemisphere
let kernel: array<vec3<f32>, 16> = array<vec3<f32>, 16>(
    vec3<f32>( 0.5381,  0.1856, -0.4319),
    vec3<f32>( 0.1379,  0.2486,  0.4430),
    vec3<f32>( 0.3371,  0.5679, -0.0057),
    vec3<f32>(-0.6999, -0.0451, -0.0019),
    vec3<f32>( 0.0689, -0.1598, -0.8547),
    vec3<f32>( 0.0560,  0.0069, -0.1843),
    vec3<f32>(-0.0146,  0.1402,  0.0762),
    vec3<f32>( 0.0100, -0.1924, -0.0344),
    vec3<f32>(-0.3577, -0.5301, -0.4358),
    vec3<f32>(-0.3169,  0.1063,  0.0158),
    vec3<f32>( 0.0103, -0.5869,  0.0046),
    vec3<f32>(-0.0897, -0.4940,  0.3287),
    vec3<f32>( 0.7119, -0.0154, -0.0918),
    vec3<f32>(-0.0533,  0.0596, -0.5411),
    vec3<f32>( 0.0352, -0.0631,  0.5460),
    vec3<f32>(-0.4776,  0.2847, -0.0271)
);
