fn transition(c: Crossing) -> Crossing {
    var o = c;
    let borde = max(param(0u).x, 0.001);
    let tinte = param(1u).rgb;
    let aspecto = vec2<f32>(c.resolution.x / max(c.resolution.y, 1.0), 1.0);
    let d = length((c.uv - 0.5) * aspecto) / 0.9;
    let r = c.progress * (1.0 + borde);
    let k = smoothstep(r - borde, r, d);
    let aro = (1.0 - abs(k * 2.0 - 1.0)) * (1.0 - c.progress);
    o.color = vec4<f32>(mix(after(c.uv).rgb, before(c.uv).rgb, k) + tinte * aro * 0.6, 1.0);
    return o;
}
