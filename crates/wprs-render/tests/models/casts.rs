use super::*;
use wprs_scene::Tier;

pub fn backdrop(block: Vec<u8>, material: Value, wind: [f32; 2]) -> Scene {
    let ground = Forge::default()
        .material(ForgeMaterial { roughness: 1.0, ..ForgeMaterial::new("ground", [1.0; 4]) })
        .part("ground", ForgeShape::Plane, [0.0; 3], 6.0, 0)
        .glb();
    let sun = json!({ "id": "sun", "type": "light", "light": "directional", "intensity": 3,
        "transform": { "tilt": [45, 0] }, "shadows": true });
    let layers = json!([sun, { "id": "world", "type": "group", "space": "3d", "layers": [
        { "id": "ground", "type": "model", "model": "ground" },
        { "id": "block", "type": "model", "model": "block", "position": [0, 0.8, 1], "materials": { "block": material } }] }]);
    let canvas = json!({ "canvas": { "size": [SIDE, SIDE], "background": "#000000", "wind": wind } });
    let mut s = scene(canvas, layers, BTreeMap::from([("ground", ground), ("block", block)]));
    s.ext.insert(
        "editor".into(),
        json!({ "debug3d": { "show": "shadows", "layer": "ground", "material": "ground" } }),
    );
    s.drafted();
    s
}

pub fn shaded(px: &[u8]) -> Vec<(u32, u32)> {
    let all = (0..SIDE).flat_map(|y| (0..SIDE).map(move |x| (x, y)));
    all.filter(|(x, y)| {
        let [r, g, b] = rgb(px, *x, *y);
        r > 120 && u32::from(r) > u32::from(g) + 50 && b > g
    })
    .collect()
}

fn middle(points: &[(u32, u32)]) -> f32 {
    points.iter().map(|p| p.0 as f32).sum::<f32>() / points.len().max(1) as f32
}

fn green(px: &[u8]) -> Vec<(u32, u32)> {
    let all = (0..SIDE).flat_map(|y| (0..SIDE).map(move |x| (x, y)));
    all.filter(|(x, y)| {
        let [r, g, b] = rgb(px, *x, *y);
        g > 150 && r < 80 && b < 80
    })
    .collect()
}

#[test]
fn the_shadow_of_foliage_sways_with_its_surface() {
    let Some(r) = renderer() else { return };
    r.force_tier(Some(Tier::T3));
    let block = Forge::default()
        .material(unlit("block", [0.0, 1.0, 0.0, 1.0]))
        .part("block", ForgeShape::Cube, [0.0; 3], 0.6, 0)
        .glb();
    let leaf = json!({ "shader": "foliage", "params": { "strength": 10, "speed": 1 } });
    let mut shot = shoot(r, backdrop(block, leaf, [400.0, 0.0]), &[]);
    let mut seen = Vec::new();
    for dt in [0.0, 0.8, 0.8, 0.8] {
        let px = shot.frame(r, dt, Quality::Q3);
        let (body, shadow) = (green(&px), shaded(&px));
        assert!(body.len() > 10 && shadow.len() > 10, "{} {}", body.len(), shadow.len());
        seen.push((middle(&body), middle(&shadow)));
    }
    let moved = seen.windows(2).filter(|w| (w[1].0 - w[0].0).abs() > 0.8).count();
    assert!(moved >= 2, "the surface sways: {seen:?}");
    for w in seen.windows(2) {
        let (body, shadow) = (w[1].0 - w[0].0, w[1].1 - w[0].1);
        assert!((body - shadow).abs() < 0.6 + body.abs() * 0.5, "the shadow follows the surface: {seen:?}");
    }
}

#[test]
fn fur_casts_a_shadow_as_long_as_its_strands_above_the_lowest_qualities() {
    let Some(r) = renderer() else { return };
    r.force_tier(Some(Tier::T3));
    let ball = || {
        let skin = unlit("block", [0.0, 1.0, 0.0, 1.0]);
        Forge::default().material(skin).part("ball", ForgeShape::Sphere(12), [0.0; 3], 0.6, 0).glb()
    };
    let fur = json!({ "shader": "fur", "params": { "layers": 12, "length": 60, "density": 0.9 } });
    let cast =
        |material: Value, q: Quality| shaded(&render(r, backdrop(ball(), material, [0.0; 2]), q)).len();
    let (bare, long, low) =
        (cast(json!({}), Quality::Q3), cast(fur.clone(), Quality::Q3), cast(fur.clone(), Quality::Q1));
    let bare_low = cast(json!({}), Quality::Q1);
    assert!(bare > 20 && long > bare * 2, "the strands darken the backdrop beyond the skin: {bare} {long}");
    assert!(low.abs_diff(bare_low) <= 6, "below Q2 fur casts its skin only: {low} {bare_low}");
}
