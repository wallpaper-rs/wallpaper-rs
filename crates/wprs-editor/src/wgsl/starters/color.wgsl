fn color_NAME(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {
    let tinted = vec4<f32>(color.rgb * param(1u).rgb, color.a);
    return mix(color, tinted, param(0u).x);
}
