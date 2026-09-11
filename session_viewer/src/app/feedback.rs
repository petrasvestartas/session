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

/// One row of the layers panel, as the panel needs it.
pub struct LayerRow {
    pub key: String,
    pub label: String,
    pub count: usize,
    pub hidden: bool,
}

/// Fill the layers panel, or empty it. Every label goes in with `textContent`, so a document
/// named after a tag cannot become markup.
#[cfg(target_arch = "wasm32")]
pub fn layers_panel(rows: &[LayerRow]) {
    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let Some(panel) = document.get_element_by_id("viewer-layers") else {
        return;
    };
    panel.set_text_content(None);
    for row in rows {
        let Ok(line) = document.create_element("div") else {
            continue;
        };
        let _ = line.set_attribute("data-layer", &row.key);
        let _ = line.set_attribute(
            "style",
            "padding:2px 10px;cursor:pointer;white-space:nowrap;opacity:1",
        );
        if row.hidden {
            let _ = line.set_attribute(
                "style",
                "padding:2px 10px;cursor:pointer;white-space:nowrap;opacity:0.45",
            );
        }
        let mark = if row.hidden { "·" } else { "•" };
        line.set_text_content(Some(&format!("{mark} {} ({})", row.label, row.count)));
        let _ = panel.append_child(&line);
    }
}

/// Show or hide the panel; returns it so a caller can attach its one listener.
#[cfg(target_arch = "wasm32")]
pub fn layers_visible(open: bool) -> Option<web_sys::Element> {
    let panel = web_sys::window()?
        .document()?
        .get_element_by_id("viewer-layers")?;
    let _ = if open {
        panel.remove_attribute("hidden")
    } else {
        panel.set_attribute("hidden", "")
    };
    Some(panel)
}

/// Whether the panel is open, so a refresh can skip the work while it is not.
#[cfg(target_arch = "wasm32")]
pub fn layers_open() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("viewer-layers"))
        .is_some_and(|panel| !panel.has_attribute("hidden"))
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
