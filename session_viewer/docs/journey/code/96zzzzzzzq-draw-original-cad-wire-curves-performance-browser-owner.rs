    let frames = crate::browser_frame::Frame::new(fault.clone(), &renderer.queue);
    let owned_frames = frames.clone();
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if active_fault.message().is_some() { frames.close(); return; }
        if frames.take(&event) {
            let result = present_latest(&browser_window, &pointer_canvas, &surface, &mut config,
                &mut renderer, &mut shared.borrow_mut(), &mut panel, sample_budget, sample_forced);
            frames.submitted(&renderer.queue);
            if let Err(error) = result { report(&format!("Cannot redraw: {error:?}")); }
            return;
        }
