}

struct Drag {started:f64,id:i32,button:i16,start:[f64;2],last:[f64;2],moved:bool,pan:bool}
#[derive(Default)]
pub struct Gesture {active:Option<Drag>,repeat:bool}
impl Gesture {
    pub fn press(&mut self,id:i32,button:i16,position:[f64;2])->bool {self.press_with(id,button,position,false)}
    pub fn press_with(&mut self,id:i32,button:i16,position:[f64;2],pan:bool)->bool {self.press_at(id,button,position,pan,0.)}
    pub fn press_at(&mut self,id:i32,button:i16,position:[f64;2],pan:bool,now:f64)->bool {
        if !now.is_finite()||now<0.||self.active.is_some()||!matches!(button,0|1|2)||position.iter().any(|v|!v.is_finite()) {return false;}
        self.repeat=false;self.active=Some(Drag {started:now,id,button,start:position,last:position,moved:false,pan:pan||button==1});true
    }
    pub fn move_to(&mut self,id:i32,position:[f64;2])->Option<Motion> {self.move_at(id,position,200.)}
    pub fn move_at(&mut self,id:i32,position:[f64;2],now:f64)->Option<Motion> {
        let drag=self.active.as_mut().filter(|d|d.id==id)?;
        if !now.is_finite()||now<drag.started||position.iter().any(|v|!v.is_finite()) {return None;}
        let moved=(position[0]-drag.start[0]).abs().max((position[1]-drag.start[1]).abs());
        let was=drag.moved;drag.moved|=if drag.button==0 {drags(moved,now-drag.started)}else{moved>4.};
        let from=if was {drag.last} else {drag.start};let delta=std::array::from_fn(|i|position[i]-from[i]);drag.last=position;
        if !drag.moved {return None;}
