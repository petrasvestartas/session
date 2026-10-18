pub enum Action {
    StreamRefresh,
    CloseTool,
    Draw {shape:crate::tool::Shape,text:String},
    ToolText(String), FinishTool,
