    /// The option `words` start with and how many words spell it, ignoring case and spaces.
    pub(crate) fn option_in(&self, words: &[&str]) -> Option<(usize, usize)> {
        options::find(self.options, words)
    }
}

/// A positive, finite size, or a message naming it.
pub fn positive(value: f64, what: &str) -> Result<f64, String> {
    if !(value.is_finite() && value > 1e-9 && value <= 1e9) {
        return Err(format!("{what} must be above zero"));
    }

    Ok(value)
}

/// A nonzero, finite height, or a message.
pub fn nonzero(value: f64, what: &str) -> Result<f64, String> {
    if !(value.is_finite() && value.abs() > 1e-9 && value.abs() <= 1e9) {
        return Err(format!("{what} must not be zero"));
    }

    Ok(value)
}

/// The frame at the first answered point, else at the plane's origin.
pub fn frame_of(plane: &Plane, answers: &[Answer]) -> Frame {
    let origin = match answers.first() {
        Some(Answer::Point(p)) => p.clone(),
        _ => plane.origin(),
    };
    Frame::new(plane, origin)
}

/// Center, then radius: the questions of a sphere-like shape.
pub fn center_radius(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Center")),
        1 => Some(Ask::size("Radius")),
        _ => None,
    }
}

/// The radius as the distance from the center.
pub fn read_radius(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(radius) = answers.get(1) {
        part.sizes
            .push(positive(frame.distance(radius), "The radius")?);
    }

    Ok(part)
}

/// Three great circles.
pub fn sphere_outline(part: &Part) -> Vec<Vec<Point>> {
    let [radius] = part.sizes[..] else {
        return Vec::new();
    };
    let side = part.frame.rolled();
    vec![
        part.frame.ring(radius, 0.0),
        side.ring(radius, 0.0),
        side.rolled().ring(radius, 0.0),
    ]
}

