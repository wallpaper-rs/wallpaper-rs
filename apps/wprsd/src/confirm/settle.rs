use super::{ASKED, QUIET, ask};
use crate::compositor::lock;
use crate::inputs::Hub;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use wprs_core::model::Input;

pub(super) async fn ask_group(
    bus: zbus::Connection,
    hub: Arc<Hub>,
    path: PathBuf,
    (scene, key, title): (String, String, String),
    group: Vec<Input>,
) {
    let answer = ask(&bus, crate::words::request_group(&title, &group)).await;
    let answer = answer.inspect_err(|e| tracing::warn!("cannot ask the user: {e}"));
    let quiet = matches!(answer, Ok(None | Some(false)));
    if let Ok(Some(allowed)) = answer {
        let (hub, path, scene, key, group) =
            (hub.clone(), path.clone(), scene.clone(), key.clone(), group.clone());
        let set = tokio::task::spawn_blocking(move || {
            for input in group {
                let done = match allowed {
                    true => hub.grant(&path, &scene, &key, input),
                    false => hub.decline(&path, &scene, &key, input),
                };
                if let Err(e) = done {
                    tracing::warn!("cannot keep the answer of the user: {e:#}");
                }
            }
        });
        let _ = set.await;
    }
    let until = quiet.then(|| Instant::now() + QUIET);
    {
        let mut all = lock(&ASKED);
        for input in &group {
            let asked = (scene.clone(), *input);
            match until {
                Some(until) => drop(all.insert(asked, Some(until))),
                None => drop(all.remove(&asked)),
            }
        }
    }
    hub.changed();
    let Some(until) = until else { return };
    tokio::time::sleep_until(until.into()).await;
    let mut all = lock(&ASKED);
    let mut freed = false;
    for input in &group {
        let asked = (scene.clone(), *input);
        if all.get(&asked) == Some(&Some(until)) {
            all.remove(&asked);
            freed = true;
        }
    }
    drop(all);
    if freed {
        hub.changed();
    }
}
