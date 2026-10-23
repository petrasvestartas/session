    ACTIVE.with(|slot| slot.replace(Some((token, listeners, Suspension::new(hidden)))));
    crate::browser_runtime::refresh_frame(permits()); Ok(())
