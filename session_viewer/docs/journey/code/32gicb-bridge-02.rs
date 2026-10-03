pub fn start(shared: Shared, keys: Vec<ReloadKey>, delivery: Delivery) -> Result<(), String> {
    start_with(shared, keys, None, delivery)
}

pub fn start_with(shared: Shared, keys: Vec<ReloadKey>, intent: Option<crate::edit_intent::Intent>, delivery: Delivery) -> Result<(), String> {
    let (request, signal) = shared.borrow_mut().begin(keys, intent)?;