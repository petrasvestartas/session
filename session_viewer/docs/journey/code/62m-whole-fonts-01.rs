use std::rc::Rc;
use std::cell::{Cell,RefCell};
#[derive(Default)]
pub struct Inbox {pub asked:Cell<bool>,epoch:Cell<u64>,retry_after:Cell<f64>,pub reply:RefCell<Option<Result<Vec<Vec<u8>>,String>>>}
impl Inbox {
 pub fn clear(&self) {self.epoch.set(self.epoch.get().wrapping_add(1));self.asked.set(false);self.retry_after.set(0.);self.reply.borrow_mut().take();}
 pub fn failed(&self) {self.asked.set(false);self.retry_after.set(crate::load_measure::now()+1000.);}
 pub fn need(self:&Rc<Self>,words:impl IntoIterator<Item=impl AsRef<str>>) {
  if self.asked.get()||crate::load_measure::now()<self.retry_after.get()||words.into_iter().all(|s|crate::text::covers(s.as_ref())) {return;}
  #[cfg(target_arch="wasm32")]
  {self.asked.set(true);let inbox=Rc::clone(self);let epoch=self.epoch.get();wasm_bindgen_futures::spawn_local(async move {let reply=fetch().await;if inbox.epoch.get()!=epoch {return;}if reply.is_err() {inbox.failed();}*inbox.reply.borrow_mut()=Some(reply);crate::browser_pick::wake();});}
 }
}
#[cfg(target_arch="wasm32")]
async fn fetch()->Result<Vec<Vec<u8>>,String> {
 use wasm_bindgen::JsCast;
 let window=web_sys::window().ok_or("Missing browser window")?;let mut faces=Vec::new();
 for name in crate::text::FULL_FONTS {
  let response=wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(&format!("text/{name}"))).await.map_err(|e|format!("Cannot fetch {name}: {e:?}"))?.dyn_into::<web_sys::Response>().map_err(|_|"Invalid font response")?;
  if !response.ok() {return Err(format!("Font {name}: HTTP {}",response.status()));}
  let promise=response.array_buffer().map_err(|e|format!("Cannot read {name}: {e:?}"))?;let data=wasm_bindgen_futures::JsFuture::from(promise).await.map_err(|e|format!("Cannot read {name}: {e:?}"))?;let bytes=js_sys::Uint8Array::new(&data);
  if bytes.length()==0||bytes.length()>4*1024*1024 {return Err(format!("Font {name} exceeds the 4 MiB limit"));}faces.push(bytes.to_vec());
 }
 Ok(faces)
}
