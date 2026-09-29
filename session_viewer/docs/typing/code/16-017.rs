    let source_memory = SOURCE_MEMORY.with_borrow_mut(|cache| cache.snapshot(&state.scene.docs)); // register:release
