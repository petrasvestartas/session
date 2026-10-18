
pub enum Action {
    SelectLasso,LassoPress{at:[f64;2],size:[f64;2],density:f64,pointer:i32},LassoMove{at:[f64;2],pointer:i32},LassoRelease{at:[f64;2],pointer:i32,add:bool,remove:bool},LassoCancel,
    AddGroup(String),AddEdge,
    StreamRefresh,
    Trim,TrimPick(Option<ObjectId>),TrimScreen{screen:[f32;2],size:[f64;2],commit:bool},CancelTrim,
    TextBegin(String),
    Explode,
    Query(crate::selection_query::Query),Arrowhead(session_rust::Arrowhead),Escape,
    NamedView(crate::camera::View),
    Measure(crate::measure_mark::Kind),MeasureDistance(String),BoundingBox,
    Split,SplitPick(Option<ObjectId>),CancelSplit,
