//! The scene manifest: WHICH files a scene is made of and WHERE each one sits (`at` =
//! translation, `xform` = all 16 numbers, neither = the auto-grid). Edit, reload, no rebuild.

use serde::Deserialize;
use session_rust::Xform;

/// One manifest entry: a file and its placement.
#[derive(Clone, Deserialize)]
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
    /// Cloud point size in px for this file; 0 = the pb's own.
    #[serde(default)]
    pub point_size: f64,
    /// Legacy memory hint, accepted for compatibility. Full-file sources stay retained
    /// so original source controls remain available to F10.
    #[serde(default)]
    pub display_only: bool,
}

/// One fixed world-space text plane; its frame and em height use the scene's world units.
#[derive(Clone, Debug, Deserialize)]
pub struct TextItem {
    /// UTF-8 text, retained verbatim after rejecting empty or whitespace-only content.
    pub text: String,
    /// World-space plane origin, independent of any geometry item's placement.
    #[serde(default)]
    pub at: [f64; 3],
    /// Unit direction of increasing text X; defaults to world +X.
    #[serde(default = "text_right")]
    pub right: [f64; 3],
    /// Unit direction of increasing text Y; defaults to world +Y.
    #[serde(default = "text_up")]
    pub up: [f64; 3],
    /// Required positive finite em height in world units.
    pub height: f64,
}

/// The parsed scene file: ordered geometry items and optional fixed world-space text planes.
#[derive(Clone, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub name: String,
    pub items: Vec<Item>,
    #[serde(default)]
    pub texts: Vec<TextItem>,
}

impl Item {
    /// The placement this item asks for, or `None` when it wants the auto-grid.
    pub fn placement(&self) -> Option<Xform> {
        if let Some(m) = self.xform {
            let mut x = Xform::identity();
            x.m = m;
            return Some(x);
        }
        self.at.map(translation)
    }
}

impl Manifest {
    /// Read existing YAML/JSON and compatible TOML; reject invalid transforms before upload.
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > 4 * 1024 * 1024 {
            return Err("manifest exceeds 4 MiB".to_string());
        }
        let text = match std::str::from_utf8(bytes) {
            Ok(text) => text,
            Err(error) => return Err(format!("manifest is not UTF-8: {error}")),
        };
        let manifest: Self = match serde_yaml_ng::from_str(text) {
            Ok(manifest) => manifest,
            Err(yaml) => match toml::from_str(text) {
                Ok(manifest) => manifest,
                Err(toml) => {
                    return Err(format!(
                        "Invalid manifest. YAML: {}{}. TOML: {toml}",
                        yaml,
                        yaml_at(&yaml)
                    ));
                }
            },
        };
        if manifest.items.len() > 100_000 {
            return Err("manifest exceeds 100,000 items".to_string());
        }
        for (index, item) in manifest.items.iter().enumerate() {
            if item.file.trim().is_empty() {
                return Err(format!("item {index}: missing geometry file"));
            }
            if !item.point_size.is_finite() || item.point_size < 0.0 {
                return Err(format!("item {index}: invalid point size"));
            }
            if item.at.is_some_and(nonfinite_transform)
                || item.xform.is_some_and(nonfinite_transform)
            {
                return Err(format!("item {index}: non-finite transform"));
            }
            if let Some(matrix) = item.xform
                && (matrix[3] != 0.0 || matrix[7] != 0.0 || matrix[11] != 0.0 || matrix[15] != 1.0)
            {
                return Err(format!("item {index}: placement must be affine"));
            }
        }
        if manifest.texts.len() > 1024 {
            return Err("manifest exceeds 1,024 text records".to_string());
        }
        let mut text_bytes = 0usize;
        for (index, item) in manifest.texts.iter().enumerate() {
            text_bytes = text_bytes.saturating_add(item.text.len());
            if text_bytes > 256 * 1024 {
                return Err("manifest text exceeds 256 KiB of UTF-8 content".to_string());
            }
            item.validate(index)?;
        }
        Ok(manifest)
    }

    /// Where item `i` sits: its own placement, else its `auto_grid` slot.
    pub fn place(&self, i: usize, cell: [f64; 2]) -> Xform {
        match self.items[i].placement() {
            Some(placement) => placement,
            None => auto_grid(i, self.items.len(), cell),
        }
    }

    /// Item `i`'s display name: the manifest's, else `fallback`.
    pub fn name_of(&self, i: usize, fallback: &str) -> String {
        let n = &self.items[i].name;
        if n.is_empty() {
            fallback.to_string()
        } else {
            n.clone()
        }
    }
}

impl TextItem {
    /// Reject invalid world frames before a loader creates any text or GPU resources.
    fn validate(&self, index: usize) -> Result<(), String> {
        if self.text.trim().is_empty() {
            return Err(format!("text {index}: missing text content"));
        }
        if !self.height.is_finite() || self.height <= 0.0 {
            return Err(format!("text {index}: height must be positive and finite"));
        }
        if nonfinite_transform(self.at) {
            return Err(format!("text {index}: non-finite position"));
        }
        for axis in [self.right, self.up] {
            if nonfinite_transform(axis) {
                return Err(format!("text {index}: non-finite plane axis"));
            }
            let length = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
            if (length - 1.0).abs() > 1e-6 {
                return Err(format!(
                    "text {index}: plane axes must be unit length within 1e-6"
                ));
            }
        }
        let dot =
            self.right[0] * self.up[0] + self.right[1] * self.up[1] + self.right[2] * self.up[2];
        if dot.abs() > 1e-6 {
            return Err(format!(
                "text {index}: plane axes must be orthogonal within 1e-6"
            ));
        }
        Ok(())
    }
}

/// Default text-plane horizontal direction in world coordinates.
fn text_right() -> [f64; 3] {
    [1.0, 0.0, 0.0]
}

/// Default text-plane vertical direction in world coordinates.
fn text_up() -> [f64; 3] {
    [0.0, 1.0, 0.0]
}

/// Convert the manifest's optional translation into the shared transform representation.
fn translation(at: [f64; 3]) -> Xform {
    Xform::translation(at[0], at[1], at[2])
}

/// Reject any non-finite component in a manifest translation or matrix.
fn nonfinite_transform<const N: usize>(values: [f64; N]) -> bool {
    !values.into_iter().all(f64::is_finite)
}

/// " (line l, column c)" of a YAML error, or nothing when it carries no location.
fn yaml_at(e: &serde_yaml_ng::Error) -> String {
    match e.location() {
        Some(l) => format!(" (line {}, column {})", l.line(), l.column()),
        None => String::new(),
    }
}

/// Fallback for items with no placement: a grid of `cell` steps, in list order.
pub fn auto_grid(index: usize, count: usize, cell: [f64; 2]) -> Xform {
    let cols = (count as f64).sqrt().ceil().max(1.0) as usize;
    Xform::translation(
        (index % cols) as f64 * cell[0],
        (index / cols) as f64 * cell[1],
        0.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yaml_json_and_toml_have_identical_scene_semantics() {
        let forms = [
            "name: sample\nitems:\n  - file: pb/box.pb\n    at: [1, 2, 3]\n",
            "{\"name\":\"sample\",\"items\":[{\"file\":\"pb/box.pb\",\"at\":[1,2,3]}]}",
            "name = \"sample\"\n[[items]]\nfile = \"pb/box.pb\"\nat = [1, 2, 3]\n",
        ];
        for form in forms {
            let manifest = Manifest::parse(form.as_bytes()).unwrap();
            assert!(manifest.texts.is_empty());
            assert_eq!(manifest.name, "sample");
            assert_eq!(manifest.items[0].file, "pb/box.pb");
            assert_eq!(manifest.items[0].at, Some([1.0, 2.0, 3.0]));
        }
    }

    #[test]
    fn invalid_manifest_does_not_reach_gpu_preparation() {
        for source in [
            "items: [{file: ''}]",
            "items: [{file: box.pb, point_size: -.inf}]",
            "items: [{file: box.pb, at: [.nan, 0, 0]}]",
            "items: [",
        ] {
            assert!(Manifest::parse(source.as_bytes()).is_err(), "{source}");
        }
        assert!(Manifest::parse(&[0xff]).is_err());
    }
    /// Every supported syntax creates the same world frame without renderer-specific types.
    #[test]
    fn world_text_yaml_json_and_toml_are_equivalent() {
        let forms = [
            "items: []\ntexts:\n  - text: 'Fixed Ω cube'\n    at: [1, 2, 3]\n    right: [0, 1, 0]\n    up: [0, 0, -1]\n    height: 12.5\n",
            r#"{"items":[],"texts":[{"text":"Fixed Ω cube","at":[1,2,3],"right":[0,1,0],"up":[0,0,-1],"height":12.5}]}"#,
            "items = []\n[[texts]]\ntext = \"Fixed Ω cube\"\nat = [1, 2, 3]\nright = [0, 1, 0]\nup = [0, 0, -1]\nheight = 12.5\n",
        ];
        for form in forms {
            let manifest = Manifest::parse(form.as_bytes()).unwrap();
            assert_eq!(manifest.texts.len(), 1);
            let text = &manifest.texts[0];
            assert_eq!(text.text, "Fixed Ω cube");
            assert_eq!(text.at, [1.0, 2.0, 3.0]);
            assert_eq!(text.right, [0.0, 1.0, 0.0]);
            assert_eq!(text.up, [0.0, 0.0, -1.0]);
            assert_eq!(text.height, 12.5);
        }
    }

    /// Text defaults are explicit while required geometry-list and height fields stay required.
    #[test]
    fn world_text_defaults_preserve_manifest_compatibility() {
        let manifest = Manifest::parse(b"items: []\ntexts: [{text: label, height: 2}]").unwrap();
        let text = &manifest.texts[0];
        assert_eq!(text.at, [0.0; 3]);
        assert_eq!(text.right, [1.0, 0.0, 0.0]);
        assert_eq!(text.up, [0.0, 1.0, 0.0]);
        assert!(Manifest::parse(b"texts: [{text: label, height: 2}]").is_err());
        assert!(Manifest::parse(b"items: []\ntexts: [{text: label}]").is_err());
    }

    /// Bad values fail as parse errors, before any partially valid record can reach the loader.
    #[test]
    fn invalid_world_text_frames_and_heights_are_recoverable() {
        for record in [
            "{text: '', height: 1}",
            "{text: '   ', height: 1}",
            "{text: label, height: 0}",
            "{text: label, height: -1}",
            "{text: label, height: .inf}",
            "{text: label, height: .nan}",
            "{text: label, height: 1, at: [0, .inf, 0]}",
            "{text: label, height: 1, right: [0, 0, 0]}",
            "{text: label, height: 1, right: [2, 0, 0]}",
            "{text: label, height: 1, right: [.nan, 0, 0]}",
            "{text: label, height: 1, up: [1, 0, 0]}",
            "{text: label, height: 1, up: [0, 1, .inf]}",
        ] {
            let source = format!("items: []\ntexts: [{record}]");
            assert!(Manifest::parse(source.as_bytes()).is_err(), "{record}");
        }
        assert!(Manifest::parse(b"items: []\ntexts: [{text: valid, height: 1}]").is_ok());
    }

    /// Slight decimal rounding in an authored unit frame is accepted without silently normalizing it.
    #[test]
    fn world_text_axis_tolerance_is_bounded() {
        let source = b"items: []\ntexts: [{text: label, height: 1, right: [1.0000005, 0, 0], up: [0.0000005, 1, 0]}]";
        let manifest = Manifest::parse(source).unwrap();
        assert_eq!(manifest.texts[0].right[0], 1.0000005);
        for source in [
            b"items: []\ntexts: [{text: label, height: 1, right: [1.000002, 0, 0]}]".as_slice(),
            b"items: []\ntexts: [{text: label, height: 1, up: [0.000002, 1, 0]}]".as_slice(),
        ] {
            assert!(Manifest::parse(source).is_err());
        }
    }

    /// Count UTF-8 bytes across all records, including multibyte text and the exact inclusive limit.
    #[test]
    fn world_text_content_and_record_limits_are_enforced() {
        let half = "é".repeat(65_536);
        let mut records = vec![serde_json::json!({"text":half,"height":1}); 2];
        let accepted = serde_json::json!({"items":[],"texts":records});
        assert!(Manifest::parse(accepted.to_string().as_bytes()).is_ok());
        records.push(serde_json::json!({"text":"x","height":1}));
        let rejected = serde_json::json!({"items":[],"texts":records});
        assert!(
            Manifest::parse(rejected.to_string().as_bytes())
                .err()
                .unwrap()
                .contains("256 KiB")
        );
        let mut records = vec![serde_json::json!({"text":"x","height":1}); 1024];
        let accepted = serde_json::json!({"items":[],"texts":records});
        assert!(Manifest::parse(accepted.to_string().as_bytes()).is_ok());
        records.push(serde_json::json!({"text":"x","height":1}));
        let rejected = serde_json::json!({"items":[],"texts":records});
        assert!(
            Manifest::parse(rejected.to_string().as_bytes())
                .err()
                .unwrap()
                .contains("1,024")
        );
    }
}
