pub const DATA_BASE:&str="https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/";
pub const DEFAULT_SOURCE:&str="https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/scenes/view_live.yaml";

#[derive(Clone)]
pub struct Route {pub manifest:String,pub base:String,pub poll_ms:Option<i32>,pub notify:Option<String>}
pub fn local(url:&str)->bool {url.starts_with("http://localhost:")||url.starts_with("http://127.0.0.1:")||url.starts_with("http://[::1]:")}
pub fn safe_scene(value:&str)->bool {!value.is_empty()&&!value.starts_with('/')&&!value.contains("//")&&!value.contains(':')&&!value.split('/').any(|segment|segment=="..")}
pub fn path_scene(path:&str)->Option<&str> {let last=path.rsplit('/').next()?;(!last.is_empty()&&!last.ends_with(".html")&&!last.contains(':')&&!last.starts_with('.')).then_some(last)}
pub fn join(base:&str,file:&str)->String {if file.starts_with("https://")||file.starts_with("http://") {file.into()} else {format!("{base}{}",file.trim_start_matches("./"))}}
pub fn directory(url:&str)->String {url.rsplit_once('/').map_or_else(String::new,|(prefix,_)|format!("{prefix}/"))}

pub fn select(scene:Option<&str>,path:&str,data:Option<&str>,live:Option<&str>,poll:Option<&str>,notify:Option<&str>)->Option<Route> {
    let base=match data {Some("off"|"")=>String::new(),Some(value) if value.starts_with("https://")||local(value)=>{if value.ends_with('/') {value.into()} else {format!("{value}/")}},_=>DATA_BASE.into()};
    let named=scene.filter(|value|safe_scene(value)).or_else(||path_scene(path));
    let watched=match live {Some("off"|"0")=>None,Some(value) if value.starts_with("https://")||local(value)=>Some(value.to_owned()),Some(_)=>Some(DEFAULT_SOURCE.into()),None if named==Some("view_live")=>Some(DEFAULT_SOURCE.into()),_=>None};
    if let Some(manifest)=watched {
        let source_base=if !base.is_empty()&&manifest.starts_with(&base) {base} else {directory(&manifest)};
        let seconds=poll.and_then(|v|v.parse::<f64>().ok()).filter(|v|v.is_finite()&&*v>=1.).unwrap_or(5.);
        let notify=match notify {Some("off"|"0")=>None,Some(value) if value.starts_with("https://")||local(value)=>Some(value.into()),_=>None};
        return Some(Route {manifest,base:source_base,poll_ms:Some((seconds*1000.).min(i32::MAX as f64) as i32),notify});
    }
    let name=named?;let mut name=if name.contains('.') {name.to_owned()} else {format!("{name}.yaml")};if !name.contains('/') {name=format!("scenes/{name}");}
    Some(Route {manifest:join(&base,&name),base,poll_ms:None,notify:None})
}
