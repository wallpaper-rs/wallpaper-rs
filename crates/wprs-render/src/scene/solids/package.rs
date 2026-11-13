pub(super) const FALLBACK: &str = "its models are shaded with pbr instead, since ";

const OWN: [&str; 61] = [
    "Light",
    "Scene3",
    "Object",
    "Bones",
    "Draw",
    "Shadows",
    "Finish",
    "In",
    "Bound",
    "Out",
    "Placed",
    "scene",
    "env",
    "env_smp",
    "shadow_a",
    "shadow_b",
    "shadow_cmp",
    "shade",
    "object",
    "bones",
    "draw",
    "morphs",
    "base_map",
    "orm_map",
    "normal_map",
    "occlusion_map",
    "emissive_map",
    "base_smp",
    "orm_smp",
    "normal_smp",
    "occlusion_smp",
    "emissive_smp",
    "uvs",
    "Uvs",
    "map_uv",
    "finish",
    "PI",
    "place",
    "skin",
    "delta",
    "out",
    "posed",
    "vs_solid",
    "vs_skinned",
    "vs_shadow",
    "vs_shadow_skinned",
    "ggx",
    "smith",
    "equirect",
    "falloff",
    "tap",
    "shadowed",
    "hashed",
    "Cut",
    "moved",
    "vs_shadow_cut",
    "vs_shadow_cut_skinned",
    "fs_shadow_cut",
    "shadow_c",
    "shadow_d",
    "round_tap",
];
const MORE: [&str; 3] = ["fs_solid", "shown", "lit"];

fn prefixed(src: &str) -> String {
    let (mut out, mut word) = (String::with_capacity(src.len() + 1024), String::new());
    for c in src.chars().chain([' ']) {
        if c.is_alphanumeric() || c == '_' {
            word.push(c);
            continue;
        }
        if OWN.contains(&word.as_str()) || MORE.contains(&word.as_str()) {
            out.push_str("wprs_");
        }
        out.push_str(&word);
        word.clear();
        out.push(c);
    }
    out.pop();
    out
}

pub(crate) fn solid_source() -> String {
    let own = prefixed(&super::SOURCE.concat());
    format!(
        "@group(0) @binding(3) var<uniform> frame: Frame;\nfn param(i: u32) -> vec4<f32> {{\n    return wprs_finish.params[min(i, 7u)];\n}}\n{own}"
    )
}
