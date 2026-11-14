use crate::callers::caller;
use crate::compositor::lock;
use crate::daemon::Daemon;
use crate::error::{Result, denied};
use crate::inputs::{declined_path, grants_path, load, load_bound};
use std::sync::Arc;
use wprs_core::model::Input;
use zbus::message::Header;

pub(crate) fn start(daemon: &Arc<Daemon>, connection: zbus::Connection) {
    let path = grants_path(daemon);
    crate::inputs::start(daemon, connection, (load_bound(&path), load(&declined_path(&path))));
}

impl super::Inputs {
    pub(super) async fn give(
        &self,
        call: (&zbus::Connection, &Header<'_>),
        scene: &str,
        key: Option<&str>,
        input: &str,
    ) -> Result<()> {
        let (d, hub) = self.parts()?;
        let (input, scene) = (super::ids::parse(input)?, super::ids::scene_id(scene)?.to_owned());
        let (key, title) = lock(&hub.control).package(&scene, key)?;
        let trusted = caller(call.0, call.1).await.trusted();
        if matches!(input, Input::Network | Input::Rgb | Input::MediaNetwork | Input::AudioNetwork)
            && !trusted
        {
            return Err(denied("only the app, the tray and wprs-ctl give wallpapers the network or lights"));
        }
        if !trusted {
            if lock(&hub.control).has(&scene, &key, input) {
                return Ok(());
            }
            return crate::confirm::grant(call.0, hub.clone(), grants_path(&d), (scene, key, title), input)
                .await;
        }
        d.run(move |d| {
            hub.grant(&grants_path(d), &scene, &key, input)?;
            super::ids::reopen(d, input)
        })
        .await
    }

    pub(super) async fn decide_all(
        &self,
        call: (&zbus::Connection, &Header<'_>),
        scene: &str,
        key: &str,
        (allow, deny): (Vec<String>, Vec<String>),
    ) -> Result<()> {
        let (d, hub) = self.parts()?;
        if !caller(call.0, call.1).await.trusted() {
            return Err(denied("only the app, the tray and wprs-ctl answer for a wallpaper"));
        }
        let scene = super::ids::scene_id(scene)?.to_owned();
        let answers: Result<Vec<_>> = allow
            .iter()
            .map(|i| Ok((super::ids::parse(i)?, true)))
            .chain(deny.iter().map(|i| Ok((super::ids::parse(i)?, false))))
            .collect();
        let answers = answers?;
        let clash = |a: &(Input, bool)| answers.iter().any(|b| b.0 == a.0 && b.1 != a.1);
        if answers.iter().any(clash) {
            return Err(crate::error::invalid("an input cannot be both allowed and left out"));
        }
        let key = lock(&hub.control).package(&scene, (!key.is_empty()).then_some(key))?.0;
        d.run(move |d| {
            for (input, on) in answers {
                match on {
                    true => hub.grant(&grants_path(d), &scene, &key, input)?,
                    false => hub.decline(&grants_path(d), &scene, &key, input)?,
                }
                super::ids::reopen(d, input)?;
            }
            Ok::<(), crate::error::Error>(())
        })
        .await
    }
}
