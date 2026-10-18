}

pub trait ByteSize {fn bytes(&self)->usize;}
impl ByteSize for Vec<u8> {fn bytes(&self)->usize {self.capacity()}}

pub struct Held<T> {pub value:T,charge:Charge}
impl<T> Held<T> {pub fn bytes(&self)->usize {self.charge.bytes()}}
struct Entry<T> {value:Rc<Held<T>>,touch:u64}
pub struct Cache<K:Ord,T> {budget:Rc<Budget>,entries:BTreeMap<K,Entry<T>>,clock:u64}
impl<K:Ord+Clone,T:ByteSize> Cache<K,T> {
    pub fn new(budget:Rc<Budget>)->Self {Self {budget,entries:BTreeMap::new(),clock:0}}
    pub fn get(&mut self,key:&K)->Option<Rc<Held<T>>> {
        self.clock=self.clock.checked_add(1)?;let entry=self.entries.get_mut(key)?;entry.touch=self.clock;Some(entry.value.clone())
    }
    pub fn oldest(&self)->Option<K> {self.entries.iter().min_by_key(|(_,entry)|entry.touch).map(|(key,_)|key.clone())}
    pub fn remove(&mut self,key:&K)->bool {self.entries.remove(key).is_some()}
    pub fn clear(&mut self) {self.entries.clear();}
    pub fn keys(&self)->Vec<K> {self.entries.keys().cloned().collect()}
    pub fn len(&self)->usize {self.entries.len()}
    pub fn used(&self)->usize {self.budget.used()}
}
