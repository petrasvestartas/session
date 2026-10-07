pub fn paths<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 5] {
    let mut sources = HashSet::new(); let mut displays = HashSet::new(); let mut usage = [0; 5];
    for scene in scenes {
        usage[0] += scene.paths().len(); usage[3] += scene.path_capacity_bytes();
        for row in scene.paths() {
            sources.insert(row.prepared.source.owner());
            if displays.insert(Rc::as_ptr(&row.prepared.points)) { usage[4] += row.prepared.points.capacity() * 12; }
        }
    }
    usage[1] = sources.len(); usage[2] = displays.len(); usage
}

pub fn lines<'a>