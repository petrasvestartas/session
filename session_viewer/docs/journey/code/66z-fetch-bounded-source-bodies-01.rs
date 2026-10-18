use wasm_bindgen::{JsCast,JsValue};
use wasm_bindgen_futures::JsFuture;

pub struct Response {pub status:u16,pub etag:Option<String>,pub bytes:Vec<u8>}
pub async fn get(url:&str,etag:Option<String>,limit:usize,signal:&web_sys::AbortSignal)->Result<Response,String> {
    let window=web_sys::window().ok_or("No browser window")?;
    let init=web_sys::RequestInit::new();init.set_signal(Some(signal));init.set_cache(web_sys::RequestCache::NoCache);
    let headers=web_sys::Headers::new().map_err(|_|"Cannot create request headers")?;
    if let Some(etag)=etag {headers.set("If-None-Match",&etag).map_err(|_|"Cannot set HTTP validator")?;}init.set_headers(&headers);
    let started=crate::browser_phase::now();
    let response=JsFuture::from(window.fetch_with_str_and_init(url,&init)).await.map_err(|error|format!("Fetch failed: {error:?}"))?.dyn_into::<web_sys::Response>().map_err(|_|"Invalid fetch response")?;
    let status=response.status();let etag=response.headers().get("ETag").map_err(|_|"Cannot read HTTP validator")?;
    if status==304 {crate::browser_phase::finish("HTTP unchanged",started,0,url);return Ok(Response {status,etag,bytes:vec![]});}
    if !response.ok() {return Ok(Response {status,etag,bytes:vec![]});}
    if let Some(size)=response.headers().get("Content-Length").map_err(|_|"Cannot read source length")? {if size.parse::<u64>().map_err(|_|"Invalid source length")?>limit as u64 {return Err("Response exceeds its byte limit".into());}}
    let reader=response.body().ok_or("Source response has no body")?.get_reader().dyn_into::<web_sys::ReadableStreamDefaultReader>().map_err(|_|"Cannot read source stream")?;
    let result=read(&reader,limit).await;if result.is_err() {let _=JsFuture::from(reader.cancel()).await;}reader.release_lock();
    let bytes=result?;crate::browser_phase::finish("HTTP source",started,bytes.len() as u64,url);Ok(Response {status,etag,bytes})
}
async fn read(reader:&web_sys::ReadableStreamDefaultReader,limit:usize)->Result<Vec<u8>,String> {
    let mut bytes=Vec::new();loop {
        let part=JsFuture::from(reader.read()).await.map_err(|error|format!("Cannot read source: {error:?}"))?;
        if js_sys::Reflect::get(&part,&JsValue::from_str("done")).map_err(|_|"Invalid stream reply")?.as_bool()==Some(true) {break;}
        let chunk=js_sys::Reflect::get(&part,&JsValue::from_str("value")).map_err(|_|"Missing source chunk")?.dyn_into::<js_sys::Uint8Array>().map_err(|_|"Invalid source chunk")?;
        if chunk.length() as usize>limit-bytes.len() {return Err("Response exceeds its byte limit".into());}bytes.extend_from_slice(&chunk.to_vec());
    }Ok(bytes)
}
