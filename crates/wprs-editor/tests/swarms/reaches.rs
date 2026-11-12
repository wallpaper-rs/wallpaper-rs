use super::*;

fn reach(p: &mut Project, bounds: Value) -> Value {
    let n = system(p)["collide"].as_array().map_or(0, Vec::len);
    edit(p, "collider", json!({ "collider": { "bounds": bounds } })).unwrap();
    handles(p)["colliders"][n].clone()
}

fn solid_and_open(wall: &Value) -> (usize, usize) {
    let field = wall["field"].as_array().unwrap();
    let inner = field.iter().filter(|f| f[2].as_f64().unwrap() < 0.0).count();
    (inner, field.len() - inner)
}

#[test]
fn every_kind_of_layer_and_a_mask_show_what_a_collider_covers() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    act(&mut p, json!({ "mesh_from_picture": { "image": "pic", "cells": 8, "id": "net" } })).unwrap();
    let wall = reach(&mut p, json!({ "layer": { "id": "net" } }));
    let (inner, open) = solid_and_open(&wall);
    assert!(inner > 0 && open > 0 && wall["approximate"] == false, "{wall}");
    let label = json!({ "id": "label", "type": "text", "text": "Hi", "size": 64,
        "transform": { "position": [960, 540] } });
    act(&mut p, json!({ "add": { "pointer": "/scene/layers/-", "value": label } })).unwrap();
    let wall = reach(&mut p, json!({ "layer": { "id": "label" } }));
    let (inner, open) = solid_and_open(&wall);
    assert!(inner > 0 && open > 0 && wall["approximate"] == false, "{wall}");
    act(&mut p, json!({ "group": { "ids": ["logo"], "id": "pack" } })).unwrap();
    let wall = reach(&mut p, json!({ "layer": { "id": "pack" } }));
    let (inner, open) = solid_and_open(&wall);
    assert!(inner > 0 && open > 0, "{wall}");
    let near: Vec<f64> = wall["field"].as_array().unwrap().iter().map(|f| f[0].as_f64().unwrap()).collect();
    assert!(near.iter().all(|x| (900.0..1020.0).contains(x)), "the field stays around the group: {near:?}");
    let mask = reach(&mut p, json!({ "mask": { "texture": "pic", "threshold": 0.5 } }));
    let (inner, open) = solid_and_open(&mask);
    assert!(inner > 0 && open > 0 && mask["approximate"] == false, "{mask}");
    assert_eq!((mask["kind"].clone(), mask["corners"].as_array().map(Vec::len)), (json!("mask"), Some(4)));
    act(&mut p, json!({ "set": { "pointer": "/scene/assets/gone", "value": { "path": "gone.png" } } }))
        .unwrap();
    let lost = reach(&mut p, json!({ "mask": { "texture": "gone" } }));
    assert!(lost["field"] == json!([]) && lost["approximate"] == true, "{lost}");
}

#[test]
fn a_layer_and_a_mask_collider_give_a_grip_to_drag_on_the_canvas() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let wall = reach(&mut p, json!({ "layer": { "id": "logo" } }));
    let (outline, grip) = (wall["outline"].as_array().unwrap(), wall["grip"].as_array().unwrap());
    let at = |i: usize, c: usize| outline[i][c].as_f64().unwrap();
    for c in 0..2 {
        let middle = (at(0, c) + at(2, c)) / 2.0;
        assert!((grip[c].as_f64().unwrap() - middle).abs() < 0.02, "the grip is the middle of the layer: {wall}");
    }
    let mask = reach(&mut p, json!({ "mask": { "texture": "pic", "threshold": 0.25 } }));
    let (a, b) = (mask["corners"][0].clone(), mask["corners"][1].clone());
    let expect = a[0].as_f64().unwrap() + (b[0].as_f64().unwrap() - a[0].as_f64().unwrap()) * 0.25;
    assert!((mask["grip"][0].as_f64().unwrap() - expect).abs() < 0.02, "{mask}");
    assert_eq!(mask["grip"][1], a[1]);
}
