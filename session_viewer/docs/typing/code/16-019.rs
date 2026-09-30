
thread_local! {
    static SOURCE_MEMORY: std::cell::RefCell<source_memory::SourceCache> = Default::default();
}
