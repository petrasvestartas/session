on(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;

pub mod surfacing;

pub mod extrude_geometry;

pub mod revolve_geometry;
