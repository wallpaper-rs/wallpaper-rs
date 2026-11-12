use crate::daemon::Daemon;

impl Daemon {
    pub fn forget_data(&self, removed: &[wprs_library::Item]) {
        let kept = |id: &str| self.store.read(|lib| lib.copies(id)).is_ok_and(|n| n > 0);
        for package in removed.iter().filter_map(|i| i.package.as_ref()).filter(|p| !kept(&p.id)) {
            if let Err(e) = wprs_core::stash::forget(&self.store.data.join("storage"), &package.id) {
                tracing::warn!("cannot remove what the scene scripts stored: {e}");
            }
            if package.kind == "web"
                && let Err(e) = self.clear_page_data(&package.id)
            {
                tracing::warn!("cannot remove the data of a web page: {e}");
            }
        }
    }
}
