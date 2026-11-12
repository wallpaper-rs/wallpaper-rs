use super::{layer, light, project, undone};
use serde_json::json;

#[test]
fn volumetrics_are_set_checked_undone_and_dropped_with_the_type() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    light(&mut p, json!({ "add": { "light": "spot", "id": "s" } })).unwrap();
    let before = p.doc.clone();
    light(&mut p, json!({ "volumetric": { "layer": "s", "density": 0.4, "samples": 24 } })).unwrap();
    undone(&mut p, &before);
    assert_eq!(layer(&p, "s")["volumetric"], json!({ "density": 0.4, "samples": 24 }));
    light(&mut p, json!({ "volumetric": { "layer": "s", "samples": 8 } })).unwrap();
    assert_eq!(layer(&p, "s")["volumetric"], json!({ "density": 0.4, "samples": 8 }));
    let before = p.doc.clone();
    light(&mut p, json!({ "volumetric": { "layer": "s", "shafts": 0.7 } })).unwrap();
    undone(&mut p, &before);
    assert_eq!(layer(&p, "s")["volumetric"], json!({ "density": 0.4, "samples": 8, "shafts": 0.7 }));
    assert_eq!(
        wprs_editor::lamps::air(&p.dir, &p.doc, "s", 0.0)["shafts"].as_f64().map(|v| (v * 100.0).round()),
        Some(70.0)
    );
    let shown = wprs_editor::lamps::air(&p.dir, &p.doc, "s", 0.0);
    let (full, dark) = (&shown["curve"]["points"][0][1], &shown["shaded"][0][1]);
    let k = dark.as_f64().unwrap() / full.as_f64().unwrap();
    assert!((k - 0.3).abs() < 1e-3, "over a dark part the glow keeps what the shafts let through: {k}");
    light(&mut p, json!({ "volumetric": { "layer": "s", "shafts": 0 } })).unwrap();
    assert_eq!(layer(&p, "s")["volumetric"], json!({ "density": 0.4, "samples": 8 }));
    for bad in [
        json!({ "shafts": 1.5 }),
        json!({ "shafts": -1 }),
        json!({ "density": 2 }),
        json!({ "samples": 2 }),
        json!({ "samples": 65 }),
        json!({ "density": -0.1 }),
    ] {
        let mut args = bad.clone();
        args["layer"] = json!("s");
        assert!(light(&mut p, json!({ "volumetric": args })).is_err(), "{bad}");
    }
    assert!(light(&mut p, json!({ "volumetric": { "layer": "logo" } })).is_err(), "an image is no light");
    light(&mut p, json!({ "set": { "layer": "s", "light": "tube" } })).unwrap();
    assert!(layer(&p, "s").get("volumetric").is_none(), "a tube does not light the air");
    assert!(light(&mut p, json!({ "volumetric": { "layer": "s" } })).is_err());
    light(&mut p, json!({ "set": { "layer": "s", "light": "point" } })).unwrap();
    light(&mut p, json!({ "volumetric": { "layer": "s" } })).unwrap();
    light(&mut p, json!({ "volumetric": { "layer": "s", "off": true } })).unwrap();
    assert!(layer(&p, "s").get("volumetric").is_none());
}

#[test]
fn the_air_preview_shows_its_curve_and_what_each_level_keeps() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    light(&mut p, json!({ "add": { "light": "point", "id": "l" } })).unwrap();
    let q = |p: &wprs_editor::Project| wprs_editor::lamps::air(&p.dir, &p.doc, "l", 0.0);
    assert_eq!(q(&p)["on"], false);
    assert_eq!(q(&p)["fits"], true);
    light(&mut p, json!({ "volumetric": { "layer": "l", "density": 0.5, "samples": 32 } })).unwrap();
    let air = q(&p);
    let curve = air["curve"]["points"].as_array().or(air["curve"].as_array()).cloned().unwrap_or_default();
    assert!(!curve.is_empty(), "{air}");
    let levels = air["levels"].as_array().unwrap();
    assert_eq!(levels[0]["on"], false, "off at Q0");
    assert_eq!((levels[1]["scale"].as_f64(), levels[1]["samples"].as_u64()), (Some(0.25), Some(16)));
    assert_eq!((levels[3]["scale"].as_f64(), levels[3]["samples"].as_u64()), (Some(0.5), Some(32)));
    assert_eq!(air["phone"][3]["scale"].as_f64(), Some(0.25), "phones work at a quarter");
    let taps = air["taps"].as_array().unwrap();
    assert!(taps[2].as_f64() > taps[0].as_f64(), "{taps:?}");
    let cost = wprs_editor::lamps::cost(&p.dir, &p.doc, 0.0, wprs_scene::Quality::Q3);
    assert_eq!(cost["volumetric"], 1);
    assert_eq!(cost["levels"][0]["air_taps"], 0.0);
    assert!(cost["levels"][3]["air_taps"].as_f64() > cost["phone"][3]["air_taps"].as_f64());
    assert_eq!(cost["levels"][3]["shaft_copy"], 0.0, "no shafts, no copy of the layers below");
    light(&mut p, json!({ "volumetric": { "layer": "l", "shafts": 0.5 } })).unwrap();
    let cost = wprs_editor::lamps::cost(&p.dir, &p.doc, 0.0, wprs_scene::Quality::Q3);
    let copy = |row: &serde_json::Value| row["shaft_copy"].as_f64().unwrap();
    assert!(copy(&cost["levels"][3]) > copy(&cost["levels"][1]) && copy(&cost["levels"][0]) == 0.0, "{cost}");
    let all = wprs_editor::lamps::lights(&p.dir, &p.doc, 0.0);
    assert_eq!(all["lights"][0]["volumetric"]["samples"], 32);
}
