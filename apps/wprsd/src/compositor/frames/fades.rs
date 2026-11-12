use super::super::Content;
use super::super::output::{Output, Shown, Sink};
use std::time::Instant;
use wprs_core::Fit;

#[test]
fn a_stream_nobody_watches_changes_picture_without_a_fade() {
    let shown = Shown {
        content: Content::Color,
        fit: Fit::Stretch,
        background: [0; 3],
        image: None,
        transition: None,
        uv: None,
        relief: None,
        scene: None,
        tone: Default::default(),
    };
    let mut output = Output::new((16, 9), shown, Sink::Stream(None), 60);
    assert!(!output.animates(true, true, false), "a fade before the first frame");
    output.last_render = Some(Instant::now());
    assert!(output.animates(true, true, false));
    assert!(
        !output.animates(true, false, false),
        "a consumer that arrives later got the first frame mid-fade"
    );
    assert!(!output.animates(false, true, false) && !output.animates(true, true, true));
}
