use crate::daemon::Daemon;
use futures_util::StreamExt;
use std::sync::Arc;

#[zbus::proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower/devices/DisplayDevice"
)]
trait Display {
    #[zbus(property)]
    fn percentage(&self) -> zbus::Result<f64>;
    #[zbus(property)]
    fn is_present(&self) -> zbus::Result<bool>;
}

pub(crate) fn level(present: bool, percentage: f64) -> Option<u8> {
    (present && percentage.is_finite()).then(|| percentage.round().clamp(0.0, 100.0) as u8)
}

pub(crate) async fn follow(daemon: Arc<Daemon>) {
    let follow = async {
        let display = DisplayProxy::new(&zbus::Connection::system().await?).await?;
        let (mut levels, mut presence) =
            (display.receive_percentage_changed().await, display.receive_is_present_changed().await);
        loop {
            let (present, percentage) = (display.is_present().await?, display.percentage().await?);
            daemon.set_battery_percent(level(present, percentage));
            tokio::select! {
                Some(_) = levels.next() => {}
                Some(_) = presence.next() => {}
                else => break,
            }
        }
        zbus::Result::Ok(())
    };
    if let Err(e) = follow.await {
        tracing::info!("battery level unknown, it is reported as none: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::level;
    use crate::daemon::State;

    fn state(on_battery: bool, percent: Option<u8>, pause: u8, lowest: u8) -> State {
        let mut st = State { on_battery, battery_percent: percent, ..State::default() };
        st.config.pause.on_battery = false;
        (st.config.pause.below_percent, st.config.playback.lowest_below_percent) = (pause, lowest);
        st
    }

    #[test]
    fn a_chosen_battery_level_pauses_or_lowers_the_quality_only_on_battery_and_when_known() {
        assert!(state(true, Some(30), 20, 0).awake());
        assert!(!state(true, Some(19), 20, 0).awake());
        assert!(state(false, Some(5), 20, 0).awake(), "plugged in and paused");
        assert!(state(true, None, 20, 0).awake(), "an unknown level paused");
        assert!(state(true, Some(5), 0, 0).awake() && !state(true, Some(5), 0, 0).lowest());
        assert!(state(true, Some(19), 0, 20).lowest() && state(true, Some(19), 0, 20).awake());
        assert!(!state(true, Some(20), 0, 20).lowest() && !state(false, Some(1), 0, 20).lowest());
    }

    #[test]
    fn the_level_is_none_without_a_battery_and_rounded_with_one() {
        assert_eq!(level(false, 0.0), None);
        assert_eq!(level(false, 87.0), None);
        assert_eq!(level(true, f64::NAN), None);
        assert_eq!(level(true, 19.6), Some(20));
        assert_eq!(level(true, 140.0), Some(100));
    }
}
