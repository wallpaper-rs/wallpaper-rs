use super::*;

const TOOLED: &str = "blend_gradient map gray; color_lut lut table; depth_parallax depth gray depth;
    displace map gray flow; fire gradient ramp; flow map gray flow; flow phase gray; fluid gradient ramp;
    light_shafts gradient ramp; pulse phase gray; shake flow flow; shake phase gray; shine gradient ramp; sway phase gray;
    tilt_shift depth gray depth; water_ripple reflection gray; water_ripple walls gray; waves phase gray";

#[test]
fn built_in_effects_declare_the_editor_tools_of_their_pictures() {
    let mut found = Vec::new();
    for d in builtin() {
        for p in d.params.iter().filter(|p| !p.tools.is_empty()) {
            let names: Vec<String> = p.tools.iter().map(|t| format!("{t:?}").to_lowercase()).collect();
            found.push(format!("{} {} {}", d.id, p.id, names.join(" ")));
        }
        for p in &d.params {
            assert_eq!(
                p.tools(&d.id),
                p.tools,
                "{} {}: the declaration matches the id convention",
                d.id,
                p.id
            );
        }
    }
    found.sort();
    let pinned: Vec<String> =
        TOOLED.split(';').map(|r| r.split_whitespace().collect::<Vec<_>>().join(" ")).collect();
    assert_eq!(found, pinned);
}

#[test]
fn declared_tools_override_the_id_and_must_fit_the_kind() {
    let made = |params: Json| {
        let json = json!({ "id": "own", "cost": {}, "passes": [{ "entry": "fs_own" }], "params": params });
        Descriptor::parse(&json.to_string(), "").unwrap()
    };
    let d = made(json!([{ "id": "colors", "type": "texture", "default": "white", "tools": ["ramp"] }]));
    assert_eq!(d.params[0].tools("own"), [Tool::Ramp]);
    let plain = made(json!([{ "id": "map", "type": "texture", "default": "white" }]));
    assert_eq!(plain.params[0].tools("own"), [Tool::Gray, Tool::Flow], "the id is the fallback");
    let wrong = made(json!([{ "id": "gain", "type": "float", "default": 0, "tools": ["gray"] }]));
    assert!(wrong.problems().iter().any(|p| p.contains("tools")), "{:?}", wrong.problems());
    let table = made(json!([{ "id": "grade", "type": "texture", "default": "white", "tools": ["table"] }]));
    assert!(table.problems().iter().any(|p| p.contains("tools")));
}
