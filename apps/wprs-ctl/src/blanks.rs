use wprs_client::Client;
use wprs_core::model::State;

pub async fn note(client: &Client) {
    let state = client.daemon.get_state().await.ok();
    let plasma = state.and_then(|s| serde_json::from_str::<State>(&s).ok()).is_some_and(|s| s.plasma);
    if plasma {
        println!(
            "Plasma now shows the background color; plasma-back gives the screens back to Plasma's own wallpaper"
        );
    }
}

pub fn handed(screens: u32) -> String {
    match screens {
        0 => "no screen was using the wallpaper-rs wallpaper type".into(),
        1 => {
            "1 screen is back on Plasma's own wallpaper; restore or set a wallpaper to use wallpaper-rs again"
                .into()
        }
        n => format!(
            "{n} screens are back on Plasma's own wallpaper; restore or set a wallpaper to use wallpaper-rs again"
        ),
    }
}
