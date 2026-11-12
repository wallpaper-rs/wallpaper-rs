use crate::canvas::gizmos;
use crate::project::Project;
use serde_json::{Value, json};

pub fn followed(project: &Project, time: f64) -> Vec<Value> {
    if project.scope.is_none() {
        return Vec::new();
    }
    gizmos(&project.dir, &project.doc, time)
        .into_iter()
        .filter(|g| g.kind == "mesh")
        .map(|g| json!(g))
        .collect()
}
