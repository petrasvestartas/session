
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjectId(u32);

impl ObjectId { pub fn value(self) -> u32 { self.0 } }

#[derive(Clone)]
pub struct Object {
