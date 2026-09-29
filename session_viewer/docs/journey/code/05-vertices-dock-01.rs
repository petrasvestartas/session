    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Six uploaded corners make one rectangle.");
    Ok(())
}

