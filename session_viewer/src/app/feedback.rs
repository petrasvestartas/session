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
    super::ui::command_line::STATE
        .with_borrow_mut(|model| model.status = message.chars().take(256).collect());
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
            super::ui::command_line::STATE
                .with_borrow_mut(|model| model.status = message.to_string());
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

/// Open or close the command line.
#[cfg(target_arch = "wasm32")]
pub fn command_line(open: bool) {
    super::ui::command_line::STATE.with_borrow_mut(|model| {
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

/// Raise the phone keyboard over an empty field; works while a tap is handled.
#[cfg(target_arch = "wasm32")]
pub fn raise_keyboard() {
    super::agent::raise();
}

/// No phone keyboard on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn raise_keyboard() {}

/// No command line on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn command_line(_open: bool) {}

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

/// Replace the rows of the layers panel.
#[cfg(target_arch = "wasm32")]
pub fn layers_panel(rows: &[LayerRow]) {
    super::ui::layers::STATE.with_borrow_mut(|model| model.rows = rows.to_vec());
}

/// Replace the rows of the graph table; `total` counts the edges not listed too.
#[cfg(target_arch = "wasm32")]
pub fn graph_panel(edges: Vec<EdgeRow>, total: usize) {
    super::ui::layers::STATE.with_borrow_mut(|model| {
        model.edges = edges;
        model.edge_total = total;
    });
}

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn graph_panel(_edges: Vec<EdgeRow>, _total: usize) {}

/// Whether the graph table is unfolded.
#[cfg(target_arch = "wasm32")]
pub fn graph_open() -> bool {
    super::ui::layers::STATE.with_borrow(|model| model.graph_open)
}

/// Fold or unfold the graph table.
#[cfg(target_arch = "wasm32")]
pub fn toggle_graph() {
    super::ui::layers::STATE.with_borrow_mut(|model| model.graph_open = !model.graph_open);
}

/// Start editing the name of layer row `index`.
#[cfg(target_arch = "wasm32")]
pub fn rename_row(index: usize, label: &str) {
    super::ui::layers::STATE.with_borrow_mut(|model| {
        model.renaming = Some(super::ui::layers::Rename {
            node: index.to_string(),
            text: label.to_string(),
            focused: false,
            done: false,
        });
    });
}

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn graph_open() -> bool {
    false
}

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn toggle_graph() {}

/// No panel on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn rename_row(_index: usize, _label: &str) {}

/// Show or hide the layers panel.
#[cfg(target_arch = "wasm32")]
pub fn layers_visible(open: bool) {
    super::ui::layers::STATE.with_borrow_mut(|model| {
        model.layers_open = open;

        if !open {
            model.rows.clear();
            model.edges.clear();
        }
    });
}

/// Whether the layers panel is open.
#[cfg(target_arch = "wasm32")]
pub fn layers_open() -> bool {
    super::ui::layers::STATE.with_borrow(|model| model.layers_open)
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
