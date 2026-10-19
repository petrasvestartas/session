pub enum Action {
    StreamRefresh,
    ConstructionPlane(Option<crate::cplane::CPlane>),
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
