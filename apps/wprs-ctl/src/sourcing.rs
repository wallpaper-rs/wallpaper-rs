use crate::told::{fill, told};
use crate::usage;
use anyhow::Result;
use std::path::Path;
use wprs_client::{Client, explain};
use wprs_core::model::SourcesReply;

const SHORTEST: usize = 6;

pub fn pick<'a>(sources: impl Iterator<Item = (&'a str, &'a Path)>, text: &str) -> Result<String> {
    let wanted = text.to_ascii_uppercase();
    let known: Vec<_> = sources.collect();
    let exact = known.iter().find(|(uid, path)| *uid == text || *path == Path::new(text));
    if let Some((uid, _)) = exact {
        return Ok((*uid).to_owned());
    }
    let ends = |uid: &str| wanted.len() >= SHORTEST && uid.to_ascii_uppercase().ends_with(&wanted);
    let found: Vec<&str> = known.iter().map(|(uid, _)| *uid).filter(|uid| ends(uid)).collect();
    match found[..] {
        [uid] => Ok(uid.to_owned()),
        [] => Ok(text.to_owned()),
        _ => Err(usage(fill(told("%1 matches more than one library folder; use more characters"), &[&text]))),
    }
}

pub async fn source(client: &Client, text: &str) -> Result<String> {
    let Ok(json) = client.library.list_sources().await else { return Ok(text.to_owned()) };
    let reply: SourcesReply = serde_json::from_str(&json)?;
    pick(reply.sources.iter().map(|s| (s.uid.as_ref(), s.path.as_path())), text)
}

pub async fn scanned(client: &Client, text: Option<String>) -> Result<String> {
    match text {
        Some(text) => source(client, &text).await,
        None => Ok(String::new()),
    }
}

pub async fn unsource(client: &Client, text: &str) -> Result<()> {
    client.library.remove_source(&source(client, text).await?).await.map_err(explain)?;
    println!("{}", fill(told("removed the folder %1 from the library"), &[&text]));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::pick;
    use std::path::Path;

    #[test]
    fn the_short_id_of_the_listing_names_the_folder() {
        let sources = [("01KABCDEFGHJKMNPQRSTVW044Q3NMP", "/a"), ("01KABCDEFGHJKMNPQRSTVW099Z9ZZZ", "/b")];
        let list = || sources.iter().map(|(u, p)| (*u, Path::new(*p)));
        assert_eq!(pick(list(), "044Q3NMP").unwrap(), sources[0].0);
        assert_eq!(pick(list(), "044q3nmp").unwrap(), sources[0].0);
        assert_eq!(pick(list(), sources[1].0).unwrap(), sources[1].0);
        assert_eq!(pick(list(), "/b").unwrap(), sources[1].0);
        let twins = [("AAAAAAAAAAAAXYZ123", "/a"), ("BBBBBBBBBBBBXYZ123", "/b")];
        assert!(pick(twins.iter().map(|(u, p)| (*u, Path::new(*p))), "XYZ123").is_err());
        assert_eq!(pick(list(), "ZZZ").unwrap(), "ZZZ");
        assert_eq!(pick(list(), "FFFFFFFF").unwrap(), "FFFFFFFF");
    }
}
