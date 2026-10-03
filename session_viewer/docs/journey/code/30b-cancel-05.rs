            let accepted = request.borrow_mut().finish(id);
            if accepted { failure("This checkpoint accepts files up to 4 MiB"); }
