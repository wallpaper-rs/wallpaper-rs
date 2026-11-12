use super::kit::{act, query, rigged};
use serde_json::json;
use wprs_editor::scripts;

fn ask(p: &wprs_editor::Project, q: serde_json::Value) -> serde_json::Value {
    query(p, "net", json!({ "script_calls": q }))
}

#[test]
fn a_script_picks_the_bones_and_shapes_of_a_mesh_and_names_those_it_lacks() {
    let root = tempfile::tempdir().unwrap();
    let mut p = rigged(&root);
    act(&mut p, json!({ "set": { "pointer": "/scene/layers/0/physics", "value": [{ "bone": "arm" }] } }))
        .unwrap();
    act(&mut p, json!({ "shape_sculpt": { "layer": "net", "shape": "puff", "points": [[0.5, 0.5]], "mode": "push", "by": 5 } }))
        .unwrap();
    let listing = ask(&p, json!({ "layer": "net" }));
    assert_eq!(
        (listing["bones"].clone(), listing["shapes"].clone(), listing["bodies"].clone()),
        (json!(["root", "arm", "node_0"]), json!(["puff"]), json!(["arm"]))
    );
    let code = |kind: &str, name: &str, what: &str| {
        ask(&p, json!({ "layer": "net", "kind": kind, "name": name, "what": what }))
    };
    assert_eq!(code("bone", "arm", "turn")["code"], r#"scene.layer("net").bone("arm").rotation = 0;"#);
    assert_eq!(code("shape", "puff", "set")["code"], r#"scene.layer("net").shape("puff", 0.5);"#);
    assert_eq!(code("impulse", "arm", "push")["code"], r#"scene.layer("net").impulse("arm", [0, -300]);"#);
    for bad in [code("bone", "tail", "turn"), code("impulse", "root", "push"), code("shape", "puff", "turn")]
    {
        assert!(bad["error"].is_string(), "{bad}");
    }
    let text = "export function update() {\n  const n = scene.layer('net');\n  n.bone(\"arm\").rotation = 3;\n  n.bone(\"tail\").reset();\n  n.shape(\"frown\", 1);\n  n.shape('puff', 1);\n  n.impulse(\"arm\", [0, 1]);\n}\n";
    std::fs::create_dir_all(p.dir.join("scripts")).unwrap();
    std::fs::write(p.dir.join("scripts/x.js"), text).unwrap();
    act(&mut p, json!({ "script": { "add": { "path": "scripts/x.js" } } })).unwrap();
    assert_eq!(p.doc["scene"]["scripts"][0]["uses"], json!(["animations"]));
    let found = scripts::audit(&p.dir, &p.doc, "scripts/x.js").unwrap();
    assert_eq!(
        found["rig"],
        json!([
            { "kind": "bone", "name": "tail", "line": 4, "layers": ["net"] },
            { "kind": "shape", "name": "frown", "line": 5, "layers": ["net"] }
        ])
    );
}
