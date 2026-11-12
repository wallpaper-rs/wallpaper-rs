fn shake_wave(t: f32) -> vec2<f32> {
    if u32(param(8u).x + 0.5) == 1u {
        let push = select(sin(TAU * t), 0.5 - 0.5 * cos(TAU * t), param(9u).x > 0.5);
        return vec2<f32>(push);
    }
    let calm = vec2<f32>(noise1(t), noise1(t + 17.3)) * 2.0 - 1.0;
    let jitter = vec2<f32>(noise1(t * 7.0 + 3.1), noise1(t * 7.0 + 41.9)) * 2.0 - 1.0;
    let n = mix(calm, jitter, param(3u).x);
    return select(n, n * 0.5 + 0.5, param(9u).x > 0.5);
}

fn uv_shake(uv: vec2<f32>) -> vec2<f32> {
    let t = frame.time * param(1u).x + time_phase(aux0, uv) * 8.0;
    let n = shake_wave(t) * select(1.0, -1.0, u32(param(9u).x + 0.5) == 2u);
    let held = sign(n) * pow(abs(n), vec2<f32>(1.0 + 3.0 * param(4u).x));
    let kept = clamp(held, -param(5u).xy, param(5u).xy);
    let painted = asset_at(aux1, uv);
    let along = painted.rg * 2.0 - 1.0;
    let flow = select(vec2<f32>(0.0), along / max(length(along), 1.0), dot(along, along) > 0.0004);
    let push = select(kept * param(2u).xy, flow * kept.x, painted.a > 0.0);
    return uv + push * px(param(0u).x) * mask_at(uv);
}
