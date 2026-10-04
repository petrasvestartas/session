pub fn adapter(info: crate::adapter_info::AdapterInfo) -> Result<(), JsValue> {
    REPORT.with(|slot| slot.borrow_mut().as_mut().ok_or("No report")?
        .set_adapter(info).map_err(JsValue::from_str))?;
    let _ = persist(); Ok(())
}

pub fn fatal(message: &str) {