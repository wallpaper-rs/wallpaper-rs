use super::*;

#[test]
fn the_form_gives_an_effect_a_lighter_variant_with_its_passes_cost_and_level() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    shader(&mut p, json!({ "new": { "name": "soft", "template": "passes" } })).unwrap();
    let up_to = json!({ "effect": "soft", "set": { "up_to": "q1" } });
    shader(&mut p, json!({ "lighter": up_to })).unwrap();
    let made = descriptor(&p, "soft")["lighter"].clone();
    assert_eq!(
        made,
        json!({ "passes": [{ "entry": "fs_soft_lite" }], "cost": { "taps": 1, "passes": 1, "scale": 1 }, "up_to": "q1" })
    );
    let code = std::fs::read_to_string(p.dir.join("effects/soft.wgsl")).unwrap();
    assert!(code.contains("fn fs_soft_lite(") && code.contains("fn fs_soft_soft("), "{code}");
    let checked = ask(&p, json!({ "shader_check": { "effect": "soft" } }));
    assert_eq!(checked["ok"], true, "{checked}");
    let forms: Vec<Value> = checked["levels"].as_array().unwrap().iter().map(|l| l["form"].clone()).collect();
    assert_eq!(forms, [json!("lighter"), json!("lighter"), json!("full"), json!("full")]);
    assert!(checked["lighter"].as_f64().unwrap() < checked["units"].as_f64().unwrap(), "{checked}");
    let pass = json!({ "effect": "soft", "lighter": true, "set": { "scale": 0.5 } });
    shader(&mut p, json!({ "pass": pass })).unwrap();
    assert_eq!(descriptor(&p, "soft")["lighter"]["passes"][1]["entry"], "fs_soft_lite_1");
    let missing = ask(&p, json!({ "shader_check": { "effect": "soft" } }));
    assert!(
        missing["error"].as_str().unwrap().contains("fs_soft_lite_1 for its lighter variant"),
        "{missing}"
    );
    shader(&mut p, json!({ "pass": { "effect": "soft", "lighter": true, "index": 1, "remove": true } }))
        .unwrap();
    let dear = json!({ "effect": "soft", "set": { "cost": { "taps": 64, "passes": 2, "scale": 1 } } });
    shader(&mut p, json!({ "lighter": dear })).unwrap();
    let refused = ask(&p, json!({ "shader_check": { "effect": "soft" } }));
    assert!(refused["error"].as_str().unwrap().contains("must cost less than the full effect"), "{refused}");
    assert!(shader(&mut p, json!({ "lighter": { "effect": "soft", "set": { "passes": [] } } })).is_err());
    let cheap = json!({ "cost": { "taps": 1, "passes": 1, "scale": 1 }, "up_to": null });
    shader(&mut p, json!({ "lighter": { "effect": "soft", "set": cheap } })).unwrap();
    assert!(descriptor(&p, "soft")["lighter"].get("up_to").is_none());
    act(&mut p, json!({ "add_layer": { "kind": "solid", "id": "fill" } })).unwrap();
    act(&mut p, json!({ "effects": { "add": { "layer": "fill", "effect": "soft" } } })).unwrap();
    let cost = ask(&p, json!({ "effect_cost": { "effect": "soft", "optional": false } }));
    assert!(cost["levels"][3]["form"] == "full" && cost.get("lighter").is_some(), "{cost}");
    shader(&mut p, json!({ "lighter": { "effect": "soft", "remove": true } })).unwrap();
    assert!(descriptor(&p, "soft").get("lighter").is_none());
}
