@group(2) @binding(7) var seen_depth: texture_2d<f32>;

@fragment
fn fs_seen(in: Corner) -> @location(0) vec4<f32> {
    let px = vec3<f32>(in.position.xy + look.place.xy, 1.0);
    let uv = vec2<f32>(dot(look.seen_a.xyz, px), dot(look.seen_b.xyz, px));
    if all(uv >= vec2<f32>(0.0)) && all(uv < vec2<f32>(1.0)) {
        let at = vec2<i32>(uv * vec2<f32>(textureDimensions(seen_depth)));
        let n = look.window.x;
        let f = look.window.y;
        let d = clamp(look.window.z / max(in.scale, 1e-6), n, f);
        if textureLoad(seen_depth, at, 0).x > n * (f - d) / ((f - n) * d) {
            discard;
        }
    }
    return painted(in);
}
