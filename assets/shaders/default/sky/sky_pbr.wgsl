// Global Bind Group
@group(0) @binding(0) var<uniform> camera_view_uniforms: CameraViewUniforms;

// Sky Gradient Bind Group
@group(1) @binding(0) var<uniform> sky_pbr_uniforms: SkyPbrUniforms;
@group(1) @binding(1) var<uniform> sun_uniforms: SunUniforms;

struct CameraViewUniforms {
    view_projection_matrix: mat4x4<f32>,
    view_projection_matrix_inverse: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    world_position: vec4<f32>,
};

struct SkyPbrUniforms {
    rayleigh: vec4<f32>,
    mie: vec4<f32>,
    sun_direction: vec4<f32>,
    mie_anisotropy: f32,
    turbidity: f32,
    rayleigh_factor: f32,
    mie_factor: f32,
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

const PI: f32 = 3.14159;
const ATMOSPHERE_SCALE_HEIGHT: f32 = 8000.0;
const BASE_SPECTRAL_RADIANCE: vec3<f32> = vec3(1.0, 1.0, 1.0);

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
    let fragment_direction = normalize((camera_view_uniforms.view_projection_matrix_inverse * input.clip_position).xyz);
    let sun_direction = sky_pbr_uniforms.sun_direction.xyz;
    let rayleigh_coefficients = sky_pbr_uniforms.rayleigh.rgb;

    // Angle between fragment direction and sun direction.
    let gamma = dot(fragment_direction, sun_direction);

    // Angle between fragment direction and zenith.
    let theta = dot(fragment_direction, vec3(0.0, -1.0, 0.0));

    // Atmospheric distance between sun and fragment
    let view_airmass = 1.0 / max(0.01, theta); // Simple secant approximation

    let cos_sun_zenith = dot(sun_direction, vec3(0.0, 1.0, 0.0));
    let sun_airmass = 1.0 / max(0.01, cos_sun_zenith);

    let sun_elevation_deg = degrees(asin(cos_sun_zenith)); // in [-90, +90]
    let spectral_radiance = BASE_SPECTRAL_RADIANCE * exp(-rayleigh_coefficients * sun_airmass * ATMOSPHERE_SCALE_HEIGHT);


    // Total Rayleigh scattered light along this view ray.
    let rayleigh_color = rayleigh_scatter(rayleigh_coefficients, spectral_radiance, gamma, view_airmass, sun_airmass);

    let mie_phase = hg_phase(gamma, sky_pbr_uniforms.mie_anisotropy);
    let mie_extinction = exp(-sky_pbr_uniforms.mie.rgb * sun_airmass * ATMOSPHERE_SCALE_HEIGHT);
    let mie_scattering = sky_pbr_uniforms.mie.rgb * view_airmass * ATMOSPHERE_SCALE_HEIGHT;
    let mie_color = mie_scattering * mie_extinction * mie_phase * BASE_SPECTRAL_RADIANCE;


    let final_color = rayleigh_color + mie_color;


    return vec4<f32>(final_color, 1.0);
}

fn rayleigh_scatter(rayleigh_coefficients: vec3<f32>, spectral_radiance: vec3<f32>, theta: f32, view_airmass: f32, sun_airmass: f32) -> vec3<f32>  {
    let phase = (3.0 / (16.0 * PI)) * (1.0 + theta * theta);

    let sun_extinction = exp(-rayleigh_coefficients * sun_airmass * ATMOSPHERE_SCALE_HEIGHT);
    let solar_irradiance = BASE_SPECTRAL_RADIANCE * sun_extinction;
    let scattering = rayleigh_coefficients * view_airmass * ATMOSPHERE_SCALE_HEIGHT; // How much gets scattered along the view ray.
    let color = phase * sun_extinction * scattering * solar_irradiance;

    return color;
}

// Henyey-Greenstein phase function for Mie scattering
fn hg_phase(mu: f32, g: f32) -> f32 {
    let g2 = g * g;
    let denom = pow(1.0 + g2 - 2.0 * g * mu, 1.5);
    return (1.0 - g2) / (4.0 * 3.141592 * denom);
}

