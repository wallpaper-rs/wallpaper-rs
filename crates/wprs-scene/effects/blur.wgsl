fn blur_mixed(s: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {
    let mode = u32(param(1u).x + 0.5);
    let k = param(2u).x;
    if mode == 0u && k >= 1.0 {
        return s;
    }
    let b = sample(aux0, uv);
    let a = mix(b.a, select(max(b.a, s.a), s.a, mode == 0u), k);
    let rgb = max(blend_mode(mode, straight(b).rgb, straight(s).rgb, k), vec3<f32>(0.0));
    return vec4<f32>(rgb * a, a);
}

fn blur_spread(t: texture_2d<f32>, uv: vec2<f32>, radius: f32) -> vec4<f32> {
    let step = px(radius * 0.5);
    var sum = vec4<f32>(0.0);
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let w = f32((2 - abs(x)) * (2 - abs(y)));
            sum += sample(t, uv + vec2<f32>(f32(x), f32(y)) * step) * w;
        }
    }
    return sum / 16.0;
}

@fragment
fn fs_blur_h(in: VertexOut) -> @location(0) vec4<f32> {
    return gauss(input_tex, in.uv, vec2<f32>(1.0, 0.0), param(0u).x);
}

@fragment
fn fs_blur_v(in: VertexOut) -> @location(0) vec4<f32> {
    return blur_mixed(gauss(input_tex, in.uv, vec2<f32>(0.0, 1.0), param(0u).x), in.uv);
}

@fragment
fn fs_blur_lite(in: VertexOut) -> @location(0) vec4<f32> {
    return blur_spread(input_tex, in.uv, param(0u).x);
}

@fragment
fn fs_blur_lite_mix(in: VertexOut) -> @location(0) vec4<f32> {
    return blur_mixed(sample(input_tex, in.uv), in.uv);
}
