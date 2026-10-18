impl Drop for Read {
    fn drop(&mut self) {
        if let Some((signal,callback))=self.external.take() {let _=signal.remove_event_listener_with_callback("abort",callback.as_ref().unchecked_ref());}
        REQUESTS.with(|requests|requests.borrow_mut().remove(&self.key));self.controller.abort();
        if let Some(window)=web_sys::window() {window.clear_timeout_with_handle(self.timer);}
