}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::parse as parse_line;
    use session_rust::Point;
    use wood::WoodElement;

    #[test]
    fn element_column_lofts_the_rectangle_up_the_line() {
        assert!(
            parse_line("Element Column 200 300").is_ok() && parse_line("Element Column x").is_err()
        );
        let axis = Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 3000.0),
        ]);
        let columns = build(&[axis], 200.0, 300.0).unwrap();
        let solid = columns[0].solid();
        assert!((solid.volume().abs() - 200.0 * 300.0 * 3000.0).abs() < 1e-6 && solid.is_closed());
        assert!(columns[0].base_plane().is_some());
    }
}
