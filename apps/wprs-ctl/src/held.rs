use crate::told::{fill, told};
use wprs_core::model::InstalledPackage;

pub fn asked(packages: &[InstalledPackage]) -> String {
    let held = packages.iter().filter(|p| p.update.is_some() && !p.asks.is_empty());
    held.map(|p| {
        let new = p.update.as_deref().unwrap_or_default();
        let asks = p.asks.join(", ");
        fill(told("\n%1 %2 asks for new permissions (%3) and is not updated automatically; run `pkg install %4/%5` to allow them\n"), &[&p.id, &new, &asks, &p.repo_id, &p.id])
    })
    .collect()
}
