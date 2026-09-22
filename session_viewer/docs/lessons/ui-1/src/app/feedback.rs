/// Show a message in the status line.
pub fn status(message: &str) {
    // an empty message shows the reload notice, if any
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

    // --8<-- [start:step-1]
    #[cfg(target_arch = "wasm32")]
    super::ui::MODEL.with_borrow_mut(|model| model.status = message.chars().take(256).collect());
    // --8<-- [end:step-1]
    log::info!("{message}");
}

/// Show the error panel with a reload button.
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

// --8<-- [start:step-1b]
/// Open or close the command line.
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
// --8<-- [end:step-1b]

/// Give the canvas keyboard focus.
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

/// No canvas on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn focus_canvas() {}

// --8<-- [start:step-1c]
/// No command line on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn command_line(_open: bool) {}

/// One row of the layers panel, as the panel needs it.
#[derive(Clone)]
pub struct LayerRow {
// --8<-- [end:step-1c]
    pub key: String,                 // unique id of the row
    pub label: String,               // text shown
    pub count: usize,                // objects under it
    pub hidden: bool,                // eye toggled off
}

// --8<-- [start:step-1d]
/// Replace the rows of the layers panel.
#[cfg(target_arch = "wasm32")]
pub fn layers_panel(rows: &[LayerRow]) {
    super::ui::MODEL.with_borrow_mut(|model| model.rows = rows.to_vec());
}

/// Show or hide the layers panel.
#[cfg(target_arch = "wasm32")]
pub fn layers_visible(open: bool) {
    super::ui::MODEL.with_borrow_mut(|model| {
        model.layers_open = open;

        if !open {
            model.rows.clear();
        }
    });
}

/// Whether the layers panel is open.
#[cfg(target_arch = "wasm32")]
pub fn layers_open() -> bool {
    super::ui::MODEL.with_borrow(|model| model.layers_open)
}
// --8<-- [end:step-1d]

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn layers_panel(_rows: &[LayerRow]) {}

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn layers_visible(_open: bool) {}

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn layers_open() -> bool {
    false
}
