use crate::daemon::Daemon;
use crate::error::{Error, failed};
use crate::streams::HELD;
use std::sync::Arc;
use wprs_core::model::{MODEL, OutputState, State};
use zbus::fdo::DBusProxy;
use zbus::message::Header;

pub(crate) fn rendering(running: bool) -> Result<(), Error> {
    if running { Ok(()) } else { Err(failed("the renderer stopped after an error; restart wprsd")) }
}

pub(crate) fn client(header: &Header<'_>) -> String {
    header.sender().map_or(HELD, |s| s.as_str()).to_owned()
}

async fn gone(bus: Option<&zbus::Connection>, client: &str) -> bool {
    let Some(bus) = bus else { return false };
    let owner = async { DBusProxy::new(bus).await?.name_has_owner(client.try_into()?).await };
    owner.await.is_ok_and(|owned| !owned)
}

impl Daemon {
    pub(crate) async fn run<T: Send + 'static, E: Into<Error>>(
        self: &Arc<Self>,
        work: impl FnOnce(&Arc<Daemon>) -> Result<T, E> + Send + 'static,
    ) -> Result<T, Error> {
        let daemon = self.clone();
        let done = tokio::task::spawn_blocking(move || work(&daemon).map_err(Into::into)).await;
        done.map_err(|e| failed(format!("internal error: {e}")))?
    }

    pub(crate) async fn render<T: Send + 'static, E: Into<Error>>(
        self: &Arc<Self>,
        work: impl FnOnce(&Arc<Daemon>) -> Result<T, E> + Send + 'static,
    ) -> Result<T, Error> {
        rendering(self.compositor.running())?;
        self.run(work).await
    }

    pub(crate) async fn request(
        self: &Arc<Self>,
        output: String,
        size: (u32, u32),
        client: String,
        bus: Option<&zbus::Connection>,
    ) -> Result<(u32, u64), Error> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let (claimant, reply) = (client.clone(), Box::new(|r| drop(tx.send(r))));
        let output = self.render(move |d| d.claim(&output, &claimant, size, reply)).await?;
        let ready = match gone(bus, &client).await {
            true => Err("the client left the bus".into()),
            false => rx.await.unwrap_or_else(|_| Err("PipeWire thread stopped".into())),
        };
        let info = match ready {
            Ok(info) => info,
            Err(e) => {
                self.unclaim(&mut self.state(), &output, &client, 1).ok();
                return Err(failed(e));
            }
        };
        if info.created {
            tracing::info!(output, node = info.node_id, serial = info.serial, "stream ready");
        }
        self.run(move |d| Ok::<_, Error>(d.configure_output(&output).is_some())).await?;
        Ok((info.node_id, info.serial))
    }

    pub fn state_json(&self) -> String {
        let status = self.compositor.status();
        let serial = self.changes.serial();
        let (jobs, away) = (self.jobs.active(), self.away_items());
        let st = self.state();
        let placed = crate::calibration::monitors(&st);
        let output = |(name, info): (&String, &crate::streams::OutputInfo)| {
            let (r, s) = (st.config.resolve(name), status.get(name).cloned().unwrap_or_default());
            let ((width, height), [red, green, blue]) = (info.size, r.background);
            let own = st.config.outputs.get(name).filter(|o| o.sets_content());
            let preset = own.unwrap_or(&st.config.default).preset.clone();
            let item = r.image.as_ref().and_then(|image| st.items.get(image).cloned());
            let credit = r.image.as_deref().and_then(|image| self.credit(image));
            let mut state = OutputState {
                width,
                height,
                image: r.image.clone(),
                fit: r.fit,
                shown_fit: info.shown_fit(r.fit),
                background: format!("#{red:02x}{green:02x}{blue:02x}"),
                kind: s.shown(),
                playing: s.playing,
                active: st.active(info),
                covered: info.covered(),
                clients: info.claims.len() as u32,
                error: s.error.or_else(|| st.held.get(name).cloned()),
                item,
                preset,
                playlist: None,
                credit,
                edited: info.edited(),
                screen: info.screen.clone(),
                mm: placed.iter().find(|m| m.id == *name).map(|m| m.mm),
                follows: st.config.layout.follow.get(name).cloned(),
                parallax: s.parallax,
                still: r
                    .image
                    .as_deref()
                    .filter(|_| !s.parallax)
                    .and_then(|i| st.decks.still(i))
                    .or(s.software_still.then_some(wprs_core::travel::Still::Software)),
                frames: s.frames,
                page_frames: s.page_frames,
                refresh: st.refresh(name),
                fps: s.fps,
                buffers: self.exporter.buffers(name).into(),
                sound: s.sound.into(),
                flashes: s.flashes.into(),
                particles: s.particles.into(),
                quality: s.level.quality.into(),
                quality_reason: s.level.reason.into(),
                quality_choice: st.config.own_quality(name).map(|q| q.as_str().into()),
                quality_cost: f64::from((s.level.cost * 10.0).round() / 10.0),
                quality_budget: f64::from(s.level.budget),
                quality_auto: s.level.automatic.into(),
                quality_timing: s.level.timing(),
                decode: s.decode.into(),
                drafted: st.drafts.contains_key(name),
                left_out: s.left_out,
                page_errors: s.problems,
                blank: st.config.is_blank(name),
                previous: st.config.has_previous(name),
                released: info.idle,
                sandbox: s.sandbox.into(),
                helper: s.helper,
                accent: info.accent.clone(),
                filter: Some(st.config.filter(name)).filter(|f| !f.is_identity()),
                hdr: crate::lights::state(&st.config, name, &s.hdr),
                away: crate::account::away_title(&away, st.config.profile.as_deref(), name),
            };
            crate::rotation::overlay(&st, name, &mut state);
            (name.clone(), state)
        };
        let playback = &st.config.playback;
        let shown = st.outputs.iter().filter(|(name, _)| !crate::drafts::virtual_output(name));
        let state = State {
            model: MODEL,
            version: wprs_core::VERSION.into(),
            adapter: self.adapter.clone(),
            tier: self.tier.0.into(),
            tier_measured: self.tier.1,
            config: st.config_path.clone(),
            config_error: st.config_error.clone(),
            library_error: self.store.error.clone(),
            serial,
            paused: st.user_paused,
            on_battery: st.on_battery,
            battery_percent: st.battery_percent,
            locked: st.locked,
            volume: playback.volume,
            muted: playback.muted,
            profile: st.config.profile.clone(),
            jobs,
            options: serde_json::to_value(st.config.options())
                .and_then(serde_json::from_value)
                .unwrap_or_default(),
            outputs: shown.map(output).collect(),
            previews: crate::drafts::previews(&status, |name| st.drafts.contains_key(name)),
            calibrating: crate::calibration::calibrating(&st),
            compute: self.compute,
            adapters: self.gpus.adapters(),
            gpu_warning: self.gpus.warning.clone(),
            gpu_note: self.gpus.note(),
            gpu_restart: self.gpus.restart(&st.config.render.gpu),
            qualities: st.config.own_qualities(),
            hdrs: st.config.own_hdrs(),
            layer_shell: crate::layers::state(&st.config),
            plasma: crate::plasma::PLASMA.load(std::sync::atomic::Ordering::Relaxed),
        };
        serde_json::to_string(&state).unwrap_or_default()
    }
}
