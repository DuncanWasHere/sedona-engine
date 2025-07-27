@group(0) @binding(0) var<uniform> camera_view_uniforms: CameraViewUniforms;

@group(1) @binding(0) var<uniform> post_process_uniforms: PostProcessUniforms;
@group(1) @binding(1) var<uniform> lens_flare_uniforms: LensFlareUniforms;

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

struct LensFlareUniforms {
    intensity: f32,
    ghost_count: u32,
    halo_size: f32,
    chromatic_offset: f32,
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

    // Sample original HDR color
    let input_color = textureSample(screen_color, texture_sampler, uv).rgb;

    // Threshold to detect bright spots (cheap bloom thresholding)
    let brightness = extract_brightness(input_color);
    if (brightness < 1.0) {
        return vec4<f32>(0.0); // discard low-brightness regions
    }

    let flare_intensity = lens_flare_uniforms.intensity;
    let ghost_count = i32(lens_flare_uniforms.ghost_count);
    let halo_size = lens_flare_uniforms.halo_size;
    let chromatic_offset = lens_flare_uniforms.chromatic_offset;

    // Screen center in UV
    let screen_center = vec2<f32>(0.5, 0.5);
    let dir = screen_center - uv;

    // Accumulated flare color
    var flare = vec3<f32>(0.0);

    // === Ghosts ===
    for (var i = 0; i < ghost_count; i = i + 1) {
        let t = f32(i) / f32(ghost_count);
        let ghost_uv = clamp(uv + dir * t * 2.0, vec2<f32>(0.0), vec2<f32>(1.0));

        // Chromatic aberration per ghost
        let red   = textureSample(screen_color, texture_sampler, ghost_uv + dir * chromatic_offset * 0.01).r;
        let green = textureSample(screen_color, texture_sampler, ghost_uv).g;
        let blue  = textureSample(screen_color, texture_sampler, ghost_uv - dir * chromatic_offset * 0.01).b;

        flare += vec3<f32>(red, green, blue) * (1.0 - t); // fade out further ghosts
    }

    // === Halo ===
    let dist = length(dir);
    let halo_uv = clamp(uv + normalize(dir) * halo_size * 0.1, vec2<f32>(0.0), vec2<f32>(1.0));
    let halo_color = textureSample(screen_color, texture_sampler, halo_uv).rgb;
    let halo_weight = exp(-dist * dist * 10.0); // falloff
    flare += halo_color * halo_weight;

    // === Final flare intensity ===
    flare *= flare_intensity;

    return vec4<f32>(flare, 1.0);
}


fn extract_brightness(color: vec3<f32>) -> f32 {
    // Simple brightness function (can be HDR)
    return max(max(color.r, color.g), color.b);
}
