use crate::{archive::{Bundle,Metadata,Document},scene::{ObjectId,Scene}};
use prost::Message;
use std::{collections::HashSet,rc::Rc};

// A namespace is one source document; imported files can reuse original GUIDs while their placements and presentation remain independent.
pub(crate) struct Namespace {pub name:String,pub doc:Option<Rc<crate::hierarchy::Document>>,pub ids:HashSet<ObjectId>}
impl Namespace {pub fn ordered(&self)->Vec<ObjectId> {let mut ids=self.ids.iter().copied().collect::<Vec<_>>();ids.sort_unstable_by_key(|id|id.value());ids}}
fn inventory(scene:&Scene)->Vec<Namespace> {
    let mut out=scene.forest.documents.iter().map(|doc|Namespace {name:doc.label.clone(),doc:Some(doc.clone()),ids:doc.nodes.iter().filter_map(|node|node.object).filter(|id|scene.contains(*id)).collect()}).collect::<Vec<_>>();
    let all=scene.objects().iter().map(|row|row.id).chain(scene.lines().iter().map(|row|row.id)).chain(scene.paths().iter().map(|row|row.id)).chain(scene.points().iter().map(|row|row.id)).chain(scene.clouds().iter().map(|row|row.id)).chain(scene.texts().iter().map(|row|row.id)).collect::<HashSet<_>>();
    let assigned=out.iter().flat_map(|namespace|namespace.ids.iter().copied()).collect::<HashSet<_>>();let ids=all.difference(&assigned).copied().collect::<HashSet<_>>();if !ids.is_empty() {out.push(Namespace {name:"Created".into(),doc:None,ids});}out
}

