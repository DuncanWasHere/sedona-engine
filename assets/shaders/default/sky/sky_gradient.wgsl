// Global Bind Group
@group(0) @binding(0) var<uniform> camera_view_uniforms: CameraViewUniforms;

// Sky Gradient Bind Group
@group(1) @binding(0) var<uniform> sky_gradient_uniforms: SkyGradientUniforms;

struct CameraViewUniforms {
    view_projection_matrix: mat4x4<f32>,
    view_projection_matrix_inverse: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    world_position: vec4<f32>,
};

struct SkyGradientUniforms {
    horizon_color: vec4<f32>,
    lower_color: vec4<f32>,
    upper_color: vec4<f32>,
    sun_direction: vec3<f32>,
    turbidity: f32,
    mie: vec3<f32>,
    mie_anisotropy: f32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) clip_position: vec4<f32>,
};

const PI: f32 = 3.1415926535;

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
    let world_dir = normalize((camera_view_uniforms.view_projection_matrix_inverse * input.clip_position).xyz);
    let vertical_factor = clamp(world_dir.y, -1.0, 1.0);

    var base_color: vec3<f32>;
    if (vertical_factor < 0.0) {
        base_color = sky_gradient_uniforms.horizon_color.rgb;
    } else {
        let lower_blend = smoothstep(0.0, 0.5, vertical_factor);
        let upper_blend = smoothstep(0.5, 1.0, vertical_factor);
        let intermediate = mix(sky_gradient_uniforms.lower_color.rgb, sky_gradient_uniforms.upper_color.rgb, upper_blend);
        base_color = mix(sky_gradient_uniforms.horizon_color.rgb, intermediate, lower_blend);
    }

    // Sun direction and Mie scattering
    let sun_dir = normalize(sky_gradient_uniforms.sun_direction);
    let cos_theta = clamp(dot(world_dir, sun_dir), -1.0, 1.0);

    let mie_phase = hg_phase(cos_theta, sky_gradient_uniforms.mie_anisotropy);

    // Use a scale that grows with turbidity but doesn't blow out
    let mie_intensity = mie_phase * (sky_gradient_uniforms.turbidity * 0.5);

    // Falloff should produce a narrow sun glow (~1-2 degrees in radians)
    let sun_angular_radius = 0.2935; // ~0.535 degrees in radians
    let angular_falloff = smoothstep(cos(sun_angular_radius * 2.0), cos(sun_angular_radius), cos_theta);

    let sun_color = compute_sun_color(cos_theta, sky_gradient_uniforms.turbidity);
    let mie_color = sky_gradient_uniforms.mie * mie_intensity * angular_falloff * sun_color;

    var final_color = base_color + mie_color;

    // Add subtle noise to reduce banding
    let noise = hash12(input.position.xy) * 0.005;
    final_color += vec3<f32>(noise);

    return vec4<f32>(final_color, 1.0);
}

// Henyey-Greenstein phase function
fn hg_phase(cos_theta: f32, g: f32) -> f32 {
    let g2 = g * g;
    return (1.0 - g2) / pow(1.0 + g2 - 2.0 * g * cos_theta, 1.5) * (1.0 / (4.0 * PI));
}

fn hash12(p: vec2<f32>) -> f32 {
    let p3 = fract(vec3<f32>(p.x, p.y, p.x) * 0.1031);
    let p3_dot = dot(p3, vec3<f32>(p3.y, p3.z, p3.x) + 33.33);
    return fract((p3.x + p3.y) * p3.z * p3_dot);
}

fn compute_sun_color(cos_theta: f32, turbidity: f32) -> vec3<f32> {
    // Empirical approximation: as sun gets lower, more red, less blue
    let sun_elevation = acos(clamp(cos_theta, -1.0, 1.0));
    let extinction = exp(-turbidity * vec3<f32>(0.65, 1.0, 1.5) * (1.0 / max(cos_theta, 0.01)));
    return extinction;
}
