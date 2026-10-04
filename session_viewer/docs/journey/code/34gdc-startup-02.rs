pub fn phase(phase: crate::load_phase::Phase) -> Result<(), JsValue> {
    REPORT.with(|slot| slot.borrow_mut().as_mut().ok_or("No report")?
        .phase(js_sys::Date::new_0().to_iso_string().into(), phase).map_err(JsValue::from_str))?;
    let _ = persist(); Ok(())
}

pub fn adapter(info: crate::adapter_info::AdapterInfo) -> Result<(), JsValue> {