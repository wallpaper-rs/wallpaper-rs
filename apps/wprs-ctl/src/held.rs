use wprs_core::model::InstalledPackage;

pub fn asked(packages: &[InstalledPackage]) -> String {
    let held = packages.iter().filter(|p| p.update.is_some() && !p.asks.is_empty());
    held.map(|p| {
        let new = p.update.as_deref().unwrap_or_default();
        let asks = p.asks.join(", ");
        format!(
            "\n{} {new} asks for new permissions ({asks}) and is not updated automatically; run `pkg install {}/{}` to allow them\n",
            p.id, p.repo_id, p.id
        )
    })
    .collect()
}
