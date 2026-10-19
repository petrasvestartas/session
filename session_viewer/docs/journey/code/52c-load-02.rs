    run("validation", size, clock, observe, || validate(&message))?;
    let mut origin = crate::origin::Origin::new(&message, bytes); origin.location = location;
    let origin = Rc::new(origin);
    let tree=message.tree.as_ref().and_then(|tree|tree.root.clone());
    let document = run("kernel", size, clock, observe, || {
        Session::from_proto(message).map(Rc::new).map_err(|_| "Cannot construct session")
    })?;
