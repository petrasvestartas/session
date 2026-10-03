    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    canvas.set_attribute("data-gpu-stats", &format!("{:?}", renderer.stats()))?;
