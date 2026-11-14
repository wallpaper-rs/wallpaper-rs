use wprs_core::model::PackageInfo;

pub(super) fn shape(p: &mut PackageInfo, exposes: &[String], flashing_off: bool) {
    (p.exposes, p.unshown) = (exposes.to_vec(), unshown(&p.kind));
    p.flash_disabled = flashing_off && p.photosensitive == Some(true);
}

fn unshown(kind: &str) -> Option<String> {
    (kind == "web").then(crate::pages::unavailable).flatten()
}
