use session_rust::{Element,Geometry,Plane,Polyline};
use wood::WoodElement;
use std::rc::Rc;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Kind{Beam,Column,Plate,Support,Variable,Block}
impl Kind{
 pub fn name(self)->&'static str{match self{Self::Beam=>"Element Beam",Self::Column=>"Element Column",Self::Plate=>"Element Plate",Self::Support=>"Element Support",Self::Variable=>"Element Beam Variable",Self::Block=>"Element Block"}}
 pub fn params(self)->Vec<Param>{match self{Self::Beam=>vec![Param{name:"Width",value:120.},Param{name:"Height",value:200.}],Self::Column=>vec![Param{name:"Width",value:200.},Param{name:"Depth",value:300.}],Self::Plate=>vec![Param{name:"Thickness",value:40.}],_=>vec![]}}
 pub fn read(self,text:&str)->Result<(Vec<Param>,bool),String>{let words=text.split_whitespace().collect::<Vec<_>>();let mut params=self.params();let count=params.len();let values=match self{Self::Beam=>crate::element_values::lengths(&words,[120.,200.],self.name())?.to_vec(),Self::Column=>crate::element_values::lengths(&words,[200.,300.],self.name())?.to_vec(),Self::Plate=>crate::element_values::lengths(&words,[40.],self.name())?.to_vec(),_=>{if !words.is_empty(){return Err(format!("{} takes no arguments",self.name()))}vec![]}};for(p,v)in params.iter_mut().zip(values){p.value=v;}Ok((params,count==0||!words.is_empty()))}
 pub fn fits(self,g:&Geometry)->bool{match self{Self::Support=>matches!(g,Geometry::Point(_)|Geometry::Plane(_)),_=>matches!(g,Geometry::Line(_)|Geometry::Polyline(_))}}
}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Param{pub name:&'static str,pub value:f64}

// No option toggles the current value; explicit On/Off never depends on the prior value.
pub fn visibility(words:&[&str])->Result<Option<bool>,String> {
    match words {
        []=>Ok(None),
        [word] if word.eq_ignore_ascii_case("On")=>Ok(Some(true)),
        [word] if word.eq_ignore_ascii_case("Off")=>Ok(Some(false)),
        _=>Err("Expected On or Off".into()),
    }
}
