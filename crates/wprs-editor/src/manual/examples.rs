const TEXT: &str = include_str!("examples.txt");

pub fn lines(kind: &str, name: &str) -> Option<Vec<&'static str>> {
    let head = format!("{kind} {name}");
    TEXT.split("\n\n").find_map(|block| {
        let mut lines = block.lines();
        (lines.next()? == head).then(|| lines.collect())
    })
}
