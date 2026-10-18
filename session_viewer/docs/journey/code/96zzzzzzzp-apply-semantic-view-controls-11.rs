    display(&mut renderer,&editor).map_err(JsValue::from_str)?;
    crate::browser_phase::finish("renderer setup", pipeline_started, 0, "GPU");