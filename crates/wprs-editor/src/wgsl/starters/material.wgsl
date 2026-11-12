fn material(s: Surface) -> Surface {
    var o = s;
    let tint = param(0u) * image0(o.uv);
    o.albedo = vec4<f32>(o.albedo.rgb * tint.rgb, o.albedo.a);
    o.emissive = o.emissive + o.albedo.rgb * param(1u).x * (0.5 + 0.5 * sin(frame.time));
    return o;
}
