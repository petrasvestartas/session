    let (x,y)=frame.axes();let mut all=previous.to_vec();let mut added=Vec::new();
    for word in text.split_whitespace() {
        if all.len()>=crate::tool::MAX_POINTS {return Err("Drawing commands accept at most 4096 points");}
        if word.trim_start_matches(['w','W','@']).split([',','<']).any(|part|part.parse::<f64>().is_ok_and(|v|!v.is_finite())) {return Err("Coordinates must be finite and within ±1e12");}
        let (typed,world)=coords::parse_world(word).ok_or("Use x,y,z, @x,y,z, distance<angle, or w for world")?;
