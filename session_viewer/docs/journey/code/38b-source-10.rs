pub fn curves<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 4] {
    let mut sources = HashSet::new(); let mut samples = HashSet::new(); let mut usage = [0; 4];
    for scene in scenes {
        for row in scene.paths() {
            if let crate::chain::Source::Curve(source, points) = &row.prepared.source {
                usage[0] += 1; sources.insert(Rc::as_ptr(source));
                if samples.insert(Rc::as_ptr(points)) { usage[3] += points.capacity() * 24; }
            }
        }
    }
    usage[1] = sources.len(); usage[2] = samples.len(); usage
}

pub fn paths<'a>