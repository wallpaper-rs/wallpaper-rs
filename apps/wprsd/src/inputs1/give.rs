use crate::callers::caller;
use crate::compositor::lock;
use crate::error::{Result, denied};
use crate::inputs::grants_path;
use wprs_core::model::Input;
use zbus::message::Header;

impl super::Inputs {
    pub(super) async fn give(
        &self,
        call: (&zbus::Connection, &Header<'_>),
        scene: &str,
        key: Option<&str>,
        input: &str,
    ) -> Result<()> {
        let (d, hub) = self.parts()?;
        let (input, scene) = (super::parse(input)?, super::scene_id(scene)?.to_owned());
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
            super::reopen(d, input)
        })
        .await
    }
}
