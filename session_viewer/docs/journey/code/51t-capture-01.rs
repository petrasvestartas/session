    pub fn commit(&self,scene:&mut Scene,history:&mut crate::history::History) -> Result<bool,&'static str> {match self {Self::Whole(p)=>p.commit(scene,history),Self::Part(p)=>p.commit(scene,history)}}
}

pub struct Edit { pub gizmo:Gizmo,pub preview:Preview,pub pointer:Option<i32>,start:[f32;2],moved:bool }
impl Edit {
    pub fn begin(mut gizmo:Gizmo,handle:Handle,ray:&crate::camera::Ray,preview:Preview,pointer:i32,screen:[f32;2]) -> Option<Self> {
        gizmo.begin(handle,&ray.origin,&ray.direction)?;
        Some(Self {gizmo,preview,pointer:Some(pointer),start:screen,moved:false})
    }
    pub fn update(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],ray:&crate::camera::Ray,shift:bool) -> Result<bool,&'static str> {
        if self.pointer!=Some(pointer) {return Ok(false);}
        let distance=((screen[0]-self.start[0]) as f64*size[0]/2.).hypot((screen[1]-self.start[1]) as f64*size[1]/2.);
        self.moved|=distance>4.;
        if !self.moved {return Ok(true);}
        let drag=self.gizmo.drag.as_ref().ok_or("No handle drag is running")?;
        if let Some(transform)=self.gizmo.update(drag,&ray.origin,&ray.direction,shift) {self.preview.set(transform)?;}
        Ok(true)
    }
    pub fn click(&mut self) -> bool {
        if self.moved {return false;}
        self.gizmo.typing=self.gizmo.drag.take().map(|d|d.handle);self.pointer=None;true
    }
    pub fn active(&self)->Option<Handle> {self.gizmo.drag.as_ref().map(|d|d.handle).or(self.gizmo.typing)}
    pub fn origin(&self)->[f64;3] {
        let origin=if matches!(self.active(),Some(Handle::Translate(_))) {{ let mut matrix=Xform::identity(); matrix.m=*self.preview.matrix(); matrix.transform_point(&self.gizmo.origin) }} else {self.gizmo.origin.clone()};
        [origin[0],origin[1],origin[2]]
    }
}

pub fn code(handle:Option<Handle>)->f32 {
    use crate::gizmo::Axis;
    let axis=|a|match a {Axis::X=>0.,Axis::Y=>1.,Axis::Z=>2.};
    match handle {Some(Handle::Translate(a))=>axis(a),Some(Handle::Rotate(a))=>3.+axis(a),Some(Handle::Scale(a))=>6.+axis(a),Some(Handle::ScaleUniform)=>9.,None=>-1.}
}
