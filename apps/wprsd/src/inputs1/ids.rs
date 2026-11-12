use crate::daemon::Daemon;
use crate::error::{Result, invalid};
use wprs_core::model::Input;

pub(crate) fn scene_id(scene: &str) -> Result<&str> {
    let valid = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || "._-".contains(c);
    let fine = (1..=255).contains(&scene.len()) && scene.chars().all(valid);
    fine.then_some(scene).ok_or_else(|| invalid(format!("{scene} is not a scene package id")))
}

pub(super) fn parse(input: &str) -> Result<Input> {
    Input::parse(input).ok_or_else(|| {
        invalid(format!(
            "{input} is not audio, pointer, clicks, network, rgb, media, media_network or audio_network"
        ))
    })
}

pub(super) fn reopen(d: &Daemon, input: Input) -> anyhow::Result<()> {
    if input == Input::Network {
        d.configure_all();
    }
    Ok(())
}
