        let accepted = request.borrow_mut().finish(id);
        if !accepted { return; }
        let result = result.and_then(deliver);
