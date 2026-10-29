use crate::{math_quality::{self as quality, Quality, Projection}, scene::{Scene,ObjectId}};
use session_rust::{NurbsCurve,Xform};
use std::rc::Rc;

#[derive(Clone)]
enum Source {
    Curve(Rc<NurbsCurve>),
}
enum Prepared {
    Curve(crate::chain::PreparedChain),
}
impl Prepared {
    fn upload_bytes(&self)->usize {
        match self {
            Self::Curve(prepared)=>prepared.points.len().saturating_mul(256),
        }
    }
}
struct Request { id:ObjectId, source:Source, model:Xform }
struct Entry { key:(u32,u8,usize), _source:Source, target:Quality, accepted:Quality, rejected:Option<Quality>, error:Option<&'static str>, data:Option<Prepared>, bytes:usize }

#[derive(Default)]
pub struct Presentation {
    entries:Vec<Entry>, dirty:bool, scheduled:bool,
    pub limited:Option<&'static str>, pub bytes:usize, pub generations:u64, pub uploaded:bool,
}

impl Source {
    fn key(&self,id:ObjectId)->(u32,u8,usize) {
        match self {
            Self::Curve(source)=>(Scene::math_id(id),0,Rc::as_ptr(source) as usize),
        }
    }
    fn projection(&self,projection:&mut Projection)->Result<(), &'static str> {
        match self {
            Self::Curve(source)=>projection.curve(source),
            
        }
    }
    fn initial_chord(&self,current:Quality)->Option<f64> {
        if current.0.is_finite() {return None;}
        None
    }
    fn reserve(&self)->usize {
        match self {
            Self::Curve(_)=>2*1024*1024,
        }
    }
    fn prepare(&self,requested:Quality)->Result<Prepared,&'static str> {
        quality::using(requested,||match self {
            Self::Curve(source)=>Ok(Prepared::Curve(crate::chain::PreparedChain::curve(source.clone())?)),
            
            
        })
    }
}

fn requests(scene:&Scene)->Result<Vec<Request>,&'static str> {
    if scene.objects().len()+scene.paths().len()+scene.lines().len()+scene.points().len()>1024 {
        return Err("Presentation row count exceeds the refinement budget");
    }
    let display_units=scene.objects().iter().fold(0usize,|n,row|n.saturating_add(row.mesh.vertices().len()).saturating_add(row.mesh.indices().len()))
        .saturating_add(scene.paths().iter().map(|row|row.prepared.points.len()).sum::<usize>());
    let names=scene.objects().iter().map(|row|row.guid.len()).sum::<usize>()
        .saturating_add(scene.paths().iter().map(|row|row.guid.len()).sum::<usize>());
    if display_units.saturating_mul(256)>16*1024*1024 || names>256*1024 {
        return Err("Scene upload scratch exceeds the refinement budget");
    }
    let mut out=Vec::new();
    for row in scene.paths() {
        if let crate::chain::Source::Curve(source,_)=&row.prepared.source {
            if out.len()>=128 {return Err("Mathematical source count exceeds the refinement budget");}
            out.push(Request{id:row.id,source:Source::Curve(source.clone()),model:row.model.clone()});
        }
    }
    
    Ok(out)
}

impl Presentation {
    pub fn qualities(&self)->Vec<(u32,Option<f64>,Option<f64>,bool)> {
        self.entries.iter().map(|entry|(entry.key.0,entry.target.0.is_finite().then_some(entry.target.0),entry.accepted.0.is_finite().then_some(entry.accepted.0),entry.error.is_some())).collect()
    }
    pub fn invalidate(&mut self,scene:&Scene) {
        self.dirty=true;
        match requests(scene) {
            Ok(requests)=>self.entries.retain(|entry|requests.iter().any(|request|request.source.key(request.id)==entry.key)),
            Err(error)=>{self.entries.clear();self.limited=Some(error);}
        }
        self.bytes=self.entries.iter().map(|entry|entry.bytes).sum();
    }

    #[cfg(target_arch="wasm32")]
    pub fn upload(&mut self,renderer:&mut crate::renderer::Renderer,scene:&Scene,selected:Option<ObjectId>,fault:crate::gpu_fault::Fault) {
        self.invalidate(scene);
        crate::browser_upload::prepare(renderer,scene,selected,fault);
    }

    pub fn scene(&self,source:&Scene)->Scene {
        let mut view=source.clone();
        for row in view.math_paths_mut() {
            if let crate::chain::Source::Curve(source,_)=&row.prepared.source {
                let key=(Scene::math_id(row.id),0,Rc::as_ptr(source) as usize);
                if let Some(Prepared::Curve(prepared))=self.entries.iter().find(|entry|entry.key==key).and_then(|entry|entry.data.as_ref()) {
                    row.prepared=prepared.clone();
                }
            }
        }
        
        view
    }

    pub fn refresh(&mut self,renderer:&mut crate::renderer::Renderer,source:&Scene,camera:&crate::camera::Camera,size:[u32;2],selected:Option<ObjectId>)->bool {
        self.refresh_projection(renderer,source,&camera.view_projection(),size,selected)
    }
    fn refresh_projection(&mut self,renderer:&mut crate::renderer::Renderer,source:&Scene,matrix:&Xform,size:[u32;2],selected:Option<ObjectId>)->bool {
        self.uploaded=false;
        self.limited=None;
        let requests=match requests(source) {Ok(rows)=>rows,Err(error)=>{self.limited=Some(error);return false;}};
        self.entries.retain(|entry|requests.iter().any(|request|request.source.key(request.id)==entry.key));
        self.bytes=self.entries.iter().map(|entry|entry.bytes).sum();
        let mut attempted=false;let mut pending=false;let mut changed=false;
        for request in requests {
            let key=request.source.key(request.id);
            let index=match self.entries.iter().position(|entry|entry.key==key) {
                Some(index)=>index,
                None=>{self.entries.push(Entry{key,_source:request.source.clone(),target:Quality::default(),accepted:Quality::default(),rejected:None,error:None,data:None,bytes:0});self.entries.len()-1}
            };
            let entry=&self.entries[index];
            let mut projection=Projection::from_matrix(matrix,&request.model,size);
            let requested=match request.source.projection(&mut projection).and_then(|()|projection.quality(entry.accepted)) {
                Ok(value)=>value,Err(error)=>{self.entries[index].error=Some(error);self.limited=Some(error);continue;}
            };
            self.entries[index].target=requested;
            let entry=&self.entries[index];
            if requested==entry.accepted {self.entries[index].error=None;continue;}
            if entry.rejected==Some(requested) {self.limited=entry.error;continue;}
            if attempted {pending=true;continue;}
            attempted=true;
            let reserve=request.source.reserve();
            if self.bytes.saturating_add(reserve)>quality::SCENE_BYTES {
                let error="Presentation cache reached its finite byte budget";
                self.entries[index].rejected=Some(requested);self.entries[index].error=Some(error);self.limited=Some(error);continue;
            }
            let mut effective=requested;let mut result=Err("Requested camera quality is unavailable");
            let initial_chord=request.source.initial_chord(self.entries[index].accepted);
            for _ in 0..4 {
                if initial_chord.is_some_and(|chord|!chord.is_finite()||effective.0>chord) {
                    result=Err("Initial CAD refinement exceeds the existing absolute chord");break;
                }
                result=request.source.prepare(effective).and_then(|data|{
                    let retained=self.entries.iter().filter(|entry|entry.key!=key).filter_map(|entry|entry.data.as_ref()).fold(0usize,|n,data|n.saturating_add(data.upload_bytes()));
                    let baseline=source.objects().iter().fold(0usize,|n,row|n.saturating_add(row.mesh.vertices().len()).saturating_add(row.mesh.indices().len()))
                        .saturating_add(source.paths().iter().map(|row|row.prepared.points.len()).sum::<usize>()).saturating_mul(256);
                    if baseline.saturating_add(retained).saturating_add(data.upload_bytes())>quality::TRANSIENT_BYTES {Err("Combined scene upload exceeds the transient budget")}else{Ok(data)}
                });
                if result.is_ok() {break;}
                effective=Quality(effective.0*2.);
            }
            match result {
                Ok(data) if effective.0<self.entries[index].accepted.0||requested.0>=self.entries[index].accepted.0=>{
                    let error=(effective!=requested).then_some("Requested camera quality exceeds the bounded presentation; using a coarser display");
                    self.bytes=self.bytes-self.entries[index].bytes+reserve;
                    self.entries[index]=Entry{key,_source:request.source.clone(),target:requested,accepted:effective,rejected:(effective!=requested).then_some(requested),error,data:Some(data),bytes:reserve};
                    self.limited=error.or(self.limited);
                    changed=true;self.generations+=1;
                }
                Ok(_)=>{let error="Requested camera quality exceeds the bounded presentation; retaining the finer existing display";self.entries[index].rejected=Some(requested);self.entries[index].error=Some(error);self.limited=Some(error);}
                Err(error)=>{self.entries[index].rejected=Some(requested);self.entries[index].error=Some(error);self.limited=Some(error);}
            }
        }
        if changed || self.dirty {
            let view=self.scene(source);
            renderer.set_scene(&view,selected);
            self.dirty=false;self.uploaded=true;
        }
        pending
    }

    #[cfg(target_arch="wasm32")]
    pub fn schedule(&mut self,window:&web_sys::Window,canvas:&web_sys::HtmlCanvasElement) {
        use wasm_bindgen::JsCast;
        if self.scheduled {return;}
        self.scheduled=true;let canvas=canvas.clone();
        let callback=wasm_bindgen::closure::Closure::once_into_js(move || {
            if let Ok(event)=web_sys::Event::new("viewer-math") {let _=canvas.dispatch_event(&event);}
        });
        if window.request_animation_frame(callback.unchecked_ref()).is_err() {self.scheduled=false;}
    }
    pub fn event(&mut self) {self.scheduled=false;}
    pub fn drain_matrix(&mut self,renderer:&mut crate::renderer::Renderer,scene:&Scene,matrix:&[f32;16],size:[u32;2],selected:Option<ObjectId>)->Result<(),&'static str> {
        let mut projection=Xform::identity();projection.m=matrix.map(f64::from);
        for _ in 0..=128 {if !self.refresh_projection(renderer,scene,&projection,size,selected) {return self.limited.map_or(Ok(()),Err);}}
        Err("Presentation refinement did not finish within its bounded pass count")
    }
    pub fn drain(&mut self,renderer:&mut crate::renderer::Renderer,scene:&Scene,camera:&crate::camera::Camera,size:[u32;2],selected:Option<ObjectId>)->Result<(),&'static str> {
        for _ in 0..=128 {if !self.refresh(renderer,scene,camera,size,selected) {return self.limited.map_or(Ok(()),Err);}}
        Err("Presentation refinement did not finish within its bounded pass count")
    }
}
