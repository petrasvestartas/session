
pub struct Editor {
    pub clipping_enabled:bool,pub clipping_solid:bool,
    pub interaction_epoch:u64,
    pub lasso:Option<crate::lasso::Lasso>,
