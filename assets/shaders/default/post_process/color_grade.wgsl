@group(0) @binding(0) var<uniform> camera_view_uniforms: CameraViewUniforms;

@group(1) @binding(0) var<uniform> post_process_uniforms: PostProcessUniforms;
@group(1) @binding(1) var<uniform> color_grade_uniforms: ColorGradeUniforms;

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

struct ColorGradeUniforms {
    temperature: f32,
    tint: f32,
    contrast: f32,
    saturation: f32,
    shadows: vec4<f32>,
    midtones: vec4<f32>,
    highlights: vec3<f32>,
    brightness: f32,
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

    // Sample base color
    var color = textureSample(screen_color, texture_sampler, uv).rgb;

    // Apply grading steps
    color = apply_temperature_tint(color, color_grade_uniforms.temperature, color_grade_uniforms.tint);
    color = apply_contrast(color, color_grade_uniforms.contrast);
    color = apply_saturation(color, color_grade_uniforms.saturation);
    color = apply_smh_grading(color, color_grade_uniforms.shadows, color_grade_uniforms.midtones, color_grade_uniforms.highlights);
    color = apply_brightness(color, color_grade_uniforms.brightness);

    return vec4<f32>(clamp(color, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}

fn apply_temperature_tint(color: vec3<f32>, temperature: f32, tint: f32) -> vec3<f32> {
    // Simple approximation using color balance
    let temp_adjust = vec3<f32>(
        temperature * 0.1,      // more red
        -tint * 0.1,            // less green
        -(temperature + tint) * 0.1 // more blue
    );
    return color + temp_adjust;
}

fn apply_contrast(color: vec3<f32>, contrast: f32) -> vec3<f32> {
    return ((color - vec3<f32>(0.5)) * contrast + vec3<f32>(0.5));
}

fn apply_saturation(color: vec3<f32>, saturation: f32) -> vec3<f32> {
    let gray = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    return mix(vec3<f32>(gray), color, saturation);
}

fn apply_brightness(color: vec3<f32>, brightness: f32) -> vec3<f32> {
    return color * brightness;
}

fn apply_smh_grading(color: vec3<f32>, shadows: vec4<f32>, midtones: vec4<f32>, highlights: vec3<f32>) -> vec3<f32> {
    let luminance = dot(color, vec3<f32>(0.299, 0.587, 0.114));

    let shadow_mask = clamp(1.0 - luminance * 3.0, 0.0, 1.0);
    let midtone_mask = 1.0 - abs(luminance * 2.0 - 1.0);
    let highlight_mask = clamp((luminance - 0.5) * 2.0, 0.0, 1.0);

    let smh_color =
        shadows.rgb * shadow_mask +
        midtones.rgb * midtone_mask +
        highlights * highlight_mask;

    return clamp(color + smh_color, vec3<f32>(0.0), vec3<f32>(1.0));
}
