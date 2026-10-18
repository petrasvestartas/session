    (count > 0).then(|| from..from + count)
}
#[cfg(target_arch = "wasm32")]
pub struct Cursor {
    pub prepared: Rc<crate::sheet_source::Prepared>,
    pub from: u32,
    pub ceiling: u32,
}
#[cfg(target_arch = "wasm32")]
fn reserve(stream: &Stream, mut count: u32, changed: &dyn Fn()) -> Result<(u32, Charge), String> {
    loop {
        let bytes = temporary_bytes(count).ok_or("Drawing page size overflow")?;
        let result = stream.pages.borrow_mut().reserve(bytes);
        if let Ok(charge) = result {
            return Ok((count, charge));
        }
        let key = stream.pages.borrow().oldest();
        if let Some(key) = key {
            stream.pages.borrow_mut().remove(&key);
            changed();
        } else if count > 1 {
            count /= 2;
        } else {
            return Err("Original drawing pages are still held by readers".into());
        }
    }
}
#[cfg(target_arch = "wasm32")]
pub async fn start(
    source: Rc<Source>,
    budget: Rc<Budget>,
    remaining: u32,
) -> Result<Cursor, String> {
    let stream = Stream::new(source, budget);
    stream
        .generation
        .borrow_mut()
        .start()
        .map_err(str::to_owned)?;
    let requested = PREFIX.min(remaining).min(stream.source.fields.count);
    if requested == 0 {
        return Err("Manifest exceeds the drawing segment ceiling".into());
    }
    let (count, charge) = reserve(&stream, requested, &|| {})?;
    let page = stream.source.page(0..count).await?;
    drop(stream.adopt(page, charge).map_err(str::to_owned)?);
    let prepared = Rc::new(crate::sheet_source::Prepared::streamed(stream));
    Ok(Cursor {
        prepared,
        from: count,
        ceiling: count,
    })
}
#[cfg(test)]
mod tests {
