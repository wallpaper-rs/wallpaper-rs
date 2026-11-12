use super::PageSlot;
use super::view::Snap;
use std::time::{Duration, Instant};
use wprs_web::ToWeb;

const RETRY: Duration = Duration::from_secs(3);

pub enum Taken {
    Missing,
    Asked,
    Ready((u32, u32), Vec<u8>),
}

pub(in crate::compositor) fn take<'a>(
    pages: impl Iterator<Item = &'a mut PageSlot>,
    output: &str,
    now: Instant,
) -> Taken {
    for slot in pages {
        let Some(id) = slot.views.get(output).map(|v| v.id) else { continue };
        let view = slot.views.get_mut(output).expect("found above");
        return match view.snap.take() {
            Some(Snap::Ready(size, rgba)) => Taken::Ready(size, rgba),
            Some(Snap::Asked(at)) if now.duration_since(at) < RETRY => {
                view.snap = Some(Snap::Asked(at));
                Taken::Asked
            }
            _ => {
                view.snap = Some(Snap::Asked(now));
                slot.send(&ToWeb::Shot { view: id });
                Taken::Asked
            }
        };
    }
    Taken::Missing
}
