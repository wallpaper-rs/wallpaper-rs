
fn image0(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(aux0, wrap, uv);
}

fn image1(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(aux1, wrap, uv);
}

fn image2(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(aux2, wrap, uv);
}

fn image3(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(aux3, wrap, uv);
}
