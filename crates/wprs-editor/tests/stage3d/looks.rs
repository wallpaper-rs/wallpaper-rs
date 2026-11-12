use super::{act, ask, edit, layer, project, undone};
use image::{Rgba, RgbaImage};
use serde_json::json;

#[test]
fn a_model_file_tells_its_counts_and_what_each_tier_gets() {
    let root = tempfile::tempdir().unwrap();
    let p = project(&root);
    let info = ask(&p, json!({ "model_info": { "asset": "toy" } }));
    assert_eq!(info["triangles"], json!(12 + 2 * 8 * 16));
    assert_eq!(info["materials"].as_array().unwrap().len(), 2);
    assert_eq!(info["textures"][0]["size"], json!([8, 8]));
    assert_eq!(info["tiers"].as_array().unwrap().len(), 4);
    assert!(info["tiers"].as_array().unwrap().iter().all(|t| t["fits"] == true));
    assert_eq!(info["phone"]["texture"], json!(2048));
    let file = ask(&p, json!({ "model_info": { "file": "toy.glb" } }));
    assert_eq!(file["triangles"], info["triangles"]);
    std::fs::write(p.dir.join("bad.glb"), b"nothing").unwrap();
    assert!(ask(&p, json!({ "model_info": { "file": "bad.glb" } }))["error"].is_string());
}

#[test]
fn the_materials_panel_lists_maps_with_thumbnails_and_a_preview_ball() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    edit(&mut p, json!({ "model": { "add": { "asset": "toy", "id": "doll" } } })).unwrap();
    let plain = ask(&p, json!({ "materials3d": { "layer": "doll" } }));
    edit(&mut p, json!({ "material": { "layer": "doll", "name": "paint", "color": "#00ff00" } })).unwrap();
    let list = ask(&p, json!({ "materials3d": { "layer": "doll" } }));
    let paint = &list["materials"][0];
    assert_eq!(paint["name"], "paint");
    assert!(paint["maps"][0]["thumb"].as_str().unwrap().starts_with("data:image/png;base64,"));
    assert!(paint["maps"][1]["thumb"].is_null());
    assert_eq!(paint["set"]["color"], "#00ff00");
    assert_ne!(paint["sphere"], plain["materials"][0]["sphere"]);
    assert!(list["shaders"].as_array().unwrap().contains(&json!("unlit")));
}

#[test]
fn the_cost_panel_counts_triangles_texture_memory_and_tiers_per_quality() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    edit(&mut p, json!({ "model": { "add": { "asset": "toy", "id": "doll" } } })).unwrap();
    let cost = ask(&p, json!({ "cost3d": { "quality": "q2" } }));
    assert_eq!(cost["levels"].as_array().unwrap().len(), 4);
    assert_eq!(cost["levels"][3]["triangles"], json!(268));
    assert_eq!(cost["levels"][0]["specular"], false);
    assert!(cost["tiers"][0]["memory"].as_u64().unwrap() > 0);
    let lit = ask(&p, json!({ "lighting": { "time": 0, "quality": "q1" } }));
    assert_eq!(lit["models"]["models"][0]["id"], "doll");
}

#[test]
fn a_material_map_is_chosen_from_the_pictures_imported_into_its_slot_and_removed() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    edit(&mut p, json!({ "model": { "add": { "asset": "toy", "id": "doll" } } })).unwrap();
    let before = p.doc.clone();
    let pick = json!({ "material": { "layer": "doll", "name": "paint", "maps": { "base": "sky" } } });
    edit(&mut p, pick).unwrap();
    assert_eq!(layer(&p, "doll")["materials"]["paint"]["maps"], json!({ "base": "sky" }));
    undone(&mut p, &before);
    let wrong = json!({ "material": { "layer": "doll", "name": "paint", "maps": { "shine": "sky" } } });
    assert!(edit(&mut p, wrong).is_err());
    let missing = json!({ "material": { "layer": "doll", "name": "paint", "maps": { "orm": "toy" } } });
    assert!(edit(&mut p, missing).is_err(), "a model is not a picture");
    let bumps = root.path().join("bumps.png");
    RgbaImage::from_pixel(4, 4, Rgba([128, 128, 255, 255])).save(&bumps).unwrap();
    let file = json!({ "material": { "layer": "doll", "name": "paint", "files": { "normal": bumps } } });
    edit(&mut p, file).unwrap();
    let id = layer(&p, "doll")["materials"]["paint"]["maps"]["normal"].as_str().unwrap().to_owned();
    assert_eq!(p.doc["scene"]["assets"][&id]["linear"], json!(true), "a normal map is read as data");
    let list = ask(&p, json!({ "materials3d": { "layer": "doll" } }));
    let chosen = &list["materials"][0]["chosen"];
    assert_eq!((chosen[1]["slot"].clone(), chosen[1]["warn"].clone()), (json!("normal"), json!(false)));
    let gone = json!({ "material": { "layer": "doll", "name": "paint", "maps": { "normal": null } } });
    edit(&mut p, gone).unwrap();
    let base = json!({ "material": { "layer": "doll", "name": "paint", "maps": { "base": null } } });
    edit(&mut p, base).unwrap();
    assert!(layer(&p, "doll")["materials"]["paint"].get("maps").is_none());
}

#[test]
fn a_stroke_paints_a_map_of_a_material_over_the_picture_of_the_file() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    edit(&mut p, json!({ "model": { "add": { "asset": "toy", "id": "doll" } } })).unwrap();
    let before = p.doc.clone();
    let stroke = |slot: &str| {
        json!({ "paint": { "layer": "doll", "name": "paint", "slot": slot, "color": [0.0, 0.0, 1.0],
            "radius": 0.2, "hardness": 1.0, "points": [[0.5, 0.5]] } })
    };
    edit(&mut p, stroke("base")).unwrap();
    let id = layer(&p, "doll")["materials"]["paint"]["maps"]["base"].as_str().unwrap().to_owned();
    let asset = &p.doc["scene"]["assets"][&id];
    assert_eq!(asset["linear"], json!(false));
    let picture = image::open(p.dir.join(asset["path"].as_str().unwrap())).unwrap().into_rgba8();
    assert_eq!(picture.dimensions(), (8, 8), "the stroke starts from the map of the file");
    assert_eq!(picture.get_pixel(4, 4).0, [0, 0, 255, 255]);
    assert_eq!(picture.get_pixel(0, 0).0, [200, 40, 40, 255]);
    edit(&mut p, stroke("base")).unwrap();
    assert_eq!(
        layer(&p, "doll")["materials"]["paint"]["maps"]["base"],
        json!(id),
        "a second stroke keeps the map"
    );
    edit(&mut p, stroke("normal")).unwrap();
    let normal = layer(&p, "doll")["materials"]["paint"]["maps"]["normal"].as_str().unwrap().to_owned();
    assert_eq!(p.doc["scene"]["assets"][&normal]["linear"], json!(true));
    assert!(edit(&mut p, stroke("shine")).is_err());
    act(&mut p, json!({ "undo": {} })).unwrap();
    act(&mut p, json!({ "undo": {} })).unwrap();
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert_eq!(p.doc, before);
}
