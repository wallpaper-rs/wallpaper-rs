use zbus::{Result, proxy};

#[proxy(
    interface = "io.github.wallpaper_rs.Inputs1",
    default_service = "io.github.wallpaper_rs.Daemon",
    default_path = "/io/github/wallpaper_rs/Daemon"
)]
pub trait Inputs1 {
    fn grant(&self, scene: &str, input: &str) -> Result<()>;
    fn grant_package(&self, scene: &str, key: &str, input: &str) -> Result<()>;
    fn decide(&self, scene: &str, key: &str, allow: Vec<String>, deny: Vec<String>) -> Result<()>;
    fn revoke(&self, scene: &str, input: &str) -> Result<()>;
    fn get_inputs(&self) -> Result<String>;
    fn set_pointer(&self, x: i32, y: i32) -> Result<()>;
    fn set_clicks(&self, output: &str, on: bool) -> Result<()>;
    fn simulate_media(&self, output: &str, track: &str, cover: &[u8]) -> Result<()>;
    fn simulate_touch(&self, output: &str, x: f64, y: f64, buttons: u32) -> Result<()>;
    fn simulate_trigger(&self, output: &str, trigger: &str) -> Result<()>;
    fn bands(&self, output: &str) -> Result<String>;
}

#[proxy(
    interface = "io.github.wallpaper_rs.Sync1",
    default_service = "io.github.wallpaper_rs.Daemon",
    default_path = "/io/github/wallpaper_rs/Daemon"
)]
pub trait Sync1 {
    fn add_server(&self, url: &str) -> Result<u64>;
    fn add_pinned_server(&self, url: &str, fingerprint: &str) -> Result<u64>;
    fn link(&self, name: &str, plaintext: bool) -> Result<(String, String, u64)>;
    fn connect(&self, name: &str) -> Result<(String, String, u64)>;
    fn set_syncing(&self, on: bool, plaintext: bool) -> Result<u64>;
    fn preview_code(&self, user_code: &str) -> Result<u64>;
    fn approve(&self, user_code: &str) -> Result<u64>;
    fn approve_device(&self, user_code: &str, password: &str) -> Result<u64>;
    fn get_status(&self) -> Result<String>;
    fn sync_now(&self) -> Result<u64>;
    fn set_paused(&self, paused: bool) -> Result<()>;
    fn revoke_device(&self, device: &str, password: &str) -> Result<u64>;
    fn join(&self, device: &str) -> Result<u64>;
    fn share_key(&self, device: &str) -> Result<u64>;
    fn confirm_share(&self, device: &str, code: &str) -> Result<u64>;
    fn rotate(&self) -> Result<u64>;
    fn reset(&self) -> Result<u64>;
    fn confirm_repo(&self, repo: &str, add: bool) -> Result<u64>;
    fn set_escrow(&self, passphrase: &str) -> Result<u64>;
    fn restore(&self, passphrase: &str) -> Result<u64>;
    fn share_files(&self, targets: &[&str], share: bool) -> Result<u64>;
    fn list_files(&self) -> Result<String>;
    fn list_away(&self) -> Result<String>;
    fn fetch_file(&self, file: &str) -> Result<u64>;
    fn upload(&self, path: &str) -> Result<u64>;
    fn upload_as(&self, path: &str, visibility: &str) -> Result<u64>;
    fn list_packages(&self) -> Result<String>;
    fn set_visibility(&self, id: &str, version: &str, visibility: &str) -> Result<()>;
    fn list_author_keys(&self) -> Result<String>;
    fn add_author_key(&self, public_key: &str) -> Result<String>;
    fn rotate_author_key(&self, from: &str, public_key: &str, signature: &str) -> Result<String>;
    fn remove_author_key(&self, fingerprint: &str) -> Result<()>;
    fn list_uploads(&self) -> Result<String>;
    fn cancel_upload(&self, id: &str) -> Result<()>;
    fn vote(&self, id: &str, vote: &str) -> Result<String>;
    fn get_votes(&self, ids: &[&str]) -> Result<String>;
    fn remove_server(&self, wipe: bool) -> Result<()>;
}

#[proxy(
    interface = "io.github.wallpaper_rs.Rgb1",
    default_service = "io.github.wallpaper_rs.Daemon",
    default_path = "/io/github/wallpaper_rs/Daemon"
)]
pub trait Rgb1 {
    fn get_status(&self) -> Result<String>;
    fn connect(&self) -> Result<()>;
    fn test(&self, device: i32) -> Result<()>;
    fn get_strip(&self, output: &str) -> Result<(u32, u32, Vec<u8>)>;
}

#[proxy(
    interface = "io.github.wallpaper_rs.Playlists1",
    default_service = "io.github.wallpaper_rs.Daemon",
    default_path = "/io/github/wallpaper_rs/Daemon"
)]
pub trait Playlists1 {
    fn list(&self) -> Result<String>;
    fn get(&self, playlist: &str, offset: u32, limit: u32) -> Result<String>;
    fn create(&self, name: &str, query: &str) -> Result<String>;
    fn rename(&self, playlist: &str, name: &str) -> Result<()>;
    fn set_query(&self, playlist: &str, query: &str) -> Result<()>;
    fn delete(&self, playlist: &str) -> Result<()>;
    fn add_items(&self, playlist: &str, uids: &[&str]) -> Result<()>;
    fn remove_items(&self, playlist: &str, uids: &[&str]) -> Result<()>;
    fn remove_entries(&self, playlist: &str, indexes: &[u32], expected: &[&str]) -> Result<()>;
    fn move_item(&self, playlist: &str, from: u32, to: u32, expected: &str) -> Result<()>;
    fn set_rule(&self, playlist: &str, key: &str, value: &str) -> Result<()>;
    fn add_folder(&self, playlist: &str, path: &str, follow: bool) -> Result<u32>;
    fn rotate(
        &self,
        name: &str,
        uids: &[&str],
        folder: &str,
        follow: bool,
        every: &str,
        output: &str,
    ) -> Result<String>;
    fn start(&self, playlist: &str, output: &str) -> Result<()>;
    fn stop(&self, output: &str) -> Result<()>;
    fn next(&self, output: &str) -> Result<()>;
    fn previous(&self, output: &str) -> Result<()>;
    fn history(&self, output: &str, limit: u32) -> Result<String>;
    fn browse(&self, output: &str, source: &str, steps: i32) -> Result<String>;
    fn browse_apply(&self, output: &str, transition: &str) -> Result<String>;
    fn browse_transition(&self, output: &str, transition: &str) -> Result<String>;
    fn browse_cancel(&self, output: &str) -> Result<String>;
    fn get_browse(&self, output: &str) -> Result<String>;
}
