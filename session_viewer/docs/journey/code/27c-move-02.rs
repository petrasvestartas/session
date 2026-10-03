pub fn parse(line: &str) -> Result<[f64; 3], &'static str> {
    let mut words = line.split_whitespace();
    if !words.next().is_some_and(|word| word.eq_ignore_ascii_case("Move")) {
        return Err("Expected Move x,y,z");
    }
    let rest = words.collect::<Vec<_>>().join(" ");
    let parts: Vec<_> = rest.split(',').collect();
    if parts.len() != 3 { return Err("Move needs three coordinates: x,y,z"); }
    let mut values = [0.0; 3];
    for (value, text) in values.iter_mut().zip(parts) {
        *value = text.trim().parse::<f64>().map_err(|_| "Move coordinates must be numbers")?;
        if !value.is_finite() { return Err("Move coordinates must be finite"); }
    }
    Ok(values)
}
