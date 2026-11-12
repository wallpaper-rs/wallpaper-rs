fn bloom_bright(c: vec4<f32>) -> vec4<f32> {
    let l = max(c.r, max(c.g, c.b));
    let t = param(0u).x;
    let k = max(t * param(3u).x, 1e-4);
    let soft = clamp(l - t + k, 0.0, 2.0 * k);
    let gain = max(soft * soft / (4.0 * k), l - t) / max(l, 1e-4);
    return c * max(gain, 0.0);
}

@fragment
fn fs_bloom_bright(in: VertexOut) -> @location(0) vec4<f32> {
    return bloom_bright(tap(in.uv));
}

@fragment
fn fs_bloom_lite(in: VertexOut) -> @location(0) vec4<f32> {
    let step = px(param(2u).x * 0.5);
    var sum = vec4<f32>(0.0);
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let w = f32((2 - abs(x)) * (2 - abs(y)));
            sum += bloom_bright(tap(in.uv + vec2<f32>(f32(x), f32(y)) * step)) * w;
        }
    }
    return sum / 16.0;
}

@fragment
fn fs_bloom_h(in: VertexOut) -> @location(0) vec4<f32> {
    return gauss(input_tex, in.uv, vec2<f32>(1.0, 0.0), param(2u).x);
}

@fragment
fn fs_bloom_v(in: VertexOut) -> @location(0) vec4<f32> {
    return gauss(input_tex, in.uv, vec2<f32>(0.0, 1.0), param(2u).x);
}

@fragment
fn fs_bloom_add(in: VertexOut) -> @location(0) vec4<f32> {
    let tint = from_linear(param(4u).rgb) * param(4u).a;
    return vec4<f32>(sample(aux0, in.uv).rgb * tint * param(1u).x, 0.0);
}

@fragment
fn fs_bloom_mix(in: VertexOut) -> @location(0) vec4<f32> {
    let tint = from_linear(param(4u).rgb) * param(4u).a;
    return tap(in.uv) + sample(aux0, in.uv) * vec4<f32>(tint, 1.0) * param(1u).x;
}
