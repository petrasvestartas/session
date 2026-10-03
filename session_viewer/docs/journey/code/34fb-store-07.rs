    if let Some(message) = crate::browser_report::previous_notice() { panel.result(message); }
    panel.update(None, &canvas)?;
    if resize(