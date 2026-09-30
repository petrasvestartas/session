    tombs: HashMap<(usize, Rc<str>), Tomb>, // deleted objects whose rows stay on the GPU, hidden; register:document
    tombed: Counts,                         // lane rows the tombs hold; register:document
    tomb_points: u64,                       // cloud points the tombs hold; register:document
    burials: u64,                           // tombs made, for their order; register:document
    tomb_cap: u64,                          // lane bytes the tombs may hold; register:document
