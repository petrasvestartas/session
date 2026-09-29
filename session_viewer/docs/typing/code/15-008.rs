                let head = probe(&target).await; // register:stream
                slot.borrow_mut().0 = head; // register:stream
