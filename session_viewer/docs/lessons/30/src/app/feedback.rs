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

    #[cfg(target_arch = "wasm32")]
    super::ui::MODEL.with_borrow_mut(|model| model.status = message.chars().take(256).collect());
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

/// No command line on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn command_line(_open: bool) {}

// --8<-- [start:step-4]
/// One row of the layers panel, as the panel needs it.
#[derive(Clone, Default)]
pub struct LayerRow {
    pub key: String,                 // unique id of the row
    pub label: String,               // text shown
    pub count: usize,                // objects under it
    pub hidden: bool,                // eye toggled off
    pub locked: bool,                // not editable
    pub color: Option<[u8; 3]>,      // face colour swatch
    pub depth: usize,                // indent level
    pub expanded: Option<bool>,      // open, closed or no children
    // --8<-- [end:step-4]
}

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
