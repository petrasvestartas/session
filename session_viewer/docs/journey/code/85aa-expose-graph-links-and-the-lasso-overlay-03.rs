
pub struct Panel {
    pub lasso:Vec<[f32;2]>,
    pub rectangle:Option<crate::rect_select::Rectangle>,
    pub tree:crate::tree_panel::TreePanel,
