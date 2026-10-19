use crate::{hierarchy::Document,scene::{Scene,ObjectId}};
use std::rc::Rc;
pub struct Plan{pub owner:usize,pub document:Rc<Document>,pub name:String,pub parent:usize,pub members:Vec<ObjectId>}
impl Plan{
    pub fn new(scene:&Scene,ids:&[ObjectId],name:&str)->Result<Self,&'static str>{
    use std::collections::HashSet;
    let chosen=ids.iter().copied().collect::<HashSet<_>>();if chosen.is_empty(){return Err("Select objects first")}
    for id in &chosen{crate::source_edit::editable(scene,*id)?;}
    let owners=scene.forest.documents.iter().enumerate().filter_map(|(owner,doc)|doc.nodes.iter().any(|n|n.object.is_some_and(|id|chosen.contains(&id))).then_some(owner)).collect::<Vec<_>>();
    if owners.len()!=1{return Err("Add Group keeps objects in their document; select objects of one document")}
    let owner=owners[0];let doc=scene.forest.documents[owner].clone();if doc.nodes.len()>=crate::hierarchy::MAX_NODES{return Err("Tree exceeds its node limit")}
    let name=name.trim();if name.contains('/'){return Err("A group name must be without /")}
    let taken=|s:&str|doc.nodes.iter().any(|n|n.label==s||n.source.name==s);
    let name=if name.is_empty(){(1..).map(|i|format!("Group {i}")).find(|s|!taken(s)).unwrap()}else{if taken(name){return Err("This name is already used in the document")}name.into()};
    let mut members=Vec::new();let mut shared=None::<Vec<usize>>;
    for node in &doc.nodes{let Some(id)=node.object.filter(|id|chosen.contains(id))else{continue};let mut ancestors=Vec::new();let mut parent=node.parent;let mut follows=false;
        while let Some(p)=parent{let n=&doc.nodes[p];follows|=n.object.is_some_and(|id|chosen.contains(&id));ancestors.push(p);parent=n.parent;}if follows{continue}
        let layers=ancestors.into_iter().rev().take_while(|&p|doc.nodes[p].object.is_none()).collect::<Vec<_>>();
        shared=Some(match shared{None=>layers,Some(mut common)=>{let n=common.iter().zip(layers).take_while(|(a,b)|**a==*b).count();common.truncate(n);common}});members.push(id);
    }
    let parent=shared.and_then(|v|v.last().copied()).ok_or("This document has no layer to hold a group")?;Ok(Self{owner,document:doc,name,parent,members})
    }
}
