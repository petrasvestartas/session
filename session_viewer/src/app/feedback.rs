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
