fn supported(source:&proto::Session)->Result<(),String> {
    crate::instance_plan::validate(source)?;
    Ok(())
}
