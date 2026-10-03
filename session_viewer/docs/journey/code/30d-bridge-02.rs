    let Some(file) = input.files().and_then(|files| files.get(0)) else { request.borrow_mut().cancel(); return; };
