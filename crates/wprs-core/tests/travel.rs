use wprs_core::edit::travel::{PHONE_GAIN, Reach, Still, gain, most_zoom, shift, travel};
use wprs_core::{Edit, Fit, UvRect};

fn whole() -> UvRect {
    UvRect { offset: [0.0; 2], scale: [1.0; 2], repeat: false }
}

#[test]
fn the_zoom_hides_the_frame_edge_for_the_shift_the_picture_moves() {
    let reach = Reach { shift: 0.02, low: 0.1, high: 0.9 };
    let wide = travel(0.02, &reach, (0.3, 1.0), (16.0 / 9.0, &whole()));
    assert_eq!(wide.shift, 0.02);
    let need = 0.02 * 0.6 * 16.0 / 9.0;
    assert!((wide.zoom - 0.5 / (0.5 - need)).abs() < 1e-4, "{wide:?}");
    let square = travel(0.02, &reach, (0.3, 1.0), (1.0, &whole()));
    assert!(square.zoom < wide.zoom && square.zoom > 1.0);
    let cropped = UvRect::compute(Fit::Fill, (3000, 1000), (1920, 1080));
    let room = travel(0.02, &reach, (0.3, 1.0), (3.0, &cropped));
    let uncut = travel(0.02, &reach, (0.3, 1.0), (3.0, &whole()));
    assert!(room.zoom <= uncut.zoom + 1e-6, "a cropped picture has room at its sides: {room:?}");
    assert_eq!(travel(0.0, &reach, (0.3, 1.04), (1.0, &whole())).zoom, 1.04);
}

#[test]
fn the_shift_is_cut_by_the_set_and_by_the_largest_zoom() {
    let safe = Reach { shift: 0.01, low: 0.2, high: 0.8 };
    assert_eq!(travel(0.03, &safe, (0.3, 1.0), (1.5, &whole())).shift, 0.01);
    let deep = Reach { shift: 1.0, low: 0.0, high: 1.0 };
    let far = travel(1.0, &deep, (0.0, 1.0), (4.0, &whole()));
    assert!(far.shift < 0.1 && (far.zoom - most_zoom()).abs() < 1e-4, "{far:?}");
    let still = travel(0.02, &Reach::default(), (0.3, 1.0), (1.0, &whole()));
    assert_eq!(still.shift, 0.0);
}

#[test]
fn the_phone_and_the_desktop_share_the_gain_and_the_shift() {
    assert_eq!((gain(false, 0.4), gain(true, 1.0), gain(true, 0.5)), (1.0, PHONE_GAIN, PHONE_GAIN * 0.5));
    assert_eq!(gain(true, 5.0), PHONE_GAIN * 2.0);
    assert_eq!(shift([0.5, -0.5], 0.06, 0.5), [0.03, -0.015]);
    let reach = Reach { shift: 0.02, low: 0.1, high: 0.6 };
    assert!(reach.moves(1920) && !Reach { high: 0.1, ..reach }.moves(1920));
    assert!((reach.lean(0.5) - 0.4).abs() < 1e-6);
}

#[test]
fn still_reasons_have_words_and_messages() {
    for still in [Still::NoMap, Still::FillOff, Still::Edited, Still::Building, Still::Failed, Still::Flat] {
        let word = serde_json::to_value(still).unwrap();
        assert_eq!(word.as_str(), Some(still.word()));
        assert!(still.message().to_string().starts_with("The picture stays still"));
    }
}

#[test]
fn joins_and_splits_round_trip_and_are_checked() {
    let text = r#"{"version": 1, "depth": {"joins": [{"a": [0.1, 0.2], "b": [0.8, 0.5]}],
      "splits": [{"points": [[0.0, 0.5], [1.0, 0.5]], "radius": 0.01, "falloff": 0.0, "by": 1.0}]}}"#;
    let edit = Edit::parse(text).unwrap();
    let depth = edit.depth.clone().unwrap();
    assert_eq!((depth.joins.len(), depth.splits.len()), (1, 1));
    assert_eq!(Edit::parse(&edit.to_json()).unwrap(), edit);
    assert!(!Edit::default().to_json().contains("joins"));
    let outside = text.replace("0.8, 0.5", "1.8, 0.5");
    assert!(Edit::parse(&outside).is_err());
}
