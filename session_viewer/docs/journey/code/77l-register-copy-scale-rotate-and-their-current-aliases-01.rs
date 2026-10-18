use crate::{editor::Action,transform_draft::Kind};
pub const NAMES:&[&str]=&["Copy","Scale","Rotate","Rotate x","Rotate y","Rotate z"];
pub fn action(name:&str,rest:&[&str])->Option<Result<Action,String>> {
    if !NAMES.contains(&name){return None}
    Some((||{
        match name {
            "Copy"=>{if rest.is_empty(){Ok(Action::BeginTransformation(Kind::Copy))}else{let offset=crate::offset::parse(&format!("Move {}",rest.join(" "))).map_err(str::to_owned)?;Ok(Action::Transformation{kind:Kind::Copy,delta:session_rust::Xform::translation(offset[0],offset[1],offset[2])})}},
            "Scale"=>match rest{[]=>Ok(Action::BeginTransformation(Kind::Scale)),[value]=>{let factor=value.parse::<f64>().ok().filter(|v|v.is_finite()&&*v>0.).ok_or("Scale wants a finite factor above zero")?;Ok(Action::ScaleFactor(factor))},_=>Err("Scale takes one factor or interactive points".into())},
            "Rotate" if rest.is_empty()=>Ok(Action::BeginTransformation(Kind::Rotate)),
            _=>{let(axis,values)=if let Some(axis)=name.strip_prefix("Rotate "){(axis,rest)}else{let Some((axis,values))=rest.split_first()else{return Err("Rotate x 90".into())};(*axis,values)};let axis=match axis.to_ascii_lowercase().as_str(){"x"=>crate::gizmo::Axis::X,"y"=>crate::gizmo::Axis::Y,"z"=>crate::gizmo::Axis::Z,_=>return Err("Rotate axis must be x y or z".into())};let [degrees]=values else{return Err("Rotate needs one angle in degrees".into())};let degrees=degrees.parse::<f64>().ok().filter(|v|v.is_finite()).ok_or("Rotate needs finite degrees")?;Ok(Action::RotateWorld{axis,degrees})},
        }
    })())
}
pub fn alias(line:&str)->String {let mut words=line.split_whitespace();let Some(first)=words.next()else{return String::new()};let name=match first.to_ascii_lowercase().as_str(){"m"=>"Move","s"=>"Scale","rot"=>"Rotate",_=>return line.trim().to_owned()};std::iter::once(name).chain(words).collect::<Vec<_>>().join(" ")}
