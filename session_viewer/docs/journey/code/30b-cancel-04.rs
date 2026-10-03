        if request.borrow().pending() != Some(id) { return; }
        if file.size()