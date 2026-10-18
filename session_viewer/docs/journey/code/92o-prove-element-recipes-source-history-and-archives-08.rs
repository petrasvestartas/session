    Ok(Block::new(loops, "block"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::parse as parse_line;
    use crate::element_tests::square;
    use wood::WoodElement;

    #[test]
    fn element_block_lofts_bottom_to_top() {
        assert!(parse_line("Element Block").is_ok() && parse_line("Element Block 3").is_err());
        let solid = build(vec![square(300.0, 0.0), square(300.0, 250.0)])
            .unwrap()
            .solid();
        assert!((solid.volume().abs() - 300.0 * 300.0 * 250.0).abs() < 1e-6 && solid.is_closed());
        assert!(build(vec![square(300.0, 0.0)]).is_err());
    }
}
