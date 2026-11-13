use super::build::build;
use super::kinds::{Kind, kind, secret_of};
use super::saved::{self, Credit, Entry};
use super::tidy::{evict, tag};
use crate::state::CANCEL;
use crate::{Error, library, said, store};
use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::Ordering::Relaxed;
use wprs_providers::{Cache, Item, Media, Provider};

const BUDGET: u64 = 256 << 20;
const FOLDER: &str = "providers-cache";
const PUBLIC: [&str; 4] = ["bing", "apod", "wikimedia", "wallhaven"];

static RUNNING: Mutex<()> = Mutex::new(());

fn rejected(e: &wprs_providers::Error) -> bool {
    matches!(e, wprs_providers::Error::Http(e) if matches!(e.status().map(|s| s.as_u16()), Some(401 | 403)))
}

fn failure(id: &str, e: &wprs_providers::Error, keyed: bool) -> Error {
    match e {
        e if keyed && rejected(e) => said(
            "android.provider_secret_rejected",
            "%1 rejected the key or password: enter it again",
            &[&id],
        ),
        wprs_providers::Error::Http(e) if wprs_sync::untrusted(e) => said(
            "android.provider_untrusted",
            "%1 has a certificate this phone does not trust: if it is your own server, install its certificate authority in the phone's settings",
            &[&id],
        ),
        e => said("android.provider_failed", "%1 did not answer: %2", &[&id, e]),
    }
}

struct Fetched {
    listed: u32,
    failed: u32,
    files: Vec<(PathBuf, Item)>,
}

async fn fetch_all(
    provider: &dyn Provider,
    cache: &Cache,
    entry: &Entry,
    keep: usize,
) -> wprs_providers::Result<Fetched> {
    let items = provider.list().await?;
    let mut out = Fetched { listed: items.len() as u32, failed: 0, files: Vec::new() };
    let fresh = items.into_iter().filter(|i| i.media == Media::Image && !entry.seen.contains(&i.id));
    let mut left = BUDGET;
    for item in fresh.take(keep) {
        if CANCEL.load(Relaxed) {
            break;
        }
        cache.allow(left);
        match provider.fetch(&item, cache).await {
            Ok(path) => {
                left = left.saturating_sub(fs::metadata(&path).map_or(0, |m| m.len()));
                out.files.push((path, item));
            }
            Err(e) if rejected(&e) => return Err(e),
            Err(wprs_providers::Error::TooLarge) => break,
            Err(_) => out.failed += 1,
        }
    }
    Ok(out)
}

fn bring(entry: &mut Entry, fetched: Fetched) -> Result<u32, Error> {
    let mut brought = 0;
    let keyed = PUBLIC.contains(&entry.kind.as_str());
    for (path, item) in fetched.files {
        entry.remember(item.id.clone());
        let name = format!(
            "{}.{}",
            item.title.replace(['/', '\\'], " "),
            path.extension().and_then(|e| e.to_str()).unwrap_or("jpg")
        );
        let stored =
            fs::File::open(&path).map_err(Error::from).and_then(|mut file| store::store(&mut file, &name));
        let Ok((uid, _)) = stored else { continue };
        library::with(|lib| {
            let title = item.title.chars().filter(|c| !c.is_control()).take(256).collect::<String>();
            if !title.trim().is_empty() {
                lib.set_title(&uid, title.trim())?;
            }
            Ok(lib.add_tag(&uid, &tag(&entry.id))?)
        })?;
        let link = keyed.then(|| item.url.clone());
        entry.credits.retain(|c| c.uid != uid);
        entry.credits.push(Credit {
            uid,
            title: item.title,
            attribution: item.attribution,
            license: item.license,
            link,
        });
        brought += 1;
    }
    let present =
        library::with(|lib| Ok(entry.credits.iter().map(|c| lib.item(&c.uid).is_ok()).collect::<Vec<_>>()))?;
    let mut flags = present.into_iter();
    entry.credits.retain(|_| flags.next().unwrap_or(false));
    store::changed();
    Ok(brought)
}

fn one(id: &str) -> Result<Value, Error> {
    let entry = saved::load().into_iter().find(|e| e.id == id);
    let entry =
        entry.ok_or_else(|| said("android.provider_missing", "there is no picture source %1", &[&id]))?;
    if !entry.on {
        return Err(said("android.provider_off", "%1 is off: turn it on to let it download", &[&id]));
    }
    let kind: &Kind = kind(&entry.kind)?;
    let key = saved::secret(id);
    if secret_of(kind).is_some_and(|(_, required)| required) && key.is_none() {
        return Err(said("android.provider_secret", "%1 needs its key or password first", &[&id]));
    }
    let provider = build(kind, &entry.options, key.as_deref())?;
    let keep: usize = entry.options.get("keep").and_then(|k| k.parse().ok()).unwrap_or(30);
    let dir = store::dir()?.join(FOLDER).join(id);
    let cache = Cache::new(&dir, BUDGET << 1)
        .map_err(|e| said("android.provider_failed", "%1 did not answer: %2", &[&id, &e]))?;
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    let fetched = runtime.block_on(fetch_all(provider.as_ref(), &cache, &entry, keep));
    let fetched = fetched.map_err(|e| failure(id, &e, secret_of(kind).is_some()));
    let now = library::now();
    let outcome = fetched.and_then(|fetched| {
        let (listed, failed) = (fetched.listed, fetched.failed);
        let mut entry = entry.clone();
        let brought = bring(&mut entry, fetched)?;
        evict(id, keep, now)?;
        Ok((entry, json!({"provider": id, "listed": listed, "brought": brought, "failed": failed})))
    });
    drop(fs::remove_dir_all(&dir));
    let _held = saved::held();
    let mut all = saved::load();
    let slot = saved::find(&mut all, id)?;
    match outcome {
        Ok((fresh, report)) => {
            (slot.seen, slot.credits, slot.last_sync, slot.last_error) =
                (fresh.seen, fresh.credits, Some(now), None);
            saved::save(&all)?;
            Ok(report)
        }
        Err(e) => {
            slot.last_error = Some(e.reply());
            saved::save(&all)?;
            Err(e)
        }
    }
}

fn due(entry: &Entry, now: i64) -> bool {
    let wait = entry
        .options
        .get("refresh")
        .and_then(|r| wprs_schedule::parse_interval(r))
        .map_or(86_400, |d| d.as_secs() as i64);
    entry.on && entry.last_sync.is_none_or(|at| now - at >= wait || at > now)
}

pub(super) fn sync(id: &str, force: bool) -> Result<Value, Error> {
    let _run = RUNNING.try_lock().map_err(|_| {
        said("android.provider_busy", "another picture source is downloading: try again when it ends", &[])
    })?;
    CANCEL.store(false, Relaxed);
    let now = library::now();
    let ids: Vec<String> = match id {
        "" => saved::load().into_iter().filter(|e| e.on && (force || due(e, now))).map(|e| e.id).collect(),
        id => vec![id.to_owned()],
    };
    let mut reports = Vec::new();
    for found in ids {
        match one(&found) {
            Ok(report) => reports.push(report),
            Err(e) if id.is_empty() => log::warn!("a picture source failed: {e}"),
            Err(e) => return Err(e),
        }
    }
    Ok(json!(reports))
}
