use wasm_bindgen::{JsCast,JsValue};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name=DecompressionStream)]
    type Unpack;
    #[wasm_bindgen(constructor,js_class="DecompressionStream",catch)]
    fn new(format:&str)->Result<Unpack,JsValue>;
}

pub async fn decode(bytes:Vec<u8>,encoding:&crate::source_encoding::Encoding,signal:&web_sys::AbortSignal)->Result<Vec<u8>,String> {
    if signal.aborted() {return Err("Source decoding cancelled".into());}
    if !bytes.starts_with(&[0x1f,0x8b]) {encoding.check(bytes.len())?;return Ok(bytes);}
    if encoding.name!="gzip" {return Err("Packed source lacks its gzip manifest declaration".into());}
    let parts=js_sys::Array::of1(&js_sys::Uint8Array::from(bytes.as_slice()));let blob=web_sys::Blob::new_with_u8_array_sequence(&parts).map_err(describe)?;
    let unpack=Unpack::new("gzip").map_err(describe)?;let stream=blob.stream().pipe_through(unpack.unchecked_ref());
    let reader=stream.get_reader().dyn_into::<web_sys::ReadableStreamDefaultReader>().map_err(|error|describe(error.into()))?;
    let limit=encoding.decoded.ok_or("Missing decoded source size")?;let result=read(&reader,limit,signal).await;
    if result.is_err() {let _=reader.cancel();}reader.release_lock();let bytes=result?;encoding.check(bytes.len())?;Ok(bytes)
}
fn describe(error:JsValue)->String {format!("Source decompression failed: {error:?}")}
async fn read(reader:&web_sys::ReadableStreamDefaultReader,limit:usize,signal:&web_sys::AbortSignal)->Result<Vec<u8>,String> {
    let mut bytes=Vec::new();loop {
        if signal.aborted() {return Err("Source decoding cancelled".into());}
        let part=JsFuture::from(reader.read()).await.map_err(describe)?;
        if js_sys::Reflect::get(&part,&"done".into()).map_err(describe)?.as_bool()==Some(true) {break;}
        let chunk=js_sys::Reflect::get(&part,&"value".into()).map_err(describe)?.dyn_into::<js_sys::Uint8Array>().map_err(describe)?;
        if chunk.length() as usize>limit-bytes.len() {return Err("Decoded source exceeds its declared byte size".into());}
        bytes.extend_from_slice(&chunk.to_vec());
    }Ok(bytes)
}
