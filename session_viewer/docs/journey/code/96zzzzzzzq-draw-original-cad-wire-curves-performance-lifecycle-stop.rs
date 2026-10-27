    crate::browser_runtime::refresh_frame(false);
    let active = owner.is_some(); report::stop_periodic(); drop(owner); active
