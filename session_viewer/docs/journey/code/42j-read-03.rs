pub fn wake() {
    if let (Some(window), Ok(event)) = (web_sys::window(), web_sys::Event::new("viewer-pick")) {
        let _ = window.dispatch_event(&event);
    }
}
