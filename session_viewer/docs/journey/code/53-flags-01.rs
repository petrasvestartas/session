use crate::scene::ObjectId;
use std::collections::{HashMap,HashSet};

// Presentation stores plain identities and colours, so hiding or recolouring geometry never edits or retains its original source records.
#[derive(Clone,Default)]
pub struct Flags {pub current:Option<String>,pub hidden:HashSet<ObjectId>,pub lamps:HashSet<String>,pub held:HashSet<ObjectId>,pub locked:HashSet<ObjectId>,pub faces:HashMap<ObjectId,[u8;3]>,pub edges:HashMap<ObjectId,[u8;3]>}
impl Flags {
    // A parent lamp masks drawing without overwriting the independent visibility of its children.
    pub fn is_hidden(&self,id:&ObjectId)->bool {self.hidden.contains(id)||self.held.contains(id)}
    pub fn show_all(&mut self) {self.hidden.clear();self.lamps.clear();self.held.clear();}
    pub fn refresh(&mut self,forest:&crate::hierarchy::Forest) {
        self.held.clear();
        for doc in &forest.documents {for (index,node) in doc.nodes.iter().enumerate() {
            if self.lamps.contains(&format!("{}/{}",doc.key,node.source.guid)) {
                self.held.extend(doc.nodes[index..node.end].iter().filter_map(|node|node.object));
            }
        }}
    }

    pub fn initial(&mut self,id:ObjectId,visible:bool,locked:bool) {if !visible {self.hidden.insert(id);}if locked {self.locked.insert(id);}}
    pub fn hide(&mut self,ids:&[ObjectId],on:bool) {for id in ids {if on {self.hidden.insert(*id);} else {self.hidden.remove(id);}}}
    pub fn lock(&mut self,ids:&[ObjectId],on:bool) {for id in ids {if on {self.locked.insert(*id);} else {self.locked.remove(id);}}}
}
