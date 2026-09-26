use crate::engine::gpu::instance::Instance;

/// guid = a unique name the object keeps for life; revision counts its edits, so the GPU knows which version it holds.
pub struct SourceObject {
    pub guid: &'static str,
    pub revision: u64,
    pub row: Instance,
}

/// One triangle, two rows: each row has its own placement and colour.
pub fn objects() -> [SourceObject; 2] {
    let mut left = Instance::placeholder();
    left.model[12] = -0.8; // column-major: entries 12, 13, 14 are the x, y, z move
    left.color = [1.0, 0.35, 0.2, 1.0];
    let mut right = Instance::placeholder();
    right.model[12] = 0.8;
    right.color = [0.2, 0.7, 1.0, 1.0];
    [
        SourceObject {
            guid: "triangle-left",
            revision: 1,
            row: left,
        },
        SourceObject {
            guid: "triangle-right",
            revision: 1,
            row: right,
        },
    ]
}
