fn positive(text: &str) -> Result<u32, String> {
    let value = text.parse::<u32>().map_err(|error| error.to_string())?;
    if value == 0 {
        return Err("expected a positive count".to_string());
    }
    Ok(value)
}

fn first(values: &[u32]) -> Option<u32> {
    values.first().copied()
}

fn main() {
    assert_eq!(positive("12"), Ok(12));
    assert!(positive("zero").is_err());
    assert!(positive("0").is_err());
    assert_eq!(first(&[7, 9]), Some(7));
    assert_eq!(first(&[]), None);
}
