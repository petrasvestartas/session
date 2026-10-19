#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Ticket {generation:u64,request:u64}
#[derive(Default)]
pub struct Generation {generation:u64,issued:u64,pending:std::collections::BTreeSet<u64>,active:bool}
impl Generation {
    pub fn start(&mut self)->Result<(),&'static str> {
        self.active=false;self.pending.clear();self.generation=self.generation.checked_add(1).ok_or("Stream generations exhausted")?;self.active=true;Ok(())
    }
    pub fn request(&mut self)->Result<Ticket,&'static str> {
        if !self.active {return Err("Stream source is closed");}
        self.issued=self.issued.checked_add(1).ok_or("Stream requests exhausted")?;self.pending.insert(self.issued);Ok(Ticket {generation:self.generation,request:self.issued})
    }
    pub fn current(&self,ticket:Ticket)->bool {self.active&&ticket.generation==self.generation&&self.pending.contains(&ticket.request)}
    pub fn finish(&mut self,ticket:Ticket)->bool {if !self.current(ticket) {return false;}self.pending.remove(&ticket.request)}
    pub fn close(&mut self) {self.active=false;self.pending.clear();}
    pub fn pending(&self)->usize {self.pending.len()}
}