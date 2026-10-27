    pub object_ids: Vec<ObjectId>,
    pub pick_owners: Vec<u32>,
    pub line_count: usize,
    pub mesh_from:usize,
    pub runs:[Vec<std::ops::Range<u32>>;4],
}

pub fn object(
