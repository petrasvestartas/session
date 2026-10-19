pub(crate) fn observe(phase:crate::load_measure::Completed<'_>){
 #[cfg(target_arch="wasm32")] crate::browser_phase::completed(phase,"document");
 #[cfg(not(target_arch="wasm32"))] let _=phase;
}
pub(crate) fn prepare(bytes:&[u8],place:&Xform,point_px:f32)->Result<Source,String>{prepare_at(bytes,place,point_px,None)}
pub(crate) fn prepare_at(bytes:&[u8],place:&Xform,point_px:f32,location:Option<Rc<crate::reload_url::ReloadUrl>>)->Result<Source,String>{
 prepare_observed(bytes,place,point_px,location,&mut crate::load_measure::now,&mut observe)
}
pub(crate) fn prepare_observed(bytes:&[u8],place:&Xform,point_px:f32,location:Option<Rc<crate::reload_url::ReloadUrl>>,
 clock:&mut impl FnMut()->f64,observe:&mut impl FnMut(crate::load_measure::Completed<'_>))->Result<Source,String>{
 use crate::load_measure::run_result;
 let size=bytes.len()as u64;