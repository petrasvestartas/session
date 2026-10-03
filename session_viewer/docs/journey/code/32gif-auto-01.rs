pub fn restore_for(intent: crate::edit_intent::Intent, editor: &crate::editor::Editor, shared: Shared, delivery: Delivery) -> Result<bool, String> {
    let keys = intent.keys(editor).map_err(str::to_owned)?;
    shared.borrow_mut().cancel();
    if keys.is_empty() { return Ok(false); }
    start_with(shared, keys, Some(intent), delivery)?;
    Ok(true)
}

pub fn start(shared: Shared, keys: Vec<ReloadKey>, delivery: Delivery) -> Result<(), String> {