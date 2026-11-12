fn sway_foliage(uv: vec2<f32>, l: vec2<f32>, m: f32) -> vec2<f32> {
    let t = TAU * param(1u).x * frame.time;
    let place = TAU * (param(3u).x * (l.x + 0.5 * l.y) + time_phase(aux0, uv));
    let weights = vec4<f32>(0.5, 0.25, 0.15, 0.1);
    let x = dot(sin(place + t * vec4<f32>(1.0, 1.63, 2.71, 4.37)), weights);
    let y = dot(sin(0.4 + 0.7 * place - t * vec4<f32>(0.83, 1.37, 2.29, 3.91)), weights);
    let r = sqrt(param(10u).x);
    return uv - vec2<f32>(x * r, y / r) * px(param(0u).x) * m;
}

fn uv_sway(uv: vec2<f32>) -> vec2<f32> {
    let l = layer_uv(uv);
    let wave = TAU * (param(1u).x * frame.time + time_phase(aux0, uv));
    let m = mask_at(uv);
    let twice = param(7u).x > 0.5;
    if param(9u).x > 0.5 {
        return sway_foliage(uv, l, m);
    }
    if param(4u).x > 0.5 {
        let unit = px(1.0);
        let p = from_layer(param(6u).xy);
        let swing = select(sin(wave), 0.65 * sin(wave) + 0.35 * sin(wave * 1.7 + 1.3), twice);
        let a = radians(param(5u).x) * swing * m;
        return p + mat2x2<f32>(cos(a), sin(a), -sin(a), cos(a)) * ((uv - p) / unit) * unit;
    }
    let lift = pow(clamp(1.0 - l.y, 0.0, 1.0), param(2u).x);
    let place = TAU * param(3u).x * (l.x + 0.5 * l.y);
    let one = sin(wave + place);
    let s = select(one, 0.65 * one + 0.35 * sin(wave * 1.7 + place * 0.6 + 1.3), twice);
    return uv - vec2<f32>(px(param(0u).x).x * s * lift * m, 0.0);
}
