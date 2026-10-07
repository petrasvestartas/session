pub fn lines<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 3] {
    let mut sources = HashSet::new(); let mut count = [0; 3];
    for scene in scenes {
        count[0] += scene.lines().len();
        for row in scene.lines() { sources.insert(Rc::as_ptr(&row.prepared.source)); }
        count[2] += scene.line_capacity_bytes();
    }
    count[1] = sources.len(); count
}

pub fn gpu<'a>