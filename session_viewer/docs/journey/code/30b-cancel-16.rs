    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    canvas.set_attribute("data-projection", &format!("{:?}", editor.camera.projection))
