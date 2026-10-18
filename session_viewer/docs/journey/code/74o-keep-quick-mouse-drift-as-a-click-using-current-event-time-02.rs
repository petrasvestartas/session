        else if delta==[0.;2] {None} else {Some(if drag.pan {Motion::Pan(delta)} else {Motion::Orbit(delta)})}
    }
    pub fn release(&mut self,id:i32,position:[f64;2])->Option<Motion> {self.release_at(id,position,200.)}
    pub fn release_at(&mut self,id:i32,position:[f64;2],now:f64)->Option<Motion> {
        if self.active.as_ref()?.id!=id {return None;}
        let motion=self.move_at(id,position,now);let drag=self.active.take()?;
        if position.iter().any(|v|!v.is_finite()) {return None;}
        if drag.button!=0 {self.repeat=drag.button==2&&!drag.moved;return motion;}
