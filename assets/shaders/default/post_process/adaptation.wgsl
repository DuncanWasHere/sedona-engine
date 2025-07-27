@group(0) @binding(0) var<uniform> adaptation_uniforms: AdaptationUniforms;

@group(1) @binding(0) var current_luminance: texture_storage_2d<r16float, read>;
@group(1) @binding(1) var previous_adapted: texture_storage_2d<r16float, read>;
@group(1) @binding(2) var next_dapated: texture_storage_2d<r16float, write>;

struct AdaptationUniforms {
    adaptation_speed: f32,
    min_log_luminance: f32,
    max_log_luminance: f32,
    dt: f32,
};

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let current_log = textureLoad(current_luminance, vec2<i32>(0, 0)).r;
    let previous_log = textureLoad(previous_adapted, vec2<i32>(0, 0)).r;

    let clamped_log = clamp(current_log,
                            adaptation_uniforms.min_log_luminance,
                            adaptation_uniforms.max_log_luminance);

    let rate = 1.0 - exp(-adaptation_uniforms.adaptation_speed * adaptation_uniforms.dt);
    let adapted_log = mix(previous_log, clamped_log, rate);

    textureStore(next_dapated, vec2<i32>(0, 0), vec4<f32>(adapted_log, 0.0, 0.0, 1.0));
}
