@group(0) @binding(0) var<uniform> tone_map_uniforms: ToneMapUniforms;
@group(0) @binding(1) var<uniform> color_grade_uniforms: ColorGradeUniforms;
@group(0) @binding(2) var<uniform> vignette_uniforms: VignetteUniforms;

@group(1) @binding(0) var texture_sampler: sampler;
@group(1) @binding(1) var screen_color: texture_2d<f32>;

@group(2) @binding(0) var luminance_sampler: sampler;
@group(2) @binding(1) var adapted_luminance: texture_2d<f32>;

struct ToneMapUniforms {
    exposure: f32,
    gamma: f32,
    tone_map_operator: u32,
    apply_gamma: u32,
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

struct VignetteUniforms {
    center: vec2<f32>,
    radius: f32,
    softness: f32,
    color: vec3<f32>,
    intensity: f32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var positions = array(
        vec2<f32>(-1.0,  3.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
    );

    var uvs = array(
        vec2<f32>(0.0, -1.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
    );

    var output: VertexOutput;
    output.position = vec4<f32>(positions[vertex_index], 1.0, 1.0);
    output.uv = uvs[vertex_index];
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let hdr_color = textureSample(screen_color, texture_sampler, uv).rgb;

    let adapted_log = textureLoad(adapted_luminance, vec2<i32>(0, 0), 0).r;
    let adapted = exp2(adapted_log);
    let exposure = tone_map_uniforms.exposure / max(adapted, 0.0001);

    var color = hdr_color * exposure;
    color = apply_tone_mapping(color);
    color = apply_color_grading(color);
    color = apply_vignette(color, uv);

    return vec4<f32>(color, 1.0);
}

fn apply_tone_mapping(color: vec3<f32>) -> vec3<f32> {
    switch tone_map_uniforms.tone_map_operator {
        case 0: {
            return color / (color + vec3<f32>(1.0));
        }
        case 1: {
            let a = 2.51;
            let b = 0.03;
            let c = 2.43;
            let d = 0.59;
            let e = 0.14;
            return clamp((color * (a * color + vec3(b))) / (color * (c * color + vec3(d)) + vec3(e)), vec3(0.0), vec3(1.0));
        }
        default: {
            return color;
        }
    }
}

fn apply_color_grading(color: vec3<f32>) -> vec3<f32> {
    var graded = color;

    // White Balance
    graded *= vec3(
        1.0 + color_grade_uniforms.temperature * 0.1 - color_grade_uniforms.tint * 0.05,
        1.0 + color_grade_uniforms.tint * 0.1,
        1.0 - color_grade_uniforms.temperature * 0.1 - color_grade_uniforms.tint * 0.05
    );

    // Contrast
    graded = mix(vec3(0.5), graded, color_grade_uniforms.contrast);

    // Saturation
    let luma = dot(graded, vec3(0.2126, 0.7152, 0.0722));
    graded = mix(vec3(luma), graded, color_grade_uniforms.saturation);

    // Shadows/Midtones/Highlights
    let shadow_mask = clamp(1.0 - smoothstep(0.0, 0.3, luma), 0.0, 1.0);
    let midtone_mask = clamp(smoothstep(0.2, 0.5, luma) * (1.0 - smoothstep(0.5, 0.8, luma)), 0.0, 1.0);
    let highlight_mask = clamp(smoothstep(0.6, 1.0, luma), 0.0, 1.0);

    graded *= mix(vec3(1.0), color_grade_uniforms.shadows.rgb, shadow_mask);
    graded *= mix(vec3(1.0), color_grade_uniforms.midtones.rgb, midtone_mask);
    graded *= mix(vec3(1.0), color_grade_uniforms.highlights, highlight_mask);

    // Brightness
    graded *= vec3(color_grade_uniforms.brightness);

    return graded;
}

fn apply_vignette(color: vec3<f32>, uv: vec2<f32>) -> vec3<f32> {
    let vignette_uv = uv - vignette_uniforms.center;
    let dist = length(vignette_uv);
    let vignette = clamp(1.0 - smoothstep(
        vignette_uniforms.radius - vignette_uniforms.softness,
        vignette_uniforms.radius + vignette_uniforms.softness,
        dist
    ), 0.0, 1.0);
    return mix(color, color * vignette_uniforms.color, vignette_uniforms.intensity * (1.0 - vignette));
}
