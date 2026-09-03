//! The scene manifest: WHICH files a scene is made of and WHERE each one sits.
//!
//! A drawing is authored at its own page origin, so any number of them loaded raw would stack on
//! top of each other. Placement therefore has to come from somewhere - and the honest place is a
//! text file next to the assets, not arithmetic buried in the GPU layer. Edit `at`, reload, no
//! rebuild; a web deployment can be re-arranged without a compiler.
//!
//! ```json
//! { "items": [ { "file": "pb/draw_pf_he.pb", "name": "HE", "at": [3400, 0, 0] } ] }
//! ```
//! `at` is a translation in world units. `xform` takes all 16 numbers instead when a sheet needs
//! rotation or scale. An item with neither falls back to the auto-grid below.
use serde::Deserialize;
use session_rust::Xform;

#[derive(Deserialize)]
pub struct Item {
    pub file: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub at: Option<[f64; 3]>,
    #[serde(default)]
    pub xform: Option<[f64; 16]>,
}

#[derive(Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub name: String,
    pub items: Vec<Item>,
}

impl Item {
    /// The placement this item asks for, or `None` when it wants the auto-grid.
    pub fn placement(&self) -> Option<Xform> {
        if let Some(m) = self.xform {
            let mut x = Xform::identity();
            x.m = m;
            return Some(x);
        }
        self.at.map(|a| Xform::translation(a[0], a[1], a[2]))
    }
}

impl Manifest {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        serde_json::from_slice(bytes).ok()
    }
}

/// Fallback for items with no `at`/`xform`: lay them out on a grid of `cell` steps, in list order.
/// Deliberately dumb - it exists so a manifest can be written one sheet at a time, not as the way
/// a scene is normally described.
pub fn auto_grid(index: usize, count: usize, cell: [f64; 2]) -> Xform {
    let cols = (count as f64).sqrt().ceil().max(1.0) as usize;
    Xform::translation((index % cols) as f64 * cell[0], (index / cols) as f64 * cell[1], 0.0)
}
