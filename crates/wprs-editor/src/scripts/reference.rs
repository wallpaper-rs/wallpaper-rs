use serde_json::{Value, json};
use wprs_core::message::{Message, said};

mod more;

pub(super) type Part = (&'static str, Vec<(&'static str, Message)>);

fn parts() -> Vec<Part> {
    vec![
        (
            "scene",
            vec![
                (
                    "scene.layer(id)",
                    said(
                        "editor.api.layer",
                        "A layer by id: position, scale, rotation, opacity, visible, color and text can be read and written; on(event, handler) handles the pointer",
                        &[],
                    ),
                ),
                (
                    "scene.layers()",
                    said("editor.api.layers", "The ids of the top-level layers in drawing order", &[]),
                ),
                (
                    "scene.spawn(template, options)",
                    said(
                        "editor.api.spawn",
                        "Draws a copy of a template of the scene and gives its handle (declare layers)",
                        &[],
                    ),
                ),
                ("scene.destroy(id)", said("editor.api.destroy", "Removes a copy made with spawn", &[])),
                (
                    "scene.order(id, index)",
                    said("editor.api.order", "Moves a layer to a place in the drawing order", &[]),
                ),
                (
                    "layer.effect(name)",
                    said(
                        "editor.api.effect",
                        "An effect of the layer by its name or position: enabled, set(param, value), get and reset (declare effects)",
                        &[],
                    ),
                ),
            ],
        ),
        (
            "inputs",
            vec![
                (
                    "engine",
                    said(
                        "editor.api.engine",
                        "time, dt, daytime (0 to 1), fps, quality, locale, language, canvas, screen, device and mode",
                        &[],
                    ),
                ),
                (
                    "props.get(id)",
                    said(
                        "editor.api.props",
                        "The value of a user setting; color(id), label(id), pressed(id) and all() read the rest",
                        &[],
                    ),
                ),
                (
                    "audio.bass, mid, treble, level, beat",
                    said(
                        "editor.api.audio",
                        "The sound levels, from 0 to 1; bands(16, 32 or 64) and buffers(n) give the spectrum",
                        &[],
                    ),
                ),
                (
                    "input.cursor",
                    said(
                        "editor.api.cursor",
                        "x and y as fractions of the screen, inside, buttons and left (the last two need clicks)",
                        &[],
                    ),
                ),
                (
                    "media",
                    said(
                        "editor.api.media",
                        "What the media player plays: state, title, artist, album, position, duration, colors and has_cover",
                        &[],
                    ),
                ),
                (
                    "timeline(id)",
                    said(
                        "editor.api.timeline",
                        "play, pause, reverse, stop, seek(t), speed, time, duration and on(name, handler)",
                        &[],
                    ),
                ),
            ],
        ),
    ]
}

pub fn reference() -> Value {
    let parts = parts().into_iter().chain(more::parts()).map(|(part, items)| {
        let items: Vec<Value> =
            items.into_iter().map(|(name, text)| json!({"name": name, "text": text.local()})).collect();
        json!({"part": part, "items": items})
    });
    Value::Array(parts.collect())
}
