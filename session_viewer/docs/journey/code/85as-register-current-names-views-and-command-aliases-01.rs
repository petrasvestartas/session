pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Select By Name",||Action::Query(crate::selection_query::Query::Name(String::new()))),
    ("Select Small",||Action::Query(crate::selection_query::Query::Small(0.))),
    ("Add Group",||Action::AddGroup(String::new())),
    ("Add Edge",||Action::AddEdge),
    ("Select Lasso",||Action::SelectLasso),
    ("Trim",||Action::Trim),
    ("Explode",||Action::Explode),
    ("Arrowhead",||Action::Arrowhead(session_rust::Arrowhead::NONE)),
    ("Snap", || Action::Snap(crate::snap::Setting::Objects(None))),
    ("Snap On", || Action::Snap(crate::snap::Setting::Objects(Some(true)))),
