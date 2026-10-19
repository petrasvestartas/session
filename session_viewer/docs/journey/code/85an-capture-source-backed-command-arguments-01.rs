use crate::{editor::{Action,Editor},scene::ObjectId};
#[derive(Clone,Debug,PartialEq)]
pub enum Request{
    BoundingBox,Measure(crate::measure_mark::Kind),Explode,Arrowhead(session_rust::Arrowhead),Group(String),Edge,Project(Vec<String>),Interval(crate::interval_geometry::Interval),Extend{distance:Option<f64>,asking:bool},Split,Trim,
    Transform{kind:crate::transform_draft::Kind,delta:[f64;16]},BeginTransform(crate::transform_draft::Kind),Scale(f64),Rotate{axis:crate::gizmo::Axis,degrees:f64},Move([f64;3]),Delete,
    Gather{name:String,text:String},Pick{kind:u8,id:Option<ObjectId>},
}
impl Request{
    pub fn capture(action:&Action)->Option<Self>{Some(match action{
        Action::BoundingBox=>Self::BoundingBox,Action::Measure(kind)=>Self::Measure(*kind),Action::Explode=>Self::Explode,Action::Arrowhead(head)=>Self::Arrowhead(*head),Action::AddGroup(name)=>Self::Group(name.clone()),Action::AddEdge=>Self::Edge,Action::Project(words)=>Self::Project(words.clone()),Action::Interval(interval)=>Self::Interval(*interval),Action::ExtendBegin{distance,asking}=>Self::Extend{distance:*distance,asking:*asking},Action::Split=>Self::Split,Action::Trim=>Self::Trim,
        Action::Transformation{kind,delta}=>Self::Transform{kind:*kind,delta:delta.m},Action::BeginTransformation(kind)=>Self::BeginTransform(*kind),Action::ScaleFactor(factor)=>Self::Scale(*factor),Action::RotateWorld{axis,degrees}=>Self::Rotate{axis:*axis,degrees:*degrees},Action::Translate(offset)=>Self::Move(*offset),Action::Delete=>Self::Delete,
        Action::GatherBegin{recipe,text}=>Self::Gather{name:recipe.name.into(),text:text.clone()},Action::TrimPick(id)=>Self::Pick{kind:0,id:*id},Action::SplitPick(id)=>Self::Pick{kind:1,id:*id},Action::ExtendPick(id)=>Self::Pick{kind:2,id:*id},Action::GatherPick(id)=>Self::Pick{kind:3,id:*id},_=>return None,
    })}


}
