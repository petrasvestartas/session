pub enum Action {
    StreamRefresh,
    PlaneMenu, Plane3,
    ConstructionPlane(Option<crate::cplane::CPlane>),
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
