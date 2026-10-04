        .request_device(&wgpu::DeviceDescriptor::default())
        .await;
    if !startup.permits() {
        if let Ok((device, _)) = device { device.destroy(); }
        return Ok(());
    }
    let (device, queue) = device.map_err(|error| JsValue::from_str(&error.to_string()))?;