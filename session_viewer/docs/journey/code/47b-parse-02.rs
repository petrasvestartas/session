        return Err("Expected Move x,y,z");
    }
    let rest = words.collect::<Vec<_>>().join(" ");
    let parts: Vec<_> = if rest.contains(',') { rest.split(',').collect() } else { rest.split_whitespace().collect() };
    if parts.len() != 3 { return Err("Move needs three coordinates: x,y,z"); }
    let mut values = [0.0; 3];
    for (value, text) in values.iter_mut().zip(parts) {
