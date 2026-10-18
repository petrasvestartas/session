
pub fn canonical(line: &str) -> String {
    let line=if let Some(rest)=line.trim().strip_prefix("nurbssurface_revolve"){format!("Nurbs Surface Revolve{rest}")}else{line.to_owned()};
    let alias=crate::transform_commands::alias(&line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(|| line.trim().to_owned(), |(name, count)| {
