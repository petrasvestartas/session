pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Arctic",||Action::Arctic(None)),
    ("Select By Name",||Action::Query(crate::selection_query::Query::Name(String::new()))),
    ("Select Small",||Action::Query(crate::selection_query::Query::Small(0.))),
