    Ok(BeamVariable::through(sections, "beam_variable"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::parse as parse_line;
    use crate::element_tests::square;
    use wood::WoodElement;

    #[test]
    fn element_beam_variable_lofts_through_its_stations() {
        // the longest name wins: this is not Element Beam with the word Variable
        assert!(parse_line("Element Beam Variable").is_ok());
        let beam = build(vec![
            square(100.0, 0.0),
            square(100.0, 500.0),
            square(100.0, 1000.0),
        ])
        .unwrap();
        let solid = beam.solid();
        assert!((solid.volume().abs() - 100.0 * 100.0 * 1000.0).abs() < 1e-6 && solid.is_closed());
        assert!((beam.axis.length() - 1000.0).abs() < 1e-9);
        assert!(build(vec![square(100.0, 0.0)]).is_err());
    }
}
