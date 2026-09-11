//! Short viewer status and recoverable browser errors; messages use textContent, never HTML.

/// Show a non-disruptive message in the focused viewer's status area.
pub fn status(message: &str) {
    // A page that reloaded after a device loss keeps saying so whenever the line is cleared.
    #[cfg(target_arch = "wasm32")]
    let message = if message.is_empty() {
        super::route::recovered_notice().unwrap_or(message)
    } else {
        message
    };
    #[cfg(target_arch = "wasm32")]
    if let Some(window) = web_sys::window()
        && let Some(document) = window.document()
        && let Some(status) = document.get_element_by_id("viewer-status")
    {
        status.set_text_content(Some(message));
    }
    log::info!("{message}");
}

/// Preserve the page and expose an explicit reload action after initialization/device failure.
pub fn error(message: &str) {
    #[cfg(target_arch = "wasm32")]
    if let Some(window) = web_sys::window()
        && let Some(document) = window.document()
        && let Some(panel) = document.get_element_by_id("viewer-error")
    {
        if let Some(text) = document.get_element_by_id("viewer-error-message") {
            text.set_text_content(Some(message));
        }
        let _ = panel.remove_attribute("hidden");
    }
    log::error!("{message}");
}

/// Open or close the command line. Opening focuses it; closing hands the keyboard back to the
/// canvas, or a letter typed next would reach the viewer's key bindings instead of the box.
#[cfg(target_arch = "wasm32")]
pub fn command_line(open: bool) -> Option<web_sys::HtmlInputElement> {
    use wasm_bindgen::JsCast;
    let document = web_sys::window()?.document()?;
    let input: web_sys::HtmlInputElement = document
        .get_element_by_id("viewer-command")?
        .dyn_into()
        .ok()?;
    if open {
        input.set_hidden(false);
        input.set_value("");
        let _ = input.focus();
    } else {
        input.set_hidden(true);
        if let Some(canvas) = document.get_element_by_id("canvas")
            && let Ok(canvas) = canvas.dyn_into::<web_sys::HtmlElement>()
        {
            let _ = canvas.focus();
        }
    }
    Some(input)
}

/// The command line, when it is open.
#[cfg(target_arch = "wasm32")]
pub fn command_text() -> Option<String> {
    use wasm_bindgen::JsCast;
    let input: web_sys::HtmlInputElement = web_sys::window()?
        .document()?
        .get_element_by_id("viewer-command")?
        .dyn_into()
        .ok()?;
    (!input.hidden()).then(|| input.value())
}

/// Native builds have no command box; the callers stay free of `cfg`.
#[cfg(not(target_arch = "wasm32"))]
pub fn command_line(_open: bool) {}
