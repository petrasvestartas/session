use crate::{manifest::Manifest,scene::Scene};
use std::{collections::{HashMap,HashSet},rc::Rc};

pub struct Index {
    base:String,
    revisions:crate::live_revision::Revisions,
    manifest:Option<Manifest>,
    sources:HashMap<String,Scene>,
    sizes:HashMap<String,usize>,
    pending:bool,
}
impl Index {
    pub fn new(base:String)->Self {Self {base,revisions:Default::default(),manifest:None,sources:HashMap::new(),sizes:HashMap::new(),pending:false}}
    pub fn etag(&self,url:&str)->Option<String> {self.revisions.etag(url)}
    pub fn has_manifest(&self)->bool {self.manifest.is_some()}
    pub fn cached(&self,url:&str)->bool {self.sources.contains_key(url)}
    pub fn forget(&mut self,url:&str) {self.revisions.forget(url);}
    pub fn urls(&self)->Vec<String> {self.manifest.as_ref().map_or_else(Vec::new,|manifest|manifest.items.iter().map(|item|crate::publication_route::join(&self.base,&item.file)).collect())}
    pub fn manifest(&mut self,url:&str,etag:Option<String>,bytes:&[u8])->Result<bool,String> {
        if !self.revisions.changed(url,etag,bytes) {return Ok(false);}
        let manifest=match Manifest::parse(bytes) {Ok(value)=>value,Err(error)=>{self.revisions.forget(url);return Err(error);}};
        self.manifest=Some(manifest);self.pending=true;let used=self.urls().into_iter().collect::<HashSet<_>>();self.sources.retain(|url,_|used.contains(url));self.sizes.retain(|url,_|used.contains(url));Ok(true)
    }
    pub fn source(&mut self,url:&str,etag:Option<String>,bytes:&[u8])->Result<bool,String> {
        if !self.revisions.changed(url,etag,bytes) {return Ok(false);}
        let size=self.sizes.iter().filter(|(key,_)|key.as_str()!=url).try_fold(bytes.len(),|sum,(_,size)|sum.checked_add(*size)).ok_or("Source inventory size overflow")?;
        if size>crate::archive::LIMIT {self.revisions.forget(url);return Err("Source inventory exceeds the 512 MiB file limit".into());}
        let source=match crate::publication::prepare(&Scene::empty(),bytes,true,Some(Rc::new(crate::reload_url::ReloadUrl::new(url.into())))) {Ok(source)=>source,Err(error)=>{self.revisions.forget(url);return Err(error);}};
        self.sources.insert(url.into(),source);self.sizes.insert(url.into(),bytes.len());self.pending=true;Ok(true)
    }
    pub fn replacement(&mut self,previous:&Scene)->Result<Option<Scene>,String> {
        if !self.pending {return Ok(None);}
        let manifest=self.manifest.as_ref().ok_or("Live manifest is missing")?;
        for (item,url) in manifest.items.iter().zip(self.urls()) {if let Some(expected)=item.encoded_size() {if self.sizes.get(&url).copied().map(|size|size as u64)!=Some(expected) {self.revisions.forget(&url);return Err("Encoded source decoded size does not match the manifest".into());}}}
        let scene=crate::scene_manifest::prepare(previous,manifest,&self.urls(),&self.sources)?;self.pending=false;Ok(Some(scene))
    }
}
