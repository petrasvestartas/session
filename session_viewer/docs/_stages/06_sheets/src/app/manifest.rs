//! The scene manifest: WHICH files a scene is made of and WHERE each one sits (`at` =
//! translation, `xform` = all 16 numbers, neither = the auto-grid). Edit, reload, no rebuild.

use serde::Deserialize;
use session_rust::Xform;

/// One manifest entry: a file and its placement.
#[derive(Deserialize)]
pub struct Item {
    /// Asset path, e.g. `pb/view_lines_he.pb`, relative to the scene's base.
    pub file: String,
    /// Display name; empty = the session's own.
    #[serde(default)]
    pub name: String,
    /// Translation in world units.
    #[serde(default)]
    pub at: Option<[f64; 3]>,
    /// Full 4x4 column-major (wins over `at`).
    #[serde(default)]
    pub xform: Option<[f64; 16]>,
    /// Release this file's kernel `Session` after the walk: a sheet is looked at, never
    /// picked or edited, and its document is the biggest thing in memory. Never on a model.
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
    /// JSON for a `{`-led document, TOML otherwise; the error names the format and position.
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        let text = std::str::from_utf8(bytes).map_err(|e| format!("not UTF-8 text: {e}"))?;
        if text.trim_start().starts_with('{') {
            return serde_json::from_str(text).map_err(|e| format!("JSON: {e}"));
        }
        toml::from_str(text).map_err(|e| format!("TOML: {}{}", e.message().replace('\n', " "), toml_span(&e)))
    }

    /// Where item `i` sits: its own placement, else its `auto_grid` slot.
    pub fn place(&self, i: usize, cell: [f64; 2]) -> Xform {
        self.items[i].placement().unwrap_or_else(|| auto_grid(i, self.items.len(), cell))
    }

    /// Item `i`'s display name: the manifest's, else `fallback`.
    pub fn name_of(&self, i: usize, fallback: &str) -> String {
        let n = &self.items[i].name;
        if n.is_empty() { fallback.to_string() } else { n.clone() }
    }
}

/// " (byte offset a-b)" of a TOML error, or nothing when it carries no span.
fn toml_span(e: &toml::de::Error) -> String {
    match e.span() {
        Some(span) => format!(" (byte offset {}-{})", span.start, span.end),
        None => String::new(),
    }
}

/// Fallback for items with no placement: a grid of `cell` steps, in list order.
pub fn auto_grid(index: usize, count: usize, cell: [f64; 2]) -> Xform {
    let cols = (count as f64).sqrt().ceil().max(1.0) as usize;
    Xform::translation((index % cols) as f64 * cell[0], (index / cols) as f64 * cell[1], 0.0)
}
