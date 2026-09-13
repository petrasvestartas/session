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
    #[cfg(target_arch = "wasm32")]
    super::ui::MODEL.with_borrow_mut(|model| model.status = message.chars().take(256).collect());
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

#[cfg(target_arch = "wasm32")]
pub fn command_line(open: bool) {
    super::ui::MODEL.with_borrow_mut(|model| {
        model.command_open = open;
        model.focus_command = open;
        if open {
            model.command.clear();
        }
    });
}

/// Give the canvas the keyboard back. Every key binding is on the canvas, so anything that
/// takes the focus - a click in a panel, a closing text box - has to hand it back or the
/// viewer stops answering keys with no way to say so.
#[cfg(target_arch = "wasm32")]
pub fn focus_canvas() {
    use wasm_bindgen::JsCast;
    if let Some(document) = web_sys::window().and_then(|w| w.document())
        && let Some(canvas) = document.get_element_by_id("canvas")
        && let Ok(canvas) = canvas.dyn_into::<web_sys::HtmlElement>()
    {
        let _ = canvas.focus();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn focus_canvas() {}

/// Native builds have no command box; the callers stay free of `cfg`.
#[cfg(not(target_arch = "wasm32"))]
pub fn command_line(_open: bool) {}

/// One row of the layers panel, as the panel needs it.
#[derive(Clone)]
pub struct LayerRow {
    pub key: String,
    pub label: String,
    pub count: usize,
    pub hidden: bool,
}

#[cfg(target_arch = "wasm32")]
pub fn layers_panel(rows: &[LayerRow]) {
    super::ui::MODEL.with_borrow_mut(|model| model.rows = rows.to_vec());
}

#[cfg(target_arch = "wasm32")]
pub fn layers_visible(open: bool) {
    super::ui::MODEL.with_borrow_mut(|model| {
        model.layers_open = open;
        if !open {
            model.rows.clear();
        }
    });
}

#[cfg(target_arch = "wasm32")]
pub fn layers_open() -> bool {
    super::ui::MODEL.with_borrow(|model| model.layers_open)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn layers_panel(_rows: &[LayerRow]) {}

/// Native builds have no panel either; the one caller stays free of `cfg`.
#[cfg(not(target_arch = "wasm32"))]
pub fn layers_visible(_open: bool) {}

#[cfg(not(target_arch = "wasm32"))]
pub fn layers_open() -> bool {
    false
}
