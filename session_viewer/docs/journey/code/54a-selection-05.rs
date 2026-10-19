
pub enum Action {
    SelectRows {ids:Vec<ObjectId>,add:bool,remove:bool},
    PanScreen {delta:[f64;2],size:[f64;2]},ZoomAt {factor:f32,screen:[f32;2]},
    LayerEdit {key:String,edit:crate::layer_edit::Edit},
