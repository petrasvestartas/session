    let mut background = Background::default();
    let mut camera = Camera::default();
    present(&surface, &renderer, &background, &camera.uniform())?;
    let controls = document.get_element_by_id("controls").ok_or("Missing controls")?;
    let click = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let Some(target) = event.target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok()) else { return; };
        match target.id().as_str() {
            "background" => background.toggle(),
            "zoom-in" => camera.zoom(2.0),
            "zoom-out" => camera.zoom(0.5),
            "left" => camera.pan(-0.25, 0.0),
            "right" => camera.pan(0.25, 0.0),
            "reset" => camera = Camera::default(),
            _ => return,
        }
        if let Err(error) = present(&surface, &renderer, &background, &camera.uniform()) {
            report(&format!("Cannot redraw: {error:?}"));
        }
    });
    controls.add_event_listener_with_callback("click", click.as_ref().unchecked_ref())?;
    controls.remove_attribute("disabled")?;
