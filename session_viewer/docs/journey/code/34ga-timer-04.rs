    let _ = persist();
    if let Err(error) = start_periodic() { let _ = observe("diagnostic", &format!("Heartbeat unavailable: {error:?}")); }
    Ok(())
}

pub fn observe