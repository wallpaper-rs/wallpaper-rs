@group(2) @binding(12) var material_image0: texture_2d<f32>;
@group(2) @binding(13) var material_image1: texture_2d<f32>;
@group(2) @binding(14) var material_image2: texture_2d<f32>;
@group(2) @binding(15) var material_image3: texture_2d<f32>;
@group(2) @binding(16) var material_sampler: sampler;

fn image0(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(material_image0, material_sampler, uv);
}

fn image1(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(material_image1, material_sampler, uv);
}

fn image2(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(material_image2, material_sampler, uv);
}

fn image3(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(material_image3, material_sampler, uv);
}
