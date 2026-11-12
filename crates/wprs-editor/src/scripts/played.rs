pub(super) fn played(source: &str) -> bool {
    let word = |c: char| c.is_alphanumeric() || c == '_' || c == '$';
    let video = source.match_indices(".video").any(|(i, m)| !source[i + m.len()..].starts_with(word));
    let ended = source.match_indices(".on(").any(|(i, m)| {
        let rest = source[i + m.len()..].trim_start();
        ["\"ended\"", "'ended'", "`ended`"].iter().any(|q| rest.starts_with(q))
    });
    video || ended
}
