use crate::show::table;
use anyhow::Result;
use std::path::Path;
use wprs_editor::Project;

pub fn run(dir: &Path, category: Option<&str>, json: bool) -> Result<()> {
    let project = Project::open(dir)?;
    let mut rows = wprs_editor::outline::catalog(&project);
    rows.retain(|r| category.is_none_or(|c| r["category"] == c));
    rows.sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));
    if json {
        rows.iter().for_each(|r| println!("{r}"));
        return Ok(());
    }
    let cells = rows.iter().map(|r| {
        vec![
            r["id"].as_str().unwrap_or_default().to_owned(),
            r["category"].as_str().unwrap_or("-").to_owned(),
            r["units"].as_f64().map_or_else(String::new, |u| format!("{u:.1}")),
            r["description"].as_str().unwrap_or_default().to_owned(),
        ]
    });
    print!("{}", table(&["ID", "CATEGORY", "UNITS", "DESCRIPTION"], cells));
    Ok(())
}
