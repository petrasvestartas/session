    Surface(Rc<session_rust::NurbsSurface>),
}
enum Prepared {
    Curve(crate::chain::PreparedChain),
    Surface(Rc<crate::surface::Prepared>,Rc<crate::mesh::Mesh>),
}
impl Prepared {
    fn upload_bytes(&self)->usize {
        match self {
            Self::Curve(prepared)=>prepared.points.len().saturating_mul(256),
            Self::Surface(_,mesh)=>mesh.vertices().len().saturating_add(mesh.indices().len()).saturating_mul(256),
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
            Self::Surface(source)=>(Scene::math_id(id),1,Rc::as_ptr(source) as usize),
        }
    }
    fn projection(&self,projection:&mut Projection)->Result<(), &'static str> {
        match self {
            Self::Curve(source)=>projection.curve(source),
            Self::Surface(source)=>projection.surface(source),
            
        }
    }
    fn initial_chord(&self,current:Quality)->Option<f64> {
        if current.0.is_finite() {return None;}
        None
    }
    fn reserve(&self)->usize {
        match self {
            Self::Curve(_)=>2*1024*1024,
            Self::Surface(_)=>quality::OBJECT_BYTES,
        }
    }
    fn prepare(&self,requested:Quality)->Result<Prepared,&'static str> {
        quality::using(requested,||match self {
            Self::Curve(source)=>Ok(Prepared::Curve(crate::chain::PreparedChain::curve(source.clone())?)),
            
            Self::Surface(source)=>{
                let prepared=Rc::new(crate::surface::Prepared::new(source.clone())?);
                let mesh=Rc::new(prepared.mesh()?.display);Ok(Prepared::Surface(prepared,mesh))
            }
            
            
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
    
    for row in scene.objects() {
        if let Some(surface)=&row.surface {
            if out.len()>=128 {return Err("Mathematical source count exceeds the refinement budget");}
            out.push(Request{id:row.id,source:Source::Surface(surface.source.clone()),model:row.model.clone()});
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
        
        for row in view.objects_mut() {
            if let Some(surface)=&row.surface {
                let key=(Scene::math_id(row.id),1,Rc::as_ptr(&surface.source) as usize);
                if let Some(Prepared::Surface(prepared,mesh))=self.entries.iter().find(|entry|entry.key==key).and_then(|entry|entry.data.as_ref()) {
                    row.surface=Some(prepared.clone());row.mesh=mesh.clone();
                }
            }
            
