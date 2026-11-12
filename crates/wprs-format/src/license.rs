use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "verdict", content = "conditions", rename_all = "snake_case")]
pub enum Derivatives {
    Allowed(Vec<String>),
    Refused(String),
}

fn terms(branch: &str) -> Vec<String> {
    branch
        .split(" AND ")
        .map(|t| t.trim_matches(['(', ')', ' ']).to_owned())
        .filter(|t| !t.is_empty())
        .collect()
}

fn free(term: &str) -> bool {
    let upper = term.to_ascii_uppercase();
    let parts: Vec<&str> = upper.split('-').collect();
    !parts.contains(&"ND")
}

fn conditions(branch: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for term in branch {
        let parts: Vec<String> = term.to_ascii_uppercase().split('-').map(str::to_owned).collect();
        for (mark, what) in [("SA", "share-alike"), ("NC", "non-commercial"), ("BY", "attribution")] {
            if parts.iter().any(|p| p == mark) && !out.contains(&what.to_owned()) {
                out.push(what.to_owned());
            }
        }
        if term.starts_with("LicenseRef-") {
            out.push("custom terms".to_owned());
        }
        if term.starts_with("GPL") || term.starts_with("AGPL") || term.starts_with("LGPL") {
            out.push("copyleft".to_owned());
        }
    }
    out
}

pub fn derivatives(expression: &str) -> Derivatives {
    let expression = expression.trim();
    if expression.is_empty() {
        return Derivatives::Refused("the package states no license".to_owned());
    }
    let branches = expression.split(" OR ").map(terms);
    match branches.into_iter().find(|b| !b.is_empty() && b.iter().all(|t| free(t))) {
        Some(branch) => Derivatives::Allowed(conditions(&branch)),
        None => Derivatives::Refused(format!("{expression} does not allow derived works")),
    }
}
