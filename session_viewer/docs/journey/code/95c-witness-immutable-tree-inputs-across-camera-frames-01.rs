use std::{collections::{HashMap,HashSet},rc::Rc};
use crate::{hierarchy::Document,scene::{ObjectId,Scene},view_flags::Flags};

pub struct Witness{
    documents:Vec<Rc<Document>>,
    groups:Rc<HashSet<(u32,String)>>,
    graphs:Rc<HashMap<u32,Rc<session_rust::Graph>>>,
    view:Rc<Flags>,
    selection:Vec<ObjectId>,
    epoch:u64,
}

impl Witness{
    pub fn new(scene:&Scene,selection:&[ObjectId],epoch:u64)->Self{
        Self{documents:scene.forest.documents.clone(),groups:scene.groups.clone(),graphs:scene.graphs.clone(),view:scene.view.clone(),selection:selection.to_vec(),epoch}
    }
    pub fn matches(&self,scene:&Scene,selection:&[ObjectId],epoch:u64)->bool{
        self.epoch==epoch&&self.selection==selection&&Rc::ptr_eq(&self.groups,&scene.groups)&&Rc::ptr_eq(&self.graphs,&scene.graphs)&&Rc::ptr_eq(&self.view,&scene.view)&&self.documents.len()==scene.forest.documents.len()&&self.documents.iter().zip(&scene.forest.documents).all(|(a,b)|Rc::ptr_eq(a,b))
    }
}
