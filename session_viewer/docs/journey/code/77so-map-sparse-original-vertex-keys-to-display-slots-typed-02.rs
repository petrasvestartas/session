use crate::source_topology_colour::{BLACK,pack_rgba};
use session_rust::{Mesh, Tolerance};

/// Vertex key to its position in the sorted key list.
pub struct SlotMap {
    dense: Vec<u32>,                               // key indexes directly
    sparse: std::collections::HashMap<usize, u32>, // used for sparse keys
}

impl SlotMap {
    /// Build from the sorted vertex keys.
    pub fn new(keys: &[usize]) -> Self {
        let max_key = keys.last().copied().unwrap_or(0);

        // dense when keys are not too spread out
        if max_key < 4 * keys.len().max(1) {
            let mut dense = vec![u32::MAX; max_key + 1];

            for (s, &k) in keys.iter().enumerate() {
                dense[k] = s as u32;
            }

            return Self {
                dense,
                sparse: std::collections::HashMap::new(),
            };
        }

        let mut sparse = std::collections::HashMap::with_capacity(keys.len());

        for (s, &k) in keys.iter().enumerate() {
            sparse.insert(k, s as u32);
        }

        Self {
            dense: Vec::new(),
            sparse,
        }
    }

    /// The slot of key `k`.
    pub fn slot(&self, k: usize) -> usize {
        if self.dense.is_empty() {
            self.sparse[&k] as usize
        } else {
            self.dense[k] as usize
        }
    }
}

