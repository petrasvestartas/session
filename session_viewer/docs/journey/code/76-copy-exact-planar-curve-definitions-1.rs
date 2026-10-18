pub static SHAPE: Shape = Shape {
    name: "Nurbs Curve Parabola",
    options: CURVE,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

const SAMPLES: usize = 32; // preview segments

/// Start, end, then the apex.
fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Start")),
        1 => Some(Ask::size("End or length")),
        2 => Some(Ask::size("Apex or height")),
        _ => None,
    }
}

/// The frame runs from start to end; the apex is where the curve passes half way, off the chord.
fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    let Some(end) = answers.get(1) else {
        return Ok(part);
    };

    if let Answer::Point(end) = end {
        let [u, v, _] = frame.local(end);
        part.frame = frame.turned(u, v);
    }

    let length = positive(frame.size(end), "The length")?; // a click lands on the plane
    part.sizes.push(length);

    if let Some(apex) = answers.get(2) {
        let [u, v, w] = match apex {
            Answer::Number(height) => [length * 0.5, *height, 0.0], // to the left of start → end
            Answer::Point(p) => part.frame.local(p),
        };

        if v.hypot(w) <= 1e-9 * length {
            return Err("The apex must lie off the line from start to end".into());
        }

        part.sizes.extend([u, w, v]); // the height last: the cursor readout shows it
    }

    Ok(part)
}
