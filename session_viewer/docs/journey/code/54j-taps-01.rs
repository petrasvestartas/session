        if first>1.&&last>1. {if let Some(Action::Pick(screen))=Motion::Pick(after).action(rect) {actions.push(Action::ZoomAt {factor:(last/first).clamp(0.1,10.) as f32,screen});}}actions
    }
    pub fn release(&mut self,id:i32,at:[f64;2],time:f64,rect:[f64;4])->Vec<Action> {
        let mut actions=self.move_to(id,at,rect);let Some(index)=self.fingers.iter().position(|f|f.id==id) else {return actions;};
        let finger=self.fingers.remove(index);
        if !self.multiple&&time.is_finite()&&time>=finger.time&&time-finger.time<=300.&&(at[0]-finger.from[0]).hypot(at[1]-finger.from[1])<=12.&&!finger.moved {
            if self.tap.is_some_and(|(last,when)|time>=when&&time-when<=320.&&(at[0]-last[0]).hypot(at[1]-last[1])<=40.) {self.tap=None;actions.push(Action::Fit);}
            else {self.tap=Some((at,time));actions.extend(Motion::Pick(at).action(rect));}
        }
        if self.fingers.is_empty() {self.multiple=false;}actions
    }
    pub fn contains(&self,id:i32)->bool {self.fingers.iter().any(|f|f.id==id)}
    pub fn count(&self)->usize {self.fingers.len()}
