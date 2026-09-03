//! The scene manifest: WHICH files a scene is made of and WHERE each one sits. A drawing is
//! authored at its own page origin, so placement has to come from a text file next to the
//! assets (`at` = translation, `xform` = all 16 numbers, neither = the auto-grid); edit,
//! reload, no rebuild. Nothing here touches a kernel object or the GPU.

use serde::Deserialize;
use session_rust::Xform;

/// One manifest entry: a file to load and where to place it. Every file is authored at its
/// own origin, so an item carries `at` or `xform`; with neither it takes an `auto_grid` slot.
#[derive(Deserialize)]
pub struct Item {
    pub file: String,                 // asset path, e.g. "pb/draw_pf_he.pb"
    #[serde(default)]
    pub name: String,                 // display name; empty = use the session's own
    #[serde(default)]
    pub at: Option<[f64; 3]>,         // translation in world units
    #[serde(default)]
    pub xform: Option<[f64; 16]>,     // full 4x4 (wins over `at`); neither = auto_grid
    #[serde(default)]
    pub point_size: f64,              // raw-cloud px for this file; 0 = keep the pb'own
    #[serde(default)]
    pub stream: bool,                 // Range-stream this file's cloud instead of parsing it
    /// Release this file's kernel `Session` after the walk: a sheet is looked at, never picked
    /// or edited, and 10 sheets of `drawings` held 1.2 GB of documents for tables the GPU
    /// already owns (2056 MB -> 899 MB resident, frame byte-identical). Never on a model file.
    #[serde(default)]
    pub display_only: bool,
}

/// The parsed scene file: an ordered list of items, loaded in list order.
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
    /// JSON first (every existing scene), TOML as the fallback - a .toml manifest gets
    /// real comments and no trailing-comma landmines; both land in the same structs.
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        serde_json::from_slice(bytes).ok()
            .or_else(|| std::str::from_utf8(bytes).ok().and_then(|s| toml::from_str(s).ok()))
    }

    /// Where item `i` sits: its own placement, else its `auto_grid` slot on a grid of `cell` steps.
    pub fn place(&self, i: usize, cell: [f64; 2]) -> Xform {
        self.items[i].placement().unwrap_or_else(|| auto_grid(i, self.items.len(), cell))
    }
}

/// Fallback for items with no `at`/`xform`: lay them out on a grid of `cell` steps, in list order.
/// Deliberately dumb - it exists so a manifest can be written one sheet at a time, not as the way
/// a scene is normally described.
pub fn auto_grid(index: usize, count: usize, cell: [f64; 2]) -> Xform {
    let cols = (count as f64).sqrt().ceil().max(1.0) as usize;
    Xform::translation((index % cols) as f64 * cell[0], (index / cols) as f64 * cell[1], 0.0)
}
