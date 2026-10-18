}
impl Read {
    fn new(url:&str)->Result<Self,String> {
        let controller=web_sys::AbortController::new().map_err(|error|describe(error.into()))?;
        let key=NEXT.with(|next| {let key=next.get().checked_add(1).ok_or("Range request IDs exhausted")?;next.set(key);Ok::<_,String>(key)})?;
