    pub(crate) released: HashMap<usize, Released>, // documents drawn without their kernel objects; register:release
    asked: RefCell<Vec<usize>>, // released documents a read-only path needs; register:release
