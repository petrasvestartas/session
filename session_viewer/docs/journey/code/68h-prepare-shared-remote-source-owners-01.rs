use std::{cell::RefCell,rc::Rc};
use crate::{bounds::Bounds,cloud::LodNode,range_source::Source,range_page::Page,page_cache::{Budget,Cache,Held}};

pub struct Prepared {pub source:Rc<Source>,pub nodes:Rc<Vec<LodNode>>,pub bounds:Bounds,pub pages:RefCell<Cache<u32,Page>>,pub root:RefCell<Option<Rc<Held<Page>>>>,pub generation:RefCell<crate::stream_generation::Generation>,pub loaded:std::cell::Cell<u32>}
impl Prepared {
    pub fn new(source:Rc<Source>,budget:Rc<Budget>)->Result<Rc<Self>,&'static str> {
        if source.guid.is_empty()||source.fields.count==0||!source.lod.valid(source.fields.count)||!source.properties.point_px.is_finite()||source.properties.point_px<=0.||source.fields.coords_len!=u64::from(source.fields.count)*24||source.fields.coords_at.checked_add(source.fields.coords_len).is_none_or(|end|end>source.fields.end)||source.fields.revision.is_none()&&!crate::manifest::immutable_key(&source.url) {return Err("Invalid streamed original source");}
        let n=source.lod.len();let mut nodes=Vec::with_capacity(n);
        for i in 0..n {
            let size=source.lod.size[i];let center=std::array::from_fn(|axis|(source.lod.min[i*3+axis]+size*0.5) as f32);
            nodes.push(LodNode {center,size:size as f32,spacing:source.lod.spacing[i] as f32,first:source.lod.first[i] as u32,count:source.lod.count[i] as u32,children:source.lod.children[i*8..i*8+8].try_into().unwrap()});
        }
        let size=source.lod.size[0];let min=std::array::from_fn(|i|source.lod.min[i]);let max=std::array::from_fn(|i|min[i]+size);
        Ok(Rc::new(Self {source,nodes:Rc::new(nodes),bounds:Bounds {min,max},pages:RefCell::new(Cache::new(budget)),root:RefCell::new(None),generation:RefCell::new(Default::default()),loaded:std::cell::Cell::new(0)}))
    }
}
