    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Four shared corners make two triangles.");
    Ok(())
}

