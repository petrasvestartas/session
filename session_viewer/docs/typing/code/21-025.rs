    snapshot["selected_geometry"] = parent.map_or(serde_json::Value::Null, |row| shape(state, row)); // register:editing
