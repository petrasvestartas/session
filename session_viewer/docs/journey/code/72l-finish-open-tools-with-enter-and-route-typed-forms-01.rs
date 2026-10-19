pub enum Action {
    StreamRefresh,
    ToolText(String), FinishTool,
    PlaneMenu, Plane3,
    ConstructionPlane(Option<crate::cplane::CPlane>),
