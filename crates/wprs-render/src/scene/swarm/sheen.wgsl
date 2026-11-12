fn bent_across(a: f32, across: f32, bend: vec3<f32>) -> vec4<f32> {
    let shift = bend.xy * (across * 2.0 - 1.0) * look.bend.x * 0.5 * bend.z;
    return vec4<f32>(shift * a, 0.0, a);
}

@fragment
fn fs_lit_trail(in: Ribbon) -> @location(0) vec4<f32> {
    return lamp_lit(trailed(in), vec3<f32>(0.0, 0.0, 1.0), in.position.xy + look.place.xy);
}

@fragment
fn fs_lit_cord(in: Cord) -> @location(0) vec4<f32> {
    return lamp_lit(corded(in), vec3<f32>(0.0, 0.0, 1.0), in.position.xy + look.place.xy);
}

@fragment
fn fs_bend_trail(in: Ribbon) -> @location(0) vec4<f32> {
    return bent_across(trailed(in).a, in.across, in.bend);
}

@fragment
fn fs_bend_cord(in: Cord) -> @location(0) vec4<f32> {
    return bent_across(corded(in).a, in.uv.y, in.bend);
}
