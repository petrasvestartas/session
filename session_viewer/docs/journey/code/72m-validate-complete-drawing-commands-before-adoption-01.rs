pub enum Action {
    StreamRefresh,
    Draw {shape:crate::tool::Shape,text:String},
    ToolText(String), FinishTool,
    PlaneMenu, Plane3,
