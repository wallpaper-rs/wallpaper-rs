@fragment
fn fs_NAME_soft(in: VertexOut) -> @location(0) vec4<f32> {
    let turn = radians(param(1u).x);
    let along = px(param(0u).x) * vec2<f32>(cos(turn), sin(turn));
    var sum = vec4<f32>(0.0);
    for (var i = 0; i < 9; i++) {
        sum = sum + tap(in.uv + along * (f32(i - 4) / 4.0));
    }
    return sum / 9.0;
}

@fragment
fn fs_NAME(in: VertexOut) -> @location(0) vec4<f32> {
    return max(sample(aux0, in.uv), tap(in.uv));
}
