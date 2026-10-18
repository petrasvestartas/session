pub static SHAPE: Shape = Shape {
    name: "Nurbs Curve Arc",
    options: &[
        ("Center", "Center"),
        ("3 Points", "3Points"),
        ("Cancel", "Escape"),
    ],
    upfront: true,
    ask,
    read,
    outline,
    build,
};

/// Center, start, end; or start, end, a point on the arc.
fn ask(answers: &[Answer], _part: &Part, option: &str) -> Option<Ask> {
    match (option, answers.len()) {
        ("Center", 0) => Some(Ask::point("Center")),
        ("Center", 1) => Some(Ask::size("Start or radius")),
        ("Center", 2) => Some(Ask::size("End or angle in degrees")),
        (_, 0) => Some(Ask::point("Start")),
        (_, 1) => Some(Ask::size("End or chord length")),
        (_, 2) => Some(Ask::size("Point on the arc or bulge")),
        _ => None,
    }
}

/// The circle's frame, x toward the start, and the radius and angle in degrees counterclockwise.
fn read(frame: &Frame, answers: &[Answer], option: &str) -> Result<Part, String> {
    match option {
        "Center" => read_center(frame, answers),
        _ => read_points(frame, answers),
    }
}

/// A click's angle about the center, or typed degrees; negative runs clockwise.
fn read_center(frame: &Frame, answers: &[Answer]) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    let Some(start) = answers.get(1) else {
        return Ok(part);
    };

    if let Answer::Point(start) = start {
        let [u, v, _] = frame.local(start);
        part.frame = frame.turned(u, v);
    }

    part.sizes.push(positive(frame.size(start), "The radius")?);
    let Some(end) = answers.get(2) else {
        return Ok(part);
    };
    let degrees = match end {
        Answer::Number(degrees) => *degrees,
        Answer::Point(end) => {
            let [u, v, _] = part.frame.local(end);
            let degrees = v.atan2(u).to_degrees();
            if degrees < 0.0 {
                degrees + 360.0
            } else {
                degrees
            }
        }
    };

    if !(degrees.abs() > 1e-9 && degrees.abs() < 360.0) {
        return Err(format!(
            "The angle must be above 0 and below 360 degrees, not {degrees}"
        ));
    }

    if degrees < 0.0 {
        part.frame = part.frame.flipped();
    }

    part.sizes.push(degrees.abs());
    Ok(part)
}

/// The circle through start, a point on it and end, running from start past that point.
fn read_points(frame: &Frame, answers: &[Answer]) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    let Some(end) = answers.get(1) else {
        return Ok(part);
    };
    let start = frame.origin.clone();
    let end = match end {
        Answer::Number(length) => frame.at(*length, 0.0, 0.0),
        Answer::Point(end) => end.clone(),
    };
    let chord = positive(start.distance(&end, None), "The chord")?;
    let [u, v, _] = frame.local(&end);
    part.frame = frame.turned(u, v);
    part.sizes.push(chord);
    let Some(middle) = answers.get(2) else {
        return Ok(part);
    };
    let middle = match middle {
        Answer::Number(bulge) => part.frame.at(chord * 0.5, *bulge, 0.0), // to the left of start → end
        Answer::Point(p) => p.clone(),
    };
    let (ab, ac) = (&end - &start, &middle - &start);
    let normal = ac.cross(&ab); // start, middle, end run counterclockwise about it

    if normal.magnitude() <= 1e-9 * ab.magnitude() * ac.magnitude() {
        return Err("The three points are in a line".into());
    }

    let offset = &(&normal.cross(&ab) * ac.dot(&ac)) + &(&ac.cross(&normal) * ab.dot(&ab));
    let center = &start - (&offset * (0.5 / normal.dot(&normal)));
    let radius = center.distance(&start, None);
    let x = (&start - &center).normalized();
    let z = normal.normalized();
    let circle = Frame {
        origin: center,
        y: z.cross(&x),
        x,
        z,
    };
    let [u, v, _] = circle.local(&end);
    let degrees = v.atan2(u).to_degrees();
    part.frame = circle;
    part.sizes = vec![
        radius,
        if degrees <= 0.0 {
            degrees + 360.0
        } else {
            degrees
        },
    ];
