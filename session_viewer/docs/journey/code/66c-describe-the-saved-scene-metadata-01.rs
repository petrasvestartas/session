use crate::text::TextLabel;
use prost::Message;
use serde::{Deserialize,Serialize};

pub const MAGIC:&[u8]=b"SESSION-VIEWER\x01\n";
pub const LIMIT:usize=512*1024*1024;

// Each source document keeps its own GUID namespace; small presentation metadata does not replace its original geometry protobuf.
#[derive(Clone,PartialEq,Message)]
pub struct Archive {
    #[prost(bytes="vec",repeated,tag="1")]
    pub documents:Vec<Vec<u8>>,
    #[prost(bytes="vec",tag="2")]
    pub metadata:Vec<u8>,
}
#[derive(Clone,Serialize,Deserialize)]
pub struct Document {
    pub name:String,
    pub place:[f64;16],
    pub point_px:f32,
}
#[derive(Clone,Serialize,Deserialize)]
pub struct Metadata {
    #[serde(default)]
    pub created_doc:Option<usize>,
    #[serde(default)]
    pub current_layer:Option<(usize,String)>,
    pub documents:Vec<Document>,
    pub hidden:Vec<(usize,String)>,
    #[serde(default)]
    pub locked:Vec<(usize,String)>,
    #[serde(default)]
    pub colors:Vec<(usize,String,[u8;3])>,
    #[serde(default)]
    pub edge_colors:Option<Vec<(usize,String,[u8;3])>>,
    pub texts:Vec<(String,TextLabel,bool)>,
    #[serde(default)]
    pub groups:Vec<(usize,String)>,
}
impl Metadata {
    pub fn validate(&self,count:usize)->Result<(),String> {
        if self.documents.len()!=count {return Err("Document inventory does not match".into());}
        if self.created_doc.is_some_and(|index|index>=count) {return Err("Created document index is outside the inventory".into());}
        if self.documents.iter().any(|doc|!doc.place.into_iter().all(f64::is_finite)||!doc.point_px.is_finite()) {return Err("Non-finite document placement".into());}
        let identity=|(doc,key):&(usize,String)| *doc<count && !key.is_empty();
        if self.current_layer.as_ref().is_some_and(|key|!identity(key))
            ||self.hidden.iter().chain(&self.locked).chain(&self.groups).any(|key|!identity(key))
            ||self.colors.iter().chain(self.edge_colors.as_deref().unwrap_or_default()).any(|(doc,key,_)|*doc>=count||key.is_empty()) {
            return Err("Metadata owner is outside the document inventory".into());
        }
        let mut keys=std::collections::HashSet::new();
        for (key,label,_) in &self.texts {
            if key.is_empty()||!keys.insert(key) {return Err("Text records need distinct source keys".into());}
            crate::text::validate_label(label).map_err(|e|e.to_string())?;
        }
        Ok(())
    }
}

