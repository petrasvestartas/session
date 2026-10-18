# 20.7.3 · Reload scenes through the existing publication owner

**Chapter 20 · Shared GPU rendering · Section 20.7 · Draw slots and primitive lookup** · [Contents](contents.md)

**Typing: 29–57 minutes.** [Estimate](typing-load.md).

The reader already adopts complete live scenes and clears their history. Add the current reload_scene entry point instead of another loader. Update its stable control, cancel obsolete work and begin the existing candidate transaction. An empty manifest releases the document.

## Type

Continue from [Bind the own-row slot for ordinary surfaces](96zzv-bind-the-own-row-slot-for-ordinary-surfaces.md). [Save or recover your work](recovery.md).

### 1. `src/publication_route.rs`

Share named-scene resolution between startup and explicit reload. Reuse the existing data-base rules. Use the same path rule at startup without a second algorithm.

<details>
<summary>Locate the existing block</summary>

```rust
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn named_routes_keep_bucket_relative_files_and_refuse_parent_traversal() {
        assert!(!safe_scene("../outside"));assert!(!safe_scene("folder/../outside"));assert!(!safe_scene("https://other/scene"));assert!(!safe_scene("/scene"));
        let route=select(Some("view_floor"),"/",None,None,None,None).unwrap();assert_eq!(route.manifest,format!("{DATA_BASE}scenes/view_floor.yaml"));assert_eq!(join(&route.base,"pb/floor.pb"),format!("{DATA_BASE}pb/floor.pb"));
        let route=select(Some("fixtures/scene.yaml"),"/",Some("off"),Some("off"),None,None).unwrap();assert_eq!(route.manifest,"fixtures/scene.yaml");assert_eq!(join(&route.base,"./fixtures/a.pb"),"fixtures/a.pb");assert_eq!(path_scene("/view_floor"),Some("view_floor"));assert!(path_scene("/index.html").is_none());assert!(select(None,"/",Some("off"),None,None,None).is_none());
    }
    #[test]
    fn live_routes_keep_local_directories_poll_bounds_and_explicit_notification_urls() {
        let route=select(None,"/",Some("off"),Some("http://localhost:8770/live/scene.yaml"),Some("0.1"),Some("off")).unwrap();assert_eq!(route.base,"http://localhost:8770/live/");assert_eq!(route.poll_ms,Some(5000));assert!(route.notify.is_none());
        let route=select(Some("view_floor"),"/",None,Some("https://example.test/live.yaml"),Some("1"),Some("https://example.test/events")).unwrap();assert_eq!(route.poll_ms,Some(1000));assert_eq!(route.notify.as_deref(),Some("https://example.test/events"));assert_eq!(route.base,"https://example.test/");
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const DATA_BASE:&str="https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/";
pub const DEFAULT_SOURCE:&str="https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/scenes/view_live.yaml";

#[derive(Clone)]
pub struct Route {pub manifest:String,pub base:String,pub poll_ms:Option<i32>,pub notify:Option<String>}
pub fn local(url:&str)->bool {url.starts_with("http://localhost:")||url.starts_with("http://127.0.0.1:")||url.starts_with("http://[::1]:")}
pub fn safe_scene(value:&str)->bool {!value.is_empty()&&!value.starts_with('/')&&!value.contains("//")&&!value.contains(':')&&!value.split('/').any(|segment|segment=="..")}
pub fn path_scene(path:&str)->Option<&str> {let last=path.rsplit('/').next()?;(!last.is_empty()&&!last.ends_with(".html")&&!last.contains(':')&&!last.starts_with('.')).then_some(last)}
pub fn join(base:&str,file:&str)->String {if file.starts_with("https://")||file.starts_with("http://") {file.into()} else {format!("{base}{}",file.trim_start_matches("./"))}}
pub fn directory(url:&str)->String {url.rsplit_once('/').map_or_else(String::new,|(prefix,_)|format!("{prefix}/"))}

fn data_base(data:Option<&str>)->String {match data {Some("off"|"")=>String::new(),Some(value) if value.starts_with("https://")||local(value)=>{if value.ends_with('/') {value.into()} else {format!("{value}/")}},_=>DATA_BASE.into()}}
pub fn named(path:&str,data:Option<&str>)->Route {
 let base=data_base(data);let mut path=if path.contains('.') {path.to_owned()} else {format!("{path}.yaml")};
 if !path.contains('/') {path=format!("scenes/{path}");}
 Route {manifest:join(&base,&path),base,poll_ms:None,notify:None}
}

pub fn select(scene:Option<&str>,path:&str,data:Option<&str>,live:Option<&str>,poll:Option<&str>,notify:Option<&str>)->Option<Route> {
    let base=data_base(data);
    let named=scene.filter(|value|safe_scene(value)).or_else(||path_scene(path));
    let watched=match live {Some("off"|"0")=>None,Some(value) if value.starts_with("https://")||local(value)=>Some(value.to_owned()),Some(_)=>Some(DEFAULT_SOURCE.into()),None if named==Some("view_live")=>Some(DEFAULT_SOURCE.into()),_=>None};
    if let Some(manifest)=watched {
        let source_base=if !base.is_empty()&&manifest.starts_with(&base) {base} else {directory(&manifest)};
        let seconds=poll.and_then(|v|v.parse::<f64>().ok()).filter(|v|v.is_finite()&&*v>=1.).unwrap_or(5.);
        let notify=match notify {Some("off"|"0")=>None,Some(value) if value.starts_with("https://")||local(value)=>Some(value.into()),_=>None};
        return Some(Route {manifest,base:source_base,poll_ms:Some((seconds*1000.).min(i32::MAX as f64) as i32),notify});
    }
    Some(self::named(named?,data))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn named_routes_keep_bucket_relative_files_and_refuse_parent_traversal() {
        assert!(!safe_scene("../outside"));assert!(!safe_scene("folder/../outside"));assert!(!safe_scene("https://other/scene"));assert!(!safe_scene("/scene"));
        let route=select(Some("view_floor"),"/",None,None,None,None).unwrap();assert_eq!(route.manifest,format!("{DATA_BASE}scenes/view_floor.yaml"));assert_eq!(join(&route.base,"pb/floor.pb"),format!("{DATA_BASE}pb/floor.pb"));
        let route=select(Some("fixtures/scene.yaml"),"/",Some("off"),Some("off"),None,None).unwrap();assert_eq!(route.manifest,"fixtures/scene.yaml");assert_eq!(join(&route.base,"./fixtures/a.pb"),"fixtures/a.pb");assert_eq!(path_scene("/view_floor"),Some("view_floor"));assert!(path_scene("/index.html").is_none());assert!(select(None,"/",Some("off"),None,None,None).is_none());
    }
    #[test]
    fn live_routes_keep_local_directories_poll_bounds_and_explicit_notification_urls() {
        let route=select(None,"/",Some("off"),Some("http://localhost:8770/live/scene.yaml"),Some("0.1"),Some("off")).unwrap();assert_eq!(route.base,"http://localhost:8770/live/");assert_eq!(route.poll_ms,Some(5000));assert!(route.notify.is_none());
        let route=select(Some("view_floor"),"/",None,Some("https://example.test/live.yaml"),Some("1"),Some("https://example.test/events")).unwrap();assert_eq!(route.poll_ms,Some(1000));assert_eq!(route.notify.as_deref(),Some("https://example.test/events"));assert_eq!(route.base,"https://example.test/");
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-1.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-2.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-3.rs"
```

</details>

### 2. `src/browser_publication.rs`

Keep one stable publication slot for browser callbacks. Let callback clones observe the current owner through the same slot. Retain a stable control slot even when startup has no scene route. Reuse one publication state constructor for startup and explicit replacement. Keep route-less reload calls as no-ops, matching the current entry point. Keep the existing callback identity when a scene is replaced. Read the latest state through the stable weak control slot. Cancel the old reader and move its replacement into the same slot.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    timer:Option<i32>,tick:Option<Closure<dyn FnMut()>>,
    notify:Option<web_sys::EventSource>,message:Option<Closure<dyn FnMut(web_sys::MessageEvent)>>,
}
pub struct Publication(Option<Rc<RefCell<State>>>);
#[derive(Clone)]
pub struct Control(Option<Weak<RefCell<State>>>);
impl Publication {
    pub fn new(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault)->Result<Self,JsValue> {
        let window=web_sys::window().ok_or("No browser window")?;let url=web_sys::Url::new(&window.location().href()?)?;let q=url.search_params();
        let scene=q.get("scene");let data=q.get("data");let live=q.get("live");let poll=q.get("poll");let notify=q.get("notify");
        let Some(route)=crate::publication_route::select(scene.as_deref(),&window.location().pathname()?,data.as_deref(),live.as_deref(),poll.as_deref(),notify.as_deref()) else {return Ok(Self(None));};
        let index=Index::new(route.base.clone());Ok(Self(Some(Rc::new(RefCell::new(State {active:true,inflight:false,again:false,pending_sheets:Vec::new(),sheets:Vec::new(),pending_streams:Vec::new(),streams:Vec::new(),cancelled:Rc::new(Cell::new(false)),gate:Default::default(),abort:None,route,index:Some(index),document,fault,delivery:None,last_error:None,timer:None,tick:None,notify:None,message:None})))))
    }
    pub fn control(&self)->Control {Control(self.0.as_ref().map(Rc::downgrade))}
}
impl Drop for Publication {fn drop(&mut self) {self.control().cancel();}}
impl Control {
    fn state(&self)->Option<Rc<RefCell<State>>> {self.0.as_ref()?.upgrade()}
    pub fn begin(&self) {
        let Some(state)=self.state() else {return;};let Some(window)=web_sys::window() else {return;};
        let route=state.borrow().route.clone();
```

</details>

Replace it with:

```rust
    timer:Option<i32>,tick:Option<Closure<dyn FnMut()>>,
    notify:Option<web_sys::EventSource>,message:Option<Closure<dyn FnMut(web_sys::MessageEvent)>>,
}
type Slot=RefCell<Option<Rc<RefCell<State>>>>;
pub struct Publication(Rc<Slot>);
#[derive(Clone)]
pub struct Control(Weak<Slot>);
impl Publication {
    pub fn new(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault)->Result<Self,JsValue> {
        let window=web_sys::window().ok_or("No browser window")?;let url=web_sys::Url::new(&window.location().href()?)?;let q=url.search_params();
        let scene=q.get("scene");let data=q.get("data");let live=q.get("live");let poll=q.get("poll");let notify=q.get("notify");
        let Some(route)=crate::publication_route::select(scene.as_deref(),&window.location().pathname()?,data.as_deref(),live.as_deref(),poll.as_deref(),notify.as_deref()) else {return Ok(Self(Rc::new(RefCell::new(None))));};
        Ok(Self::from_route(document,fault,route))
    }
    pub fn at(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault,path:&str)->Result<Self,JsValue>{
        let window=web_sys::window().ok_or("No browser window")?;let url=web_sys::Url::new(&window.location().href()?)?;
        Ok(Self::from_route(document,fault,crate::publication_route::named(path,url.search_params().get("data").as_deref())))
    }
    fn from_route(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault,route:Route)->Self{
        let index=Index::new(route.base.clone());Self(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(State {active:true,inflight:false,again:false,pending_sheets:Vec::new(),sheets:Vec::new(),pending_streams:Vec::new(),streams:Vec::new(),cancelled:Rc::new(Cell::new(false)),gate:Default::default(),abort:None,route,index:Some(index),document,fault,delivery:None,last_error:None,timer:None,tick:None,notify:None,message:None}))))))
    }
    pub fn replace(&mut self,next:Self){
        self.control().cancel();*self.0.borrow_mut()=next.0.borrow_mut().take();
    }
    pub fn has_route(&self)->bool{self.0.borrow().is_some()}
    pub fn control(&self)->Control {Control(Rc::downgrade(&self.0))}
}
impl Drop for Publication {fn drop(&mut self) {self.control().cancel();}}
impl Control {
    fn state(&self)->Option<Rc<RefCell<State>>> {self.0.upgrade()?.borrow().clone()}
    pub fn begin(&self) {
        let Some(state)=self.state() else {return;};let Some(window)=web_sys::window() else {return;};
        let route=state.borrow().route.clone();
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{publication_index::Index,publication_route::Route};
use std::{cell::{Cell,RefCell},collections::HashSet,rc::{Rc,Weak}};
use wasm_bindgen::{JsCast,JsValue,closure::Closure};

struct State {
    active:bool,inflight:bool,again:bool,
    pending_sheets:Vec<crate::sheet_stream::Cursor>,sheets:Vec<Weak<crate::sheet_source::Prepared>>,
    pending_streams:Vec<crate::stream_load::Cursor>,streams:Vec<std::rc::Weak<crate::range_cloud::Prepared>>,cancelled:Rc<Cell<bool>>,
    gate:crate::read_gate::ReadGate,
    abort:Option<web_sys::AbortController>,
    route:Route,index:Option<Index>,
    document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault,
    delivery:Option<Result<Box<crate::scene::Scene>,String>>,last_error:Option<String>,
    timer:Option<i32>,tick:Option<Closure<dyn FnMut()>>,
    notify:Option<web_sys::EventSource>,message:Option<Closure<dyn FnMut(web_sys::MessageEvent)>>,
}
type Slot=RefCell<Option<Rc<RefCell<State>>>>;
pub struct Publication(Rc<Slot>);
#[derive(Clone)]
pub struct Control(Weak<Slot>);
impl Publication {
    pub fn new(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault)->Result<Self,JsValue> {
        let window=web_sys::window().ok_or("No browser window")?;let url=web_sys::Url::new(&window.location().href()?)?;let q=url.search_params();
        let scene=q.get("scene");let data=q.get("data");let live=q.get("live");let poll=q.get("poll");let notify=q.get("notify");
        let Some(route)=crate::publication_route::select(scene.as_deref(),&window.location().pathname()?,data.as_deref(),live.as_deref(),poll.as_deref(),notify.as_deref()) else {return Ok(Self(Rc::new(RefCell::new(None))));};
        Ok(Self::from_route(document,fault,route))
    }
    pub fn at(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault,path:&str)->Result<Self,JsValue>{
        let window=web_sys::window().ok_or("No browser window")?;let url=web_sys::Url::new(&window.location().href()?)?;
        Ok(Self::from_route(document,fault,crate::publication_route::named(path,url.search_params().get("data").as_deref())))
    }
    fn from_route(document:crate::drawing_document::Shared,fault:crate::gpu_fault::Fault,route:Route)->Self{
        let index=Index::new(route.base.clone());Self(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(State {active:true,inflight:false,again:false,pending_sheets:Vec::new(),sheets:Vec::new(),pending_streams:Vec::new(),streams:Vec::new(),cancelled:Rc::new(Cell::new(false)),gate:Default::default(),abort:None,route,index:Some(index),document,fault,delivery:None,last_error:None,timer:None,tick:None,notify:None,message:None}))))))
    }
    pub fn replace(&mut self,next:Self){
        self.control().cancel();*self.0.borrow_mut()=next.0.borrow_mut().take();
    }
    pub fn has_route(&self)->bool{self.0.borrow().is_some()}
    pub fn control(&self)->Control {Control(Rc::downgrade(&self.0))}
}
impl Drop for Publication {fn drop(&mut self) {self.control().cancel();}}
impl Control {
    fn state(&self)->Option<Rc<RefCell<State>>> {self.0.upgrade()?.borrow().clone()}
    pub fn begin(&self) {
        let Some(state)=self.state() else {return;};let Some(window)=web_sys::window() else {return;};
        let route=state.borrow().route.clone();
        if let Some(ms)=route.poll_ms {
            let weak=Rc::downgrade(&state);let callback=Closure::<dyn FnMut()>::new(move ||{if let Some(state)=weak.upgrade() {poll(state);}});
            match window.set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(),ms) {Ok(id)=>{let mut state=state.borrow_mut();state.timer=Some(id);state.tick=Some(callback);},Err(error)=>crate::browser::report(&format!("Cannot schedule live polling: {error:?}"))}
            if let Some(url)=route.notify {if let Ok(source)=web_sys::EventSource::new(&url) {let weak=Rc::downgrade(&state);let callback=Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |_|{if let Some(state)=weak.upgrade() {poll(state);}});source.set_onmessage(Some(callback.as_ref().unchecked_ref()));let mut state=state.borrow_mut();state.notify=Some(source);state.message=Some(callback);}}
        }
        poll(state);
    }
    pub fn cancel(&self) {
        let Some(state)=self.state() else {return;};let mut state=state.borrow_mut();state.active=false;state.cancelled.set(true);for source in state.streams.drain(..).filter_map(|s|s.upgrade()) {source.generation.borrow_mut().close();}for source in state.sheets.drain(..).filter_map(|s|s.upgrade()) {if let Some(stream)=&source.streamed {stream.generation.borrow_mut().close();}}state.pending_sheets.clear();state.pending_streams.clear();crate::stream::reset_range_gate();state.gate.cancel();state.delivery=None;state.index=None;
        if let Some(abort)=state.abort.take() {abort.abort();}if let Some(id)=state.timer.take() {if let Some(window)=web_sys::window() {window.clear_interval_with_handle(id);}}
        if let Some(source)=state.notify.take() {source.set_onmessage(None);source.close();}state.message=None;state.tick=None;
    }
    pub fn take(&self)->Result<Option<crate::editor::Action>,String> {
        let Some(state)=self.state() else {return Ok(None);};let mut owner=state.borrow_mut();if !owner.active {owner.delivery=None;return Ok(None);}
        let scene=owner.delivery.take().transpose()?;
        if let Some(next)=scene.as_ref() {
            owner.streams.retain(|weak|weak.upgrade().is_some_and(|source| {if next.streams().iter().any(|row|Rc::ptr_eq(&row.prepared,&source)) {true} else {source.generation.borrow_mut().close();crate::range_fetch::cancel_url(&source.source.url);false}}));
            owner.sheets.retain(|weak|weak.upgrade().is_some_and(|source| {if next.sheets().iter().any(|row|Rc::ptr_eq(&row.prepared,&source)) {true} else {if let Some(stream)=&source.streamed {stream.generation.borrow_mut().close();crate::range_fetch::cancel_url(&stream.source.url);}false}}));
            let sheet_cursors=std::mem::take(&mut owner.pending_sheets);
            let cursors=std::mem::take(&mut owner.pending_streams);let cancelled=owner.cancelled.clone();drop(owner);
            for cursor in sheet_cursors {let weak=Rc::downgrade(&state);let current=Rc::new(move |source:&Rc<crate::sheet_source::Prepared>|weak.upgrade().is_some_and(|state| {let owner=state.borrow();owner.active&&crate::browser_runtime::owns(&owner.fault)&&owner.fault.message().is_none()&&owner.document.borrow().scene.sheets().iter().any(|row|Rc::ptr_eq(&row.prepared,source))}));let changed=Rc::new(|| {if let Some(window)=web_sys::window() {if let Ok(event)=web_sys::Event::new("viewer-stream-page") {let _=window.dispatch_event(&event);}}});let error=Rc::new(|message:String|crate::browser::report(&format!("Drawing page read stopped: {message}")));crate::sheet_stream::continue_read(cursor,current,changed,error,cancelled.clone());}
            for cursor in cursors {
                let weak=Rc::downgrade(&state);let current=Rc::new(move |source:&Rc<crate::range_cloud::Prepared>|weak.upgrade().is_some_and(|state| {let owner=state.borrow();owner.active&&crate::browser_runtime::owns(&owner.fault)&&owner.fault.message().is_none()&&owner.document.borrow().scene.streams().iter().any(|row|Rc::ptr_eq(&row.prepared,source))}));
                let changed=Rc::new(|| {if let Some(window)=web_sys::window() {if let Ok(event)=web_sys::Event::new("viewer-stream-page") {let _=window.dispatch_event(&event);}}});
                let error=Rc::new(|message:String|crate::browser::report(&format!("Source page read stopped: {message}")));
                crate::stream_load::continue_read(cursor,current,changed,error,cancelled.clone());
            }
        }
        Ok(scene.map(crate::editor::Action::Live))
    }

}

fn poll(state:Rc<RefCell<State>>) {
    let mut owner=state.borrow_mut();if !owner.active||!crate::browser_runtime::owns(&owner.fault)||owner.fault.message().is_some() {return;}
    if owner.inflight {owner.again=true;return;}
    let Ok(abort)=web_sys::AbortController::new() else {return;};let Ok(ticket)=owner.gate.begin() else {return;};let Some(mut index)=owner.index.take() else {return;};
    owner.inflight=true;owner.abort=Some(abort.clone());let route=owner.route.clone();drop(owner);
    wasm_bindgen_futures::spawn_local(async move {
        let result=read(&route,&mut index,&abort.signal()).await;
        let mut owner=state.borrow_mut();let current=owner.active&&owner.gate.finish(ticket)&&crate::browser_runtime::owns(&owner.fault)&&owner.fault.message().is_none();
        owner.inflight=false;owner.abort=None;if !current {return;}
        let outcome=result.and_then(|(cursors,sheets)| {for cursor in &sheets {owner.sheets.push(Rc::downgrade(&cursor.prepared));}owner.pending_sheets.extend(sheets);for cursor in &cursors {owner.streams.push(Rc::downgrade(&cursor.prepared));}owner.pending_streams.extend(cursors);index.replacement(&owner.document.borrow().scene)});owner.index=Some(index);
        let changed=match outcome {Ok(Some(scene))=>{owner.last_error=None;owner.delivery=Some(Ok(Box::new(scene)));true},Ok(None)=>false,Err(error)=>{if owner.last_error.as_ref()==Some(&error) {false} else {let _=crate::browser_report::observe("publication",&error);owner.last_error=Some(error.clone());owner.delivery=Some(Err(error));true}}};
        let again=std::mem::take(&mut owner.again);drop(owner);
        if changed {if let Some(window)=web_sys::window() {if let Ok(event)=web_sys::Event::new("viewer-publication") {let _=window.dispatch_event(&event);}}}
        if again {poll(state);}
    });
}

async fn read(route:&Route,index:&mut Index,signal:&web_sys::AbortSignal)->Result<(Vec<crate::stream_load::Cursor>,Vec<crate::sheet_stream::Cursor>),String> {
    let mut sheets=Vec::new();let mut segments=index.streamed_segments();let mut cursors=Vec::new();let mut points=index.streamed_points();let budget=crate::page_cache::Budget::new(crate::stream_budget::PAGE_BYTES);
    let started=crate::browser_phase::now();let mut response=crate::publication_fetch::get(&route.manifest,index.etag(&route.manifest),4*1024*1024,signal).await?;
    if response.status==404&&route.manifest.ends_with(".toml") {let yaml=format!("{}.yaml",route.manifest.trim_end_matches(".toml"));response=crate::publication_fetch::get(&yaml,index.etag(&route.manifest),4*1024*1024,signal).await?;}
    match response.status {304 if index.has_manifest()=>{},200..=299=>{index.manifest(&route.manifest,response.etag,&response.bytes)?;crate::browser_phase::finish("manifest",started,response.bytes.len() as u64,&route.manifest);},status=>return Err(format!("Manifest HTTP {status}"))}
    let mut seen=HashSet::new();
    for url in index.urls() {
        if !seen.insert(url.clone())||crate::manifest::immutable_key(&url)&&index.cached(&url) {continue;}
        let encoding=index.encoding(&url)?;
        if encoding.name.is_empty()&&url.split('?').next().is_some_and(|path|path.ends_with(".pb")) {
            if let Some(probe)=crate::stream::probe(&url).await {
                if crate::sheet_range::identity(&probe.bytes).is_some() {
                    let source=crate::sheet_range::Source::from_probe(url.clone(),&probe).await?;
                    if source.fields.count>crate::sheet_stream::PREFIX {
                        if index.cached(&url)&&source.fields.revision.is_some()&&index.etag(&url)==source.fields.revision {continue;}if signal.aborted() {return Err("Drawing source read cancelled".into());}segments=segments.saturating_sub(index.source_segments(&url));let remaining=crate::sheet_stream::CEILING.saturating_sub(segments);let cursor=crate::sheet_stream::start(Rc::new(source),budget.clone(),remaining).await?;if signal.aborted() {cursor.prepared.streamed.as_ref().unwrap().generation.borrow_mut().close();return Err("Drawing source read cancelled".into());}segments=segments.saturating_add(cursor.from);index.stream_sheet(&url,cursor.prepared.clone())?;sheets.push(cursor);continue;
                    }
                }
                if crate::range_shape::single_cloud(&probe.bytes)&&crate::stream::cloud_identity(&probe.bytes).is_some() {
                    let source=crate::range_source::Source::from_probe(url.clone(),&probe).await?;
                    if source.fields.count>crate::stream_budget::PREFIX_POINTS||source.fields.coords_len>=crate::stream_budget::MIN_BYTES {
                        if index.cached(&url)&&source.fields.revision.is_some()&&index.etag(&url)==source.fields.revision {continue;}
                        if signal.aborted() {return Err("Source read cancelled".into());}
                        points=points.saturating_sub(index.source_points(&url));
                        let remaining=crate::stream_budget::MAX_POINTS.saturating_sub(points);
                        if remaining==0 {return Err("Manifest sources exceed the point ceiling".into());}
                        let cursor=crate::stream_load::start(Rc::new(source),budget.clone(),remaining).await?;
                        if signal.aborted() {cursor.prepared.generation.borrow_mut().close();return Err("Source read cancelled".into());}
                        points=points.saturating_add(cursor.loaded);index.stream(&url,cursor.prepared.clone())?;cursors.push(cursor);continue;
                    }
                }
            }
        }
        let response=crate::publication_fetch::get(&url,index.etag(&url),crate::archive::LIMIT,signal).await?;
        match response.status {
            304 if index.cached(&url)=>{},
            200..=299=>{let started=crate::browser_phase::now();let bytes=crate::source_unpack::decode(response.bytes,&encoding,signal).await?;index.source(&url,response.etag,&bytes)?;crate::browser_phase::finish("decode/validation/display",started,bytes.len() as u64,&url);},
            status=>{index.forget(&url);return Err(format!("Source HTTP {status}: {url}"));}
        }
    }
    let extra=crate::stream_budget::MAX_POINTS.saturating_sub(points)/(cursors.len().max(1) as u32);for cursor in &mut cursors {cursor.ceiling=cursor.loaded+extra;}
    let extra=crate::sheet_stream::CEILING.saturating_sub(segments)/(sheets.len().max(1) as u32);for cursor in &mut sheets {cursor.ceiling=cursor.from.saturating_add(extra);}
    Ok((cursors,sheets))
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-4.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-5.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-6.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-7.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-8.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-9.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-10.rs"
```

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-11.rs"
```

</details>

### 3. `src/browser_runtime.rs`

Export the current replacement entry point and start reading after the runtime borrow ends.

<details>
<summary>Locate the existing block</summary>

```rust
    ACTIVE.with(|slot| slot.borrow_mut().take()).is_some()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn runtime_running() -> bool { ACTIVE.with(|slot| slot.borrow().is_some()) }
```

</details>

After that block, type:

```rust

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn reload_scene(url:Option<String>)->Result<(),wasm_bindgen::JsValue>{
    let control=ACTIVE.with(|slot| {
        let mut active=slot.borrow_mut();let runtime=active.as_mut().ok_or("Viewer runtime is not running")?;
        let publication=match url {Some(path)=>crate::browser_publication::Publication::at(runtime.document.clone(),runtime.fault.clone(),&path)?,None=>crate::browser_publication::Publication::new(runtime.document.clone(),runtime.fault.clone())?};
        if !publication.has_route(){return Ok(None);}
        runtime.request.borrow_mut().cancel();runtime.reload.borrow_mut().cancel();
        runtime._publication.replace(publication);let control=runtime._publication.control();
        Ok::<_,wasm_bindgen::JsValue>(Some(control))
    })?;
    if let Some(control)=control{control.begin();}Ok(())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/reload_complete_tests.rs`

Supply actual history, original-owner and manifest-cache release assertions.

<details>
<summary>Locate the existing block</summary>

```text
ly.complete(&mut e).unwrap_err(), "Network failed");
    assert_eq!(e.scene.objects()[0].model.m, model); assert!(e.scene.objects()[0].geometry().is_none());
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-13.rs" skip=160
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/publication_route.rs`

Supply current absolute, nested and bare named-scene routing checks.

<details>
<summary>Locate the existing block</summary>

```text
t_eq!(route.poll_ms,Some(1000));assert_eq!(route.notify.as_deref(),Some("https://example.test/events"));assert_eq!(route.base,"https://example.test/");
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzw-reload-scenes-through-the-existing-publication-owner-14.rs" skip=160
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.7.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for reload scenes through the existing publication owner.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



reload_scene → publication owner → validated candidate → Live adoption.



Why start the read after releasing the runtime borrow?

The publication checks which runtime owns its device. Starting it while that runtime is mutably borrowed would reenter the same RefCell.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Create `empty.yaml` containing `items: []` and temporarily add `<link data-trunk rel="copy-file" href="empty.yaml">` to `index.html`. Call `window.wasmBindings.reload_scene(new URL("empty.yaml",location.href).href)` in the browser console. Compare valid, malformed and empty manifests. Empty reload must clear the drawing and its Undo history.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.7.3
npm --prefix ../session_tests run course -- save 20.7.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This reuses full source preparation and current named-scene routing. Browser acceptance must prove cancellation and release; it does not establish the remaining winit and recovery parity.

Actual Chrome at DPR 1/2 checks scene candidates, unchanged drawing on malformed input, route-less calls, obsolete-read cancellation, empty-manifest release, cleared Undo/Redo, original URL revocation and zero retained mesh CPU/surface GPU ownership. The retained slot and palette pixel checks run at DPR 1.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.7.3
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
