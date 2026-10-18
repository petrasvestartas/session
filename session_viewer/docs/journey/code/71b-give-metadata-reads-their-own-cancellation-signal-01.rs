
struct Read {
    external:Option<(web_sys::AbortSignal,Closure<dyn FnMut()>)>,
    key:u64,
    controller:web_sys::AbortController,
