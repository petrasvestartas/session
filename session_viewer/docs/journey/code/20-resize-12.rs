        }
        if resize(&browser_window, &canvas, &surface, &mut config, &mut renderer, &mut editor) {
            if let Err(error) = present(&surface, &renderer, &editor.background, &editor.camera.uniform()) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        }
    });
    document.add_event_listener_with_callback("click", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("resize", update.as_ref().unchecked_ref())?;
    controls.remove_attribute("disabled")?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Canvas pixels, depth and camera resize together.");
