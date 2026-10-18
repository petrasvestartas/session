pub fn lengths<const N: usize>(
    rest: &[&str],
    defaults: [f64; N],
    usage: &str,
) -> Result<[f64; N], String> {
    if rest.len() > N {
        return Err(format!("Too many numbers · {usage}"));
    }

    let mut values = defaults;

    for (value, word) in values.iter_mut().zip(rest) {
        *value = word.parse::<f64>().ok().filter(|v|v.is_finite())
            .filter(|value| *value > 0.0)
            .ok_or_else(|| format!("`{word}` is not a positive length · {usage}"))?;
    }

    Ok(values)
}

