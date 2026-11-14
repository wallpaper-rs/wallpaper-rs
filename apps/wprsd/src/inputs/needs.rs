use super::{SceneInputs, hub};
use crate::compositor::lock;
use std::collections::BTreeSet;

pub(crate) fn needs(
    output: &str,
    (scene, title, key): (&str, &str, &str),
    [audio, pointer, clicks, network, rgb, media]: [bool; 6],
) {
    let Some(hub) = hub() else { return };
    let (scene, title, key, granted) = (scene.into(), title.into(), key.into(), BTreeSet::new());
    let (without, undecided) = (BTreeSet::new(), BTreeSet::new());
    let need = SceneInputs {
        scene,
        audio,
        pointer,
        clicks,
        network,
        rgb,
        media,
        title,
        key,
        granted,
        without,
        undecided,
    };
    let mut control = lock(&hub.control);
    let changed = match audio || pointer || clicks || network || rgb || media {
        true => control.shown.insert(output.into(), need.clone()) != Some(need),
        false => control.shown.remove(output).is_some(),
    };
    drop(control);
    if changed {
        hub.poke.notify_one();
        (hub.changed)();
    }
}
