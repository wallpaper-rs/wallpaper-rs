fn color_duotono(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {
    let c = straight(color);
    let luma = dot(c.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    let toned = mix(from_linear(param(0u).rgb), from_linear(param(1u).rgb), luma);
    let rgb = mix(c.rgb, toned, param(2u).x * mask_at(uv));
    return vec4<f32>(rgb * c.a, c.a);
}
