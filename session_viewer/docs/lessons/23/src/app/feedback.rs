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

/// No command line on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn command_line(_open: bool) {}

pub struct LayerRow {
    pub key: String,                 // unique id of the row
    pub label: String,               // text shown
    pub count: usize,                // objects under it
    pub hidden: bool,                // eye toggled off
}

/// Replace the rows of the layers panel.
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
        // a real button, so keyboard focus works
        let Ok(line) = document.create_element("button") else {
            continue;
        };
        let _ = line.set_attribute("type", "button");
        let _ = line.set_attribute("aria-pressed", if row.hidden { "true" } else { "false" });
        let _ = line.set_attribute("data-layer", &row.key);
        let _ = line.set_attribute(
            "style",
            "display:block;width:100%;text-align:left;border:0;background:none;color:inherit;font:inherit;padding:2px 10px;cursor:pointer;white-space:nowrap;opacity:1",
        );

        if row.hidden {
            let _ = line.set_attribute(
                "style",
                "display:block;width:100%;text-align:left;border:0;background:none;color:inherit;font:inherit;padding:2px 10px;cursor:pointer;white-space:nowrap;opacity:0.45",
            );
        }

        let mark = if row.hidden { "·" } else { "•" };
        line.set_text_content(Some(&format!("{mark} {} ({})", row.label, row.count)));
        let _ = panel.append_child(&line);
    }
}

/// Show or hide the layers panel.
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

/// Whether the layers panel is open.
#[cfg(target_arch = "wasm32")]
pub fn layers_open() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("viewer-layers"))
        .is_some_and(|panel| !panel.has_attribute("hidden"))
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
