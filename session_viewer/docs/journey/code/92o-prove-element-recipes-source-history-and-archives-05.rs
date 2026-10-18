}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::parse as parse_line;
    use crate::element_tests::square;
    use wood::WoodElement;

    #[test]
    fn element_plate_lofts_the_outline_by_its_thickness() {
        assert!(parse_line("Element Plate").is_ok() && parse_line("element plate 25").is_ok());
        assert!(parse_line("Element Plate -4").is_err());
        let plates = build(&[square(600.0, 0.0)], 40.0).unwrap();
        let solid = plates[0].solid();
        assert!((solid.volume().abs() - 600.0 * 600.0 * 40.0).abs() < 1e-6 && solid.is_closed());
        assert!(build(&[], 40.0).is_err());
    }
}
