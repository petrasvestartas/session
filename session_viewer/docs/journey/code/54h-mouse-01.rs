#[derive(Debug, PartialEq)]
pub enum Motion {
    Pick([f64;2]),Orbit([f64;2]),Pan([f64;2]),
    Probe {from:[f64;2],to:[f64;2]},Drag([f64;2]),Finish([f64;2]),
}

impl Motion {
    pub fn action(self,rect:[f64;4])->Option<Action> {
        let [left,top,width,height]=rect;
        if rect.iter().any(|v|!v.is_finite())||width<=0.||height<=0. {return None;}
        let screen=|[x,y]:[f64;2]|[(2.*(x-left)/width-1.) as f32,(1.-2.*(y-top)/height) as f32];let size=[width,height];
        Some(match self {
            Self::Pick(at)=>Action::Pick(screen(at)),
            Self::Orbit([dx,dy])=>Action::Orbit(-0.005*dx,-0.005*dy),
            Self::Pan(delta)=>Action::PanScreen {delta,size},
            Self::Probe {from,to}=>Action::CanvasStart {from:screen(from),to:screen(to),size,modified:false,pointer:0},
            Self::Drag(at)=>Action::CanvasMove {screen:screen(at),size},
            Self::Finish(at)=>Action::CanvasRelease {screen:screen(at),size,add:false,remove:false},
        })
    }
}

struct Drag {id:i32,button:i16,start:[f64;2],last:[f64;2],moved:bool,pan:bool}
#[derive(Default)]
pub struct Gesture {active:Option<Drag>,repeat:bool}
impl Gesture {
    pub fn press(&mut self,id:i32,button:i16,position:[f64;2])->bool {self.press_with(id,button,position,false)}
    pub fn press_with(&mut self,id:i32,button:i16,position:[f64;2],pan:bool)->bool {
        if self.active.is_some()||!matches!(button,0|1|2)||position.iter().any(|v|!v.is_finite()) {return false;}
        self.repeat=false;self.active=Some(Drag {id,button,start:position,last:position,moved:false,pan:pan||button==1});true
    }
    pub fn move_to(&mut self,id:i32,position:[f64;2])->Option<Motion> {
        let drag=self.active.as_mut().filter(|d|d.id==id)?;
        if position.iter().any(|v|!v.is_finite()) {return None;}
        let was=drag.moved;drag.moved|=(position[0]-drag.start[0]).abs().max((position[1]-drag.start[1]).abs())>4.;
        let from=if was {drag.last} else {drag.start};let delta=std::array::from_fn(|i|position[i]-from[i]);drag.last=position;
        if !drag.moved {return None;}
        if drag.button==0 {Some(if was {Motion::Drag(position)} else {Motion::Probe {from:drag.start,to:position}})}
        else if delta==[0.;2] {None} else {Some(if drag.pan {Motion::Pan(delta)} else {Motion::Orbit(delta)})}
    }
    pub fn release(&mut self,id:i32,position:[f64;2])->Option<Motion> {
        if self.active.as_ref()?.id!=id {return None;}
        let motion=self.move_to(id,position);let drag=self.active.take()?;
        if position.iter().any(|v|!v.is_finite()) {return None;}
        if drag.button!=0 {self.repeat=drag.button==2&&!drag.moved;return motion;}
        Some(if drag.moved {if matches!(motion,Some(Motion::Probe {..})) {motion.unwrap()} else {Motion::Finish(position)}} else {Motion::Pick(position)})
    }
    pub fn take_repeat(&mut self)->bool {std::mem::take(&mut self.repeat)}
    pub fn cancel(&mut self) {self.active=None;self.repeat=false;}
    pub fn cancel_pointer(&mut self,id:i32) {if self.active.as_ref().is_some_and(|d|d.id==id) {self.cancel();}}
}
