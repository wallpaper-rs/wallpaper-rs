use wgpu::ShaderStages as S;
use wgpu::TextureViewDimension::{Cube, D2Array};

fn entry(binding: u32, visibility: S, ty: wgpu::BindingType) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry { binding, visibility, ty, count: None }
}

fn uniform(dynamic: bool) -> wgpu::BindingType {
    wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Uniform,
        has_dynamic_offset: dynamic,
        min_binding_size: None,
    }
}

fn picture() -> wgpu::BindingType {
    wgpu::BindingType::Texture {
        sample_type: wgpu::TextureSampleType::Float { filterable: true },
        view_dimension: wgpu::TextureViewDimension::D2,
        multisampled: false,
    }
}

fn depth(view_dimension: wgpu::TextureViewDimension) -> wgpu::BindingType {
    wgpu::BindingType::Texture {
        sample_type: wgpu::TextureSampleType::Depth,
        view_dimension,
        multisampled: false,
    }
}

pub(super) fn layouts(device: &wgpu::Device) -> [wgpu::BindGroupLayout; 4] {
    let sampler = wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering);
    let compare = wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison);
    let deltas = wgpu::BindingType::Texture {
        sample_type: wgpu::TextureSampleType::Float { filterable: false },
        view_dimension: wgpu::TextureViewDimension::D2,
        multisampled: false,
    };
    let both = S::VERTEX | S::FRAGMENT;
    let make = |label: &str, entries: &[wgpu::BindGroupLayoutEntry]| {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some(label), entries })
    };
    let maps: Vec<_> = (0..5)
        .map(|k| entry(k, S::FRAGMENT, picture()))
        .chain((5..10).map(|k| entry(k, S::FRAGMENT, sampler)))
        .chain([entry(10, S::FRAGMENT, uniform(false))])
        .collect();
    [
        make(
            "wprs-render solid scene",
            &[
                entry(0, both, uniform(true)),
                entry(1, S::FRAGMENT, picture()),
                entry(2, S::FRAGMENT, sampler),
                entry(3, both, uniform(false)),
                entry(4, S::FRAGMENT, depth(D2Array)),
                entry(5, S::FRAGMENT, depth(D2Array)),
                entry(6, S::FRAGMENT, compare),
                entry(7, S::FRAGMENT, uniform(false)),
                entry(8, S::FRAGMENT, depth(Cube)),
                entry(9, S::FRAGMENT, depth(Cube)),
            ],
        ),
        make(
            "wprs-render solid object",
            &[
                entry(0, S::VERTEX, uniform(true)),
                entry(1, S::VERTEX, uniform(true)),
                entry(2, S::VERTEX, uniform(true)),
                entry(3, S::VERTEX, deltas),
            ],
        ),
        make("wprs-render solid maps", &maps),
        make("wprs-render solid finish", &[entry(0, both, uniform(true))]),
    ]
}
