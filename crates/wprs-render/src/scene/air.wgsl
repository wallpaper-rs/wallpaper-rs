@fragment
fn fs_air(in: Out) -> @location(0) vec4<f32> {
    let b = lights.bulbs[u32(item.lit_at.w + 0.5)];
    let p = item.lit_at.xy + in.position.xy * item.lit_at.z;
    let n = max(4u, u32(b.air.y * item.lit.x + 0.5));
    let reach_of = b.air.x;
    let area = vec4<f32>(item.lit_at.xy, max(item.lit.yz, vec2<f32>(1.0)));
    let luma = vec3<f32>(0.2126, 0.7152, 0.0722);
    let ring = b.at.w * 0.25;
    var around = 0.04;
    for (var k = 0u; k < select(0u, 5u, b.air.w > 0.0); k++) {
        let turn = f32(k) * 1.2566371;
        let o = select(vec2<f32>(cos(turn), sin(turn)) * ring, vec2<f32>(0.0), k == 4u);
        let tap = textureSampleLevel(src_tex, samp, clamp((b.at.xy + o - area.xy) / area.zw, vec2<f32>(0.0), vec2<f32>(1.0)), 0.0).rgb;
        around = max(around, dot(tap, luma));
    }
    var sum = vec3<f32>(0.0);
    for (var i = 0u; i < n; i++) {
        let q = mix(p, b.at.xy, reach_of * f32(i) / f32(n));
        let r = reach(b, q);
        if r.w > 0.0 {
            let back = textureSampleLevel(src_tex, samp, (q - area.xy) / area.zw, 0.0).rgb;
            let open = mix(1.0, smoothstep(0.0, 0.25, dot(back, luma) / around), b.air.w);
            sum += gobo(b, r.xyz) * r.w * open;
        }
    }
    let d = length(p - b.at.xy) / max(b.at.w, 1e-3);
    let edge = clamp(1.0 - d * d, 0.0, 1.0);
    return vec4<f32>(b.color.rgb * sum / f32(n) * b.air.x * edge, 0.0);
}

@fragment
fn fs_refract(in: Out) -> @location(0) vec4<f32> {
    let d = textureSampleLevel(mask_tex, samp, in.uv, 0.0);
    return textureSampleLevel(src_tex, samp, in.uv + d.xy * item.lit_env.zw, 0.0);
}

@fragment
fn fs_shift_view(in: Out) -> @location(0) vec4<f32> {
    let d = textureSampleLevel(mask_tex, samp, in.uv, 0.0);
    let back = textureSampleLevel(src_tex, samp, in.uv, 0.0);
    let k = min(d.a, 1.0);
    let s = clamp(d.xy / max(d.a, 1e-4) / 16.0, vec2<f32>(-1.0), vec2<f32>(1.0));
    let a = max(back.a, k);
    let tint = vec3<f32>(0.5 + 0.5 * s.x, 0.5 - 0.5 * s.y, 1.0) * a;
    return vec4<f32>(mix(back.rgb * 0.35, tint, k), a);
}
