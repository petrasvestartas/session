
pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Point", || Action::BeginTool(crate::tool::Shape::Point)),
    ("Line", || Action::BeginTool(crate::tool::Shape::Line)),
    ("Example Box", || Action::AddBox),
    ("Example Line", || Action::AddLine),
    ("Example Point", || Action::AddPoint),
