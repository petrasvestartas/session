/// Byte positions of a cloud's arrays in its file.
#[derive(Clone, Debug)]
pub struct CloudFields {
    pub end: u64,                 // end of the cloud message
    pub coords_at: u64,
    pub coords_len: u64,
    pub colors_at: u64,
    pub colors_len: u64,          // their length
    pub count: u32,               // points in the cloud
    pub ids_at: u64,              // start of the original ids, 0 = none
    pub ids_len: u64,             // their length
    pub revision: Option<String>, // file ETag every read must match
}

/// A cloud's octree node table.
#[derive(Clone, Default)]
pub struct CloudLod {
    pub min: Vec<f64>,     // cube corner, three per node
    pub size: Vec<f64>,    // cube size per node
    pub spacing: Vec<f64>, // point spacing per node
    pub level: Vec<i32>,   // depth per node
    pub first: Vec<i32>,   // first point per node
    pub count: Vec<i32>,   // points per node
    pub children: Vec<i32>, // eight child indices per node, -1 = none
}

impl CloudLod {
    /// Number of nodes.
    pub fn len(&self) -> usize {
        self.size.len()
    }

    /// True when the file carried no octree.
    pub fn is_empty(&self) -> bool {
        self.size.is_empty()
    }
}
