    let mut background = Background::default();
    present(&surface, &renderer, &background)?;
    let button = document.get_element_by_id("background").ok_or("Missing button")?;
    let click = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
        background.toggle();
        if let Err(error) = present(&surface, &renderer, &background) {
            report(&format!("Cannot redraw: {error:?}"));
        }
    });
    button.add_event_listener_with_callback("click", click.as_ref().unchecked_ref())?;
    button.remove_attribute("disabled")?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Change the background. The triangle keeps its shape and colour.");
