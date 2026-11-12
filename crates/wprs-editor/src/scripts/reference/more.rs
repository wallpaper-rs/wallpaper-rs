use super::Part;
use wprs_core::message::said;

pub(super) fn parts() -> Vec<Part> {
    vec![
        (
            "tools",
            vec![
                (
                    "setTimeout(fn, ms), setInterval(fn, ms)",
                    said(
                        "editor.api.timers",
                        "Timers on the scene clock; they return a function that cancels them (declare timers)",
                        &[],
                    ),
                ),
                (
                    "console.log(...)",
                    said(
                        "editor.api.console",
                        "Writes to the console of the preview; at most 20 lines a second",
                        &[],
                    ),
                ),
                (
                    "storage.get(key), set(key, value)",
                    said(
                        "editor.api.storage",
                        "What the scripts keep between runs; delete, clear and keys too",
                        &[],
                    ),
                ),
                (
                    "shared",
                    said(
                        "editor.api.shared",
                        "An object the modules of the scene share; values are copied when set",
                        &[],
                    ),
                ),
                (
                    "particles(layer)",
                    said(
                        "editor.api.particles",
                        "play, pause, stop, emit(n), point, from and wall for the particles of a layer",
                        &[],
                    ),
                ),
                (
                    "sound(id)",
                    said("editor.api.sound", "play, pause, stop, playing and volume of a sound layer", &[]),
                ),
                ("folder(prop)", said("editor.api.folder", "The pictures of a folder the user chose", &[])),
                (
                    "wprs/math",
                    said(
                        "editor.api.math",
                        "Vec2 and helpers such as smoothstep, to import from wprs/math",
                        &[],
                    ),
                ),
            ],
        ),
        (
            "events",
            vec![
                (
                    "init(), destroy(), resize()",
                    said(
                        "editor.api.lifecycle",
                        "Called when the module starts, stops and when the canvas changes size",
                        &[],
                    ),
                ),
                (
                    "update()",
                    said(
                        "editor.api.update",
                        "Called every frame; what it returns is the value of a value bound to the module without fn",
                        &[],
                    ),
                ),
                (
                    "properties(changed), settings(s)",
                    said(
                        "editor.api.settings",
                        "Called with the user settings, and with the language, fps and quality",
                        &[],
                    ),
                ),
                (
                    "timelineEvent(e)",
                    said("editor.api.timeline_event", "Called for each event of a timeline or clip", &[]),
                ),
                (
                    "mediaStatus, mediaPlayback, mediaProperties, mediaThumbnail, mediaTimeline",
                    said(
                        "editor.api.media_events",
                        "Called as the media player changes (declare media)",
                        &[],
                    ),
                ),
                (
                    "layer.on(\"click\", handler)",
                    said(
                        "editor.api.pointer_events",
                        "The click, down, up, enter, leave and move of a solid layer or a hit bone",
                        &[],
                    ),
                ),
            ],
        ),
        (
            "values",
            vec![
                (
                    "export const name = (value, ctx) => ...",
                    said(
                        "editor.api.function",
                        "A function a value follows by name: ctx has path, layer, key and params",
                        &[],
                    ),
                ),
                (
                    "export const name = { init, update }",
                    said(
                        "editor.api.function_pair",
                        "The same with a first call and a call every frame",
                        &[],
                    ),
                ),
            ],
        ),
    ]
}
