struct Bulb {
    at: vec4<f32>,
    color: vec4<f32>,
    dir: vec4<f32>,
    more: vec4<f32>,
    air: vec4<f32>,
};

struct Lights {
    head: vec4<f32>,
    ambient: vec4<f32>,
    sky: vec4<f32>,
    bulbs: array<Bulb, 12>,
    crops: array<vec4<f32>, 2>,
};

fn reach(b: Bulb, p: vec2<f32>) -> vec4<f32> {
    return reach_at(b, p, 0.0);
}

fn reach_at(b: Bulb, p: vec2<f32>, z: f32) -> vec4<f32> {
    let kind = b.color.w;
    if kind > 2.5 {
        return vec4<f32>(-b.dir.xyz, 1.0);
    }
    var src = b.at.xy;
    if kind > 1.5 {
        let ab = b.more.yz * 2.0;
        let t = clamp(dot(p - (src - b.more.yz), ab) / max(dot(ab, ab), 1e-6), 0.0, 1.0);
        src = src - b.more.yz + ab * t;
    }
    let v = vec3<f32>(src - p, b.at.z - z);
    let d = length(v);
    if d >= b.at.w {
        return vec4<f32>(0.0);
    }
    let f = 1.0 - d * d / (b.at.w * b.at.w);
    let l = v / max(d, 1e-3);
    var k = f * f;
    if kind > 0.5 && kind < 1.5 {
        k *= smoothstep(b.dir.w, b.more.x, dot(-l, b.dir.xyz));
    }
    return vec4<f32>(l, k);
}

fn cast_at(b: Bulb, l: vec3<f32>) -> vec3<f32> {
    let slot = u32(b.air.z + 0.5);
    if slot == 0u || b.color.w < 0.5 || b.color.w > 1.5 {
        return vec3<f32>(0.0);
    }
    let a = b.dir.xyz;
    let level = length(a.xy) > 1e-4;
    let side = select(vec3<f32>(1.0, 0.0, 0.0), normalize(vec3<f32>(a.y, -a.x, 0.0)), level);
    let up = cross(side, a);
    let w = -l;
    let wide = sqrt(max(1.0 - b.dir.w * b.dir.w, 1e-6)) / max(b.dir.w, 1e-3);
    let crop = lights.crops[(slot - 1u) / 2u];
    let fit = select(crop.xy, crop.zw, (slot - 1u) % 2u == 1u);
    let uv = clamp(0.5 + 0.5 * vec2<f32>(dot(w, side), dot(w, up)) / (max(dot(w, a), 1e-3) * wide), vec2<f32>(0.0), vec2<f32>(1.0)) * fit;
    return vec3<f32>(uv, f32(slot));
}

fn cast_from(at: vec3<f32>, a: texture_2d<f32>, b: texture_2d<f32>, c: texture_2d<f32>, d: texture_2d<f32>) -> vec3<f32> {
    let slot = u32(at.z + 0.5);
    if slot == 0u {
        return vec3<f32>(1.0);
    }
    var t = textureSampleLevel(a, samp, at.xy, 0.0);
    if slot == 2u {
        t = textureSampleLevel(b, samp, at.xy, 0.0);
    } else if slot == 3u {
        t = textureSampleLevel(c, samp, at.xy, 0.0);
    } else if slot == 4u {
        t = textureSampleLevel(d, samp, at.xy, 0.0);
    }
    return t.rgb;
}
