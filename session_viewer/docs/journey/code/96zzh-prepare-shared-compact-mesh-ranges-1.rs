use crate::mesh::Mesh;
use std::{collections::HashMap,ops::Range,rc::{Rc,Weak}};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct MeshRange {pub vertices:Range<u32>,pub indices:Range<u32>}
#[derive(Default)]
pub struct Rows {
    pub vertices:Vec<u8>,pub indices:Vec<u32>,
    sources:HashMap<usize,(Weak<Mesh>,MeshRange)>,
}
impl Rows {
    pub fn append(&mut self,source:&Rc<Mesh>)->Result<MeshRange,&'static str>{
        let key=Rc::as_ptr(source) as usize;
        if let Some((weak,range))=self.sources.get(&key) {
            if weak.upgrade().is_some_and(|saved|Rc::ptr_eq(&saved,source)){return Ok(range.clone());}
        }
        let start=u32::try_from(self.vertices.len()/20).map_err(|_|"Arena vertex range")?;
        let count=u32::try_from(source.vertices().len()).map_err(|_|"Arena vertex count")?;
        let end=start.checked_add(count).ok_or("Arena vertex range")?;
        let first=u32::try_from(self.indices.len()).map_err(|_|"Arena index range")?;
        let count=u32::try_from(source.indices().len()).map_err(|_|"Arena index count")?;
        let last=first.checked_add(count).ok_or("Arena index range")?;
        let range=MeshRange {vertices:start..end,indices:first..last};
        self.vertices.extend(crate::packed_vertex::bytes(source));
        self.indices.extend(source.indices().iter().map(|i|i+start));
        self.sources.insert(key,(Rc::downgrade(source),range.clone()));Ok(range)
    }
    pub fn index_bytes(&self)->Vec<u8>{self.indices.iter().flat_map(|i|i.to_le_bytes()).collect()}
    pub fn source_count(&self)->usize{self.sources.len()}
    pub fn reset(&mut self){self.vertices.clear();self.indices.clear();self.sources.clear();}
    pub fn release(&mut self){*self=Self::default();}
}
