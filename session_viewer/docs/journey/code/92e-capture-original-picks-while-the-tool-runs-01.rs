use crate::{scene::{Scene,ObjectId},element_input::{Kind,Param,Made}};
#[derive(Clone)]
pub struct Draft{pub kind:Kind,pub params:Vec<Param>,pub picked:Vec<crate::source_edit::Owner>,pub asking:Option<usize>,pub next:usize}
impl Draft{
 pub fn new(kind:Kind,text:&str,scene:&Scene,ids:Vec<ObjectId>)->Result<(Self,bool),String>{let(params,typed)=kind.read(text)?;let mut result=Self{kind,params,picked:vec![],asking:None,next:0};for id in ids{if let Ok(owner)=crate::source_edit::Owner::capture(scene,id){if kind.fits(&owner.source){result.picked.push(owner);}}}Ok((result,typed))}
 pub fn ids(&self)->Vec<ObjectId>{self.picked.iter().map(|o|o.id).collect()}
 pub fn pick(&mut self,scene:&Scene,id:Option<ObjectId>)->Result<(),String>{let id=id.ok_or("Nothing there: pick an original source")?;if let Some(i)=self.picked.iter().position(|o|o.id==id){self.picked.remove(i);return Ok(())}let owner=crate::source_edit::Owner::capture(scene,id).map_err(str::to_owned)?;if !self.kind.fits(&owner.source){return Err(format!("{}: pick fitting original sources",self.kind.name()))}self.picked.push(owner);Ok(())}
 pub fn valid(&self,scene:&Scene)->Result<(),String>{for o in &self.picked{o.valid(scene).map_err(str::to_owned)?;}Ok(())}
 pub fn build(&self,scene:&Scene)->Result<Vec<Made>,String>{self.valid(scene)?;crate::element_input::build(self.kind,&self.params,&self.picked.iter().map(|o|(o.source.clone(),o.model.clone())).collect::<Vec<_>>())}

 pub fn values(&self)->String{self.params.iter().map(|p|format!("{} {}",p.name,p.value)).collect::<Vec<_>>().join(" · ")}
 pub fn prompt(&self)->String{if let Some(i)=self.asking{format!("{} <{}>: type a number · Enter keeps it",self.params[i].name,self.params[i].value)}else{format!("{} · pick original sources ({} picked) · Enter creates · Esc cancels",self.kind.name(),self.picked.len())}}
 pub fn buttons(&self)->Vec<(String,String)>{self.params.iter().map(|p|(format!("{}={}",p.name,p.value),p.name.into())).chain([("Create".into(),"".into()),("Cancel".into(),"Escape".into())]).collect()}

}
