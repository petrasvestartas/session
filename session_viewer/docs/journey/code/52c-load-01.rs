pub struct Loaded {
    pub document: Rc<Session>,
    pub meshes: Vec<PreparedMesh>,
    pub tree:Option<proto::TreeNode>,
}

pub fn load(bytes: &[u8]) -> Result<Loaded, &'static str> { load_at(bytes, None) }
