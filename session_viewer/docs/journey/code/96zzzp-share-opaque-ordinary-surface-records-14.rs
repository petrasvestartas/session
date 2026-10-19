 pub(crate) fn matches(&self,row:u32,record:&Record)->bool {self.previous.get(row as usize)==Some(record)}
 pub fn bind(&self,pass: