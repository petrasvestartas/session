# 12.1.1 · Parse and bound original scene manifests

**Chapter 12 · Files and streaming · Section 12.1 · Scene manifests and range reads** · [Contents](contents.md)

**Typing: 29–58 minutes.** [Estimate](typing-load.md).

Parse supported manifest forms, then bound bytes, record counts and text content. Validate source placements, decoded sizes and affine matrices before accepting any scene records.

## Type

Continue from [Validate original manifest text and source keys](65zb-validate-original-manifest-text-and-source-keys.md). [Save or recover your work](recovery.md).

### 1. `src/manifest.rs`

Extend this original owner with today’s complete helper.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
}

/// The line and column of a YAML error, if known.
```

</details>

Replace it with:

```rust
}

/// The line and column of a YAML error, if known.
fn yaml_at(e: &serde_yaml_ng::Error) -> String {
    match e.location() {
        Some(l) => format!(" (line {}, column {})", l.line(), l.column()),
        None => String::new(),
    }
}

/// True for a content-addressed key, whose bytes never change: the browser cache may answer it.
```

<details>
<summary>Locate this block</summary>

```rust
    /// Item `i`'s placement, or its auto grid slot.
    pub fn place(&self, i: usize, cell: [f64; 2]) -> Xform {
        match self.items[i].placement() {
```

</details>

Replace it with:

```rust
    /// Parse YAML, JSON or TOML and check every value.
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
            Err(yaml) => match toml_form(text) {
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

            if !item.encoding.is_empty() && item.size == 0 {
                return Err(format!(
                    "item {index}: an encoded file needs its decoded size"
                ));
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

    /// Item `i`'s placement, or its auto grid slot.
    pub fn place(&self, i: usize, cell: [f64; 2]) -> Xform {
        match self.items[i].placement() {
```

<details>
<summary>Complete updated block reference</summary>

```rust
--8<-- "journey/code/66-read-scene-manifests-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/manifest.rs`

Supply only the unchanged original validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use serde::Deserialize;
use session_rust::Xform;

/// One manifest entry: a file and its placement.
#[derive(Clone, Deserialize)]
pub struct Item {
    pub file: String, // path like `pb/box.pb`, relative to the scene
    #[serde(default)]
    pub name: String, // display name, empty = the file's own
    #[serde(default)]
    pub at: Option<[f64; 3]>, // translation in world units
    #[serde(default)]
    pub xform: Option<[f64; 16]>, // full matrix, wins over `at`
    #[serde(default)]
    pub point_size: f64, // cloud point size in px, 0 = the file's own
    #[serde(default)]
    pub display_only: bool, // old flag, no longer changes anything
    #[serde(default)]
    pub size: u64, // decoded bytes, 0 = unknown
    #[serde(default)]
    pub encoding: String, // stored Content-Encoding such as `gzip`, empty = identity
}

/// One text placed in the world.
#[derive(Clone, Debug, Deserialize)]
pub struct TextItem {
    pub text: String, // the text
    #[serde(default)]
    pub at: [f64; 3], // world origin of the text
    #[serde(default = "text_right")]
    pub right: [f64; 3], // unit direction of the text line
    #[serde(default = "text_up")]
    pub up: [f64; 3], // unit direction up the text
    pub height: f64,  // letter height in world units
    #[serde(default)]
    pub camera_facing: bool, // always face the camera
}

/// The parsed scene file.
#[derive(Clone, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub name: String, // scene name
    pub items: Vec<Item>, // geometry files
    #[serde(default)]
    pub texts: Vec<TextItem>, // world texts
}

impl Item {
    /// The decoded size of a file stored encoded: it is never range-read.
    pub fn encoded_size(&self) -> Option<u64> {
        (!self.encoding.is_empty()).then_some(self.size)
    }

    /// The item's placement, None for the auto grid.
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
    /// Parse YAML, JSON or TOML and check every value.
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
            Err(yaml) => match toml_form(text) {
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

            if !item.encoding.is_empty() && item.size == 0 {
                return Err(format!(
                    "item {index}: an encoded file needs its decoded size"
                ));
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

    /// Item `i`'s placement, or its auto grid slot.
    pub fn place(&self, i: usize, cell: [f64; 2]) -> Xform {
        match self.items[i].placement() {
            Some(placement) => placement,
            None => auto_grid(i, self.items.len(), cell),
        }
    }

    /// Item `i`'s name, or `fallback`.
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
    /// Check the text, height and plane axes.
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

/// World +X.
fn text_right() -> [f64; 3] {
    [1.0, 0.0, 0.0]
}

/// World +Y.
fn text_up() -> [f64; 3] {
    [0.0, 1.0, 0.0]
}

/// A translation matrix.
fn translation(at: [f64; 3]) -> Xform {
    Xform::translation(at[0], at[1], at[2])
}

/// True when any value is not finite.
fn nonfinite_transform<const N: usize>(values: [f64; N]) -> bool {
    !values.into_iter().all(f64::is_finite)
}

/// A TOML manifest.
#[cfg(feature = "toml-manifests")]
fn toml_form(text: &str) -> Result<Manifest, String> {
    toml::from_str(text).map_err(|error| error.to_string())
}

/// TOML is not read without the `toml-manifests` feature.
#[cfg(not(feature = "toml-manifests"))]
fn toml_form(_text: &str) -> Result<Manifest, String> {
    Err("not read by this build (feature toml-manifests)".to_string())
}

/// The line and column of a YAML error, if known.
fn yaml_at(e: &serde_yaml_ng::Error) -> String {
    match e.location() {
        Some(l) => format!(" (line {}, column {})", l.line(), l.column()),
        None => String::new(),
    }
}

/// True for a content-addressed key, whose bytes never change: the browser cache may answer it.
pub fn immutable_key(url: &str) -> bool {
    let name = url.rsplit('/').next().unwrap_or(url);
    let stem = name.split('.').next().unwrap_or(name);
    let hash = stem.rsplit('-').next().unwrap_or(stem);
    hash.len() >= 16 && hash.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Grid slot `index` of `count`, `cell` apart.
pub fn auto_grid(index: usize, count: usize, cell: [f64; 2]) -> Xform {
    let cols = (count as f64).sqrt().ceil().max(1.0) as usize;
    Xform::translation(
        (index % cols) as f64 * cell[0],
        (index / cols) as f64 * cell[1],
        0.0,
    )
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/66-read-scene-manifests-2.rs" skip=8245
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.1.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The complete manifest parser compiles. Browser fetching and atomic publication follow next.

### Recorded visual result

**Previous native render — current verification pending.**

The original cloud and local point marker remain unchanged.

![Native renderer output for parse and bound original scene manifests.](../screenshots/journey/65za-read-original-manifest-records-and-placements.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.5 · Read original manifest records and placements](65za-read-original-manifest-records-and-placements.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original family → pure calculation → result or refusal.



Why validate the whole manifest before publishing its scene?

A malformed late record must not leave an earlier partial scene adopted. Return checked records first; later lessons own the asynchronous loading and publication.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect the original YAML/JSON fixtures and the optional TOML feature checks. Trace one invalid late item and verify that parsing returns a refusal.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.1.1
npm --prefix ../session_tests run course -- save 12.1.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Inherited browser assertions are authored and unrun. Publication browser assertions connect at the final checkpoint; native software rendering does not establish Lenovo performance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.1.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
