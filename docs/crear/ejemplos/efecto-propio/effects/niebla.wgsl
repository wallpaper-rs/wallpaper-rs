@fragment
fn fs_niebla(in: VertexOut) -> @location(0) vec4<f32> {
    let c = tap(in.uv);
    let haze = from_linear(param(0u).rgb) * c.a;
    let amount = param(1u).x * mask_at(in.uv) * in.uv.y;
    return vec4<f32>(mix(c.rgb, haze, amount), c.a);
}
