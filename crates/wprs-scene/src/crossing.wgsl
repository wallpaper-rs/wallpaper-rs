struct Crossing {
    uv: vec2<f32>,
    resolution: vec2<f32>,
    progress: f32,
    quality: f32,
    color: vec4<f32>,
};

fn before(uv: vec2<f32>) -> vec4<f32> {
    return sample(aux0, uv);
}

fn after(uv: vec2<f32>) -> vec4<f32> {
    return sample(input_tex, uv);
}
