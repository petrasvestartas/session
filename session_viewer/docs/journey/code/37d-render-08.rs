pub fn points<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 3] {
    let mut sources = HashSet::new(); let mut usage = [0; 3];
    for scene in scenes {
        usage[0] += scene.points().len(); usage[2] += scene.point_capacity_bytes();
        for row in scene.points() { sources.insert(Rc::as_ptr(&row.prepared.source)); }
    }
    usage[1] = sources.len(); usage
}

pub fn paths<'a>