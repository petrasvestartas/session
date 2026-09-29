    snapshot["selected_instance"] = // register:instancing
        serde_json::json!(parent.and_then(|row| state.scene.instance_name(row))); // register:instancing
