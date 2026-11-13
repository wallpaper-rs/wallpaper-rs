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

pub(crate) fn seen(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let sample_type = wgpu::TextureSampleType::Float { filterable: false };
    let read = wgpu::BindingType::Texture {
        sample_type,
        view_dimension: wgpu::TextureViewDimension::D2,
        multisampled: false,
    };
    let entries = [entry(7, S::FRAGMENT, read)];
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("wprs-render seen"),
        entries: &entries,
    })
}

fn sampler() -> wgpu::BindingType {
    wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
}

fn scene_entries() -> [wgpu::BindGroupLayoutEntry; 10] {
    let (both, compare) =
        (S::VERTEX | S::FRAGMENT, wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison));
    [
        entry(0, both, uniform(true)),
        entry(1, S::FRAGMENT, picture()),
        entry(2, S::FRAGMENT, sampler()),
        entry(3, both, uniform(false)),
        entry(4, S::FRAGMENT, depth(D2Array)),
        entry(5, S::FRAGMENT, depth(D2Array)),
        entry(6, S::FRAGMENT, compare),
        entry(7, S::FRAGMENT, uniform(false)),
        entry(8, S::FRAGMENT, depth(Cube)),
        entry(9, S::FRAGMENT, depth(Cube)),
    ]
}

fn maps_entries() -> Vec<wgpu::BindGroupLayoutEntry> {
    (0..5)
        .map(|k| entry(k, S::FRAGMENT, picture()))
        .chain((5..10).map(|k| entry(k, S::FRAGMENT, sampler())))
        .chain([entry(10, S::FRAGMENT, uniform(false)), entry(11, S::VERTEX, picture())])
        .chain((12..16).map(|k| entry(k, S::FRAGMENT, picture())))
        .chain([entry(16, S::FRAGMENT, sampler())])
        .collect()
}

pub(super) fn layouts(device: &wgpu::Device) -> [wgpu::BindGroupLayout; 4] {
    let deltas = wgpu::BindingType::Texture {
        sample_type: wgpu::TextureSampleType::Float { filterable: false },
        view_dimension: wgpu::TextureViewDimension::D2,
        multisampled: false,
    };
    let make = |label: &str, entries: &[wgpu::BindGroupLayoutEntry]| {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some(label), entries })
    };
    [
        make("wprs-render solid scene", &scene_entries()),
        make(
            "wprs-render solid object",
            &[
                entry(0, S::VERTEX, uniform(true)),
                entry(1, S::VERTEX, uniform(true)),
                entry(2, S::VERTEX, uniform(true)),
                entry(3, S::VERTEX, deltas),
            ],
        ),
        make("wprs-render solid maps", &maps_entries()),
        make("wprs-render solid finish", &[entry(0, S::VERTEX | S::FRAGMENT, uniform(true))]),
    ]
}

#[cfg(test)]
mod tests {
    use super::{S, maps_entries, scene_entries};

    const NEXT_MAPS_BINDING: u32 = 17;
    const FRAGMENT_TEXTURES: usize = 16;

    #[test]
    fn new_maps_bindings_start_after_the_last_and_fragment_pictures_stay_within_the_limit() {
        let maps = maps_entries();
        let bindings: Vec<u32> = maps.iter().map(|e| e.binding).collect();
        assert_eq!(bindings, (0..NEXT_MAPS_BINDING).collect::<Vec<_>>());
        let pictures = |e: &&wgpu::BindGroupLayoutEntry| {
            e.visibility.contains(S::FRAGMENT) && matches!(e.ty, wgpu::BindingType::Texture { .. })
        };
        let scene = scene_entries();
        let used = maps.iter().chain(&scene).filter(pictures).count();
        assert!(used <= FRAGMENT_TEXTURES, "{used} pictures in the fragment stage");
        assert_eq!(used, 14);
    }
}
