@binding(0) @group(0) var<uniform> vertex_uniforms: VertexUniforms;
@binding(1) @group(0) var<uniform> fragment_uniforms: FragmentUniforms;
@binding(2) @group(0) var<uniform> light_uniforms: LightUniforms;
@binding(3) @group(0) var diffuse_texture: texture_2d<f32>;
@binding(4) @group(0) var diffuse_sampler: sampler;

struct VertexUniforms {
    model_matrix: mat4x4<f32>,
    view_projection_matrix: mat4x4<f32>,
    vertex_world_normal: mat4x4<f32>,
};

struct FragmentUniforms {
    light_position: vec4<f32>,
    eye_position: vec4<f32>,
};


struct LightUniforms {
    diffuse_color: vec4<f32>,
    specular_color: vec4<f32>,
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
};

struct Output {
    @builtin(position) vertex_clip_position: vec4<f32>,
    @location(0) vertex_position: vec4<f32>,
    @location(1) vertex_normal: vec4<f32>,
    @location(2) vertex_tangent: vec4<f32>,
    @location(3) vertex_color: vec4<f32>,
    @location(4) vertex_uv: vec2<f32>,
};

@vertex
fn vs_main(
    @location(0) vertex_position: vec4<f32>,
    @location(1) vertex_normal: vec4<f32>,
    @location(2) vertex_tangent: vec4<f32>,
    @location(3) vertex_color: vec4<f32>,
    @location(4) vertex_uv: vec2<f32>,
    ) -> Output {

    var output: Output;

    let vertex_world_position: vec4<f32> = vertex_uniforms.model_matrix * vertex_position;

    output.vertex_position = vertex_world_position;
    output.vertex_normal = vertex_uniforms.vertex_world_normal * vertex_normal;
    output.vertex_color = vertex_color;
    output.vertex_uv = vertex_uv;
    output.vertex_clip_position = vertex_uniforms.view_projection_matrix * vertex_world_position;

    return output;
}

@fragment
fn fs_main(
    @location(0) vertex_position: vec4<f32>,
    @location(1) vertex_normal: vec4<f32>,
    @location(2) vertex_tangent: vec4<f32>,
    @location(3) vertex_color: vec4<f32>,
    @location(4) vertex_uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    let normal: vec3<f32> = normalize(vertex_normal.xyz);
    let incident_light: vec3<f32> = normalize(fragment_uniforms.light_position.xyz - vertex_position.xyz);
    let view_direction: vec3<f32> = normalize(fragment_uniforms.eye_position.xyz - vertex_position.xyz);
    let half_angle: vec3<f32> = normalize(incident_light + view_direction);

    let diffuse: f32 = light_uniforms.diffuse_intensity * max(dot(normal, incident_light), 0.0);
    let specular: f32 = light_uniforms.specular_intensity * pow(max(dot(normal, half_angle), 0.0),
                                                                light_uniforms.specular_shininess);
    let ambient: f32 = light_uniforms.ambient_intensity;

    let texture_color: vec4<f32> = textureSample(diffuse_texture, diffuse_sampler, vertex_uv);
    let base_color: vec4<f32> = texture_color * vertex_color;

    let lit_color = base_color * light_uniforms.diffuse_color * (ambient + diffuse) +
                    light_uniforms.specular_color * specular;

    return lit_color;
}
