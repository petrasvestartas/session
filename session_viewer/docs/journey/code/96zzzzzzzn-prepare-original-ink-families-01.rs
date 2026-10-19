 pub fn bytes(&self)->usize{self.parts.capacity()*std::mem::size_of::<(u8,Range<u32>)>()}
 pub fn visible(&self,count:u32)