    static PREVIOUS: RefCell<Option<Report>> = const { RefCell::new(None) };
    static BEAT: RefCell<Option<crate::timer::Timer>> = const { RefCell::new(None) };