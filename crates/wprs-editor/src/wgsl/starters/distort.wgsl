fn uv_NAME(uv: vec2<f32>) -> vec2<f32> {
    let d = uv - param(0u).xy;
    let pull = param(1u).x * exp(-dot(d, d) / max(param(2u).x * param(2u).x, 0.0001));
    return uv - d * pull;
}
