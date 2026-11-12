use crate::rigging::Rigging;
use crate::told::{fill, told};
use anyhow::Result;
use wprs_editor::Project;

pub(crate) fn print(
    dir: &std::path::Path,
    layer: &str,
    read: impl Fn(&Project) -> serde_json::Value,
) -> Result<()> {
    let project = Project::open(dir)?;
    let found = read(&project);
    anyhow::ensure!(
        !found.is_null(),
        "{}",
        fill(told("there is no mesh layer %1 with that"), &[&format!("{layer:?}")])
    );
    println!("{}", serde_json::to_string_pretty(&found)?);
    Ok(())
}

pub(crate) fn run(command: Rigging) -> Result<()> {
    match command {
        Rigging::Bones { dir, layer } => {
            print(&dir, &layer, |p| wprs_editor::rigs::rig(&p.dir, &p.seen(), &layer))
        }
        Rigging::PartOutlines { dir, layer } => {
            print(&dir, &layer, |p| wprs_editor::rigs::outline(&p.dir, &p.seen(), &layer))
        }
        Rigging::VertexWeights { dir, layer } => {
            print(&dir, &layer, |p| wprs_editor::rigs::vertices(&p.dir, &p.seen(), &layer))
        }
        _ => Ok(()),
    }
}
