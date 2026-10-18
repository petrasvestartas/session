    let owned_editor = std::rc::Rc::clone(&shared);
    let mut origin_wake=crate::browser_rebase::Wake::new(fault.clone());