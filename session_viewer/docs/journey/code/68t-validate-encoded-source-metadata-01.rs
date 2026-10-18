#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Encoding {pub name:String,pub decoded:Option<usize>}
impl Encoding {
    pub fn new(name:&str,size:u64)->Result<Self,String> {
        if !matches!(name,""|"gzip") {return Err(format!("Unsupported source encoding: {name}"));}
        let decoded=if name.is_empty() {None} else {let size=usize::try_from(size).map_err(|_|"Decoded source size overflow")?;if size==0||size>crate::archive::LIMIT {return Err("Decoded source size exceeds its file limit".into());}Some(size)};
        Ok(Self {name:name.into(),decoded})
    }
    pub fn check(&self,bytes:usize)->Result<(),String> {
        if bytes>crate::archive::LIMIT||self.decoded.is_some_and(|size|size!=bytes) {return Err("Decoded source size does not match its manifest".into());}Ok(())
    }
}