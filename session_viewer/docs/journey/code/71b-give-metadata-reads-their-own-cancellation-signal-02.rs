}
impl Read {
    fn new(url:&str,signal:Option<&web_sys::AbortSignal>)->Result<Self,String> {
        let controller=web_sys::AbortController::new().map_err(|error|describe(error.into()))?;
        let key=NEXT.with(|next| {let key=next.get().checked_add(1).ok_or("Range request IDs exhausted")?;next.set(key);Ok::<_,String>(key)})?;
        let stalled=Rc::new(Cell::new(false));let owned=controller.clone();let flag=stalled.clone();
        let callback=Closure::<dyn FnMut()>::new(move || {flag.set(true);owned.abort();});
        let external=if let Some(signal)=signal {let owned=controller.clone();let callback=Closure::<dyn FnMut()>::new(move ||owned.abort());signal.add_event_listener_with_callback("abort",callback.as_ref().unchecked_ref()).map_err(describe)?;if signal.aborted() {controller.abort();}Some((signal.clone(),callback))} else {None};
        let mut read=Self {external,key,controller,timer:0,stalled,callback};read.touch()?;
        REQUESTS.with(|requests|requests.borrow_mut().insert(key,(url.to_owned(),read.controller.clone())));Ok(read)
    }
