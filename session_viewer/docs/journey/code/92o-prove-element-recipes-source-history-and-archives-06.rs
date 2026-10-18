}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::parse as parse_line;
    use wood::WoodElement;

    #[test]
    fn element_support_stands_on_its_plane() {
        assert!(parse_line("Element Support").is_ok() && parse_line("Element Support 1").is_err());
        let supports = build(&[Plane::xy_plane()]).unwrap();
        assert!(supports[0].solid().number_of_faces() > 0);
        assert_eq!(
            supports[0].base_plane().unwrap().origin(),
            Plane::xy_plane().origin()
        );
        assert!(build(&[]).is_err());
    }
}
