use crate::engine::gpu::instance::Instance;

/// A source object and its GPU instance.
pub struct SourceObject {
    pub guid: &'static str,
    pub revision: u64,
    pub row: Instance,
}

/// Two independently placed/tinted copies of the same local triangle.
pub fn objects() -> [SourceObject; 2] {
    let mut left = Instance::placeholder();
    left.model[12] = -0.8;
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
