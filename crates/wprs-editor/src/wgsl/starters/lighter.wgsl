
@fragment
fn fs_NAME_lite(in: VertexOut) -> @location(0) vec4<f32> {
    return tap(in.uv);
}
