use crate::daemon::Daemon;
use crate::service::Service;
use std::sync::Arc;
use wprs_core::dbus;
use zbus::object_server::SignalEmitter;

pub(crate) async fn announce_streams(connection: zbus::Connection, daemon: Arc<Daemon>) {
    let Ok(emitter) = SignalEmitter::new(&connection, dbus::OBJECT_PATH) else { return };
    let (mut seen, mut waiting) = (daemon.exporter.generation(), Vec::new());
    loop {
        daemon.streams_changed.notified().await;
        let generation = daemon.exporter.generation();
        if generation != seen {
            (seen, waiting) =
                (generation, daemon.state().outputs.keys().map(|o| (o.clone(), None)).collect());
        }
        waiting
            .extend(std::mem::take(&mut daemon.state().resized).into_iter().map(|(o, size)| (o, Some(size))));
        waiting.extend(daemon.exporter.recreated().into_iter().map(|o| (o, None)));
        for (output, size) in std::mem::take(&mut waiting) {
            let info = daemon.exporter.stream_info(&output);
            match info.filter(|i| size.is_none_or(|s| (i.width, i.height) == s)) {
                Some(info) => {
                    tracing::info!(output, node = info.node_id, serial = info.serial, "stream recreated");
                    let sent = Service::stream_changed(&emitter, &output, info.node_id, info.serial).await;
                    sent.unwrap_or_else(|e| tracing::warn!("cannot announce a recreated stream: {e}"));
                }
                None if daemon.state().outputs.contains_key(&output) => waiting.push((output, size)),
                None => {}
            }
        }
    }
}
