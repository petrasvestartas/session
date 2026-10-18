    })
}
#[cfg(target_arch = "wasm32")]
pub fn continue_read(
    cursor: Cursor,
    current: Rc<dyn Fn(&Rc<crate::sheet_source::Prepared>) -> bool>,
    changed: Rc<dyn Fn()>,
    error: Rc<dyn Fn(String)>,
    cancelled: Rc<Cell<bool>>,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let Cursor {
            prepared,
            mut from,
            ceiling,
        } = cursor;
        let stream = prepared.streamed.as_ref().unwrap();
        while current(&prepared) && !cancelled.get() {
            let Some(rows) = next(
                from,
                stream.source.fields.count,
                ceiling.saturating_sub(from),
            ) else {
                return;
            };
            let (count, charge) = match reserve(stream, rows.end - rows.start, &*changed) {
                Ok(value) => value,
                Err(message) => {
                    error(message);
                    return;
                }
            };
            let ticket = match stream.generation.borrow_mut().request() {
                Ok(ticket) => ticket,
                Err(_) => return,
            };
            let result = stream.source.page(from..from + count).await;
            if !current(&prepared)
                || cancelled.get()
                || !stream.generation.borrow_mut().finish(ticket)
            {
                return;
            }
            match result {
                Ok(page) => {
                    if let Err(message) = stream.adopt(page, charge) {
                        error(message.into());
                        return;
                    }
                    from += count;
                    changed();
                }
                Err(message) => {
                    error(message);
                    return;
                }
            }
        }
    });
}
#[cfg(test)]
mod tests {
