@group(0) @binding(0) var<uniform> camera_view_uniforms: CameraViewUniforms;

@group(1) @binding(0) var<uniform> post_process_uniforms: PostProcessUniforms;
@group(1) @binding(1) var<uniform> depth_of_field_uniforms: DepthOfFieldUniforms;

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

struct DepthOfFieldUniforms {
    focal_distance: f32,
    focal_range: f32,
    blur_strength: f32,
    aperture: f32,
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

    // === Read depth and convert to world-space view depth ===
    let depth = textureSample(screen_depth, texture_sampler, uv).x;
    let clip_space_pos = vec4<f32>(
        uv * 2.0 - vec2<f32>(1.0),
        depth * 2.0 - 1.0,
        1.0,
    );

    let view_pos = camera_view_uniforms.view_projection_matrix_inverse * clip_space_pos;
    let world_depth = view_pos.z / view_pos.w; // view space Z (negative forward)

    // === CoC calculation ===
    let focal_distance = depth_of_field_uniforms.focal_distance;
    let focal_range = depth_of_field_uniforms.focal_range;
    let blur_strength = depth_of_field_uniforms.blur_strength;

    let coc = clamp(abs(world_depth - focal_distance) / focal_range, 0.0, 1.0);
    let blur_radius = coc * blur_strength;

    // === Early return for in-focus pixels ===
    if (blur_radius < 0.001) {
        return textureSample(screen_color, texture_sampler, uv);
    }

    // === Blur pass ===
    let kernel_radius = i32(ceil(blur_radius * 5.0));
    var color = vec3<f32>(0.0);
    var total_weight = 0.0;

    for (var x = -kernel_radius; x <= kernel_radius; x = x + 1) {
        for (var y = -kernel_radius; y <= kernel_radius; y = y + 1) {
            let offset = vec2<f32>(f32(x), f32(y)) / resolution;
            let sample_uv = clamp(uv + offset, vec2<f32>(0.0), vec2<f32>(1.0));
            let dist = length(vec2<f32>(x, y));
            let weight = exp(-dist * dist / (2.0 * blur_radius * blur_radius + 1e-5));

            color += textureSample(screen_color, texture_sampler, sample_uv).rgb * weight;
            total_weight += weight;
        }
    }

    color /= total_weight;

    return vec4<f32>(color, 1.0);
}
