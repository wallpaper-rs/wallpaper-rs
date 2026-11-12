use super::*;

#[test]
fn every_template_makes_an_effect_that_passes_the_check_of_readers() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    for (i, template) in wprs_editor::wgsl::TEMPLATES.iter().enumerate() {
        let name = format!("made_{i}");
        shader(&mut p, json!({ "new": { "name": name, "template": template } })).unwrap();
        assert!(p.dir.join(format!("effects/{name}.wgsl")).is_file());
        let checked = ask(&p, json!({ "shader_check": { "effect": name } }));
        assert_eq!(checked["ok"], true, "{template}: {checked}");
        let source = ask(&p, json!({ "shader_source": { "effect": name } }));
        assert_eq!(source["parsed"]["id"], json!(name));
    }
    let distort = ask(&p, json!({ "shader_check": { "effect": "made_1" } }));
    assert_eq!(distort["handles"][0], json!({ "param": "center", "handle": "point", "partner": null }));
    let passes = ask(&p, json!({ "shader_check": { "effect": "made_2" } }));
    assert!(passes["handles"].as_array().unwrap().iter().any(|h| h["handle"] == "angle"), "{passes}");
    assert_eq!(ask(&p, json!({ "shader_source": { "effect": "made_3" } }))["stage"], "material");
    let taken = shader(&mut p, json!({ "new": { "name": "made_0" } })).unwrap_err().to_string();
    assert!(taken.contains("already"), "{taken}");
    let builtin = shader(&mut p, json!({ "new": { "name": "blur" } })).unwrap_err().to_string();
    assert!(builtin.contains("already"), "{builtin}");
    let unknown = shader(&mut p, json!({ "new": { "name": "odd", "template": "teapot" } }));
    assert!(unknown.unwrap_err().to_string().contains("teapot"));
    assert!(shader(&mut p, json!({ "new": { "name": "Bad Name" } })).is_err());
}
