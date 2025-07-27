@group(0) @binding(0) var src: texture_storage_2d<r16float, read>;
@group(0) @binding(1) var dst: texture_storage_2d<r16float, write>;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let x = i32(gid.x * 2u);
    let y = i32(gid.y * 2u);

    // Read 4 texels from src
    let c0 = textureLoad(src, vec2<i32>(x,     y));
    let c1 = textureLoad(src, vec2<i32>(x + 1, y));
    let c2 = textureLoad(src, vec2<i32>(x,     y + 1));
    let c3 = textureLoad(src, vec2<i32>(x + 1, y + 1));

    let avg = (c0.r + c1.r + c2.r + c3.r) * 0.25;

    textureStore(dst, vec2<i32>(i32(gid.x), i32(gid.y)), vec4<f32>(avg, 0.0, 0.0, 1.0));
}
