use std::{cell::Cell,collections::BTreeMap,rc::Rc};

pub struct Budget {limit:usize,used:Cell<usize>}
impl Budget {
    pub fn new(limit:usize)->Rc<Self> {Rc::new(Self {limit,used:Cell::new(0)})}
    pub fn used(&self)->usize {self.used.get()}
    pub fn available(&self)->usize {self.limit-self.used.get()}
    pub fn reserve(self:&Rc<Self>,bytes:usize)->Result<Charge,&'static str> {
        if bytes>self.available() {return Err("Page memory budget exhausted");}
        self.used.set(self.used.get()+bytes);Ok(Charge {budget:self.clone(),bytes})
    }
}
pub struct Charge {budget:Rc<Budget>,bytes:usize}
impl Charge {
    pub fn resize(&mut self,bytes:usize)->Result<(),&'static str> {
        if bytes>self.bytes {
            let extra=bytes-self.bytes;if extra>self.budget.available() {return Err("Page memory budget exhausted");}
            self.budget.used.set(self.budget.used.get()+extra);
        } else {self.budget.used.set(self.budget.used.get()-(self.bytes-bytes));}
        self.bytes=bytes;Ok(())
    }
    pub fn bytes(&self)->usize {self.bytes}
}
impl Drop for Charge {
    fn drop(&mut self) {self.budget.used.set(self.budget.used.get()-self.bytes);}
}

