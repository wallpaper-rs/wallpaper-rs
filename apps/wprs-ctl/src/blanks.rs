use crate::told::{fill, told, tr};
use wprs_client::Client;
use wprs_core::model::State;

pub async fn note(client: &Client) {
    let state = client.daemon.get_state().await.ok();
    let plasma = state.and_then(|s| serde_json::from_str::<State>(&s).ok()).is_some_and(|s| s.plasma);
    if plasma {
        let line = told(
            "Plasma now shows the background color; plasma-back gives the screens back to Plasma's own wallpaper",
        );
        println!("{}", tr(line));
    }
}

pub fn handed(screens: u32) -> String {
    match screens {
        0 => tr(told("no screen was using the wallpaper-rs wallpaper type")),
        1 => tr(told(
            "1 screen is back on Plasma's own wallpaper; restore or set a wallpaper to use wallpaper-rs again",
        )),
        n => fill(
            told(
                "%1 screens are back on Plasma's own wallpaper; restore or set a wallpaper to use wallpaper-rs again",
            ),
            &[&n],
        ),
    }
}
