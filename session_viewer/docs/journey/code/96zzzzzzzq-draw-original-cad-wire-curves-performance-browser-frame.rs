use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast};

struct State {
    schedule: crate::frame_schedule::Schedule,
    fault: crate::gpu_fault::Fault,
    handle: Option<i32>,
    callback: Option<Closure<dyn FnMut(f64)>>,
    event: Option<web_sys::Event>,
}

impl State {
    fn cancel(&mut self) {
        if let (Some(handle), Some(window)) = (self.handle.take(), web_sys::window()) {
            let _ = window.cancel_animation_frame(handle);
        }
        self.event = None;
    }
}

impl Drop for State { fn drop(&mut self) { self.cancel(); } }

#[derive(Clone)]
pub struct Frame(Rc<RefCell<State>>);

impl Frame {
    pub fn new(fault: crate::gpu_fault::Fault, queue: &wgpu::Queue) -> Self {
        let frame = Self(Rc::new(RefCell::new(State {
            schedule: crate::frame_schedule::Schedule::after_submit(crate::report_lifecycle::permits()),
            fault, handle: None, callback: None, event: None,
        })));
        frame.submitted(queue);
        frame
    }

    fn current(&self) -> bool {
        let state = self.0.borrow();
        state.fault.message().is_none() && crate::browser_runtime::owns(&state.fault)
    }

    pub fn request(&self) {
        if !self.current() { self.close(); return; }
        let ticket = self.0.borrow_mut().schedule.request();
        self.schedule(ticket);
    }

    pub fn refresh(&self, allowed: bool) {
        let ticket = {
            let mut state = self.0.borrow_mut();
            if !allowed { state.cancel(); }
            state.schedule.allow(allowed)
        };
        self.schedule(ticket);
    }

    fn schedule(&self, ticket: Option<u64>) {
        let Some(ticket) = ticket else { return; };
        let weak = Rc::downgrade(&self.0);
        let callback = Closure::<dyn FnMut(f64)>::new(move |_| {
            let Some(state) = weak.upgrade() else { return; };
            let frame = Frame(state);
            if !frame.current() { frame.close(); return; }
            if !crate::report_lifecycle::permits() { frame.refresh(false); return; }
            let Ok(event) = web_sys::Event::new("viewer-tiles") else { frame.close(); return; };
            {
                let mut state = frame.0.borrow_mut();
                if !state.schedule.begin(ticket) { return; }
                state.handle = None;
                state.event = Some(event.clone());
            }
            if let Some(window) = web_sys::window() { let _ = window.dispatch_event(&event); }
            frame.0.borrow_mut().event = None;
        });
        let result = web_sys::window().ok_or_else(|| wasm_bindgen::JsValue::from_str("No browser window"))
            .and_then(|window| window.request_animation_frame(callback.as_ref().unchecked_ref()));
        match result {
            Ok(handle) => {
                let mut state = self.0.borrow_mut();
                state.handle = Some(handle);
                state.callback = Some(callback);
            }
            Err(error) => { self.close(); crate::browser::report(&format!("Cannot schedule frame: {error:?}")); }
        }
    }

    pub fn take(&self, event: &web_sys::Event) -> bool {
        let mut state = self.0.borrow_mut();
        if state.event.as_ref() != Some(event) { return false; }
        state.event = None;
        true
    }

    pub fn submitted(&self, queue: &wgpu::Queue) {
        let Some(ticket) = self.0.borrow().schedule.flight else { return; };
        let fault = self.0.borrow().fault.clone();
        queue.on_submitted_work_done(move || {
            crate::browser_runtime::complete_frame(&fault, ticket);
        });
    }

    pub fn complete(&self, ticket: u64) {
        if !self.current() { self.close(); return; }
        let ticket = self.0.borrow_mut().schedule.complete(ticket);
        self.schedule(ticket);
    }

    pub fn close(&self) {
        let mut state = self.0.borrow_mut();
        state.cancel();
        state.schedule.close();
    }
}
