impl Control {
    fn state(&self)->Option<Rc<RefCell<State>>> {self.0.as_ref()?.upgrade()}
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
        let Some(state)=self.state() else {return;};let mut state=state.borrow_mut();state.active=false;state.gate.cancel();state.delivery=None;state.index=None;
