use crate::Manifest;
use std::collections::BTreeSet;

pub fn asks(manifest: &Manifest) -> BTreeSet<String> {
    let inputs = manifest.inputs.iter().map(|i| i.name().to_owned());
    let permissions = manifest.permissions.iter();
    let rgb = permissions.clone().filter(|p| p.rgb).map(|_| "rgb".to_owned());
    let origins = permissions.flat_map(|p| &p.network).flat_map(|n| &n.allow);
    inputs.chain(rgb).chain(origins.map(|o| format!("network {o}"))).collect()
}
