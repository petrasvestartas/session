pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[
    ("View Outline",||Action::ViewOutline(None)),
    ("Arctic",||Action::Arctic(None)),
    ("Select By Name",||Action::Query(crate::selection_query::Query::Name(String::new()))),
