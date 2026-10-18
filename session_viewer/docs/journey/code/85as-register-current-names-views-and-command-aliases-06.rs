
pub fn canonical(line: &str) -> String {
    let line=command_alias(line);let line=line.as_str();
    let line=crate::gather_registry::alias(line);
    let alias=crate::transform_commands::alias(&line);let line=alias.as_str();
