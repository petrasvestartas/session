// --8<-- [start:feedback-status]
/// Show a message in the status line.
pub fn status(message: &str) {
    // an empty message shows the reload notice, if any
    // `#[cfg]` on a `let`: this shadowing line exists only in the browser build
    #[cfg(target_arch = "wasm32")]
    let message = if message.is_empty() {
        super::route::recovered_notice().unwrap_or(message)
    } else {
        message
    };

    // the status line is a plain element of index.html, not drawn by the GPU
    #[cfg(target_arch = "wasm32")]
    if let Some(window) = web_sys::window()
        && let Some(document) = window.document()
        && let Some(status) = document.get_element_by_id("viewer-status")
    {
        status.set_text_content(Some(message));
    }

    log::info!("{message}");
}

/// Show a download's progress, unless another message is up; nothing is logged.
#[cfg(target_arch = "wasm32")]
pub fn progress(message: &str, last: &str) {
    if let Some(window) = web_sys::window()
        && let Some(document) = window.document()
        && let Some(status) = document.get_element_by_id("viewer-status")
    {
        let shown = status.text_content().unwrap_or_default();

        if shown.is_empty() || shown == last {
            status.set_text_content(Some(message));
        }
    }
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
// --8<-- [end:feedback-status]

// --8<-- [start:feedback-focus]
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
// --8<-- [end:feedback-focus]

// --8<-- [start:feedback-rows]
// The rows the layers panel shows: this lesson fills them, lesson 30 draws the panel.
/// One row of the layers panel.
#[derive(Clone, Default, serde::Serialize)]
pub struct LayerRow {
    pub key: String,                 // unique id of the row
    pub label: String,               // text shown
    pub count: usize,                // objects under it
    pub hidden: bool,                // eye toggled off
    pub locked: bool,                // not editable
    pub selected: bool,              // highlighted
    pub color: Option<[u8; 3]>,      // face colour swatch
    pub edge_color: Option<[u8; 3]>, // edge colour swatch
    pub has_faces: bool,             // shows a face swatch
    pub depth: usize,                // indent level
    pub expanded: Option<bool>,      // open, closed or no children
    pub layer: bool,                 // a group or document, not an object
    pub current: bool,               // where new objects go
    pub root: bool,                  // the top layer of its document
}

/// One row of the graph table: an edge between two objects.
#[derive(Clone, Default, serde::Serialize)]
pub struct EdgeRow {
    pub key: String,    // `pair/<row>/<row>`
    pub from: String,   // first object's name, else its short guid
    pub to: String,     // second object's name, else its short guid
    pub guids: String,  // both guids, for the tooltip
    pub selected: bool, // both ends selected
}
// --8<-- [end:feedback-rows]
