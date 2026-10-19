use crate::{editor::Action,gesture::Motion};

struct Finger {id:i32,from:[f64;2],at:[f64;2],time:f64,moved:bool}
#[derive(Default)]
pub struct Touch {fingers:Vec<Finger>,multiple:bool,tap:Option<([f64;2],f64)>}
impl Touch {
    pub fn press(&mut self,id:i32,at:[f64;2],time:f64)->bool {
        if at.iter().any(|v|!v.is_finite())||!time.is_finite()||self.fingers.iter().any(|f|f.id==id) {return false;}
        self.multiple|=!self.fingers.is_empty();self.fingers.push(Finger {id,from:at,at,time,moved:false});true
    }
    pub fn move_to(&mut self,id:i32,at:[f64;2],rect:[f64;4])->Vec<Action> {
        if at.iter().any(|v|!v.is_finite()) {return Vec::new();}
        let Some(index)=self.fingers.iter().position(|f|f.id==id) else {return Vec::new();};
        let old=self.fingers.iter().map(|f|f.at).collect::<Vec<_>>();
        let finger=&mut self.fingers[index];let was=finger.moved;finger.at=at;finger.moved|=(at[0]-finger.from[0]).abs().max((at[1]-finger.from[1]).abs())>4.;
        if self.fingers.len()==1&&!self.multiple {
            if !self.fingers[0].moved {return Vec::new();}
            let from=if was {old[0]} else {self.fingers[0].from};let delta=std::array::from_fn(|i|at[i]-from[i]);return Motion::Orbit(delta).action(rect).into_iter().collect();
        }
        if self.fingers.len()!=2 {return Vec::new();}
        let centre=|a:[f64;2],b:[f64;2]|std::array::from_fn::<_,2,_>(|i|(a[i]+b[i])/2.);
        let before=centre(old[0],old[1]);let after=centre(self.fingers[0].at,self.fingers[1].at);
        let distance=|a:[f64;2],b:[f64;2]|(a[0]-b[0]).hypot(a[1]-b[1]);let first=distance(old[0],old[1]);let last=distance(self.fingers[0].at,self.fingers[1].at);
        let mut actions=Motion::Pan(std::array::from_fn(|i|after[i]-before[i])).action(rect).into_iter().collect::<Vec<_>>();
        if first>1.&&last>1. {if let Some(Action::Pick(screen))=Motion::Pick(after).action(rect) {actions.push(Action::ZoomAt {factor:(last/first).clamp(0.1,10.) as f32,screen});}}actions
    }
    pub fn contains(&self,id:i32)->bool {self.fingers.iter().any(|f|f.id==id)}
    pub fn count(&self)->usize {self.fingers.len()}
    pub fn cancel(&mut self) {self.fingers.clear();self.multiple=false;self.tap=None;}
}
