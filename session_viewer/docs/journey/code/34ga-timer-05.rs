#[cfg_attr(debug_assertions, wasm_bindgen::prelude::wasm_bindgen)]
pub fn start_periodic() -> Result<(), JsValue> {
    stop_periodic();
    let timer = crate::timer::Timer::every(15000, || { let _ = heartbeat(); })?;
    BEAT.with(|slot| slot.replace(Some(timer))); Ok(())
}

#[cfg_attr(debug_assertions, wasm_bindgen::prelude::wasm_bindgen)]
pub fn stop_periodic() -> bool {
    let timer = BEAT.with(|slot| slot.borrow_mut().take());
    let active = timer.is_some(); drop(timer); active
}

fn persist() -> bool {