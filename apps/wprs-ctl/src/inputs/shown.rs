use crate::show::list;
use crate::told::{fill, told};
use std::collections::BTreeSet;
use wprs_core::model::{Input, InputsReply};

pub(super) fn status(reply: InputsReply) -> String {
    let on = |on| if on { "on" } else { "off" };
    let mut lines = vec![
        fill(told("audio capture: %1"), &[&on(reply.capturing)]),
        format!("pointer: {}", on(reply.tracking)),
    ];
    let inputs = |set: &BTreeSet<Input>| match set.is_empty() {
        true => "none".to_owned(),
        false => list(set.iter().map(|i| i.as_str())),
    };
    let granted = |(scene, set): (&String, &BTreeSet<Input>)| {
        fill(told("granted to some version or signer of %1: %2"), &[&scene, &inputs(set)])
    };
    lines.extend(reply.grants.iter().map(granted));
    let declined = |(scene, set): (&String, &BTreeSet<Input>)| {
        fill(told("declined for %1: %2"), &[&scene, &inputs(set)])
    };
    lines.extend(reply.declined.iter().map(declined));
    let asked = |(scene, set): (&String, &BTreeSet<Input>)| {
        fill(told("asked in a desktop notification for %1: %2"), &[&scene, &inputs(set)])
    };
    lines.extend(reply.asking.iter().map(asked));
    let quiet = |(scene, set): (&String, &BTreeSet<Input>)| {
        fill(told("not asked again for a few minutes for %1: %2"), &[&scene, &inputs(set)])
    };
    lines.extend(reply.quiet.iter().map(quiet));
    lines.extend(reply.shown.iter().map(|(output, shown)| {
        let used =
            [(shown.audio, Input::Audio), (shown.pointer, Input::Pointer), (shown.clicks, Input::Clicks)];
        let more = [(shown.network, Input::Network), (shown.rgb, Input::Rgb), (shown.media, Input::Media)];
        let both = [(shown.media && shown.network, Input::MediaNetwork)];
        let both = both.into_iter().chain([(shown.audio && shown.network, Input::AudioNetwork)]);
        let more = more.into_iter().chain(both);
        let used = used.into_iter().chain(more);
        let used: BTreeSet<Input> = used.filter_map(|(on, input)| on.then_some(input)).collect();
        fill(
            told("%1 shows %2 using %3 (allowed: %4)"),
            &[&output, &shown.scene, &inputs(&used), &inputs(&shown.granted)],
        )
    }));
    let follows =
        |output: &String| fill(told("%1 shows an image whose parallax follows the pointer"), &[&output]);
    lines.extend(reply.parallax.iter().map(follows));
    let screens = |set: &BTreeSet<String>| if set.is_empty() { "none".to_owned() } else { list(set.iter()) };
    lines.push(fill(told("clicks reach the wallpaper on: %1"), &[&screens(&reply.clicks)]));
    lines.push(fill(told("taking clicks now: %1"), &[&screens(&reply.clicking)]));
    if !reply.unreached.is_empty() {
        lines
            .push(fill(told("clicks cannot reach the wallpaper here on: %1"), &[&screens(&reply.unreached)]));
    }
    lines.join("\n") + "\n"
}

pub(super) const AUDIO: &str =
    told("the sound that plays (turned into levels on this computer, never recorded or sent)");

pub(super) const POINTER: &str = told("the pointer (only its latest position over the screen, never kept)");

pub(super) const CLICKS: &str =
    told("clicks and taps on the screen (only real ones, never kept; keys are never read)");

pub(super) const SLOW: &str = told("its particles run on the processor, so fewer of them are shown");

pub(super) const NETWORK: &str =
    told("the network, to reach the sites it names (they can learn that it runs on this computer)");

pub(super) const MEDIA: &str = told(
    "what the media player plays (title, artist, album, cover and progress, never kept, logged or sent)",
);

pub(super) const RGB: &str = told(
    "the RGB lighting, to color it through the OpenRGB server set with rgb.server (only while rgb.enabled is on)",
);
pub(super) const MEDIA_NETWORK: &str = told(
    "what plays even though it can reach the network, where the title, artist and cover could leave this computer (a second confirmation besides media)",
);
pub(super) const AUDIO_NETWORK: &str = told(
    "the sound that plays even though it can reach the network, where what it hears (the sound of the computer or the microphone) could leave this computer (a second confirmation besides audio)",
);
