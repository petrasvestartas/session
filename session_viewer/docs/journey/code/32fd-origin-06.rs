    let origins: Vec<_> = editor.scene.objects().iter().filter_map(|row| row.source().map(|source|
        (&row.guid, source.origin.id.to_string(), source.origin.version.hex()))).collect();
    canvas.set_attribute("data-source-origins", &serde_json::to_string(&origins)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-row-metadata", &serde_json::to_string(&metadata)
