}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::parse as parse_line;
    use session_rust::Point;
    use wood::WoodElement;

    #[test]
    fn element_beam_sweeps_the_rectangle_along_the_axis() {
        assert!(parse_line("Element Beam 100 200").is_ok() && parse_line("Element Beam").is_ok());
        assert!(parse_line("Element Beam 1 2 3").is_err());
        let axis = Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1000.0, 0.0, 0.0),
        ]);
        let solid = build(&[axis], 100.0, 200.0).unwrap()[0].solid();
        assert!((solid.volume().abs() - 1000.0 * 100.0 * 200.0).abs() < 1e-6 && solid.is_closed());
        assert!(build(&[Polyline::new(vec![Point::new(0.0, 0.0, 0.0)])], 1.0, 1.0).is_err());
    }
}
