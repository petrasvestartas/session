ce_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;

mod draw_slots;
#[cfg(test)] mod draw_slots_tests;

#[cfg(test)] mod source_phase_tests;
