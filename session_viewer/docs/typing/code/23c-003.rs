
#[cfg(test)]
mod surfacing_tests {
    use super::*;

    /// A leading option word picks it; every other word must be a point or a number.
    #[test]
    fn typed_words_pick_an_option_then_answer() {
        let shape = &verbs::r#box::SHAPE;
        assert_eq!(shape.option_in(&["mesh", "0,0,0"]), Some((1, 1)));
        assert_eq!(shape.option_in(&["0,0,0"]), None);
        assert!(start(shape, &["Mesh", "0,0,0", "@50,25", "30"]).is_ok());
        assert!(start(shape, &["wide"]).is_err());
        let arc = &verbs::nurbs_curve_arc::SHAPE;
        assert_eq!(arc.option_in(&["3", "Points"]), Some((1, 2)));
        assert_eq!(arc.option_in(&["3points"]), Some((1, 1)));
        assert_eq!(arc.option_in(&["3"]), None);
    }
}
