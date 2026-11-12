use crate::told::{fill, told};
use crate::usage;
use anyhow::Result;
use std::path::Path;

pub fn repository(text: &str) -> Result<String> {
    if text.contains("://") {
        return Ok(text.to_owned());
    }
    let (folder, query) = text.split_once('?').map_or((text, None), |(folder, query)| (folder, Some(query)));
    if !Path::new(folder).is_dir() {
        return Err(usage(fill(told("%1 is neither a folder nor a URL"), &[&folder])));
    }
    let absolute = std::path::absolute(folder)?;
    let base = format!("file://{}/", absolute.display().to_string().trim_end_matches('/'));
    Ok(query.map_or(base.clone(), |query| format!("{base}?{query}")))
}

#[cfg(test)]
mod tests {
    use super::repository;

    #[test]
    fn a_folder_becomes_a_file_url_that_keeps_its_anchor() {
        let tmp = tempfile::tempdir().unwrap();
        let folder = tmp.path().display().to_string();
        let anchor = format!("fingerprint={}", "ab".repeat(32));
        let url = repository(&format!("{folder}?{anchor}")).unwrap();
        assert_eq!(url, format!("file://{folder}/?{anchor}"));
        let trailing = repository(&format!("{folder}/?{anchor}")).unwrap();
        assert_eq!(trailing, url);
        assert_eq!(repository(&folder).unwrap(), format!("file://{folder}/"));
    }

    #[test]
    fn urls_pass_through_and_missing_folders_are_refused() {
        let url = "https://example.org/repo?fingerprint=ab";
        assert_eq!(repository(url).unwrap(), url);
        let error = repository("no/such/folder?repo_id=ab").unwrap_err().to_string();
        assert!(error.contains("neither a folder nor a URL"), "{error}");
    }
}
