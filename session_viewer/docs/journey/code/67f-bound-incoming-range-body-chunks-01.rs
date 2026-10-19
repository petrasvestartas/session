pub fn retryable(error:&str)->bool {error.starts_with("network error")}

#[derive(Default)]
pub struct GetOpts {pub range:Option<(u64,u64)>,pub revalidate:bool}
pub struct Reply {pub status:u16,pub etag:Option<String>,pub total:Option<u64>,pub bytes:Vec<u8>}
impl Reply {
    pub fn whole(self)->Option<Vec<u8>> {(self.status==206&&self.total==Some(self.bytes.len() as u64)).then_some(self.bytes)}
}

async fn body(reader:&web_sys::ReadableStreamDefaultReader,limit:u64,read:&mut Read)->Result<Vec<u8>,String> {
    let mut bytes=Vec::new();loop {
        let part=JsFuture::from(reader.read()).await.map_err(|error|read.error(error))?;
        if js_sys::Reflect::get(&part,&"done".into()).map_err(|error|describe(error.into()))?.as_bool()==Some(true) {break;}
        let chunk=js_sys::Reflect::get(&part,&"value".into()).map_err(|error|describe(error.into()))?.dyn_into::<js_sys::Uint8Array>().map_err(|error|describe(error.into()))?;
        if u64::from(chunk.length())>limit-bytes.len() as u64 {return Err("Range response exceeds its byte limit".into());}
        bytes.extend_from_slice(&chunk.to_vec());read.touch()?;
    }Ok(bytes)
}
