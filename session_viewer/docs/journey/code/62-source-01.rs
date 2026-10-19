use crate::text::{TextDocument,TextLabel,TextPlacement};
use std::rc::Rc;

#[derive(Clone,Debug,PartialEq,serde::Serialize)]
pub struct Source {pub guid:String,pub name:String,pub visible:bool,pub locked:bool,pub label:TextLabel}
#[derive(Clone)]
pub struct Prepared {pub source:Rc<Source>,pub extent:[f64;2]}
impl Prepared {
 pub fn new(source:Rc<Source>)->Result<Self,&'static str> {
  if matches!(source.label.placement,TextPlacement::Screen {..}) {return Err("Screen presentation text has no scene row");}
  if source.label.object.is_some() {return Err("Original text does not carry a viewer pick address");}
  let mut document=TextDocument::new();document.set_labels(vec![source.label.clone()]).map_err(|_|"Invalid scene text source")?;
  let extent=document.runs[0].buffer.layout_runs().fold([0f64;2],|[w,h],line|[w.max(line.line_w as f64),h.max((line.line_top+line.line_height) as f64)]);
  Ok(Self {source,extent})
 }
}
pub fn example(placement:TextPlacement)->Source {let mut label=crate::text_example::label();label.text="Original CAD · Ø25 · Ω".into();label.color=[255;4];label.placement=placement;Source {guid:uuid::Uuid::new_v4().to_string(),name:"Original CAD label".into(),visible:true,locked:false,label}}
