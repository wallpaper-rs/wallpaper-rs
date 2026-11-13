use super::kit::{Gl, arm, fox, scene};
use serde_json::json;
use wprs_scene::{MAX_RIG_PALETTE, MAX_RIG_VERTICES, Rigged, VERTEX_FLOATS};

fn refused(gl: &Gl) -> String {
    Rigged::decode(&gl.bytes()).unwrap_err()
}

fn attribute(gl: &mut Gl, name: &str, accessor: usize) {
    gl.doc["meshes"][0]["primitives"][0]["attributes"][name] = json!(accessor);
}

#[test]
fn the_arm_decodes_into_bones_parts_shapes_and_clips() {
    let rig = Rigged::decode(&arm().bytes()).unwrap();
    assert_eq!((rig.joints.len(), rig.palette.len(), rig.pieces.len(), rig.vertex_count()), (5, 4, 2, 14));
    assert_eq!(
        (rig.parts.clone(), rig.shapes.clone()),
        (vec!["body".to_string(), "patch".into()], vec!["lift".to_string()])
    );
    assert_eq!((rig.motions[0].name.as_str(), rig.motions[0].length, rig.morphs.len()), ("wave", 1.0, 10));
    assert!(rig.descends(3, 1) && !rig.descends(1, 3));
    let scene = scene(fox(json!({})), arm().bytes()).unwrap();
    assert!(scene.rigs.contains_key("arm"));
}

#[test]
fn bone_cycles_and_nodes_with_two_parents_are_refused() {
    let mut gl = arm();
    gl.doc["nodes"][3]["children"] = json!([1]);
    assert!(refused(&gl).contains("cycle"), "{}", refused(&gl));
    let mut gl = arm();
    gl.doc["nodes"][4]["children"] = json!([3]);
    assert!(refused(&gl).contains("two parents"));
    let mut gl = arm();
    gl.doc["nodes"][2]["children"] = json!([2]);
    assert!(refused(&gl).contains("cycle"));
}

#[test]
fn weights_must_add_up_to_one_and_bones_and_indices_must_exist() {
    let mut gl = arm();
    let w = gl.floats("VEC4", &[[0.5f32, 0.2, 0.0, 0.0]; 10].concat());
    attribute(&mut gl, "WEIGHTS_0", w);
    assert!(refused(&gl).contains("add up to 0.7"), "{}", refused(&gl));
    let mut gl = arm();
    let w = gl.floats("VEC4", &[[1.0f32, 0.0, 0.0, 0.0]; 10].concat());
    let j = gl.joints(&[[7u16, 0, 0, 0]; 10].concat());
    attribute(&mut gl, "WEIGHTS_0", w);
    attribute(&mut gl, "JOINTS_0", j);
    assert!(refused(&gl).contains("missing bone 7"));
    let mut gl = arm();
    let i = gl.indices(&[0, 1, 99]);
    gl.doc["meshes"][0]["primitives"][0]["indices"] = json!(i);
    assert!(refused(&gl).contains("index out of the range"));
    let mut gl = arm();
    gl.doc["skins"][0]["joints"] = json!([1, 2, 30]);
    assert!(refused(&gl).contains("missing node 30"));
}

#[test]
fn values_that_are_not_numbers_and_unread_layouts_are_refused() {
    let mut gl = arm();
    let p = gl.floats("VEC3", &[[f32::NAN, 0.0, 0.0]; 10].concat());
    attribute(&mut gl, "POSITION", p);
    assert!(refused(&gl).contains("not a number"));
    let mut gl = arm();
    let keys = gl.indices(&[2]);
    let moved = gl.floats("VEC3", &[100.0, 0.0, 0.0]);
    let view = |gl: &Gl, a: usize| gl.doc["accessors"][a]["bufferView"].clone();
    let (keys, moved) = (view(&gl, keys), view(&gl, moved));
    gl.doc["accessors"][0]["sparse"] = json!({ "count": 1, "indices": { "bufferView": keys, "componentType": 5125 },
        "values": { "bufferView": moved } });
    let rig = Rigged::decode(&gl.bytes()).unwrap();
    assert_eq!(rig.vertices[2 * VERTEX_FLOATS..2 * VERTEX_FLOATS + 3], [100.0, 0.0, 0.0]);
    assert_eq!(rig.vertices[3 * VERTEX_FLOATS], 12.0);
    gl.doc["accessors"][0]["sparse"]["count"] = json!(4);
    assert!(refused(&gl).contains("sparse values past"));
    gl.doc["accessors"][0]["sparse"]["count"] = json!(1);
    gl.doc["accessors"][0]["sparse"]["indices"]["componentType"] = json!(5126);
    assert!(refused(&gl).contains("sparse indices of an unknown type"));
    let mut gl = arm();
    gl.doc["meshes"][1]["primitives"][0]["mode"] = json!(1);
    assert!(refused(&gl).contains("other than triangles"));
    let mut gl = arm();
    gl.doc["accessors"][0]["count"] = json!(1000);
    assert!(refused(&gl).contains("past its buffer view"));
    let mut gl = arm();
    gl.doc["nodes"][1]["translation"] = json!([1e39, 0.0, 0.0]);
    assert!(Rigged::decode(&gl.bytes()).is_err());
    let mut gl = arm();
    let backwards = gl.floats("SCALAR", &[1.0, 0.0]);
    gl.doc["animations"][0]["samplers"][0]["input"] = json!(backwards);
    assert!(refused(&gl).contains("unsorted"));
}

#[test]
fn meshes_are_capped_in_vertices_bones_and_parts() {
    let mut gl = Gl::new();
    let many = gl.floats("VEC3", &vec![0.0; (MAX_RIG_VERTICES + 1) * 3]);
    gl.doc["nodes"] = json!([{ "mesh": 0 }]);
    gl.doc["meshes"] = json!([{ "primitives": [{ "attributes": { "POSITION": many } }] }]);
    assert!(refused(&gl).contains("vertices"), "{}", refused(&gl));
    let mut gl = Gl::new();
    let one = gl.floats("VEC3", &[0.0; 9]);
    let nodes: Vec<_> = (0..=MAX_RIG_PALETTE).map(|_| json!({ "mesh": 0 })).collect();
    gl.doc["nodes"] = json!(nodes);
    gl.doc["meshes"] = json!([{ "primitives": [{ "attributes": { "POSITION": one } }] }]);
    assert!(refused(&gl).contains("bones and parts"), "{}", refused(&gl));
}

#[test]
fn every_picture_its_parts_name_is_kept_with_the_part_that_shows_it() {
    let mut gl = arm();
    let first = gl.indices(&[1, 2, 3]);
    let second = gl.indices(&[4, 5, 6, 7]);
    let view = |gl: &Gl, a: usize| gl.doc["accessors"][a]["bufferView"].clone();
    gl.doc["images"] = json!([{ "bufferView": view(&gl, first) }, { "bufferView": view(&gl, second) }]);
    gl.doc["textures"] = json!([{ "source": 1 }, { "source": 0 }]);
    gl.doc["materials"] = json!([{ "pbrMetallicRoughness": { "baseColorTexture": { "index": 0 } } },
        { "pbrMetallicRoughness": { "baseColorTexture": { "index": 1 } } }]);
    gl.doc["meshes"][0]["primitives"][0]["material"] = json!(0);
    gl.doc["meshes"][1]["primitives"][0]["material"] = json!(1);
    let rig = Rigged::decode(&gl.bytes()).unwrap();
    assert_eq!((rig.pictures.len(), rig.sheets.clone()), (2, vec![0, 1]));
    assert_eq!(rig.pictures[1].len(), 12);
    gl.doc["meshes"][1]["primitives"][0]["material"] = json!(0);
    let rig = Rigged::decode(&gl.bytes()).unwrap();
    assert_eq!((rig.pictures.len(), rig.sheets), (1, vec![0, 0]));
}

fn textured(images: usize) -> Gl {
    let mut gl = arm();
    let data = gl.indices(&[1, 2, 3]);
    let view = gl.doc["accessors"][data]["bufferView"].clone();
    gl.doc["images"] = json!((0..images).map(|_| json!({ "bufferView": view })).collect::<Vec<_>>());
    gl.doc["textures"] = json!((0..images).map(|i| json!({ "source": i })).collect::<Vec<_>>());
    let paint = |i: usize| json!({ "pbrMetallicRoughness": { "baseColorTexture": { "index": i } } });
    gl.doc["materials"] = json!((0..images).map(paint).collect::<Vec<_>>());
    gl.doc["meshes"][0]["primitives"][0]["material"] = json!(0);
    gl
}

#[test]
fn a_file_may_hold_more_than_eight_pictures() {
    let mut gl = textured(12);
    let nodes = gl.doc["nodes"].as_array().unwrap().len();
    for k in 1..12 {
        let mut mesh = gl.doc["meshes"][1].clone();
        mesh["primitives"][0]["material"] = json!(k);
        gl.doc["meshes"].as_array_mut().unwrap().push(mesh);
        let at = gl.doc["meshes"].as_array().unwrap().len() - 1;
        gl.doc["nodes"].as_array_mut().unwrap().push(json!({ "mesh": at }));
    }
    assert_eq!(gl.doc["nodes"].as_array().unwrap().len(), nodes + 11);
    let rig = Rigged::decode(&gl.bytes()).unwrap();
    assert_eq!(rig.pictures.len(), 12);
    assert_eq!(rig.sheets.iter().copied().max(), Some(11));
}

#[test]
fn texture_coordinates_past_one_ask_for_that_many_repeats() {
    let mut gl = textured(1);
    let rig = Rigged::decode(&gl.bytes()).unwrap();
    assert!(rig.repeats.is_empty(), "{:?}", rig.repeats);
    let uvs: Vec<f32> = (0..10).flat_map(|i| [i as f32 * 0.3 - 0.2, 0.5]).collect();
    let uv = gl.floats("VEC2", &uvs);
    gl.doc["meshes"][0]["primitives"][0]["attributes"]["TEXCOORD_0"] = json!(uv);
    let rig = Rigged::decode(&gl.bytes()).unwrap();
    assert_eq!(rig.repeats, [[-1.0, 0.0, 4.0, 1.0]]);
}
