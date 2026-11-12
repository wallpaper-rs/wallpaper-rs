use super::{ask, edit, layer, project, undone};
use serde_json::{Value, json};

fn middle(p: &wprs_editor::Project) -> [f64; 2] {
    let stage = ask(p, json!({ "stage3d": { "time": 0 } }));
    let at = &stage["models"][0]["at"];
    [at[0].as_f64().unwrap(), at[1].as_f64().unwrap()]
}

#[test]
fn a_point_of_the_view_finds_the_surface_and_its_color_and_keys_a_chroma_material() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    edit(&mut p, json!({ "model": { "add": { "asset": "toy", "id": "doll" } } })).unwrap();
    let at = middle(&p);
    let hit = ask(&p, json!({ "surface3d": { "at": at, "time": 0 } }));
    assert_eq!((hit["layer"].clone(), hit["material"].clone()), (json!("doll"), json!("paint")), "{hit}");
    assert_eq!(hit["color"], "#c82828", "{hit}");
    assert!(hit["normal"][2].as_f64().unwrap() > 0.9, "the face toward the camera: {hit}");
    assert!(ask(&p, json!({ "surface3d": { "at": [-500, -500] } })).is_null());
    let key = json!({ "layer": "doll", "name": "paint", "at": at });
    assert!(edit(&mut p, json!({ "key": key.clone() })).is_err(), "only chroma takes a key");
    edit(&mut p, json!({ "material": { "layer": "doll", "name": "paint", "shader": "chroma" } })).unwrap();
    let before = p.doc.clone();
    edit(&mut p, json!({ "key": key })).unwrap();
    assert_eq!(layer(&p, "doll")["materials"]["paint"]["params"]["key"], "#c82828");
    undone(&mut p, &before);
    let off = json!({ "layer": "doll", "name": "paint", "at": [-500, -500] });
    assert!(edit(&mut p, json!({ "key": off })).is_err());
}

#[test]
fn a_drag_over_the_model_combs_its_fur_into_a_map_and_undoes() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    edit(&mut p, json!({ "model": { "add": { "asset": "toy", "id": "doll" } } })).unwrap();
    let [x, y] = middle(&p);
    let points: Vec<Value> = (0..9).map(|k| json!([x - 8.0 + 2.0 * f64::from(k), y])).collect();
    let stroke = json!({ "layer": "doll", "name": "paint", "points": points, "radius": 6 });
    assert!(edit(&mut p, json!({ "comb": stroke.clone() })).is_err(), "only fur is combed");
    edit(&mut p, json!({ "material": { "layer": "doll", "name": "paint", "shader": "fur" } })).unwrap();
    let before = p.doc.clone();
    edit(&mut p, json!({ "comb": stroke.clone() })).unwrap();
    let id = layer(&p, "doll")["materials"]["paint"]["maps"]["comb"].as_str().unwrap().to_owned();
    let asset = p.doc["scene"]["assets"][&id].clone();
    assert_eq!(asset["linear"], true);
    let picture = image::open(p.dir.join(asset["path"].as_str().unwrap())).unwrap().into_rgba8();
    let hit = ask(&p, json!({ "surface3d": { "at": [x, y] } }));
    let uv = [hit["uv"][0].as_f64().unwrap(), hit["uv"][1].as_f64().unwrap()];
    let (w, h) = picture.dimensions();
    let px = picture
        .get_pixel(
            (uv[0].rem_euclid(1.0) * f64::from(w)) as u32,
            (uv[1].rem_euclid(1.0) * f64::from(h)) as u32,
        )
        .0;
    assert!(
        px[3] > 200 && px[0] > 200 && px[1].abs_diff(128) < 30,
        "combed to the right of the model: {px:?}"
    );
    let far = picture.get_pixel(0, 0).0;
    assert_eq!(far[3], 0, "the rest stays unpainted");
    undone(&mut p, &before);
    edit(&mut p, json!({ "comb": stroke.clone() })).unwrap();
    edit(&mut p, json!({ "material": { "layer": "doll", "name": "paint", "maps": { "comb": null } } }))
        .unwrap();
    assert!(layer(&p, "doll")["materials"]["paint"].get("maps").is_none(), "the combing is cleared");
    let mut erase = stroke.clone();
    erase["erase"] = json!(true);
    edit(&mut p, json!({ "comb": erase })).unwrap();
    let missed =
        json!({ "layer": "doll", "name": "paint", "points": [[-500, -500], [-490, -500]], "radius": 6 });
    assert!(edit(&mut p, json!({ "comb": missed })).is_err());
}
