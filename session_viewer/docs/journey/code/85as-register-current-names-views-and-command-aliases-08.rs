
pub fn parse(line: &str) -> Result<Command, String> {
    let line=command_alias(line);let line=line.as_str();
    let line=crate::gather_registry::alias(line);
