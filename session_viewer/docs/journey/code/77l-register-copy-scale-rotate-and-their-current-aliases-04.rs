
pub fn canonical(line: &str) -> String {
    let alias=crate::transform_commands::alias(line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(|| line.trim().to_owned(), |(name, count)| {
