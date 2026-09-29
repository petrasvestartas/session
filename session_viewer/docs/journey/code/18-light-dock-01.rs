    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Surface direction changes brightness, not geometry.");
    Ok(())
}

