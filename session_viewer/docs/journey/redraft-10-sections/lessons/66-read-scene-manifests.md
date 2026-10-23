# 12.1 · Complete viewer files and published scene sources

**Goal:** Connect browser save open and URL adoption. · **Steps:** 44 · **Source:** [app/manifest.rs](../../../../src/app/manifest.rs) · [app/session_io.rs](../../../../src/app/session_io.rs) · [app/stream.rs](../../../../src/app/stream.rs) · [app/range_gate.rs](../../../../src/app/range_gate.rs) · [app/fetch.rs](../../../../src/app/fetch.rs) · [app/loader.rs](../../../../src/app/loader.rs) · [app/live.rs](../../../../src/app/live.rs) (`src/app/manifest.rs,src/app/streaming.rs,src/app/archive.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 12.1 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/manifest.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/live_revision.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">44 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/manifest.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/streaming.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Parse and bound scene man…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. HTTP Range Read Permits</text>
  <rect x="596" y="54" width="208" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="602" y="60" width="58" height="24" rx="3" fill="#dbeafe" stroke="#2563eb" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="631" y="76" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Header</text>
  <rect x="668" y="60" width="66" height="24" rx="3" fill="#dcfce7" stroke="#16a34a" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="701" y="76" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Page [k]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="770" y="76" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">...EOF</text>
  <rect x="596" y="100" width="208" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="122" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Range: bytes=start..end</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Complete viewer files and p…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Archive &amp; Live Revision</text>
  <rect x="880" y="54" width="192" height="36" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="76" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Saved .session Archive</text>
  <rect x="880" y="100" width="192" height="36" rx="5" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="122" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">ETag / Revision Poll</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Wire Browser Save, Open,…</text>
</svg>
</div>

## <a id="checkpoint-66-read-scene-manifests"></a>1. Parse and bound scene manifests

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/manifest.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original family</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pure calculation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">result or refusal</text>
</svg>
</div>

**What this code does:** Parse supported manifest forms, then bound bytes, record counts and text content.

### 1. `src/manifest.rs`

Extend this original owner with today’s complete helper. Supply only the unchanged original validation fixtures.

**Locate this existing block:**

```rust
impl Manifest {
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
            return Err(format!(
                "text {index}: height must be positive and finite"
            ));
        }

        if nonfinite_transform(self.at) {
            return Err(format!("text {index}: non-finite position"));
        }

        for axis in [self.right, self.up] {
            if nonfinite_transform(axis) {
                return Err(format!(
                    "text {index}: non-finite plane axis"
                ));
            }

            let length =
                (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();

            if (length - 1.0).abs() > 1e-6 {
                return Err(format!(
                    "text {index}: plane axes must be unit length within 1e-6"
                ));
            }
        }

        let dot = self.right[0] * self.up[0]
            + self.right[1] * self.up[1]
            + self.right[2] * self.up[2];

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

Edit the existing block to match this code; keep unchanged lines:

```rust
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
                return Err(format!(
                    "item {index}: missing geometry file"
                ));
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
                && (matrix[3] != 0.0
                    || matrix[7] != 0.0
                    || matrix[11] != 0.0
                    || matrix[15] != 1.0)
            {
                return Err(format!(
                    "item {index}: placement must be affine"
                ));
            }
        }

        if manifest.texts.len() > 1024 {
            return Err("manifest exceeds 1,024 text records".to_string());
        }

        let mut text_bytes = 0usize;

        for (index, item) in manifest.texts.iter().enumerate() {
            text_bytes = text_bytes.saturating_add(item.text.len());

            if text_bytes > 256 * 1024 {
                return Err(
                    "manifest text exceeds 256 KiB of UTF-8 content".to_string(),
                );
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
            return Err(format!(
                "text {index}: height must be positive and finite"
            ));
        }

        if nonfinite_transform(self.at) {
            return Err(format!("text {index}: non-finite position"));
        }

        for axis in [self.right, self.up] {
            if nonfinite_transform(axis) {
                return Err(format!(
                    "text {index}: non-finite plane axis"
                ));
            }

            let length =
                (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();

            if (length - 1.0).abs() > 1e-6 {
                return Err(format!(
                    "text {index}: plane axes must be unit length within 1e-6"
                ));
            }
        }

        let dot = self.right[0] * self.up[0]
            + self.right[1] * self.up[1]
            + self.right[2] * self.up[2];

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

#[cfg(test)]
mod tests {
    use super::*;

    /// The three formats parse the same scene.
    #[test]
    fn yaml_json_and_toml_have_identical_scene_semantics() {
        let forms = [
            "name: sample\nitems:\n  - file: pb/box.pb\n    at: [1, 2, 3]\n",
            "{\"name\":\"sample\",\"items\":[{\"file\":\"pb/box.pb\",\"at\":[1,2,3]}]}",
            "name = \"sample\"\n[[items]]\nfile = \"pb/box.pb\"\nat = [1, 2, 3]\n",
        ];

        for form in
            &forms[..forms.len() - usize::from(cfg!(not(feature = "toml-manifests")))]
        {
            let manifest = Manifest::parse(form.as_bytes()).unwrap();
            assert!(manifest.texts.is_empty());
            assert_eq!(manifest.name, "sample");
            assert_eq!(manifest.items[0].file, "pb/box.pb");
            assert_eq!(manifest.items[0].at, Some([1.0, 2.0, 3.0]));
        }
    }

    /// Encoded items carry their decoded size; plain items need neither field.
    #[test]
    fn encoded_items_carry_their_decoded_size() {
        let manifest = Manifest::parse(
            b"items: [{file: a.pb, size: 90, encoding: gzip}, {file: b.pb}]",
        )
        .unwrap();
        assert_eq!(manifest.items[0].encoded_size(), Some(90));
        assert_eq!(manifest.items[1].encoded_size(), None);
        assert!(Manifest::parse(b"items: [{file: a.pb, encoding: gzip}]").is_err());
    }

    /// Only hashed names skip revalidation.
    #[test]
    fn immutable_keys_are_the_hashed_names() {
        let revision = "https://x.dev/pb/revisions/4f24e928c29eb9effa864768903b73f3fbb9038279ccdf8a87117e588b4c72f3.pb";
        assert!(immutable_key(revision));
        assert!(immutable_key(
            "pb/sheets/querschnitt-0123456789abcdef.pb"
        ));
        assert!(!immutable_key(
            "https://x.dev/pb/view_mixed_floor_model.pb"
        ));
        assert!(!immutable_key(
            "https://x.dev/scenes/view_mixed.yaml"
        ));
        assert!(!immutable_key(
            "pb/sheets/querschnitt-cafe.pb"
        ));
    }

    /// Bad items are refused.
    #[test]
    fn invalid_manifest_does_not_reach_gpu_preparation() {
        for source in [
            "items: [{file: ''}]",
            "items: [{file: box.pb, point_size: -.inf}]",
            "items: [{file: box.pb, at: [.nan, 0, 0]}]",
            "items: [",
        ] {
            assert!(
                Manifest::parse(source.as_bytes()).is_err(),
                "{source}"
            );
        }

        assert!(Manifest::parse(&[0xff]).is_err());
    }

    /// The three formats parse the same text.
    #[test]
    fn world_text_yaml_json_and_toml_are_equivalent() {
        let forms = [
            "items: []\ntexts:\n  - text: 'Fixed Ω cube'\n    at: [1, 2, 3]\n    right: [0, 1, 0]\n    up: [0, 0, -1]\n    height: 12.5\n",
            r#"{"items":[],"texts":[{"text":"Fixed Ω cube","at":[1,2,3],"right":[0,1,0],"up":[0,0,-1],"height":12.5}]}"#,
            "items = []\n[[texts]]\ntext = \"Fixed Ω cube\"\nat = [1, 2, 3]\nright = [0, 1, 0]\nup = [0, 0, -1]\nheight = 12.5\n",
        ];

        for form in
            &forms[..forms.len() - usize::from(cfg!(not(feature = "toml-manifests")))]
        {
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

    /// Text defaults apply; `items` and `height` stay required.
    #[test]
    fn world_text_defaults_preserve_manifest_compatibility() {
        let manifest =
            Manifest::parse(b"items: []\ntexts: [{text: label, height: 2}]").unwrap();
        let text = &manifest.texts[0];
        assert_eq!(text.at, [0.0; 3]);
        assert_eq!(text.right, [1.0, 0.0, 0.0]);
        assert_eq!(text.up, [0.0, 1.0, 0.0]);
        assert!(Manifest::parse(b"texts: [{text: label, height: 2}]").is_err());
        assert!(Manifest::parse(b"items: []\ntexts: [{text: label}]").is_err());
    }

    /// Bad text records are refused.
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
            assert!(
                Manifest::parse(source.as_bytes()).is_err(),
                "{record}"
            );
        }
        assert!(
            Manifest::parse(b"items: []\ntexts: [{text: valid, height: 1}]").is_ok()
        );
    }

    /// Axes may be off unit length by 1e-6.
    #[test]
    fn world_text_axis_tolerance_is_bounded() {
        let source = b"items: []\ntexts: [{text: label, height: 1, right: [1.0000005, 0, 0], up: [0.0000005, 1, 0]}]";
        let manifest = Manifest::parse(source).unwrap();
        assert_eq!(manifest.texts[0].right[0], 1.0000005);

        for source in [
            b"items: []\ntexts: [{text: label, height: 1, right: [1.000002, 0, 0]}]"
                .as_slice(),
            b"items: []\ntexts: [{text: label, height: 1, up: [0.000002, 1, 0]}]"
                .as_slice(),
        ] {
            assert!(Manifest::parse(source).is_err());
        }
    }

    /// Text bytes and record count have limits.
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
```

## <a id="checkpoint-66a-compare-live-byte-revisions"></a>2. Compare live byte revisions

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/live_revision.rs, src/l…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">URL</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">byte hash</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">changed-scene decision</text>
</svg>
</div>

**What this code does:** Write per-URL ETag and byte-hash records.

### 1. `src/live_revision.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// HTTP validators save transfers; only changed bytes require a scene replacement.
#[derive(Default)]
pub(crate) struct Revisions {
    etags: HashMap<String, String>,
    hashes: HashMap<String, u64>,
}

impl Revisions {
    pub fn etag(&self, url: &str) -> Option<String> {
        self.etags.get(url).cloned()
    }

    pub fn forget(&mut self, url: &str) {
        self.etags.remove(url);
        self.hashes.remove(url);
    }

    pub fn changed(&mut self, url: &str, etag: Option<String>, bytes: &[u8]) -> bool {
        if let Some(etag) = etag {
            self.etags.insert(url.to_string(), etag);
        } else {
            self.etags.remove(url);
        }
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        bytes.hash(&mut hasher);
        let hash = hasher.finish();
        self.hashes.insert(url.to_string(), hash) != Some(hash)
    }
}
```

### 2. `src/lib.rs`

Register byte-revision tracking.

**Locate this existing block:**

```rust
mod cloud_pick_tests;

#[cfg(test)]
mod cloud_shader_contract;
pub mod manifest;
mod shader_check;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod cloud_pick_tests;

pub mod manifest;

#[cfg(test)]
mod cloud_shader_contract;
pub mod live_revision;
mod shader_check;
```

### 3. `src/live_revision.rs`

Supply only the unchanged original validation fixtures.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_bytes_survive_strong_weak_missing_and_new_etags() {
        let mut revisions = Revisions::default();
        assert!(revisions.changed("scene", Some("a".into()), b"one"));
        for tag in [
            Some("W/a".into()),
            None,
            Some("b".into()),
            Some("b".into()),
        ] {
            assert!(!revisions.changed("scene", tag.clone(), b"one"));
            assert_eq!(revisions.etag("scene"), tag);
        }
        assert!(revisions.changed("scene", Some("b".into()), b"two"));
        assert!(revisions.changed("other", None, b"two"));
        revisions.forget("scene");
        assert_eq!(revisions.etag("scene"), None);
        assert!(revisions.changed("scene", None, b"two"));
        assert!(!revisions.changed("other", None, b"two"));
    }
}
```

## <a id="checkpoint-66b-validate-source-records-before-allocation"></a>3. Bound finite coordinates and packed integer reads

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_validate.rs, src…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable validator</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later archive loader</text>
</svg>
</div>

**What this code does:** Guard scalar finiteness, coordinate triples and arithmetic before allocating or indexing. Decode a bounded varint without shifts past sixty-three bits.

### 1. `src/source_validate.rs`

Guard scalar finiteness, coordinate triples and arithmetic before allocating or indexing. Decode a bounded varint without shifts past sixty-three bits.

Create the file and type:

```rust
use prost::Message;
use session_rust::proto;

/// Source objects a scene may hold.
pub const MAX_OBJECTS: usize = 2_000_000;

/// Why a scene with more objects is refused.
pub const TOO_MANY: &str = "scene exceeds two million source objects";

/// Deepest tree level a scene may have.
pub const MAX_DEPTH: usize = 64;

/// Why a scene with a deeper tree is refused.
pub const TOO_DEEP: &str = "scene hierarchy exceeds 64 levels";

/// Every value must be finite.
fn finite(values: &[f64], label: &str) -> Result<(), String> {
    for value in values {
        if !value.is_finite() {
            return Err(format!("{label}: non-finite value"));
        }
    }

    Ok(())
}

/// A finite array of whole xyz triples.
fn triples(values: &[f64], label: &str) -> Result<(), String> {
    if !values.len().is_multiple_of(3) {
        return Err(format!(
            "{label}: incomplete coordinate triple"
        ));
    }

    finite(values, label)
}

/// A protobuf decode error as a message.
fn decode_error(error: prost::DecodeError) -> String {
    format!("invalid element geometry: {error}")
}

/// The varint at `i` and its byte length.
pub fn varint(b: &[u8], mut i: usize) -> Option<(u64, usize)> {
    let (mut v, mut shift) = (0u64, 0u32);
    let start = i;

    loop {
        let byte = *b.get(i)?;

        if shift == 63 && byte > 1 {
            return None;
        }

        v |= ((byte & 0x7f) as u64) << shift;
        i += 1;

        if byte & 0x80 == 0 {
            return Some((v, i - start));
        }

        shift += 7;

        if shift > 63 {
            return None;
        }
    }
}

/// The highest control index a surface reads, None on overflow.
fn addressed_surface_controls(
    rows: usize,
    columns: usize,
    stride_u: usize,
    stride_v: usize,
    size: usize,
) -> Option<usize> {
    (rows - 1)
        .checked_mul(stride_u)?
        .checked_add((columns - 1).checked_mul(stride_v)?)?
        .checked_add(size)
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod manifest;

#[cfg(test)]
mod cloud_shader_contract;
pub mod live_revision;
mod shader_check;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod manifest;

pub mod live_revision;

#[cfg(test)]
mod cloud_shader_contract;
mod shader_check;
pub mod source_validate;
```

## <a id="checkpoint-66ba-validate-one-nurbs-knot-axis"></a>4. Validate one NURBS knot axis

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_validate.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable validator</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later archive loader</text>
</svg>
</div>

**What this code does:** Relate order, control count and knot count, then reject nonfinite or decreasing knots before a kernel constructor.

### 1. `src/source_validate.rs`

Relate order, control count and knot count, then reject nonfinite or decreasing knots before a kernel constructor.

**Locate this existing block:**

```rust
        .checked_add(size)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .checked_add(size)
}

/// Check order, count and knots; returns the knot count.
fn axis(order: i32, count: i32, knots: &[f64]) -> Result<usize, String> {
    if !(2..=64).contains(&order) || count < order || count > 1_000_000 {
        return Err("invalid or oversized NURBS order/control count".into());
    }

    if knots.len() != (order + count - 2) as usize {
        return Err("NURBS knot count does not match its order and controls".into());
    }

    finite(knots, "NURBS knots")?;

    for pair in knots.windows(2) {
        if pair[0] > pair[1] {
            return Err("NURBS knots are not monotonic".into());
        }
    }

    if knots[(order - 2) as usize] >= knots[(count - 1) as usize] {
        return Err("NURBS parameter domain is empty".into());
    }

    Ok(count as usize)
}
```

## <a id="checkpoint-66bb-read-bounded-json-count-and-knot-fields"></a>5. Read bounded JSON count and knot fields

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_validate.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable validator</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later archive loader</text>
</svg>
</div>

**What this code does:** Read unsigned JSON counts with defaults and explicit caps; reuse the knot-axis contract instead of writing a second validator.

### 1. `src/source_validate.rs`

Read unsigned JSON counts with defaults and explicit caps; reuse the knot-axis contract instead of writing a second validator.

**Locate this existing block:**

```rust
    Ok(count as usize)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(count as usize)
}

/// A JSON parse error as a message.
fn json_error(error: serde_json::Error) -> String {
    format!("invalid session JSON: {error}")
}

/// A count field, defaulted and capped.
fn json_count(
    source: &serde_json::Value,
    key: &str,
    default: u64,
    maximum: u64,
) -> Result<usize, String> {
    let count = match source.get(key) {
        Some(value) => match value.as_u64() {
            Some(value) => value,
            None => return Err(format!("invalid JSON {key}")),
        },
        None => default,
    };

    if count > maximum {
        return Err(format!(
            "JSON {key} exceeds its supported count"
        ));
    }

    Ok(count as usize)
}

/// Check one JSON knot vector.
fn json_axis(
    source: &serde_json::Value,
    order_key: &str,
    count: usize,
    knots_key: &str,
) -> Result<(), String> {
    let order = json_count(source, order_key, 4, 64)?;
    let values = source[knots_key]
        .as_array()
        .ok_or("missing JSON NURBS knots")?;
    let mut knots = Vec::with_capacity(values.len());

    for value in values {
        knots.push(
            value
                .as_f64()
                .ok_or("invalid JSON NURBS knot")?,
        );
    }

    axis(order as i32, count as i32, &knots)?;
    Ok(())
}
```

## <a id="checkpoint-66bc-match-json-control-layout-to-declared-counts"></a>6. Match JSON control layout to declared counts

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_validate.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable validator</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later archive loader</text>
</svg>
</div>

**What this code does:** Check rational coordinate width, rows, columns and curve stride against the actual arrays before constructor allocation.

### 1. `src/source_validate.rs`

Check rational coordinate width, rows, columns and curve stride against the actual arrays before constructor allocation.

**Locate this existing block:**

```rust
    axis(order as i32, count as i32, &knots)?;
    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    axis(order as i32, count as i32, &knots)?;
    Ok(())
}

/// Check one JSON curve or surface's controls against its counts.
fn json_controls(source: &serde_json::Value) -> Result<(), String> {
    let controls = source["control_points"]
        .as_array()
        .ok_or("JSON controls must be an array")?;
    let dimension = json_count(source, "dimension", 3, 3)?;

    if dimension < 2 {
        return Err("invalid JSON control dimension".into());
    }

    let size = dimension
        + usize::from(
            source["is_rational"]
                .as_bool()
                .unwrap_or(false),
        );

    if source.get("cv_count_u").is_some() {
        let rows = json_count(source, "cv_count_u", 0, 1_000_000)?;
        let columns = json_count(source, "cv_count_v", 0, 1_000_000)?;
        let total = rows as u64 * columns as u64;

        if total > 4_000_000 || controls.len() as u64 != total * size as u64 {
            return Err(
                "JSON surface controls disagree with their declared counts".into(),
            );
        }

        json_axis(source, "order_u", rows, "nurbsknots_u")?;
        json_axis(source, "order_v", columns, "nurbsknots_v")?;
    } else {
        let count = json_count(source, "cv_count", 0, 1_000_000)?;
        let stride = json_count(source, "cv_stride", size as u64, 4)?;

        if count != controls.len() || stride != size {
            return Err(
                "JSON curve controls disagree with their declared count or stride"
                    .into(),
            );
        }

        for control in controls {
            if control.as_array().map(Vec::len) != Some(size) {
                return Err("incomplete JSON curve control".into());
            }
        }

        json_axis(source, "order", count, "nurbsknots")?;
    }

    Ok(())
}
```

## <a id="checkpoint-66bd-walk-json-records-within-a-depth-limit"></a>7. Walk JSON records within a depth limit

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_validate.rs, src…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable validator</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later archive loader</text>
</svg>
</div>

**What this code does:** Use an explicit bounded stack to find original control records; reject excess nesting and reuse the already-written record validators.

### 1. `src/source_validate.rs`

Use an explicit bounded stack to find original control records; reject excess nesting and reuse the already-written record validators.

**Locate this existing block:**

```rust
        json_axis(source, "order", count, "nurbsknots")?;
    }

    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        json_axis(source, "order", count, "nurbsknots")?;
    }

    Ok(())
}

/// Check every NURBS record in session JSON before it is parsed.
pub fn json(text: &str) -> Result<(), String> {
    let source: serde_json::Value = serde_json::from_str(text).map_err(json_error)?;
    let mut pending = vec![(&source, 0usize)];

    while let Some((value, depth)) = pending.pop() {
        if depth > 64 {
            return Err("session JSON exceeds 64 nested levels".into());
        }

        match value {
            serde_json::Value::Object(fields) => {
                if fields.contains_key("control_points")
                    && (fields.contains_key("cv_count")
                        || fields.contains_key("cv_count_u"))
                {
                    json_controls(value)?;
                }

                for child in fields.values() {
                    pending.push((child, depth + 1));
                }
            }
            serde_json::Value::Array(values) => {
                for child in values {
                    pending.push((child, depth + 1));
                }
            }
            _ => {}
        }
    }

    Ok(())
}
```

### 2. `src/lib.rs`

Expose the pure original-record validator for a browser-console exercise.

**Locate this existing block:**

```rust
pub mod live_revision;

#[cfg(test)]
mod cloud_shader_contract;
mod shader_check;
pub mod source_validate;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod live_revision;

pub mod source_validate;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn validate_source_json(text: &str) -> Result<(), JsValue> {
    source_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

#[cfg(test)]
mod cloud_shader_contract;
mod shader_check;
```

## <a id="checkpoint-66be-validate-original-points-lines-and-placements"></a>8. Validate points, lines, and placements

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_validate.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable validator</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later archive loader</text>
</svg>
</div>

**What this code does:** Check original coordinate-bearing records and affine placement without changing source values or identity.

### 1. `src/source_validate.rs`

Check original coordinate-bearing records and affine placement without changing source values or identity.

**Locate this existing block:**

```rust
            _ => {}
        }
    }

    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            _ => {}
        }
    }

    Ok(())
}

/// Check one protobuf point.
pub fn point(source: &proto::Point) -> Result<(), String> {
    finite(
        &[source.x, source.y, source.z, source.width],
        "point",
    )
}

/// Check one protobuf line.
pub fn line(source: &proto::Line) -> Result<(), String> {
    if source.coords.len() != 6 {
        return Err("line needs exactly two endpoint triples".into());
    }

    finite(&source.coords, "line coordinates")?;
    finite(&source.dash, "line dashes")
}

/// Check one protobuf polyline.
pub fn polyline(source: &proto::Polyline) -> Result<(), String> {
    triples(&source.coords, "polyline coordinates")
}

/// Check one object placement.
pub fn xform(entry: &proto::XformEntry) -> Result<(), String> {
    let Some(transform) = &entry.xform else {
        return Err("missing object transform".into());
    };
    finite(&transform.matrix, "object transform")?;

    if transform.matrix.len() != 16
        || transform.matrix[3] != 0.0
        || transform.matrix[7] != 0.0
        || transform.matrix[11] != 0.0
        || transform.matrix[15] != 1.0
    {
        return Err("object placement must be a sixteen-value affine matrix".into());
    }

    Ok(())
}

/// Nothing to check.
pub fn any<T>(_: &T) -> Result<(), String> {
    Ok(())
}
```

## <a id="checkpoint-66bf-validate-original-mesh-indices-and-vertex-attributes"></a>9. Validate mesh indices and vertex attributes

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 9 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_validate.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable validator</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later archive loader</text>
</svg>
</div>

**What this code does:** Check source vertex keys, finite attributes, face rings and triangulation before constructing a mesh.

### 1. `src/source_validate.rs`

Check source vertex keys, finite attributes, face rings and triangulation before constructing a mesh.

**Locate this existing block:**

```rust
pub fn any<T>(_: &T) -> Result<(), String> {
    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn any<T>(_: &T) -> Result<(), String> {
    Ok(())
}

/// Every index must name a vertex.
fn mesh_indices(source: &proto::Mesh, indices: &[u64]) -> Result<(), String> {
    for index in indices {
        if !source.vertices.contains_key(index) {
            return Err(format!(
                "mesh references missing vertex {index}"
            ));
        }
    }

    Ok(())
}

/// Check one protobuf mesh.
pub fn mesh(source: &proto::Mesh) -> Result<(), String> {
    for (key, vertex) in &source.vertices {
        if *key > u32::MAX as u64 {
            return Err("mesh vertex key exceeds the browser index range".into());
        }

        finite(
            &[vertex.x, vertex.y, vertex.z],
            "mesh vertex",
        )?;

        for value in vertex.attributes.values() {
            if !value.is_finite() {
                return Err("mesh vertex has a non-finite attribute".into());
            }
        }
    }

    for face in source.faces.values() {
        if face.vertices.len() < 3 {
            return Err("mesh face has fewer than three vertices".into());
        }

        mesh_indices(source, &face.vertices)?;

        for hole in &face.holes {
            mesh_indices(source, &hole.vertices)?;
        }
    }

    for (face, triangles) in &source.triangulation {
        if !source.faces.contains_key(face)
            || !triangles.vertices.len().is_multiple_of(3)
        {
            return Err("invalid mesh triangulation record".into());
        }

        mesh_indices(source, &triangles.vertices)?;
    }

    Ok(())
}
```

## <a id="checkpoint-66bg-validate-curve-and-surface-control-addresses"></a>10. Validate curve and surface control addresses

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 10 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_validate.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable validator</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later archive loader</text>
</svg>
</div>

**What this code does:** Use checked stride arithmetic to locate the final addressed control and reuse the shared knot and scalar contracts.

### 1. `src/source_validate.rs`

Use checked stride arithmetic to locate the final addressed control and reuse the shared knot and scalar contracts.

**Locate this existing block:**

```rust
        mesh_indices(source, &triangles.vertices)?;
    }

    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        mesh_indices(source, &triangles.vertices)?;
    }

    Ok(())
}

/// Check one protobuf curve.
pub fn curve(source: &proto::NurbsCurve) -> Result<(), String> {
    if !(2..=3).contains(&source.dimension) {
        return Err("NURBS curve dimension must be 2 or 3".into());
    }

    let count = axis(
        source.order,
        source.cv_count,
        &source.nurbsknots,
    )?;
    let size = source.dimension as usize + usize::from(source.is_rational);

    if source.cvs.len() != count * size {
        return Err("NURBS curve control storage is incomplete".into());
    }

    finite(&source.cvs, "NURBS curve controls")
}

/// Check one protobuf surface.
pub fn surface(source: &proto::NurbsSurface) -> Result<(), String> {
    if source.dimension != 3 {
        return Err("NURBS surface dimension must be 3".into());
    }

    let rows = axis(
        source.order_u,
        source.cv_count_u,
        &source.nurbsknots_u,
    )?;
    let columns = axis(
        source.order_v,
        source.cv_count_v,
        &source.nurbsknots_v,
    )?;

    if rows.saturating_mul(columns) > 4_000_000 {
        return Err("surface exceeds four million controls".into());
    }

    let size = 3 + usize::from(source.is_rational);
    let stride_u = if source.cv_stride_u > 0 {
        source.cv_stride_u as usize
    } else {
        columns * size
    };
    let stride_v = if source.cv_stride_v > 0 {
        source.cv_stride_v as usize
    } else {
        size
    };
    let addressed = addressed_surface_controls(rows, columns, stride_u, stride_v, size);

    if !matches!(addressed, Some(length) if length <= source.cvs.len()) {
        return Err("NURBS surface control storage is incomplete".into());
    }

    finite(&source.cvs, "NURBS surface controls")?;

    if let Some(cached) = &source.cached_mesh {
        mesh(cached)?;
    }

    Ok(())
}
```

## <a id="checkpoint-66bh-validate-cloud-columns-and-octree-addresses"></a>11. Validate cloud columns and octree addresses

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 11 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_validate.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable validator</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later archive loader</text>
</svg>
</div>

**What this code does:** Match optional original cloud columns and octree ranges to actual point and node counts.

### 1. `src/source_validate.rs`

Match optional original cloud columns and octree ranges to actual point and node counts.

**Locate this existing block:**

```rust
        mesh(cached)?;
    }

    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        mesh(cached)?;
    }

    Ok(())
}

/// Check one protobuf point cloud.
pub fn cloud(source: &proto::PointCloud) -> Result<(), String> {
    triples(&source.coords, "point cloud")?;
    finite(&source.normals, "point normals")?;
    let points = source.coords.len() / 3;

    if (!source.normals.is_empty() && source.normals.len() != source.coords.len())
        || (!source.colors.is_empty() && source.colors.len() != points * 4)
        || (!source.point_ids.is_empty() && source.point_ids.len() != points)
    {
        return Err("point-cloud attribute counts disagree".into());
    }

    let nodes = source.lod_size.len();

    if nodes > 0 {
        if source.lod_min.len() != nodes * 3
            || source.lod_spacing.len() != nodes
            || source.lod_level.len() != nodes
            || source.lod_first.len() != nodes
            || source.lod_count.len() != nodes
            || source.lod_children.len() != nodes * 8
        {
            return Err("point-cloud octree arrays disagree".into());
        }

        finite(&source.lod_min, "octree bounds")?;
        finite(&source.lod_size, "octree sizes")?;

        for node in 0..nodes {
            if source.lod_first[node] < 0
                || source.lod_count[node] < 0
                || source.lod_first[node] as u64 + source.lod_count[node] as u64
                    > points as u64
            {
                return Err("octree range exceeds its source points".into());
            }
        }

        for child in &source.lod_children {
            if *child < -1 || *child >= nodes as i32 {
                return Err("octree references an absent child".into());
            }
        }
    }

    Ok(())
}
```

## <a id="checkpoint-66bi-validate-brep-element-payloads-and-shared-definitions"></a>12. Validate BRep Element payloads and shared definitions

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 12 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_validate.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable validator</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later archive loader</text>
</svg>
</div>

**What this code does:** Check BRep records and encoded Element geometry, then validate every shared definition without expanding instances into copied geometry.

### 1. `src/source_validate.rs`

Check BRep records and encoded Element geometry, then validate every shared definition without expanding instances into copied geometry.

**Locate this existing block:**

```rust
            }
        }
    }

    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            }
        }
    }

    Ok(())
}

/// Check one protobuf BRep.
pub fn brep(source: &proto::BRep) -> Result<(), String> {
    for item in &source.curves_2d {
        curve(item)?;
    }

    for item in &source.curves_3d {
        curve(item)?;
    }

    for item in &source.surfaces {
        surface(item)?;
    }

    for vertex in &source.vertices {
        if let Some(point) = &vertex.point {
            finite(&[point.x, point.y, point.z], "BRep vertex")?;
        }
    }

    Ok(())
}

/// Check one protobuf element's mesh or BRep.
pub fn element(source: &proto::Element) -> Result<(), String> {
    match source.geometry_type.as_str() {
        "Mesh" => mesh(
            &proto::Mesh::decode(source.geometry_data.as_slice())
                .map_err(decode_error)?,
        ),
        "BRep" => brep(
            &proto::BRep::decode(source.geometry_data.as_slice())
                .map_err(decode_error)?,
        ),
        _ => Ok(()),
    }
}

/// Check every definition record, and the definitions with `instances` within the cap.
pub fn definitions(d: &proto::Objects, instances: usize) -> Result<(), String> {
    let count = d.points.len()
        + d.lines.len()
        + d.polylines.len()
        + d.meshes.len()
        + d.pointclouds.len()
        + d.nurbscurves.len()
        + d.nurbssurfaces.len()
        + d.breps.len()
        + d.elements.len();

    if count + instances > MAX_OBJECTS {
        return Err(TOO_MANY.into());
    }

    for item in &d.points {
        point(item)?;
    }

    for item in &d.lines {
        line(item)?;
    }

    for item in &d.polylines {
        polyline(item)?;
    }

    for item in &d.meshes {
        mesh(item)?;
    }

    for item in &d.pointclouds {
        cloud(item)?;
    }

    for item in &d.nurbscurves {
        curve(item)?;
    }

    for item in &d.nurbssurfaces {
        surface(item)?;
    }

    for item in &d.breps {
        brep(item)?;
    }

    for item in &d.elements {
        element(item)?;
    }

    Ok(())
}
```

## <a id="checkpoint-66bj-validate-the-complete-original-protobuf-session"></a>13. Validate the complete protobuf session

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 13 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_validate.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable validator</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later archive loader</text>
</svg>
</div>

**What this code does:** Apply the existing per-record validators and enforce total original-object and hierarchy-depth budgets before allocation.

### 1. `src/source_validate.rs`

Apply the existing per-record validators and enforce total original-object and hierarchy-depth budgets before allocation.

**Locate this existing block:**

```rust
        element(item)?;
    }

    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        element(item)?;
    }

    Ok(())
}

/// Check every record of a protobuf session.
pub fn session(source: &proto::Session) -> Result<(), String> {
    if let Some(objects) = &source.objects {
        let count = objects.points.len()
            + objects.lines.len()
            + objects.polylines.len()
            + objects.meshes.len()
            + objects.pointclouds.len()
            + objects.nurbscurves.len()
            + objects.nurbssurfaces.len()
            + objects.breps.len()
            + objects.elements.len();

        if count > MAX_OBJECTS {
            return Err(TOO_MANY.into());
        }

        for item in &objects.points {
            point(item)?;
        }

        for item in &objects.lines {
            line(item)?;
        }

        for item in &objects.polylines {
            polyline(item)?;
        }

        for item in &objects.meshes {
            mesh(item)?;
        }

        for item in &objects.pointclouds {
            cloud(item)?;
        }

        for item in &objects.nurbscurves {
            curve(item)?;
        }

        for item in &objects.nurbssurfaces {
            surface(item)?;
        }

        for item in &objects.breps {
            brep(item)?;
        }

        for item in &objects.elements {
            element(item)?;
        }
    }

    for entry in &source.xforms {
        xform(entry)?;
    }

    if let Some(tree) = &source.tree
        && let Some(root) = &tree.root
    {
        let mut pending = vec![(root, 0usize)];

        while let Some((node, depth)) = pending.pop() {
            if depth > MAX_DEPTH {
                return Err(TOO_DEEP.into());
            }

            for child in &node.children {
                pending.push((child, depth + 1));
            }
        }
    }

    Ok(())
}
```

## <a id="checkpoint-66bk-connect-retained-session-validation-and-independent-witnesses"></a>14. Validate Retained Session Data

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 14 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_validate.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original record</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">reusable validator</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">later archive loader</text>
</svg>
</div>

**What this code does:** Reuse the protobuf validators for already-decoded original kernel objects and register independent malformed-record witnesses.

### 1. `src/source_validate.rs`

Reuse the protobuf validators for already-decoded original kernel objects and register independent malformed-record witnesses. Supply independent hostile-count and original-record rejection fixtures.

**Locate this existing block:**

```rust
                pending.push((child, depth + 1));
            }
        }
    }

    Ok(())
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                pending.push((child, depth + 1));
            }
        }
    }

    Ok(())
}

/// Check every geometry of a loaded session.
pub fn retained(source: &session_rust::Session) -> Result<(), String> {
    for item in &source.objects.meshes {
        mesh(&item.to_proto())?;
    }

    for item in &source.objects.pointclouds {
        cloud(&item.to_proto())?;
    }

    for item in &source.objects.nurbscurves {
        curve(&item.to_proto())?;
    }

    for item in &source.objects.nurbssurfaces {
        surface(&item.to_proto())?;
    }

    for item in &source.objects.breps {
        brep(&item.to_proto())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Huge or wrong counts are refused.
    #[test]
    fn hostile_counts_do_not_reach_kernel_allocations() {
        let source = proto::NurbsCurve {
            dimension: 3,
            order: 2,
            cv_count: i32::MAX,
            ..Default::default()
        };
        assert!(curve(&source).is_err());
        let source = proto::NurbsSurface {
            dimension: 3,
            order_u: 2,
            order_v: 2,
            cv_count_u: -1,
            cv_count_v: 2,
            ..Default::default()
        };
        assert!(surface(&source).is_err());
    }

    /// Bad indices and short cloud arrays are refused.
    #[test]
    fn missing_indices_and_partial_cloud_attributes_are_errors() {
        let mut source = proto::Mesh::default();
        source.faces.insert(
            0,
            proto::FaceData {
                vertices: vec![0, 1, 2],
                ..Default::default()
            },
        );
        assert!(mesh(&source).is_err());
        let cloud_source = proto::PointCloud {
            coords: vec![0., 0., 0.],
            point_ids: vec![7, 8],
            ..Default::default()
        };
        assert!(cloud(&cloud_source).is_err());
    }

    /// JSON counts must match the controls given.
    #[test]
    fn json_declared_controls_are_checked_before_constructor_allocation() {
        assert!(
            json(
                r#"{"control_points":[],"cv_count":18446744073709551615,"dimension":3}"#
            )
            .is_err()
        );
        assert!(
            json(r#"{"control_points":[],"cv_count_u":1000000,"cv_count_v":1000000}"#)
                .is_err()
        );
        assert!(json(r#"{"control_points":[[0,0,0],[1,0,0]],"cv_count":2,"dimension":3,"order":2,"nurbsknots":[0,1]}"#).is_ok());
    }
}
```

## <a id="checkpoint-66c-describe-the-saved-scene-metadata"></a>15. Define Saved Scene Metadata

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 15 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/archive.rs, src/lib.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">document payloads</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">small metadata</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked owners</text>
</svg>
</div>

**What this code does:** Describe the saved scene metadata.

### 1. `src/archive.rs`

Define original document framing and validate small presentation metadata.

Create the file and type:

```rust
use crate::text::TextLabel;
use prost::Message;
use serde::{Deserialize, Serialize};

pub const MAGIC: &[u8] = b"SESSION-VIEWER\x01\n";
pub const LIMIT: usize = 512 * 1024 * 1024;

// Each source document keeps its own GUID namespace; small presentation metadata does not replace its original geometry protobuf.
#[derive(Clone, PartialEq, Message)]
pub struct Archive {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub documents: Vec<Vec<u8>>,
    #[prost(bytes = "vec", tag = "2")]
    pub metadata: Vec<u8>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Document {
    pub name: String,
    pub place: [f64; 16],
    pub point_px: f32,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(default)]
    pub created_doc: Option<usize>,
    #[serde(default)]
    pub current_layer: Option<(usize, String)>,
    pub documents: Vec<Document>,
    pub hidden: Vec<(usize, String)>,
    #[serde(default)]
    pub locked: Vec<(usize, String)>,
    #[serde(default)]
    pub colors: Vec<(usize, String, [u8; 3])>,
    #[serde(default)]
    pub edge_colors: Option<Vec<(usize, String, [u8; 3])>>,
    pub texts: Vec<(String, TextLabel, bool)>,
    #[serde(default)]
    pub groups: Vec<(usize, String)>,
}
impl Metadata {
    pub fn validate(&self, count: usize) -> Result<(), String> {
        if self.documents.len() != count {
            return Err("Document inventory does not match".into());
        }
        if self
            .created_doc
            .is_some_and(|index| index >= count)
        {
            return Err("Created document index is outside the inventory".into());
        }
        if self.documents.iter().any(|doc| {
            !doc.place.into_iter().all(f64::is_finite) || !doc.point_px.is_finite()
        }) {
            return Err("Non-finite document placement".into());
        }
        let identity = |(doc, key): &(usize, String)| *doc < count && !key.is_empty();
        if self
            .current_layer
            .as_ref()
            .is_some_and(|key| !identity(key))
            || self
                .hidden
                .iter()
                .chain(&self.locked)
                .chain(&self.groups)
                .any(|key| !identity(key))
            || self
                .colors
                .iter()
                .chain(
                    self.edge_colors
                        .as_deref()
                        .unwrap_or_default(),
                )
                .any(|(doc, key, _)| *doc >= count || key.is_empty())
        {
            return Err("Metadata owner is outside the document inventory".into());
        }
        let mut keys = std::collections::HashSet::new();
        for (key, label, _) in &self.texts {
            if key.is_empty() || !keys.insert(key) {
                return Err("Text records need distinct source keys".into());
            }
            crate::text::validate_label(label).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod source_validate;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod source_validate;

pub mod archive;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-66d-encode-decode-the-viewer-file"></a>16. Encode and Decode the Viewer File

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 16 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/archive.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">file header</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked protobuf</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original documents</text>
</svg>
</div>

**What this code does:** Encode decode the viewer file.

### 1. `src/archive.rs`

Frame original source bytes and checked metadata without replacing geometry. Copy archive framing, original-source and compatibility regressions.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

pub fn validate_document(bytes: &[u8]) -> Result<(), String> {
    let source =
        session_rust::proto::Session::decode(bytes).map_err(|e| e.to_string())?;
    crate::source_validate::session(&source)?;
    if let Some(definitions) = source.definitions.as_ref() {
        crate::source_validate::definitions(
            definitions,
            source
                .objects
                .as_ref()
                .map_or(0, |objects| objects.instances.len()),
        )?;
    }
    Ok(())
}

pub struct Bundle {
    pub documents: Vec<Vec<u8>>,
    pub metadata: Metadata,
}
impl Bundle {
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > LIMIT {
            return Err("Session exceeds the 512 MiB file limit".into());
        }
        let payload = bytes
            .strip_prefix(MAGIC)
            .ok_or("Not a Session Viewer file")?;
        let archive = Archive::decode(payload).map_err(|e| e.to_string())?;
        let metadata: Metadata =
            serde_json::from_slice(&archive.metadata).map_err(|e| e.to_string())?;
        metadata.validate(archive.documents.len())?;
        for document in &archive.documents {
            validate_document(document)?;
        }
        Ok(Self {
            documents: archive.documents,
            metadata,
        })
    }
    pub fn encode(&self) -> Result<Vec<u8>, String> {
        self.metadata
            .validate(self.documents.len())?;
        let metadata = serde_json::to_vec(&self.metadata).map_err(|e| e.to_string())?;
        let size = self
            .documents
            .iter()
            .try_fold(metadata.len(), |size, document| {
                size.checked_add(document.len())
            })
            .ok_or("Session size overflow")?;
        if size > LIMIT {
            return Err("Session exceeds the 512 MiB file limit".into());
        }
        for document in &self.documents {
            validate_document(document)?;
        }
        let archive = Archive {
            documents: self.documents.clone(),
            metadata,
        };
        let size = MAGIC
            .len()
            .checked_add(archive.encoded_len())
            .ok_or("Session size overflow")?;
        if size > LIMIT {
            return Err("Session exceeds the 512 MiB file limit".into());
        }
        let mut bytes = Vec::with_capacity(size);
        bytes.extend_from_slice(MAGIC);
        archive
            .encode(&mut bytes)
            .map_err(|e| e.to_string())?;
        Ok(bytes)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn metadata() -> Metadata {
        Metadata {
            created_doc: Some(0),
            current_layer: Some((0, "layer".into())),
            documents: vec![
                Document {
                    name: "first".into(),
                    place: session_rust::Xform::translation(1_000_000_000.125, 0., 0.)
                        .m,
                    point_px: 3.,
                },
                Document {
                    name: "second".into(),
                    place: session_rust::Xform::identity().m,
                    point_px: 0.,
                },
            ],
            hidden: vec![(
                0,
                "00000000-0000-0000-0000-000000000001".into(),
            )],
            locked: vec![(
                1,
                "00000000-0000-0000-0000-000000000001".into(),
            )],
            colors: vec![(
                0,
                "00000000-0000-0000-0000-000000000001".into(),
                [2, 5, 7],
            )],
            edge_colors: Some(vec![(
                1,
                "00000000-0000-0000-0000-000000000001".into(),
                [0; 3],
            )]),
            texts: vec![],
            groups: vec![],
        }
    }
    fn documents() -> Vec<Vec<u8>> {
        [1.125, 2.25]
            .into_iter()
            .map(|x| {
                let mut cloud = session_rust::PointCloud::from_coords(
                    vec![x, 2.5, 3.75],
                    vec![],
                    vec![],
                )
                .to_proto();
                cloud.guid = "00000000-0000-0000-0000-000000000001".into();
                let objects = session_rust::proto::Objects {
                    pointclouds: vec![cloud],
                    ..Default::default()
                };
                session_rust::proto::Session {
                    name: "Source cloud".into(),
                    objects: Some(objects),
                    ..Default::default()
                }
                .encode_to_vec()
            })
            .collect()
    }
    #[test]
    fn archive_round_trip_keeps_document_namespaces_flags_and_precise_placements() {
        let bundle = Bundle {
            documents: documents(),
            metadata: metadata(),
        };
        let bytes = bundle.encode().unwrap();
        assert!(bytes.starts_with(MAGIC));
        let reopened = Bundle::decode(&bytes).unwrap();
        assert_eq!(reopened.documents, bundle.documents);
        assert_eq!(
            reopened.metadata.documents[0].place[12],
            1_000_000_000.125
        );
        assert_eq!(
            reopened.metadata.hidden,
            bundle.metadata.hidden
        );
        assert_eq!(
            reopened.metadata.locked,
            bundle.metadata.locked
        );
        assert_eq!(
            reopened.metadata.edge_colors,
            bundle.metadata.edge_colors
        );
        assert_eq!(
            Bundle::decode(&reopened.encode().unwrap())
                .unwrap()
                .metadata
                .documents[0]
                .place,
            bundle.metadata.documents[0].place
        );
    }
    #[test]
    fn archive_rejects_bad_headers_counts_owners_and_placements_before_scene_adoption()
    {
        assert!(Bundle::decode(b"invalid").is_err());
        assert!(Bundle::decode(MAGIC).is_err());
        let mut m = metadata();
        m.created_doc = Some(2);
        assert!(m.validate(2).is_err());
        m = metadata();
        m.hidden.push((2, "guid".into()));
        assert!(m.validate(2).is_err());
        m = metadata();
        m.documents[0].place[12] = f64::NAN;
        assert!(m.validate(2).is_err());
        m = metadata();
        assert!(m.validate(1).is_err());
        let wrong = Archive {
            documents: vec![vec![]],
            metadata: serde_json::to_vec(&metadata()).unwrap(),
        };
        let mut bytes = MAGIC.to_vec();
        wrong.encode(&mut bytes).unwrap();
        assert!(Bundle::decode(&bytes).is_err());
    }
    #[test]
    fn archive_checks_original_geometry_before_kernel_construction() {
        let curve = session_rust::proto::NurbsCurve {
            dimension: 3,
            order: 2,
            cv_count: i32::MAX,
            ..Default::default()
        };
        let source = session_rust::proto::Session {
            objects: Some(session_rust::proto::Objects {
                nurbscurves: vec![curve],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(validate_document(&source.encode_to_vec()).is_err());
        assert!(validate_document(&[0xff]).is_err());
        for document in documents() {
            validate_document(&document).unwrap();
            let source =
                session_rust::proto::Session::decode(document.as_slice()).unwrap();
            assert_eq!(
                source.objects.unwrap().pointclouds[0].guid,
                "00000000-0000-0000-0000-000000000001"
            );
        }
    }
    #[test]
    fn old_metadata_keeps_face_colour_fallback_and_default_identity_fields() {
        let source=br#"{"documents":[{"name":"old","place":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],"point_px":0}],"hidden":[],"colors":[[0,"guid",[12,34,56]]],"texts":[]}"#;
        let m: Metadata = serde_json::from_slice(source).unwrap();
        m.validate(1).unwrap();
        assert!(m.created_doc.is_none());
        assert!(m.current_layer.is_none());
        assert!(m.locked.is_empty());
        assert!(m.groups.is_empty());
        assert!(m.edge_colors.is_none());
        assert_eq!(
            m.edge_colors
                .unwrap_or_else(|| m.colors.clone()),
            m.colors
        );
    }
}
```

## <a id="checkpoint-66e-give-copied-lines-independent-identities"></a>17. Assign Independent IDs to Copied Lines

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 17 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original line</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">row identity</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">independent copy</text>
</svg>
</div>

**What this code does:** Give copied lines independent identities.

### 1. `src/scene.rs`

Separate line row identities from shared original sources.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                self.lines
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.prepared.source.guid())
            })
```

Replace it with:

```rust
                self.lines
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
```

**Locate this block:**

```rust
        } else if let Some(row) = self.lines.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            self.lines.push(row);
        } else if let Some(row) = self.paths.iter().find(|row| row.id == id) {
            let mut row = row.clone();
```

Replace it with:

```rust
        } else if let Some(row) = self.lines.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.lines.push(row);
        } else if let Some(row) = self.paths.iter().find(|row| row.id == id) {
            let mut row = row.clone();
```

**Locate this block:**

```rust
            prepared.source.is_visible,
            prepared.source.is_locked,
        );
        let guid = prepared.source.guid().to_owned();
        let label = prepared.source.name.clone();
        self.lines.push(LineObject {
            id,
            prepared,
            model: session_rust::Xform::identity(),
```

Replace it with:

```rust
            prepared.source.is_visible,
            prepared.source.is_locked,
        );
        let mut guid = prepared.source.guid().to_owned();
        while self.lines.iter().any(|row| row.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let label = prepared.source.name.clone();
        self.lines.push(LineObject {
            guid: guid.clone(),
            id,
            prepared,
            model: session_rust::Xform::identity(),
```

**Locate this block:**

```rust
pub struct LineObject {
    pub id: ObjectId,
    pub prepared: crate::stroke::PreparedLine,
    pub model: session_rust::Xform,
```

Replace it with:

```rust
pub struct LineObject {
    pub guid: String,
    pub id: ObjectId,
    pub prepared: crate::stroke::PreparedLine,
    pub model: session_rust::Xform,
```

## <a id="checkpoint-66f-snapshot-geometry-types"></a>18. Snapshot Geometry Types

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 18 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/geometry_snapshot.rs, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original CAD curves clouds</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source protobuf</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">independent row GUID</text>
</svg>
</div>

**What this code does:** Snapshot original geometry types.

### 1. `src/geometry_snapshot.rs`

Snapshot original sources and current flags without mutating their shared records.

Create the file and type:

```rust
use crate::scene::{ObjectId, Scene};
use session_rust::proto;
use std::collections::HashSet;

// Save original source records; f32 display triangles, sampled controls and LOD subsets are not editable source geometry.
pub fn objects(
    scene: &Scene,
    ids: &HashSet<ObjectId>,
) -> Result<proto::Objects, String> {
    let mut out = proto::Objects::default();
    let mut guids = HashSet::new();
    macro_rules! push {
        ($field:ident,$id:expr,$source:expr) => {{
            let id = $id;
            let mut source = $source;
            let guid = scene
                .row_guid(id)
                .ok_or("Saved source owner is missing")?;
            if !guids.insert(guid.to_owned()) {
                return Err(
                    "Source GUID occurs twice within one saved document".into(),
                );
            }
            source.guid = guid.to_owned();
            source.is_visible = (!scene.visible(id)).then_some(false);
            source.is_locked = scene
                .view
                .locked
                .contains(&id)
                .then_some(true);
            out.$field.push(source);
        }};
    }
    for row in scene
        .objects()
        .iter()
        .filter(|row| ids.contains(&row.id))
    {
        if let Some(cad) = row.cad.as_ref() {
            push!(breps, row.id, cad.source.to_proto());
        } else if let Some(surface) = row.surface.as_ref() {
            push!(
                nurbssurfaces,
                row.id,
                surface.source.to_proto()
            );
        } else {
            push!(
                meshes,
                row.id,
                row.geometry()
                    .ok_or("Reload editable sources before saving")?
                    .to_proto()
            );
        }
    }
    for row in scene
        .lines()
        .iter()
        .filter(|row| ids.contains(&row.id))
    {
        push!(lines, row.id, row.prepared.source.to_proto());
    }
    for row in scene
        .paths()
        .iter()
        .filter(|row| ids.contains(&row.id))
    {
        match &row.prepared.source {
            crate::chain::Source::Line(source) => {
                push!(lines, row.id, source.to_proto())
            }
            crate::chain::Source::Polyline(source) => {
                push!(polylines, row.id, source.to_proto())
            }
            crate::chain::Source::Curve(source, _) => {
                push!(nurbscurves, row.id, source.to_proto())
            }
        }
    }
    for row in scene
        .points()
        .iter()
        .filter(|row| ids.contains(&row.id))
    {
        push!(
            points,
            row.id,
            row.prepared.source.to_proto()
        );
    }
    for row in scene
        .clouds()
        .iter()
        .filter(|row| ids.contains(&row.id))
    {
        push!(
            pointclouds,
            row.id,
            row.prepared.source.to_proto()
        );
    }
    Ok(out)
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod archive;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod archive;

pub mod geometry_snapshot;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/geometry_snapshot.rs`

Copy source and integration regressions.

**Locate this existing block:**

```rust
    Ok(out)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    #[test]
    fn snapshot_retains_original_cad_curves_clouds_flags_and_independent_line_guids() {
        let mut scene = Scene::demo();
        scene.clear();
        let cad = Rc::new(crate::cad_display::example());
        let _ = cad.guid();
        let cad_before = cad.to_proto();
        let cad_id = scene.insert_cad(cad.clone()).unwrap();
        let surface = Rc::new(crate::surface::example());
        let _ = surface.guid();
        let surface_before = surface.to_proto();
        let surface_id = scene
            .insert_surface(surface.clone())
            .unwrap();
        let mut original_curve = session_rust::NurbsCurve::create(
            true,
            2,
            &[
                session_rust::Point::new(-0.6, -0.4, 0.),
                session_rust::Point::new(0.123456789012345, 0.6, 0.),
                session_rust::Point::new(0.6, -0.4, 0.),
            ],
        );
        assert!(original_curve.set_cv_4d(1, 0.123456789012345 * 4., 2.4, 0., 4.));
        let curve = Rc::new(original_curve);
        let _ = curve.guid();
        let curve_before = curve.to_proto();
        let curve_id = scene
            .insert_path(crate::chain::PreparedChain::curve(curve.clone()).unwrap())
            .unwrap();
        let mut proto = crate::cloud::example().to_proto();
        proto.point_ids[0] = u32::MAX;
        let cloud = Rc::new(session_rust::PointCloud::from_proto(proto));
        let _ = cloud.guid();
        let cloud_before = cloud.to_proto();
        let cloud_id = scene
            .insert_cloud(crate::cloud::Prepared::new(cloud.clone()).unwrap())
            .unwrap();
        let line = scene
            .insert_line(
                crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(
                    0., 0., 0., 1., 0., 0.,
                )))
                .unwrap(),
            )
            .unwrap();
        let copy = scene.copy_row(line, false).unwrap();
        Rc::make_mut(&mut scene.view).hide(&[cad_id], true);
        Rc::make_mut(&mut scene.view).lock(&[cloud_id], true);
        let ids = [
            cad_id, surface_id, curve_id, cloud_id, line, copy,
        ]
        .into_iter()
        .collect();
        let out = objects(&scene, &ids).unwrap();
        assert!(out.meshes.is_empty());
        assert_eq!(out.breps.len(), 1);
        assert_eq!(out.nurbssurfaces.len(), 1);
        assert_eq!(out.nurbscurves.len(), 1);
        assert_eq!(out.lines.len(), 2);
        assert_ne!(out.lines[0].guid, out.lines[1].guid);
        assert_eq!(out.breps[0].is_visible, Some(false));
        assert_eq!(out.pointclouds[0].is_locked, Some(true));
        assert_eq!(out.pointclouds[0].point_ids[0], u32::MAX);
        assert_eq!(
            out.pointclouds[0].coords,
            cloud_before.coords
        );
        assert_eq!(out.nurbscurves[0].cvs, curve_before.cvs);
        assert!(out.nurbscurves[0].is_rational);
        assert_eq!(out.nurbscurves[0].cvs[7], 4.);
        assert_eq!(cad.to_proto(), cad_before);
        assert_eq!(surface.to_proto(), surface_before);
        assert_eq!(curve.to_proto(), curve_before);
        assert_eq!(cloud.to_proto(), cloud_before);
        crate::source_validate::session(&proto::Session {
            objects: Some(out),
            ..Default::default()
        })
        .unwrap();
    }
}
```

## <a id="checkpoint-66g-retain-geometry-free-document-headers"></a>19. Retain Geometry-Free Document Headers

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 19 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/hierarchy.rs, src/layer…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source header</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">tree namespace</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source release</text>
</svg>
</div>

**What this code does:** Retain geometry free document headers.

### 1. `src/hierarchy.rs`

Retain lightweight original document metadata through layer edits.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    node.parent_guid = parent_guid.to_owned();
    let mut matrix = parent.clone();
    if let Some(id) = original.object {
        let Some(row) = scene.objects().iter().find(|r| r.id == id) else {
            return Ok(None);
        };
        node.name = row.guid.clone();
        matrix = row.model.clone();
        let local = &parent
            .inverse()
            .ok_or("Parent placement cannot be inverted for Save")?
            * &matrix;
        if local.m != Xform::identity().m {
            xforms.push(proto::XformEntry {
                guid: row.guid.clone(),
                xform: Some(local.to_proto()),
```

Replace it with:

```rust
    node.parent_guid = parent_guid.to_owned();
    let mut matrix = parent.clone();
    if let Some(id) = original.object {
        let Some(guid) = scene.row_guid(id) else {
            return Ok(None);
        };
        node.name = guid.to_owned();
        matrix = scene
            .placement(id)
            .ok_or("Saved source placement is missing")?;
        let local = &parent
            .inverse()
            .ok_or("Parent placement cannot be inverted for Save")?
            * &matrix;
        if local.m != Xform::identity().m {
            xforms.push(proto::XformEntry {
                guid: guid.to_owned(),
                xform: Some(local.to_proto()),
```

**Locate this block:**

```rust
                .filter_map(|node| node.object),
        );
        self.documents.push(Rc::new(Document {
            key,
            label,
            nodes,
        }));
    }

    pub fn colours(&self, key: u32) -> HashMap<ObjectId, [u8; 3]> {
        let mut out = HashMap::new();
        let Some(doc) = self
```

Replace it with:

```rust
                .filter_map(|node| node.object),
        );
        self.documents.push(Rc::new(Document {
            header: None,
            key,
            label,
            nodes,
        }));
    }

    pub(crate) fn push(&mut self, document: Document) {
        Rc::make_mut(&mut self.readonly_ids).extend(
            document
                .nodes
                .iter()
                .filter(|node| node.readonly)
                .filter_map(|node| node.object),
        );
        self.documents.push(Rc::new(document));
    }

    pub fn colours(&self, key: u32) -> HashMap<ObjectId, [u8; 3]> {
        let mut out = HashMap::new();
        let Some(doc) = self
```

**Locate this block:**

```rust
pub struct Document {
    pub key: u32,
    pub label: String,
    pub nodes: Vec<Node>,
```

Replace it with:

```rust
pub struct Document {
    pub header: Option<Rc<crate::origin::Origin>>,
    pub key: u32,
    pub label: String,
    pub nodes: Vec<Node>,
```

### 2. `src/layer_edit.rs`

Retain lightweight original document metadata through layer edits.

**Locate this existing block:**

```rust
    let mut document = Document {
        key: old.key,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut document = Document {
        header: old.header.clone(),
        key: old.key,
```

### 3. `src/scene.rs`

Attach geometry-free original headers to imported namespaces.

**Locate this existing block:**

```rust
        );
        let colours = self.forest.colours(key);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        );
        if let Some(origin) = objects
            .first()
            .and_then(|(_, id, _)| self.objects.iter().find(|row| row.id == *id))
            .and_then(|row| row.origin())
            .cloned()
        {
            Rc::make_mut(self.forest.documents.last_mut().unwrap()).header =
                Some(origin);
        }
        let colours = self.forest.colours(key);
```

### 4. `src/revision.rs`

Include original nonmesh document revisions without owning source geometry.

**Locate this existing block:**

```rust
        })
        .collect();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        })
        .chain(
            scene
                .forest
                .documents
                .iter()
                .filter_map(|doc| {
                    doc.header
                        .as_ref()
                        .map(|origin| origin.version.hex())
                }),
        )
        .collect();
```

### 5. `src/archive.rs`

Copy mixed tree and line-identity integration checks after their source ownership is connected.

**Locate this existing block:**

```rust
    #[test]
    fn old_metadata_keeps_face_colour_fallback_and_default_identity_fields() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    #[test]
    fn mixed_tree_snapshot_keeps_line_copy_identity_and_nonmesh_placements() {
        use std::rc::Rc;
        let mut scene = crate::scene::Scene::demo();
        scene.clear();
        let source = Rc::new(session_rust::Line::new(
            0., 0., 0., 1., 0., 0.,
        ));
        let _ = source.guid();
        let before = source.to_proto();
        let first = scene
            .insert_line(crate::stroke::PreparedLine::new(source.clone()).unwrap())
            .unwrap();
        let key = format!(
            "{}/{}",
            scene.forest.documents[0].key,
            scene.forest.documents[0].nodes[0]
                .source
                .guid
        );
        crate::layer_edit::apply(
            &mut scene,
            &key,
            crate::layer_edit::Edit::Copy(vec![first]),
        )
        .unwrap();
        let copy = scene.lines()[1].id;
        assert_ne!(scene.row_guid(first), scene.row_guid(copy));
        assert!(Rc::ptr_eq(
            &scene.lines()[0].prepared.source,
            &scene.lines()[1].prepared.source
        ));
        scene
            .place_line(
                copy,
                session_rust::Xform::translation(1_000_000_000.125, 0., 0.),
            )
            .unwrap();
        let point = scene
            .insert_point(
                crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(
                    0., 2., 0.,
                )))
                .unwrap(),
            )
            .unwrap();
        let cloud = scene
            .insert_cloud(
                crate::cloud::Prepared::new(Rc::new(
                    session_rust::PointCloud::from_coords(
                        vec![0., 0., 0.],
                        vec![],
                        vec![],
                    ),
                ))
                .unwrap(),
            )
            .unwrap();
        let mut xforms = Vec::new();
        let (roots, placed) = scene
            .forest
            .snapshot(&scene, &mut xforms)
            .unwrap();
        for id in [first, copy, point, cloud] {
            assert!(placed.contains(&id));
        }
        assert_eq!(
            xforms
                .iter()
                .find(|entry| entry.guid == scene.row_guid(copy).unwrap())
                .unwrap()
                .xform
                .as_ref()
                .unwrap()
                .matrix[12],
            1_000_000_000.125
        );
        assert!(!roots.is_empty());
        assert_eq!(source.to_proto(), before);
    }
    #[test]
    fn old_metadata_keeps_face_colour_fallback_and_default_identity_fields() {
```

## <a id="checkpoint-66h-inventory-saved-document-namespaces"></a>20. Index Saved Document Namespaces

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 20 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene_archive.rs, src/l…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">live rows</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">document namespaces</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">ordered copy owners</text>
</svg>
</div>

**What this code does:** Inventory saved document namespaces.

### 1. `src/scene_archive.rs`

Inventory original document namespaces and deterministic owner order.

Create the file and type:

```rust
use crate::{
    archive::{Bundle, Document, Metadata},
    scene::{ObjectId, Scene},
};
use prost::Message;
use std::{collections::HashSet, rc::Rc};

// A namespace is one source document; imported files can reuse original GUIDs while their placements and presentation remain independent.
pub(crate) struct Namespace {
    pub name: String,
    pub doc: Option<Rc<crate::hierarchy::Document>>,
    pub ids: HashSet<ObjectId>,
}
impl Namespace {
    pub fn ordered(&self) -> Vec<ObjectId> {
        let mut ids = self.ids.iter().copied().collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id| id.value());
        ids
    }
}
fn inventory(scene: &Scene) -> Vec<Namespace> {
    let mut out = scene
        .forest
        .documents
        .iter()
        .map(|doc| Namespace {
            name: doc.label.clone(),
            doc: Some(doc.clone()),
            ids: doc
                .nodes
                .iter()
                .filter_map(|node| node.object)
                .filter(|id| scene.contains(*id))
                .collect(),
        })
        .collect::<Vec<_>>();
    let all = scene
        .objects()
        .iter()
        .map(|row| row.id)
        .chain(scene.lines().iter().map(|row| row.id))
        .chain(scene.paths().iter().map(|row| row.id))
        .chain(scene.points().iter().map(|row| row.id))
        .chain(scene.clouds().iter().map(|row| row.id))
        .chain(scene.texts().iter().map(|row| row.id))
        .collect::<HashSet<_>>();
    let assigned = out
        .iter()
        .flat_map(|namespace| namespace.ids.iter().copied())
        .collect::<HashSet<_>>();
    let ids = all
        .difference(&assigned)
        .copied()
        .collect::<HashSet<_>>();
    if !ids.is_empty() {
        out.push(Namespace {
            name: "Created".into(),
            doc: None,
            ids,
        });
    }
    out
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod geometry_snapshot;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod geometry_snapshot;

pub mod scene_archive;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-66i-map-saved-source-identities"></a>21. Map Saved Source Identities

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 21 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/archive_identity.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">row GUID</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">namespace alias</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source tree identity</text>
</svg>
</div>

**What this code does:** Map saved source identities.

### 1. `src/archive_identity.rs`

Map original sources, tree nodes and copies inside their document namespace.

Create the file and type:

```rust
use crate::{scene::Scene, scene_archive::Namespace};
use session_rust::proto;
use std::collections::{HashMap, HashSet};

pub(crate) fn aliases(
    scene: &Scene,
    namespace: &Namespace,
) -> Result<HashMap<String, String>, String> {
    let mut names = HashMap::new();
    let mut used = HashSet::new();
    let reserved = namespace
        .ids
        .iter()
        .filter_map(|id| scene.row_guid(*id).map(str::to_owned))
        .collect::<HashSet<_>>();
    if let Some(doc) = namespace.doc.as_ref() {
        for node in &doc.nodes {
            if let Some(id) = node
                .object
                .filter(|id| namespace.ids.contains(id))
            {
                let current = scene
                    .row_guid(id)
                    .ok_or("Saved source owner is missing")?
                    .to_owned();
                let mut saved = if scene.texts().iter().any(|row| row.id == id) {
                    current.clone()
                } else {
                    node.source.name.clone()
                };
                if saved != current && reserved.contains(&saved)
                    || used.contains(&saved)
                {
                    saved = current.clone();
                }
                if !used.insert(saved.clone()) {
                    return Err("Saved source identities collide".into());
                }
                names.insert(current, saved);
            }
        }
    }
    Ok(names)
}
pub(crate) fn objects(objects: &mut proto::Objects, names: &HashMap<String, String>) {
    macro_rules! remap {
        ($field:ident) => {
            for object in &mut objects.$field {
                if let Some(guid) = names.get(&object.guid) {
                    object.guid = guid.clone();
                }
            }
        };
    }
    remap!(meshes);
    remap!(lines);
    remap!(points);
    remap!(polylines);
    remap!(nurbscurves);
    remap!(nurbssurfaces);
    remap!(breps);
    remap!(pointclouds);
}
pub(crate) fn tree(node: &mut proto::TreeNode, names: &HashMap<String, String>) {
    if let Some(guid) = names.get(&node.name) {
        node.name = guid.clone();
    }
    for child in &mut node.children {
        tree(child, names);
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod scene_archive;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod scene_archive;

pub mod archive_identity;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-66j-store-cloud-size-as-presentation"></a>22. Store Point-Cloud Size in Presentation

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 22 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cloud.rs, src/cloud_gpu…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original point size</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">instance override</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">physical disc</text>
</svg>
</div>

**What this code does:** Store cloud size as presentation.

### 1. `src/cloud.rs`

Keep cloud point size independent of shared original source records.

**Locate this existing block:**

```rust
    pub controls_visible: bool,
    pub id: crate::scene::ObjectId,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub controls_visible: bool,
    pub point_px: f32,
    pub id: crate::scene::ObjectId,
```

### 2. `src/cloud_gpu.rs`

Derive draw radius from an instance override or the original source size.

**Locate this existing block:**

```rust
        .map_or([1., 1., 1.], |c| c.map(|v| v as f32 / 255.));
    let px = source.point_size as f32;
    let flags = u32::from(draw.selected)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .map_or([1., 1., 1.], |c| c.map(|v| v as f32 / 255.));
    let px = if row.point_px > 0. {
        row.point_px
    } else {
        source.point_size as f32
    };
    let flags = u32::from(draw.selected)
```

### 3. `src/scene.rs`

Allocate document keys and change cloud presentation without touching original source columns.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    .map(|row| row.guid.as_str())
            })
    }
```

Replace it with:

```rust
                    .map(|row| row.guid.as_str())
            })
    }

    pub(crate) fn reserve_document_key(&mut self) -> Result<u32, &'static str> {
        let key = self.next_id;
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        Ok(key)
    }

    pub(crate) fn cloud_point_size(
        &mut self,
        id: ObjectId,
        px: f32,
    ) -> Result<(), &'static str> {
        if !px.is_finite() {
            return Err("Non-finite cloud point size");
        }
        self.clouds
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Cloud object not found")?
            .point_px = px;
        Ok(())
    }
```

**Locate this block:**

```rust
        let row = crate::cloud::Row {
            controls_visible: false,
            id,
            guid: guid.clone(),
```

Replace it with:

```rust
        let row = crate::cloud::Row {
            point_px: 0.,
            controls_visible: false,
            id,
            guid: guid.clone(),
```

## <a id="checkpoint-66k-prepare-saved-documents"></a>23. Prepare Saved Scene Documents

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 23 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/archive_document.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source namespace</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">local placement</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">saved tree</text>
</svg>
</div>

**What this code does:** Prepare original saved documents.

### 1. `src/archive_document.rs`

Prepare original document records and precise relative placements for saving.

Create the file and type:

```rust
use crate::{scene::Scene, scene_archive::Namespace};
use session_rust::proto;
use std::collections::{HashMap, HashSet};

pub(crate) fn prepare(
    scene: &Scene,
    namespace: &Namespace,
) -> Result<(proto::Session, HashMap<String, String>, f32), String> {
    let names = crate::archive_identity::aliases(scene, namespace)?;
    let mut objects = crate::geometry_snapshot::objects(scene, &namespace.ids)?;
    crate::archive_identity::objects(&mut objects, &names);
    let mut xforms = Vec::new();
    let (mut roots, placed) = if let Some(doc) = namespace.doc.as_ref() {
        let mut forest = crate::hierarchy::Forest::default();
        forest.documents.push(doc.clone());
        forest
            .snapshot(scene, &mut xforms)
            .map_err(str::to_owned)?
    } else {
        (vec![], HashSet::new())
    };
    for id in namespace.ordered() {
        let current = scene
            .row_guid(id)
            .ok_or("Saved source owner is missing")?;
        let guid = names
            .get(current)
            .cloned()
            .unwrap_or_else(|| current.into());
        if !placed.contains(&id) {
            roots.push(proto::TreeNode {
                name: guid.clone(),
                ..Default::default()
            });
            let model = scene
                .placement(id)
                .ok_or("Saved source placement is missing")?;
            if model.m != session_rust::Xform::identity().m {
                xforms.push(proto::XformEntry {
                    guid,
                    xform: Some(model.to_proto()),
                });
            }
        }
    }
    for root in &mut roots {
        crate::archive_identity::tree(root, &names);
    }
    for entry in &mut xforms {
        if let Some(guid) = names.get(&entry.guid) {
            entry.guid = guid.clone();
        }
    }
    let mut source = namespace
        .doc
        .as_ref()
        .and_then(|doc| doc.header.as_ref())
        .map_or_else(proto::Session::default, |header| {
            header.header.clone()
        });
    if source.name.is_empty() {
        source.name = namespace.name.clone();
    }
    let root = if namespace.doc.is_some() && roots.len() == 1 {
        roots.pop().unwrap()
    } else {
        proto::TreeNode {
            name: source.name.clone(),
            children: roots,
            ..Default::default()
        }
    };
    source.objects = Some(objects);
    source.tree = Some(proto::Tree {
        root: Some(root),
        ..Default::default()
    });
    source.xforms = xforms;
    source.bvh_boxes.clear();
    let mut clouds = scene
        .clouds()
        .iter()
        .filter(|row| namespace.ids.contains(&row.id));
    let point_px = clouds.next().map_or(0., |row| row.point_px);
    if clouds.any(|row| row.point_px != point_px) {
        return Err(
            "Cloud placements in one saved document need the same point size override"
                .into(),
        );
    }
    Ok((source, names, point_px))
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod archive_identity;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod archive_identity;

pub mod archive_document;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-66l-save-document-display-settings"></a>24. Save Document Display Settings

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 24 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/archive_presentation.rs…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">document owner</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">display overrides</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">portable current layer</text>
</svg>
</div>

**What this code does:** Save document display settings.

### 1. `src/archive_presentation.rs`

Address presentation settings by saved original document identities.

Create the file and type:

```rust
use crate::{archive::Metadata, scene::Scene, scene_archive::Namespace};
use std::collections::HashMap;

pub(crate) fn append(
    metadata: &mut Metadata,
    index: usize,
    namespace: &Namespace,
    scene: &Scene,
    names: &HashMap<String, String>,
) -> Result<(), String> {
    for id in namespace.ordered() {
        let current = scene
            .row_guid(id)
            .ok_or("Saved source owner is missing")?;
        let guid = names
            .get(current)
            .cloned()
            .unwrap_or_else(|| current.to_owned());
        if !scene.visible(id) {
            metadata.hidden.push((index, guid.clone()));
        }
        if scene.view.locked.contains(&id) {
            metadata.locked.push((index, guid.clone()));
        }
        if let Some(rgb) = scene.view.faces.get(&id) {
            metadata
                .colors
                .push((index, guid.clone(), *rgb));
        }
        if let Some(rgb) = scene.view.edges.get(&id) {
            metadata
                .edge_colors
                .as_mut()
                .unwrap()
                .push((index, guid, *rgb));
        }
    }
    if let Some(doc) = namespace.doc.as_ref() {
        if scene.created == Some(doc.key) {
            metadata.created_doc = Some(index);
        }
        if let Some(current) = scene.view.current.as_ref() {
            if let Some(guid) = current.strip_prefix(&format!("{}/", doc.key)) {
                if let Some(node) = doc
                    .nodes
                    .iter()
                    .find(|node| node.source.guid == guid)
                {
                    metadata.current_layer = Some((index, node.portable_guid.clone()));
                }
            }
        }
    }
    Ok(())
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod archive_document;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod archive_document;

pub mod archive_presentation;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-66m-save-the-complete-supported-scene"></a>25. Serialize the Complete Scene

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 25 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene_archive.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source snapshots</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked metadata</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">viewer archive</text>
</svg>
</div>

**What this code does:** Save the complete supported scene.

### 1. `src/scene_archive.rs`

Encode original document payloads and current presentation as one bounded archive. Copy mixed scene, precision, source lifetime and repeated-namespace checks.

**Locate this existing block:**

```rust
    out
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    out
}

pub fn save(scene: &Scene) -> Result<Vec<u8>, String> {
    let mut namespaces = HashSet::new();
    let mut documents = Vec::new();
    let mut bytes_used = 0usize;
    let mut metadata = Metadata {
        created_doc: None,
        current_layer: None,
        documents: vec![],
        hidden: vec![],
        locked: vec![],
        colors: vec![],
        edge_colors: Some(vec![]),
        texts: vec![],
        groups: vec![],
    };
    for (index, namespace) in inventory(scene).iter().enumerate() {
        for id in &namespace.ids {
            if !namespaces.insert(*id) {
                return Err("Source owner occurs in two saved documents".into());
            }
        }
        let (source, names, point_px) =
            crate::archive_document::prepare(scene, namespace)?;
        crate::source_validate::session(&source)?;
        let bytes = source.encode_to_vec();
        bytes_used = bytes_used
            .checked_add(bytes.len())
            .ok_or("Session size overflow")?;
        if bytes_used > crate::archive::LIMIT {
            return Err("Session exceeds the 512 MiB file limit".into());
        }
        crate::archive_presentation::append(
            &mut metadata,
            index,
            namespace,
            scene,
            &names,
        )?;
        documents.push(bytes);
        metadata.documents.push(Document {
            name: namespace.name.clone(),
            place: session_rust::Xform::identity().m,
            point_px,
        });
    }
    for row in scene.texts() {
        let mut label = row
            .label(false, false)
            .map_err(str::to_owned)?;
        label.object = None;
        label.id = row.prepared.source.label.id;
        metadata.texts.push((
            row.guid.clone(),
            label,
            scene.visible(row.id),
        ));
    }
    metadata.hidden.sort();
    metadata.locked.sort();
    metadata.colors.sort();
    metadata.edge_colors.as_mut().unwrap().sort();
    metadata.texts.sort_by(|a, b| a.0.cmp(&b.0));
    Bundle {
        documents,
        metadata,
    }
    .encode()
}

#[cfg(test)]
mod tests {
    use super::*;
    use {session_rust::proto, std::collections::HashMap, std::rc::Rc};
    #[test]
    fn mixed_archive_keeps_original_source_types_copy_ids_and_precise_models() {
        let mut scene = Scene::demo();
        scene.clear();
        let cad = scene
            .insert_cad(Rc::new(crate::cad_display::example()))
            .unwrap();
        let cloud = Rc::new(crate::cloud::example());
        let _ = cloud.guid();
        let before = cloud.to_proto();
        let cloud_id = scene
            .insert_cloud(crate::cloud::Prepared::new(cloud.clone()).unwrap())
            .unwrap();
        let copy = scene.copy_row(cloud_id, false).unwrap();
        scene
            .place_cloud(
                copy,
                session_rust::Xform::translation(1_000_000_000.125, 0., 0.),
            )
            .unwrap();
        Rc::make_mut(&mut scene.view).hide(&[cad], true);
        let saved = Bundle::decode(&save(&scene).unwrap()).unwrap();
        let mut found = HashMap::new();
        for (index, bytes) in saved.documents.iter().enumerate() {
            let source = proto::Session::decode(bytes.as_slice()).unwrap();
            let objects = source.objects.unwrap();
            assert!(objects.meshes.is_empty());
            for c in objects.pointclouds {
                found.insert(c.guid.clone(), c);
            }
            if !objects.breps.is_empty() {
                assert_eq!(objects.breps[0].is_visible, Some(false));
                assert!(
                    saved
                        .metadata
                        .hidden
                        .iter()
                        .any(|(doc, guid)| *doc == index
                            && guid == scene.row_guid(cad).unwrap())
                );
            }
            if let Some(entry) = source
                .xforms
                .iter()
                .find(|entry| entry.guid == scene.row_guid(copy).unwrap())
            {
                assert_eq!(
                    entry.xform.as_ref().unwrap().matrix[12],
                    1_000_000_000.125
                );
            }
        }
        assert_eq!(found.len(), 2);
        assert_eq!(
            found[scene.row_guid(cloud_id).unwrap()].coords,
            before.coords
        );
        assert_eq!(
            found[scene.row_guid(copy).unwrap()].point_ids,
            before.point_ids
        );
        assert_eq!(cloud.to_proto(), before);
    }
    #[test]
    fn repeated_imported_documents_keep_independent_guid_namespaces_and_flags() {
        let mut scene = Scene::demo();
        scene.clear();
        let bytes = crate::specimen::bytes();
        scene
            .import(crate::document::load(&bytes).unwrap())
            .unwrap();
        let per_document = scene.objects().len();
        scene
            .import(crate::document::load(&bytes).unwrap())
            .unwrap();
        let a = scene.objects()[0].id;
        let b = scene.objects()[per_document].id;
        assert_ne!(scene.row_guid(a), scene.row_guid(b));
        assert_eq!(
            scene.objects()[0].geometry().unwrap().guid(),
            scene.objects()[per_document]
                .geometry()
                .unwrap()
                .guid()
        );
        Rc::make_mut(&mut scene.view).hide(&[a], true);
        Rc::make_mut(&mut scene.view).lock(&[b], true);
        let bundle = Bundle::decode(&save(&scene).unwrap()).unwrap();
        assert_eq!(bundle.documents.len(), 2);
        assert_eq!(bundle.metadata.hidden.len(), 1);
        assert_eq!(bundle.metadata.locked.len(), 1);
        assert_ne!(
            bundle.metadata.hidden[0].0,
            bundle.metadata.locked[0].0
        );
        assert_eq!(
            bundle.metadata.hidden[0].1,
            bundle.metadata.locked[0].1
        );
    }
}
```

## <a id="checkpoint-66n-prepare-document-sources"></a>26. Prepare Document Sources

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 26 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/document_source.rs, src…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source bytes</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">validation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original shared kernel do…</text>
</svg>
</div>

**What this code does:** Prepare original document sources.

### 1. `src/document_source.rs`

Validate original source types and retain their shared kernel document.

Create the file and type:

```rust
use prost::Message;
use session_rust::{Session, Xform, proto};
use std::{collections::HashSet, rc::Rc};

pub(crate) struct Source {
    pub document: Rc<Session>,
    pub origin: Rc<crate::origin::Origin>,
    pub tree: Option<proto::TreeNode>,
}
pub(crate) fn prepare(
    bytes: &[u8],
    place: &Xform,
    point_px: f32,
) -> Result<Source, String> {
    if bytes.len() > crate::archive::LIMIT {
        return Err("Session exceeds the 512 MiB file limit".into());
    }
    let message = proto::Session::decode(bytes).map_err(|e| e.to_string())?;
    crate::source_validate::session(&message)?;
    supported(&message)?;
    if !crate::placement::valid(&place.m) || !point_px.is_finite() {
        return Err("Invalid document placement or point size".into());
    }
    let tree = message
        .tree
        .as_ref()
        .and_then(|tree| tree.root.clone());
    let mut guids = HashSet::new();
    let objects = message
        .objects
        .as_ref()
        .ok_or("Session has no objects")?;
    macro_rules! identity {
        ($field:ident) => {
            for object in &objects.$field {
                if object.guid.is_empty() || !guids.insert(object.guid.as_str()) {
                    return Err(
                        "Source GUIDs must be distinct within a document".into(),
                    );
                }
            }
        };
    }
    identity!(meshes);
    identity!(points);
    identity!(lines);
    identity!(polylines);
    identity!(nurbscurves);
    identity!(nurbssurfaces);
    identity!(breps);
    identity!(pointclouds);
    if let Some(root) = tree.as_ref() {
        crate::hierarchy::validate(root, &guids).map_err(str::to_owned)?;
    }
    let mut placed = HashSet::new();
    for entry in &message.xforms {
        if !placed.insert(entry.guid.as_str())
            || !crate::placement::valid(
                &entry
                    .xform
                    .as_ref()
                    .ok_or("Missing placement matrix")?
                    .matrix,
            )
        {
            return Err("Invalid source placement".into());
        }
    }
    let origin = Rc::new(crate::origin::Origin::new(&message, bytes));
    let document = Rc::new(Session::from_proto(message).map_err(|e| e.to_string())?);
    Ok(Source {
        document,
        origin,
        tree,
    })
}

fn supported(source: &proto::Session) -> Result<(), String> {
    if source.definitions.is_some() || !source.interactions.is_empty() {
        return Err("Definitions and interactions arrive in later subjects".into());
    }
    let objects = source
        .objects
        .as_ref()
        .ok_or("Session has no objects")?;
    if !objects.planes.is_empty()
        || !objects.bboxes.is_empty()
        || !objects.elements.is_empty()
        || !objects.components.is_empty()
        || !objects.sheets.is_empty()
        || !objects.instances.is_empty()
    {
        return Err("This subject cannot open planes, boxes, assemblies, sheets or instances yet".into());
    }
    Ok(())
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod archive_presentation;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod archive_presentation;

pub mod document_source;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-66o-prepare-every-supported-geometry-row"></a>27. Prepare Geometry Rows for Export

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 27 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/document_geometry.rs, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source family</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">prepared row</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">placed world geometry</text>
</svg>
</div>

**What this code does:** Prepare every supported geometry row.

### 1. `src/document_geometry.rs`

Prepare each supported original source family and its independent world model.

Create the file and type:

```rust
use crate::{
    document_source::Source,
    scene::{ObjectId, Scene},
};
use session_rust::Xform;

pub(crate) fn append(
    scene: &mut Scene,
    loaded: &Source,
    place: &Xform,
    point_px: f32,
) -> Result<Vec<(String, ObjectId, String)>, String> {
    let mut owners = Vec::new();
    macro_rules! add {
        ($source:expr,$insert:expr) => {{
            let source = $source;
            let guid = source.guid().to_owned();
            let label = source.name.clone();
            let model = place
                * &crate::hierarchy::world(
                    loaded.tree.as_ref(),
                    &loaded.document.xforms,
                    &guid,
                );
            let id = $insert.map_err(str::to_owned)?;
            scene
                .place_following(id, model)
                .map_err(str::to_owned)?;
            owners.push((guid, id, label));
            id
        }};
    }
    for source in &loaded.document.objects.meshes {
        let mut prepared = crate::prepared::PreparedMesh::from_shared(source.clone())
            .map_err(str::to_owned)?;
        prepared.source = Some(crate::document::Source {
            document: loaded.document.clone(),
            guid: source.guid().to_owned(),
            origin: loaded.origin.clone(),
        });
        add!(source, scene.insert(prepared));
    }
    for source in &loaded.document.objects.points {
        add!(
            source,
            scene.insert_point(
                crate::marker::PreparedPoint::new(source.clone())
                    .map_err(str::to_owned)?
            )
        );
    }
    for source in &loaded.document.objects.lines {
        add!(
            source,
            scene.insert_line(
                crate::stroke::PreparedLine::new(source.clone())
                    .map_err(str::to_owned)?
            )
        );
    }
    for source in &loaded.document.objects.polylines {
        add!(
            source,
            scene.insert_path(
                crate::chain::PreparedChain::polyline(source.clone())
                    .map_err(str::to_owned)?
            )
        );
    }
    for source in &loaded.document.objects.nurbscurves {
        add!(
            source,
            scene.insert_path(
                crate::chain::PreparedChain::curve(source.clone())
                    .map_err(str::to_owned)?
            )
        );
    }
    for source in &loaded.document.objects.nurbssurfaces {
        add!(source, scene.insert_surface(source.clone()));
    }
    for source in &loaded.document.objects.breps {
        add!(source, scene.insert_cad(source.clone()));
    }
    for source in &loaded.document.objects.pointclouds {
        let id = add!(
            source,
            scene.insert_cloud(
                crate::cloud::Prepared::new(source.clone()).map_err(str::to_owned)?
            )
        );
        scene
            .cloud_point_size(id, point_px)
            .map_err(str::to_owned)?;
    }
    Ok(owners)
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod document_source;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod document_source;

pub mod document_geometry;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-66p-adopt-a-complete-source-document"></a>28. Load a Complete Source Document

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 28 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene_document.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">temporary scene</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">complete preparation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">atomic document adoption</text>
</svg>
</div>

**What this code does:** Adopt a complete source document.

### 1. `src/scene_document.rs`

Adopt original document rows atomically and preserve the existing source namespaces.

Create the file and type:

```rust
use crate::scene::Scene;
use session_rust::Xform;
use std::rc::Rc;
#[cfg(test)]
use {prost::Message, session_rust::proto, std::collections::HashSet};

// Prepare every original source in a temporary scene; one successful adoption prevents partially imported files from replacing the document.
pub fn import(
    scene: &mut Scene,
    bytes: &[u8],
    name: &str,
    place: &Xform,
    point_px: f32,
) -> Result<u32, String> {
    let source = crate::document_source::prepare(bytes, place, point_px)?;
    let mut candidate = scene.clone();
    let forest = candidate.forest.clone();
    let created = candidate.created;
    let current = candidate.view.current.clone();
    candidate.forest = Default::default();
    candidate.created = None;
    Rc::make_mut(&mut candidate.view).current = None;
    let key = candidate
        .reserve_document_key()
        .map_err(str::to_owned)?;
    let owners =
        crate::document_geometry::append(&mut candidate, &source, place, point_px)?;
    candidate.forest = forest;
    candidate.created = created;
    Rc::make_mut(&mut candidate.view).current = current;
    candidate.forest.add(
        key,
        if name.is_empty() {
            source.document.name.clone()
        } else {
            name.into()
        },
        source.tree,
        &owners,
    );
    Rc::make_mut(
        candidate
            .forest
            .documents
            .last_mut()
            .unwrap(),
    )
    .header = Some(source.origin);
    let colours = candidate.forest.colours(key);
    Rc::make_mut(&mut candidate.view)
        .faces
        .extend(colours);
    scene.restore(candidate);
    Ok(key)
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod document_geometry;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod document_geometry;

pub mod scene_document;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/scene.rs`

Prepare a source-free candidate before reopening complete documents.

**Locate this existing block:**

```rust
    }

    pub fn demo() -> Self {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn empty() -> Self {
        Self {
            created: None,
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
            clouds: Vec::new(),
            texts: Vec::new(),
            objects: Vec::new(),
            points: Vec::new(),
            paths: Vec::new(),
            lines: Vec::new(),
            next_id: 1,
            extra: None,
        }
    }

    pub fn demo() -> Self {
```

## <a id="checkpoint-66q-restore-world-labels-layer-links"></a>29. Restore World Labels and Layer Links

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 29 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/archive_restore_texts.r…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">saved world label</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original text row</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">saved layer link</text>
</svg>
</div>

**What this code does:** Restore world labels and layer links.

### 1. `src/archive_restore_texts.rs`

Restore saved world labels without serializing derived GPU addresses.

Create the file and type:

```rust
use crate::{archive::Metadata, scene::Scene};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub(crate) fn restore(scene: &mut Scene, metadata: &Metadata) -> Result<(), String> {
    let forest = scene.forest.clone();
    let mut texts = HashMap::new();
    for (key, label, active) in &metadata.texts {
        let mut label = label.clone();
        label.object = None;
        let source = crate::scene_label::Source {
            guid: key.clone(),
            name: label.text.clone(),
            visible: *active,
            locked: false,
            label,
        };
        let id = scene
            .insert_text(
                crate::scene_label::Prepared::new(Rc::new(source))
                    .map_err(str::to_owned)?,
            )
            .map_err(str::to_owned)?;
        texts.insert(key.clone(), id);
    }
    scene.forest = forest;
    scene.created = None;
    let mut linked = HashSet::new();
    for index in 0..scene.forest.documents.len() {
        let mut doc = (*scene.forest.documents[index]).clone();
        for node in &mut doc.nodes {
            node.portable_guid = node.source.guid.clone();
        }
        for node in &mut doc.nodes {
            if node.object.is_none() {
                if let Some(id) = texts.get(&node.source.name).copied() {
                    if !linked.insert(id) {
                        return Err("Text occurs in two saved layers".into());
                    }
                    node.object = Some(id);
                    node.label = scene
                        .texts()
                        .iter()
                        .find(|row| row.id == id)
                        .unwrap()
                        .prepared
                        .source
                        .name
                        .clone();
                }
            }
        }
        scene.forest.replace(index, doc);
    }
    for id in texts
        .values()
        .copied()
        .filter(|id| !linked.contains(id))
        .collect::<Vec<_>>()
    {
        let row = scene
            .texts()
            .iter()
            .find(|row| row.id == id)
            .unwrap();
        let guid = row.guid.clone();
        let label = row.prepared.source.name.clone();
        crate::layer_edit::attach_generated(scene, id, guid, label)
            .map_err(str::to_owned)?;
    }
    Ok(())
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod scene_document;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod scene_document;

pub mod archive_restore_texts;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-66r-restore-scoped-presentation-settings"></a>30. Restore Scoped Presentation Settings

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 30 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/archive_restore_flags.r…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">saved owner keys</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">new row identities</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">restored settings</text>
</svg>
</div>

**What this code does:** Restore scoped presentation settings.

### 1. `src/archive_restore_flags.rs`

Restore document-scoped metadata only after resolving every saved owner.

Create the file and type:

```rust
use crate::{
    archive::Metadata,
    scene::{ObjectId, Scene},
};
use std::{collections::HashMap, rc::Rc};

pub(crate) fn restore(
    scene: &mut Scene,
    metadata: &Metadata,
    keys: &[u32],
) -> Result<(), String> {
    let mut owners = HashMap::new();
    for (index, key) in keys.iter().enumerate() {
        let doc = scene
            .forest
            .documents
            .iter()
            .find(|doc| doc.key == *key)
            .unwrap();
        for node in &doc.nodes {
            if let Some(id) = node.object {
                owners.insert((index, node.source.name.as_str()), id);
            }
        }
    }
    let owner = |doc: usize, guid: &str| {
        owners
            .get(&(doc, guid))
            .copied()
            .ok_or_else(|| "Saved presentation owner is missing".to_string())
    };
    let hidden = metadata
        .hidden
        .iter()
        .map(|(doc, guid)| owner(*doc, guid))
        .collect::<Result<Vec<ObjectId>, _>>()?;
    let locked = metadata
        .locked
        .iter()
        .map(|(doc, guid)| owner(*doc, guid))
        .collect::<Result<Vec<ObjectId>, _>>()?;
    let colours = metadata
        .colors
        .iter()
        .map(|(doc, guid, rgb)| Ok((owner(*doc, guid)?, *rgb)))
        .collect::<Result<Vec<_>, String>>()?;
    let edges = metadata
        .edge_colors
        .as_ref()
        .unwrap_or(&metadata.colors)
        .iter()
        .map(|(doc, guid, rgb)| Ok((owner(*doc, guid)?, *rgb)))
        .collect::<Result<Vec<_>, String>>()?;
    drop(owners);
    let view = Rc::make_mut(&mut scene.view);
    view.hide(&hidden, true);
    view.lock(&locked, true);
    view.faces.extend(colours);
    view.edges.extend(edges);
    view.current = None;
    if let Some((index, guid)) = metadata.current_layer.as_ref() {
        let doc = scene
            .forest
            .documents
            .iter()
            .find(|doc| doc.key == keys[*index])
            .unwrap();
        let node = doc
            .nodes
            .iter()
            .find(|node| node.source.guid == *guid)
            .ok_or("Saved current layer is missing")?;
        let key = format!("{}/{}", doc.key, node.source.guid);
        crate::layer_edit::locate(scene, &key).map_err(str::to_owned)?;
        Rc::make_mut(&mut scene.view).current = Some(key);
    }
    if let Some(index) = metadata.created_doc {
        scene.created = Some(keys[index]);
    }
    Ok(())
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod archive_restore_texts;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod archive_restore_texts;

pub mod archive_restore_flags;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-66s-reopen-a-complete-viewer-file"></a>31. Reopen a Saved Viewer File

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 31 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene_document.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked archive</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">complete metadata</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one adoption</text>
</svg>
</div>

**What this code does:** Reopen a complete viewer file.

### 1. `src/scene_document.rs`

Reopen every original document and restore all supported scene presentation. Copy mixed-source, precision, tree-depth, namespace, release and atomic refusal tests.

**Locate this existing block:**

```rust
    Ok(key)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(key)
}

pub fn open(scene: &mut Scene, bytes: &[u8]) -> Result<(), String> {
    let bundle = crate::archive::Bundle::decode(bytes)?;
    if !bundle.metadata.groups.is_empty() {
        return Err("Object groups arrive in a later subject".into());
    }
    let mut candidate = scene.clone();
    candidate.clear();
    let mut keys = Vec::new();
    for (source, meta) in bundle
        .documents
        .iter()
        .zip(&bundle.metadata.documents)
    {
        keys.push(import(
            &mut candidate,
            source,
            &meta.name,
            &Xform::from_matrix(meta.place),
            meta.point_px,
        )?);
    }
    crate::archive_restore_texts::restore(&mut candidate, &bundle.metadata)?;
    crate::archive_restore_flags::restore(&mut candidate, &bundle.metadata, &keys)?;
    scene.restore(candidate);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mixed_save_open_restores_original_sources_placements_layers_and_presentation() {
        let mut scene = Scene::demo();
        scene.clear();
        let cad = scene
            .insert_cad(Rc::new(crate::cad_display::example()))
            .unwrap();
        let surface = scene
            .insert_surface(Rc::new(crate::surface::example()))
            .unwrap();
        let cloud = scene
            .insert_cloud(
                crate::cloud::Prepared::new(Rc::new(crate::cloud::example())).unwrap(),
            )
            .unwrap();
        let copy = scene.copy_row(cloud, false).unwrap();
        scene
            .place_cloud(
                copy,
                Xform::translation(1_000_000_000.125, 0., 0.),
            )
            .unwrap();
        scene.cloud_point_size(cloud, 5.).unwrap();
        scene.cloud_point_size(copy, 5.).unwrap();
        let point =
            scene
                .insert_point(
                    crate::marker::PreparedPoint::new(Rc::new(
                        session_rust::Point::new(0.123456789012345, 0., 0.),
                    ))
                    .unwrap(),
                )
                .unwrap();
        let line = scene
            .insert_line(
                crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(
                    -1., 0., 0., 1., 0., 0.,
                )))
                .unwrap(),
            )
            .unwrap();
        let label =
            crate::scene_label::example(crate::text::TextPlacement::WorldBillboard {
                world: [0., 0., 0.],
                world_height: 0.1,
            });
        let text = scene
            .insert_text(crate::scene_label::Prepared::new(Rc::new(label)).unwrap())
            .unwrap();
        scene
            .place_text(
                text,
                Xform::translation(0.123456789012345, 0., 0.),
            )
            .unwrap();
        let root = &scene.forest.documents[0];
        let current = format!("{}/{}", root.key, root.nodes[0].source.guid);
        let view = Rc::make_mut(&mut scene.view);
        view.current = Some(current);
        view.hide(&[cad, text], true);
        view.lock(&[surface, line, text], true);
        view.faces.insert(cad, [2, 5, 7]);
        view.edges.insert(cad, [0; 3]);
        let mut restored = Scene::demo();
        let old_ids = restored
            .objects()
            .iter()
            .map(|row| row.id.value())
            .collect::<Vec<_>>();
        let bytes = crate::scene_archive::save(&scene).unwrap();
        open(&mut restored, &bytes).unwrap();
        assert_eq!(restored.objects().len(), 2);
        assert_eq!(
            restored
                .objects()
                .iter()
                .filter(|row| row.cad.is_some())
                .count(),
            1
        );
        assert_eq!(
            restored
                .objects()
                .iter()
                .filter(|row| row.surface.is_some())
                .count(),
            1
        );
        assert_eq!(restored.clouds().len(), 2);
        assert_eq!(
            restored.clouds()[1].model.m[12],
            1_000_000_000.125
        );
        assert_eq!(
            restored.clouds()[0]
                .prepared
                .source
                .to_proto()
                .coords,
            scene.clouds()[0]
                .prepared
                .source
                .to_proto()
                .coords
        );
        assert!(
            restored
                .clouds()
                .iter()
                .all(|row| row.point_px == 5. && row.prepared.source.point_size == 3.)
        );
        assert_eq!(
            restored.points()[0].prepared.source[0],
            scene.points()[0].prepared.source[0]
        );
        assert_eq!(
            restored.points()[0].prepared.source[0],
            0.123456789012345
        );
        let _ = point;
        assert_eq!(restored.texts().len(), 1);
        assert_eq!(
            restored.texts()[0]
                .label(false, false)
                .unwrap()
                .placement,
            scene.texts()[0]
                .label(false, false)
                .unwrap()
                .placement
        );
        assert!(!restored.visible(restored.texts()[0].id));
        assert!(
            restored
                .view
                .locked
                .contains(&restored.texts()[0].id)
        );
        assert!(restored.view.current.is_some());
        assert!(restored.created.is_some());
        assert_eq!(restored.view.hidden.len(), 2);
        assert_eq!(restored.view.locked.len(), 3);
        assert_eq!(
            restored.view.faces[&restored
                .objects()
                .iter()
                .find(|row| row.cad.is_some())
                .unwrap()
                .id],
            [2, 5, 7]
        );
        assert_eq!(
            restored.view.edges[&restored
                .objects()
                .iter()
                .find(|row| row.cad.is_some())
                .unwrap()
                .id],
            [0; 3]
        );
        assert!(
            restored
                .objects()
                .iter()
                .all(|row| !old_ids.contains(&row.id.value()))
        );
        let again = crate::archive::Bundle::decode(
            &crate::scene_archive::save(&restored).unwrap(),
        )
        .unwrap();
        assert_eq!(again.documents.len(), 2);
        assert_eq!(again.metadata.hidden.len(), 2);
        assert_eq!(again.metadata.locked.len(), 3);
    }
    #[test]
    fn repeated_guid_documents_reopen_with_independent_flags_and_original_meshes() {
        let mut scene = Scene::demo();
        scene.clear();
        let raw = crate::specimen::bytes();
        scene
            .import(crate::document::load(&raw).unwrap())
            .unwrap();
        let n = scene.objects().len();
        scene
            .import(crate::document::load(&raw).unwrap())
            .unwrap();
        let a = scene.objects()[0].id;
        let b = scene.objects()[n].id;
        Rc::make_mut(&mut scene.view).hide(&[a], true);
        Rc::make_mut(&mut scene.view).lock(&[b], true);
        let saved = crate::scene_archive::save(&scene).unwrap();
        let mut restored = Scene::demo();
        open(&mut restored, &saved).unwrap();
        assert_eq!(restored.objects().len(), 2 * n);
        let a = &restored.objects()[0];
        let b = &restored.objects()[n];
        assert_ne!(a.guid, b.guid);
        assert_eq!(
            a.geometry().unwrap().guid(),
            b.geometry().unwrap().guid()
        );
        assert!(!restored.visible(a.id));
        assert!(restored.visible(b.id));
        assert!(!restored.view.locked.contains(&a.id));
        assert!(restored.view.locked.contains(&b.id));
        let document = Rc::downgrade(&a.source().unwrap().document);
        let ids = restored
            .objects()
            .iter()
            .take(n)
            .map(|row| row.id)
            .collect::<Vec<_>>();
        for id in ids {
            assert!(restored.remove(id));
        }
        assert!(document.upgrade().is_none());
    }
    #[test]
    fn rejected_files_and_late_display_preparation_errors_leave_the_scene_unchanged() {
        let mut scene = Scene::demo();
        let ids = scene
            .objects()
            .iter()
            .map(|row| row.id)
            .collect::<Vec<_>>();
        assert!(open(&mut scene, b"broken").is_err());
        assert_eq!(
            scene
                .objects()
                .iter()
                .map(|row| row.id)
                .collect::<Vec<_>>(),
            ids
        );
        let mut source =
            proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();
        source.tree = None;
        source
            .objects
            .as_mut()
            .unwrap()
            .points
            .push(session_rust::Point::new(1e100, 0., 0.).to_proto());
        let bytes = source.encode_to_vec();
        assert!(
            import(
                &mut scene,
                &bytes,
                "too large",
                &Xform::identity(),
                0.
            )
            .is_err()
        );
        assert_eq!(
            scene
                .objects()
                .iter()
                .map(|row| row.id)
                .collect::<Vec<_>>(),
            ids
        );
        assert!(scene.points().is_empty());
        assert!(
            scene
                .forest
                .documents
                .iter()
                .all(|doc| doc.label != "too large")
        );
        source
            .objects
            .as_mut()
            .unwrap()
            .points
            .clear();
        source.definitions = Some(proto::Objects::default());
        assert!(
            import(
                &mut scene,
                &source.encode_to_vec(),
                "unsupported",
                &Xform::identity(),
                0.
            )
            .unwrap_err()
            .contains("Definitions")
        );
        assert_eq!(
            scene
                .objects()
                .iter()
                .map(|row| row.id)
                .collect::<Vec<_>>(),
            ids
        );
    }
    #[test]
    fn document_placement_is_composed_without_mutating_original_source_coordinates() {
        let mut scene = Scene::demo();
        scene.clear();
        let cloud = crate::cloud::example();
        let _ = cloud.guid();
        let original = cloud.to_proto();
        let source = proto::Session {
            name: "precise cloud".into(),
            objects: Some(proto::Objects {
                pointclouds: vec![original.clone()],
                ..Default::default()
            }),
            ..Default::default()
        };
        let raw = source.encode_to_vec();
        let place = Xform::translation(1_000_000_000.125, 0., 0.);
        import(&mut scene, &raw, "placed", &place, 7.).unwrap();
        assert_eq!(scene.clouds()[0].model.m[12], place.m[12]);
        assert_eq!(
            scene.clouds()[0].prepared.source.to_proto(),
            original
        );
        assert_eq!(scene.clouds()[0].point_px, 7.);
    }
    #[test]
    fn repeated_roundtrips_keep_tree_depth_current_layer_source_header_and_precise_pose()
     {
        let mut source =
            proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();
        source.guid = "00000000-0000-0000-0000-000000000123".into();
        let raw = source.encode_to_vec();
        let mut scene = Scene::empty();
        import(
            &mut scene,
            &raw,
            "placed",
            &Xform::translation(0.123456789012345, 0., 0.),
            0.,
        )
        .unwrap();
        let doc = &scene.forest.documents[0];
        let current = format!("{}/{}", doc.key, doc.nodes[0].source.guid);
        Rc::make_mut(&mut scene.view).current = Some(current);
        let nodes = doc.nodes.len();
        let depth = doc
            .nodes
            .iter()
            .map(|node| node.depth)
            .max()
            .unwrap();
        for _ in 0..6 {
            let saved = crate::scene_archive::save(&scene).unwrap();
            let bundle = crate::archive::Bundle::decode(&saved).unwrap();
            assert_eq!(
                proto::Session::decode(bundle.documents[0].as_slice())
                    .unwrap()
                    .guid,
                source.guid
            );
            open(&mut scene, &saved).unwrap();
            assert_eq!(scene.forest.documents[0].nodes.len(), nodes);
            assert_eq!(
                scene.forest.documents[0]
                    .nodes
                    .iter()
                    .map(|node| node.depth)
                    .max()
                    .unwrap(),
                depth
            );
            assert_eq!(
                scene.objects()[0].model.m[12],
                0.123456789012345
            );
            assert!(scene.view.current.is_some());
        }
    }
    #[test]
    fn moving_repeated_source_objects_into_one_layer_saves_distinct_editable_records() {
        let raw = crate::specimen::bytes();
        let mut scene = Scene::empty();
        import(
            &mut scene,
            &raw,
            "first",
            &Xform::identity(),
            0.,
        )
        .unwrap();
        let n = scene.objects().len();
        import(
            &mut scene,
            &raw,
            "second",
            &Xform::identity(),
            0.,
        )
        .unwrap();
        let a = scene.objects()[0].id;
        let b = scene.objects()[n].id;
        let doc = &scene.forest.documents[0];
        let key = format!("{}/{}", doc.key, doc.nodes[0].source.guid);
        crate::layer_edit::apply(
            &mut scene,
            &key,
            crate::layer_edit::Edit::Move(vec![b]),
        )
        .unwrap();
        let saved = crate::scene_archive::save(&scene).unwrap();
        let bundle = crate::archive::Bundle::decode(&saved).unwrap();
        let source = proto::Session::decode(bundle.documents[0].as_slice()).unwrap();
        let meshes = source.objects.unwrap().meshes;
        assert_eq!(meshes.len(), n + 1);
        let guids = meshes
            .iter()
            .map(|mesh| &mesh.guid)
            .collect::<HashSet<_>>();
        assert_eq!(guids.len(), n + 1);
        assert_eq!(
            scene
                .objects()
                .iter()
                .find(|row| row.id == a)
                .unwrap()
                .geometry()
                .unwrap()
                .guid(),
            scene
                .objects()
                .iter()
                .find(|row| row.id == b)
                .unwrap()
                .geometry()
                .unwrap()
                .guid()
        );
        let mut restored = Scene::empty();
        open(&mut restored, &saved).unwrap();
        assert_eq!(restored.objects().len(), 2 * n);
    }
}
```

## <a id="checkpoint-66t-prepare-file-adoption-live-history"></a>32. Track File Loads in Scene History

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 32 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/publication.rs, src/lib…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">file candidate</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">editor adoption</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">controlled source history</text>
</svg>
</div>

**What this code does:** Prepare file adoption and live history.

### 1. `src/publication.rs`

Prepare raw documents or viewer archives before applying an editor action.

Create the file and type:

```rust
use crate::scene::Scene;
use std::rc::Rc;

pub fn prepare(
    scene: &Scene,
    bytes: &[u8],
    replace: bool,
    location: Option<Rc<crate::reload_url::ReloadUrl>>,
) -> Result<Scene, String> {
    let mut candidate = scene.clone();
    if bytes.starts_with(crate::archive::MAGIC) {
        crate::scene_document::open(&mut candidate, bytes)?;
        return Ok(candidate);
    }
    if replace {
        candidate.clear();
    }
    match crate::document::load_at(bytes, location) {
        Ok(loaded) => candidate
            .import(loaded)
            .map_err(str::to_owned)?,
        Err(_) => {
            crate::scene_document::import(
                &mut candidate,
                bytes,
                "",
                &session_rust::Xform::identity(),
                0.,
            )?;
        }
    }
    Ok(candidate)
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod archive_restore_flags;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod archive_restore_flags;

pub mod publication;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/publication.rs`

Copy source and integration regressions.

**Locate this existing block:**

```rust
    Ok(candidate)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::{Action, Editor};
    #[test]
    fn prepared_file_adoption_undo_redo_and_save_preserve_original_source_and_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::AddCloud).unwrap();
        let cloud = editor.scene.clouds()[0]
            .prepared
            .source
            .to_proto();
        let saved = crate::scene_archive::save(&editor.scene).unwrap();
        let camera = editor.camera.uniform();
        let old = editor.scene.clouds()[0].id;
        let ready = prepare(&editor.scene, &saved, true, None).unwrap();
        editor
            .apply(Action::Adopt {
                scene: Box::new(ready),
                replace: true,
            })
            .unwrap();
        let new = editor.scene.clouds()[0].id;
        assert_ne!(new, old);
        assert_eq!(
            editor.scene.clouds()[0]
                .prepared
                .source
                .to_proto(),
            cloud
        );
        assert_eq!(editor.camera.uniform(), camera);
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.clouds()[0].id, old);
        editor.apply(Action::Redo).unwrap();
        assert_eq!(editor.scene.clouds()[0].id, new);
        let crate::edit_replay::Reply::Saved(bytes) = crate::edit_intent::Intent::Save
            .replay(&mut editor)
            .unwrap()
        else {
            panic!("Saved viewer archive")
        };
        let mut restored = Scene::demo();
        crate::scene_document::open(&mut restored, &bytes).unwrap();
        assert_eq!(
            restored.clouds()[0]
                .prepared
                .source
                .to_proto(),
            cloud
        );
    }
    #[test]
    fn malformed_preparation_keeps_redo_and_legacy_source_reload_stays_available() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        let id = editor.scene.objects().last().unwrap().id;
        editor.apply(Action::Undo).unwrap();
        assert!(prepare(&editor.scene, b"broken", true, None).is_err());
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(id));
        let location = Rc::new(crate::reload_url::ReloadUrl::new(
            "test:publication".into(),
        ));
        let ready = prepare(
            &editor.scene,
            &crate::specimen::bytes(),
            true,
            Some(location.clone()),
        )
        .unwrap();
        assert!(
            ready
                .objects()
                .iter()
                .all(|row| row.can_unload())
        );
        assert!(Rc::ptr_eq(
            ready.objects()[0]
                .origin()
                .unwrap()
                .location
                .as_ref()
                .unwrap(),
            &location
        ));
    }
    #[test]
    fn live_adoption_releases_prior_history_sources_and_preserves_the_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        let old = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let display = Rc::downgrade(&editor.scene.objects()[0].mesh);
        let camera = editor.camera.uniform();
        let mut next = Scene::empty();
        next.insert_cloud(
            crate::cloud::Prepared::new(Rc::new(crate::cloud::example())).unwrap(),
        )
        .unwrap();
        editor
            .apply(Action::Live(Box::new(next)))
            .unwrap();
        assert!(old.upgrade().is_none());
        assert!(display.upgrade().is_none());
        assert_eq!(editor.scenes().count(), 1);
        assert_eq!(editor.camera.uniform(), camera);
        assert_eq!(editor.scene.clouds().len(), 1);
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.clouds().len(), 1);
    }
}
```

### 4. `src/editor.rs`

Add prepared file adoption and bounded live replacement without changing camera settings.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    crate::document::load(&crate::tree_specimen::nested(true))?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
```

Replace it with:

```rust
                    crate::document::load(&crate::tree_specimen::nested(true))?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::Live(ready) => {
                self.history.clear();
                self.scene.restore(*ready);
            }
            Action::Adopt { scene: ready, .. } => {
                self.history.edit(&mut self.scene, |scene| {
                    scene.restore(*ready)
                });
            }
```

**Locate this block:**

```rust
            Action::Close
                | Action::Undo
                | Action::Redo
                | Action::Import(_)
                | Action::Replace(_)
                | Action::ImportAt(_, _)
```

Replace it with:

```rust
            Action::Close
                | Action::Undo
                | Action::Redo
                | Action::Adopt { .. }
                | Action::Live(_)
                | Action::Import(_)
                | Action::Replace(_)
                | Action::ImportAt(_, _)
```

**Locate this block:**

```rust
    PickMode(crate::pick_sources::Mode),
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(
```

Replace it with:

```rust
    PickMode(crate::pick_sources::Mode),
    Live(Box<Scene>),
    Adopt {
        scene: Box<Scene>,
        replace: bool,
    },
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(
```

### 5. `src/edit_replay.rs`

Save a complete viewer archive after an original source reload.

**Locate this existing block:**

```rust
        match self {
            Self::Move { id, offset } => editor
                .move_object(id, offset)
                .map(Reply::Changed),
            Self::Delete { id } => editor.delete_object(id).map(Reply::Changed),
            Self::Save => crate::document::snapshot(&editor.scene).map(Reply::Saved),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::Action, reload_url::ReloadUrl};
    use prost::Message;
    use std::rc::Rc;

    #[test]
    fn save_reply_keeps_precision_and_does_not_consume_move_undo() {
        let mut e = Editor::default();
        e.apply(Action::Replace(crate::specimen::bytes()))
            .unwrap();
        let id = e.scene.objects()[0].id;
        let original = e.scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto();
        let later = e.scene.objects()[1].id;
        e.selected = Some(later);
        let camera = e.camera.uniform();
        let model = e.scene.objects()[0].model.m;
        assert_eq!(
            Intent::Move {
                id,
                offset: [0.25, 0.0, 0.0]
            }
            .replay(&mut e)
            .unwrap(),
            Reply::Changed(Change::Scene)
        );
        let Reply::Saved(bytes) = Intent::Save.replay(&mut e).unwrap() else {
            panic!("Save reply")
        };
        let saved = session_rust::proto::Session::decode(bytes.as_slice()).unwrap();
        assert_eq!(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        match self {
            Self::Move { id, offset } => editor.move_object(id, offset).map(Reply::Changed),
            Self::Delete { id } => editor.delete_object(id).map(Reply::Changed),
            Self::Save => crate::scene_archive::save(&editor.scene).map(Reply::Saved).map_err(|_|"Cannot save viewer session; reload editable sources and try again"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::Action, reload_url::ReloadUrl};
    use prost::Message;
    use std::rc::Rc;

    #[test]
    fn save_reply_keeps_precision_and_does_not_consume_move_undo() {
        let mut e = Editor::default();
        e.apply(Action::Replace(crate::specimen::bytes()))
            .unwrap();
        let id = e.scene.objects()[0].id;
        let original = e.scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto();
        let later = e.scene.objects()[1].id;
        e.selected = Some(later);
        let camera = e.camera.uniform();
        let model = e.scene.objects()[0].model.m;
        assert_eq!(
            Intent::Move {
                id,
                offset: [0.25, 0.0, 0.0]
            }
            .replay(&mut e)
            .unwrap(),
            Reply::Changed(Change::Scene)
        );
        let Reply::Saved(bytes) = Intent::Save.replay(&mut e).unwrap() else {
            panic!("Save reply")
        };
        let bundle = crate::archive::Bundle::decode(&bytes).unwrap();
        let saved =
            session_rust::proto::Session::decode(bundle.documents[0].as_slice())
                .unwrap();
        assert_eq!(
```

### 6. `src/reload_complete_tests.rs`

Copy the earlier source-reload assertions using the complete viewer file format.

**Locate this existing block:**

```rust
            Edit::Saved(bytes) => {
                let loaded = crate::document::load(&bytes).unwrap();
                assert_eq!(loaded.meshes.len(), e.scene.objects().len());
            }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            Edit::Saved(bytes) => {
                let mut restored = crate::scene::Scene::demo();
                crate::scene_document::open(&mut restored, &bytes).unwrap();
                assert_eq!(
                    restored.objects().len(),
                    e.scene.objects().len()
                );
            }
```

### 7. `src/reload_precision_tests.rs`

Copy the earlier source-reload assertions using the complete viewer file format.

**Locate this existing block:**

```rust
    };
    let message = session_rust::proto::Session::decode(saved.as_slice()).unwrap();
    assert_eq!(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    };
    let bundle = crate::archive::Bundle::decode(&saved).unwrap();
    let message =
        session_rust::proto::Session::decode(bundle.documents[0].as_slice()).unwrap();
    assert_eq!(
```

## <a id="checkpoint-66u-share-repeated-source-placements"></a>33. Share Repeated Source Placements

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 33 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">cached original source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared display</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">independent placement</text>
</svg>
</div>

**What this code does:** Share repeated source placements.

### 1. `src/scene.rs`

Share complete original document rows while assigning independent instance identities.

**Locate this existing block:**

```rust
    }

    pub(crate) fn reserve_document_key(&mut self) -> Result<u32, &'static str> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub(crate) fn append_document(
        &mut self,
        source: &Scene,
        name: &str,
        place: &session_rust::Xform,
        point_px: f32,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&place.m) || !point_px.is_finite() {
            return Err("Invalid document placement or point size");
        }
        let mut candidate = self.clone();
        let mut ids = source
            .ids
            .iter()
            .copied()
            .collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id| id.value());
        let mut remap = std::collections::HashMap::new();
        let mut names = std::collections::HashMap::new();
        let mut used = candidate
            .ids
            .iter()
            .filter_map(|id| candidate.row_guid(*id).map(str::to_owned))
            .collect::<std::collections::HashSet<_>>();
        for old in &ids {
            let id = ObjectId(candidate.reserve_document_key()?);
            remap.insert(*old, id);
            Rc::make_mut(&mut candidate.ids).insert(id);
            let mut guid = source
                .row_guid(*old)
                .ok_or("Original source owner is missing")?
                .to_owned();
            while !used.insert(guid.clone()) {
                guid = uuid::Uuid::new_v4().to_string();
            }
            names.insert(*old, guid);
        }
        macro_rules! rows {
            ($field:ident) => {
                for row in &source.$field {
                    let mut row = row.clone();
                    let old = row.id;
                    row.id = remap[&old];
                    row.guid = names[&old].clone();
                    candidate.$field.push(row);
                }
            };
        }
        rows!(objects);
        rows!(lines);
        rows!(paths);
        rows!(points);
        rows!(clouds);
        rows!(texts);
        for old in &ids {
            let id = remap[old];
            candidate.place_following(
                id,
                place
                    * &source
                        .placement(*old)
                        .ok_or("Original placement is missing")?,
            )?;
            let view = Rc::make_mut(&mut candidate.view);
            view.initial(
                id,
                source.visible(*old),
                source.view.locked.contains(old),
            );
            if let Some(rgb) = source.view.faces.get(old) {
                view.faces.insert(id, *rgb);
            }
            if let Some(rgb) = source.view.edges.get(old) {
                view.edges.insert(id, *rgb);
            }
            if source
                .clouds
                .iter()
                .any(|row| row.id == *old)
                && point_px > 0.
            {
                candidate.cloud_point_size(id, point_px)?;
            }
        }
        for doc in &source.forest.documents {
            let mut doc = (**doc).clone();
            let old_key = doc.key;
            doc.key = candidate.reserve_document_key()?;
            if !name.is_empty() {
                doc.label = name.to_owned();
            }
            let guids = doc
                .nodes
                .iter()
                .map(|node| {
                    (
                        node.source.guid.clone(),
                        uuid::Uuid::new_v4().to_string(),
                    )
                })
                .collect::<std::collections::HashMap<_, _>>();
            for node in &mut doc.nodes {
                node.object = node.object.map(|id| remap[&id]);
                node.source.guid = guids[&node.source.guid].clone();
                node.portable_guid = node.source.guid.clone();
                node.source.parent_guid = guids
                    .get(&node.source.parent_guid)
                    .cloned()
                    .unwrap_or_default();
            }
            if source.created == Some(old_key) && candidate.created.is_none() {
                candidate.created = Some(doc.key);
            }
            candidate.forest.push(doc);
        }
        self.restore(candidate);
        Ok(())
    }

    pub(crate) fn reserve_document_key(&mut self) -> Result<u32, &'static str> {
```

## <a id="checkpoint-66v-start-an-empty-scene-without-examples"></a>34. Initialize an Empty Scene

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 34 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">empty source owner</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">fresh document inventory</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original import</text>
</svg>
</div>

**What this code does:** Start an empty scene without examples.

### 1. `src/scene.rs`

Create an empty original document owner without changing the demo or scene identity rules.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        while self.paths.iter().any(|row| row.guid == guid)
            || self
                .objects
                .iter()
                .any(|row| row.guid == guid)
            || self
                .lines
                .iter()
                .any(|row| row.prepared.source.guid() == guid)
        {
```

Replace it with:

```rust
        while self.paths.iter().any(|row| row.guid == guid)
            || self
                .objects
                .iter()
                .any(|row| row.guid == guid)
            || self.lines.iter().any(|row| row.guid == guid)
        {
```

**Locate this block:**

```rust
            || self.paths.iter().any(|row| row.guid == guid)
            || self
                .objects
                .iter()
                .any(|row| row.guid == guid)
            || self
                .lines
                .iter()
                .any(|row| row.prepared.source.guid() == guid)
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let id = ObjectId(self.next_id);
        self.next_id = next;
```

Replace it with:

```rust
            || self.paths.iter().any(|row| row.guid == guid)
            || self
                .objects
                .iter()
                .any(|row| row.guid == guid)
            || self.lines.iter().any(|row| row.guid == guid)
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let id = ObjectId(self.next_id);
        self.next_id = next;
```

**Locate this block:**

```rust
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            created: None,
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
            clouds: Vec::new(),
            texts: Vec::new(),
            objects: Vec::new(),
            points: Vec::new(),
            paths: Vec::new(),
            lines: Vec::new(),
            next_id: 1,
            extra: None,
        };
        scene.insert(pink).expect("ID available");
        scene
            .insert(turquoise)
```

Replace it with:

```rust
        .expect("Valid turquoise triangle");
        let mut scene = Self::empty();
        scene.insert(pink).expect("ID available");
        scene
            .insert(turquoise)
```

## <a id="checkpoint-66w-assemble-placed-manifest-entries"></a>35. Assemble Placed Manifest Entries

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 35 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene_manifest.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">manifest entries</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared source documents</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">placed scene labels</text>
</svg>
</div>

**What this code does:** Assemble placed manifest entries.

### 1. `src/scene_manifest.rs`

Assemble a checked scene from shared original source documents and placed labels.

Create the file and type:

```rust
use crate::{manifest::Manifest, scene::Scene};
use std::{collections::HashMap, rc::Rc};

// Repeated manifest entries share immutable sources and display buffers; placements and viewer identities remain independent for each instance.
pub fn prepare(
    previous: &Scene,
    manifest: &Manifest,
    urls: &[String],
    sources: &HashMap<String, Scene>,
) -> Result<Scene, String> {
    if urls.len() != manifest.items.len() {
        return Err("Manifest source inventory does not match".into());
    }
    let mut scene = previous.clone();
    scene.clear();
    for (index, (item, url)) in manifest.items.iter().zip(urls).enumerate() {
        let source = sources
            .get(url)
            .ok_or_else(|| format!("Missing manifest source: {url}"))?;
        scene
            .append_document(
                source,
                &item.name,
                &manifest.place(index, [0., 0.]),
                item.point_size as f32,
            )
            .map_err(str::to_owned)?;
    }
    for (index, item) in manifest.texts.iter().enumerate() {
        let placement = if item.camera_facing {
            crate::text::TextPlacement::WorldBillboard {
                world: item.at,
                world_height: item.height,
            }
        } else {
            crate::text::TextPlacement::WorldPlane {
                world: item.at,
                right: item.right,
                up: item.up,
                world_height: item.height,
            }
        };
        let mut source = crate::scene_label::example(placement);
        source.guid = format!("manifest-text-{index}");
        source.name = item.text.clone();
        source.label.text = item.text.clone();
        source.label.id = index as u32 + 1;
        scene
            .insert_text(
                crate::scene_label::Prepared::new(Rc::new(source))
                    .map_err(str::to_owned)?,
            )
            .map_err(str::to_owned)?;
    }
    Ok(scene)
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod publication;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod publication;

pub mod scene_manifest;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/scene_manifest.rs`

Copy source and integration regressions.

**Locate this existing block:**

```rust
    Ok(scene)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(scene)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeated_manifest_instances_share_original_sources_and_display_allocations() {
        let manifest=Manifest::parse(r#"items: [{file: cloud.pb, at: [0,0,0], point_size: 4}, {file: cloud.pb, at: [1000000000.125,0,0], point_size: 7}]
texts: [{text: 'CAD Ø25', at: [0,0,0], height: 0.1}]
"#.as_bytes()).unwrap();
        let mut source = Scene::demo();
        source.clear();
        source
            .insert_cloud(
                crate::cloud::Prepared::new(Rc::new(crate::cloud::example())).unwrap(),
            )
            .unwrap();
        source
            .insert(
                crate::prepared::PreparedMesh::triangle(
                    [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
                    [1., 0., 0.],
                )
                .unwrap(),
            )
            .unwrap();
        let sources = [("cloud.pb".into(), source.clone())]
            .into_iter()
            .collect();
        let scene = prepare(
            &Scene::demo(),
            &manifest,
            &["cloud.pb".into(), "cloud.pb".into()],
            &sources,
        )
        .unwrap();
        assert_eq!(scene.clouds().len(), 2);
        assert!(Rc::ptr_eq(
            &scene.clouds()[0].prepared.source,
            &scene.clouds()[1].prepared.source
        ));
        assert!(Rc::ptr_eq(
            &scene.clouds()[0].prepared.nodes,
            &scene.clouds()[1].prepared.nodes
        ));
        assert!(Rc::ptr_eq(
            &scene.objects()[0].mesh,
            &scene.objects()[1].mesh
        ));
        assert!(Rc::ptr_eq(
            scene.objects()[0].geometry().unwrap(),
            scene.objects()[1].geometry().unwrap()
        ));
        assert_ne!(scene.clouds()[0].id, scene.clouds()[1].id);
        assert_ne!(
            scene.clouds()[0].guid,
            scene.clouds()[1].guid
        );
        assert_eq!(
            scene.clouds()[1].model.m[12],
            1_000_000_000.125
        );
        assert_eq!(scene.clouds()[0].point_px, 4.);
        assert_eq!(scene.clouds()[1].point_px, 7.);
        assert_eq!(
            scene.clouds()[0].prepared.source.point_size,
            3.
        );
        assert_eq!(
            scene.texts()[0].prepared.source.label.text,
            "CAD Ø25"
        );
        let saved = crate::scene_archive::save(&scene).unwrap();
        let mut restored = Scene::demo();
        crate::scene_document::open(&mut restored, &saved).unwrap();
        assert_eq!(
            restored
                .clouds()
                .iter()
                .map(|row| row.point_px)
                .collect::<Vec<_>>(),
            vec![4., 7.]
        );
        assert_eq!(
            source.clouds()[0].model.m,
            session_rust::Xform::identity().m
        );
    }
    #[test]
    fn invalid_manifest_adoption_and_overflowing_placements_leave_prior_scene_intact() {
        let prior = Scene::demo();
        let ids = prior
            .objects()
            .iter()
            .map(|row| row.id)
            .collect::<Vec<_>>();
        let manifest =
            Manifest::parse(r#"items: [{file: absent.pb}]"#.as_bytes()).unwrap();
        assert!(
            prepare(
                &prior,
                &manifest,
                &["absent.pb".into()],
                &HashMap::new()
            )
            .is_err()
        );
        assert_eq!(
            prior
                .objects()
                .iter()
                .map(|row| row.id)
                .collect::<Vec<_>>(),
            ids
        );
        let manifest =
            Manifest::parse(r#"items: [{file: mesh.pb, at: [1e100,0,0]}]"#.as_bytes())
                .unwrap();
        let sources = [("mesh.pb".into(), Scene::demo())]
            .into_iter()
            .collect();
        assert!(
            prepare(
                &prior,
                &manifest,
                &["mesh.pb".into()],
                &sources
            )
            .is_err()
        );
        assert_eq!(
            prior
                .objects()
                .iter()
                .map(|row| row.id)
                .collect::<Vec<_>>(),
            ids
        );
    }
}
```

## <a id="checkpoint-66x-resolve-scene-live-urls"></a>36. Resolve Scene and Live URLs

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 36 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/publication_route.rs, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">scene route</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">resolved files</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">live schedule</text>
</svg>
</div>

**What this code does:** Resolve scene and live URLs.

### 1. `src/publication_route.rs`

Resolve current named-scene, bucket-root, local-live and notification routes.

Create the file and type:

```rust
pub const DATA_BASE: &str = "https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/";
pub const DEFAULT_SOURCE: &str =
    "https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/scenes/view_live.yaml";

#[derive(Clone)]
pub struct Route {
    pub manifest: String,
    pub base: String,
    pub poll_ms: Option<i32>,
    pub notify: Option<String>,
}
pub fn local(url: &str) -> bool {
    url.starts_with("http://localhost:")
        || url.starts_with("http://127.0.0.1:")
        || url.starts_with("http://[::1]:")
}
pub fn safe_scene(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.contains("//")
        && !value.contains(':')
        && !value
            .split('/')
            .any(|segment| segment == "..")
}
pub fn path_scene(path: &str) -> Option<&str> {
    let last = path.rsplit('/').next()?;
    (!last.is_empty()
        && !last.ends_with(".html")
        && !last.contains(':')
        && !last.starts_with('.'))
    .then_some(last)
}
pub fn join(base: &str, file: &str) -> String {
    if file.starts_with("https://") || file.starts_with("http://") {
        file.into()
    } else {
        format!("{base}{}", file.trim_start_matches("./"))
    }
}
pub fn directory(url: &str) -> String {
    url.rsplit_once('/')
        .map_or_else(String::new, |(prefix, _)| {
            format!("{prefix}/")
        })
}

pub fn select(
    scene: Option<&str>,
    path: &str,
    data: Option<&str>,
    live: Option<&str>,
    poll: Option<&str>,
    notify: Option<&str>,
) -> Option<Route> {
    let base = match data {
        Some("off" | "") => String::new(),
        Some(value) if value.starts_with("https://") || local(value) => {
            if value.ends_with('/') {
                value.into()
            } else {
                format!("{value}/")
            }
        }
        _ => DATA_BASE.into(),
    };
    let named = scene
        .filter(|value| safe_scene(value))
        .or_else(|| path_scene(path));
    let watched = match live {
        Some("off" | "0") => None,
        Some(value) if value.starts_with("https://") || local(value) => {
            Some(value.to_owned())
        }
        Some(_) => Some(DEFAULT_SOURCE.into()),
        None if named == Some("view_live") => Some(DEFAULT_SOURCE.into()),
        _ => None,
    };
    if let Some(manifest) = watched {
        let source_base = if !base.is_empty() && manifest.starts_with(&base) {
            base
        } else {
            directory(&manifest)
        };
        let seconds = poll
            .and_then(|v| v.parse::<f64>().ok())
            .filter(|v| v.is_finite() && *v >= 1.)
            .unwrap_or(5.);
        let notify = match notify {
            Some("off" | "0") => None,
            Some(value) if value.starts_with("https://") || local(value) => {
                Some(value.into())
            }
            _ => None,
        };
        return Some(Route {
            manifest,
            base: source_base,
            poll_ms: Some((seconds * 1000.).min(i32::MAX as f64) as i32),
            notify,
        });
    }
    let name = named?;
    let mut name = if name.contains('.') {
        name.to_owned()
    } else {
        format!("{name}.yaml")
    };
    if !name.contains('/') {
        name = format!("scenes/{name}");
    }
    Some(Route {
        manifest: join(&base, &name),
        base,
        poll_ms: None,
        notify: None,
    })
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod scene_manifest;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod scene_manifest;

pub mod publication_route;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/publication_route.rs`

Copy source and integration regressions.

**Locate this existing block:**

```rust
        notify: None,
    })
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        notify: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn named_routes_keep_bucket_relative_files_and_refuse_parent_traversal() {
        assert!(!safe_scene("../outside"));
        assert!(!safe_scene("folder/../outside"));
        assert!(!safe_scene("https://other/scene"));
        assert!(!safe_scene("/scene"));
        let route = select(
            Some("view_floor"),
            "/",
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(
            route.manifest,
            format!("{DATA_BASE}scenes/view_floor.yaml")
        );
        assert_eq!(
            join(&route.base, "pb/floor.pb"),
            format!("{DATA_BASE}pb/floor.pb")
        );
        let route = select(
            Some("fixtures/scene.yaml"),
            "/",
            Some("off"),
            Some("off"),
            None,
            None,
        )
        .unwrap();
        assert_eq!(route.manifest, "fixtures/scene.yaml");
        assert_eq!(
            join(&route.base, "./fixtures/a.pb"),
            "fixtures/a.pb"
        );
        assert_eq!(path_scene("/view_floor"), Some("view_floor"));
        assert!(path_scene("/index.html").is_none());
        assert!(select(None, "/", Some("off"), None, None, None).is_none());
    }
    #[test]
    fn live_routes_keep_local_directories_poll_bounds_and_explicit_notification_urls() {
        let route = select(
            None,
            "/",
            Some("off"),
            Some("http://localhost:8770/live/scene.yaml"),
            Some("0.1"),
            Some("off"),
        )
        .unwrap();
        assert_eq!(route.base, "http://localhost:8770/live/");
        assert_eq!(route.poll_ms, Some(5000));
        assert!(route.notify.is_none());
        let route = select(
            Some("view_floor"),
            "/",
            None,
            Some("https://example.test/live.yaml"),
            Some("1"),
            Some("https://example.test/events"),
        )
        .unwrap();
        assert_eq!(route.poll_ms, Some(1000));
        assert_eq!(
            route.notify.as_deref(),
            Some("https://example.test/events")
        );
        assert_eq!(route.base, "https://example.test/");
    }
}
```

## <a id="checkpoint-66y-retain-validated-live-source-revisions"></a>37. Retain Validated Source Revisions

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 37 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/publication_index.rs, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">validated manifest</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source cache</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">complete replacement</text>
</svg>
</div>

**What this code does:** Retain validated live source revisions.

### 1. `src/publication_index.rs`

Keep original source revisions and build only complete changed scenes.

Create the file and type:

```rust
use crate::{manifest::Manifest, scene::Scene};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub struct Index {
    base: String,
    revisions: crate::live_revision::Revisions,
    manifest: Option<Manifest>,
    sources: HashMap<String, Scene>,
    sizes: HashMap<String, usize>,
    pending: bool,
}
impl Index {
    pub fn new(base: String) -> Self {
        Self {
            base,
            revisions: Default::default(),
            manifest: None,
            sources: HashMap::new(),
            sizes: HashMap::new(),
            pending: false,
        }
    }
    pub fn etag(&self, url: &str) -> Option<String> {
        self.revisions.etag(url)
    }
    pub fn has_manifest(&self) -> bool {
        self.manifest.is_some()
    }
    pub fn cached(&self, url: &str) -> bool {
        self.sources.contains_key(url)
    }
    pub fn forget(&mut self, url: &str) {
        self.revisions.forget(url);
    }
    pub fn urls(&self) -> Vec<String> {
        self.manifest
            .as_ref()
            .map_or_else(Vec::new, |manifest| {
                manifest
                    .items
                    .iter()
                    .map(|item| crate::publication_route::join(&self.base, &item.file))
                    .collect()
            })
    }
    pub fn manifest(
        &mut self,
        url: &str,
        etag: Option<String>,
        bytes: &[u8],
    ) -> Result<bool, String> {
        if !self.revisions.changed(url, etag, bytes) {
            return Ok(false);
        }
        let manifest = match Manifest::parse(bytes) {
            Ok(value) => value,
            Err(error) => {
                self.revisions.forget(url);
                return Err(error);
            }
        };
        self.manifest = Some(manifest);
        self.pending = true;
        let used = self
            .urls()
            .into_iter()
            .collect::<HashSet<_>>();
        self.sources
            .retain(|url, _| used.contains(url));
        self.sizes
            .retain(|url, _| used.contains(url));
        Ok(true)
    }
    pub fn source(
        &mut self,
        url: &str,
        etag: Option<String>,
        bytes: &[u8],
    ) -> Result<bool, String> {
        if !self.revisions.changed(url, etag, bytes) {
            return Ok(false);
        }
        let size = self
            .sizes
            .iter()
            .filter(|(key, _)| key.as_str() != url)
            .try_fold(bytes.len(), |sum, (_, size)| {
                sum.checked_add(*size)
            })
            .ok_or("Source inventory size overflow")?;
        if size > crate::archive::LIMIT {
            self.revisions.forget(url);
            return Err("Source inventory exceeds the 512 MiB file limit".into());
        }
        let source = match crate::publication::prepare(
            &Scene::empty(),
            bytes,
            true,
            Some(Rc::new(crate::reload_url::ReloadUrl::new(
                url.into(),
            ))),
        ) {
            Ok(source) => source,
            Err(error) => {
                self.revisions.forget(url);
                return Err(error);
            }
        };
        self.sources.insert(url.into(), source);
        self.sizes.insert(url.into(), bytes.len());
        self.pending = true;
        Ok(true)
    }
    pub fn replacement(&mut self, previous: &Scene) -> Result<Option<Scene>, String> {
        if !self.pending {
            return Ok(None);
        }
        let manifest = self
            .manifest
            .as_ref()
            .ok_or("Live manifest is missing")?;
        for (item, url) in manifest.items.iter().zip(self.urls()) {
            if let Some(expected) = item.encoded_size() {
                if self
                    .sizes
                    .get(&url)
                    .copied()
                    .map(|size| size as u64)
                    != Some(expected)
                {
                    self.revisions.forget(&url);
                    return Err(
                        "Encoded source decoded size does not match the manifest"
                            .into(),
                    );
                }
            }
        }
        let scene = crate::scene_manifest::prepare(
            previous,
            manifest,
            &self.urls(),
            &self.sources,
        )?;
        self.pending = false;
        Ok(Some(scene))
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod publication_route;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod publication_route;

pub mod publication_index;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/publication_index.rs`

Copy source and integration regressions.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validators_and_unchanged_bytes_keep_original_sources_and_viewer_ids() {
        let mut index = Index::new("files/".into());
        let manifest = b"items: [{file: mesh.pb}]";
        let raw = crate::specimen::bytes();
        let mut scene = Scene::empty();
        assert!(
            index
                .manifest("scene", Some("a".into()), manifest)
                .unwrap()
        );
        assert!(
            index
                .source("files/mesh.pb", Some("b".into()), &raw)
                .unwrap()
        );
        scene = index.replacement(&scene).unwrap().unwrap();
        let id = scene.objects()[0].id;
        let geometry = Rc::clone(scene.objects()[0].geometry().unwrap());
        assert!(
            !index
                .manifest("scene", Some("W/c".into()), manifest)
                .unwrap()
        );
        assert!(
            !index
                .source("files/mesh.pb", None, &raw)
                .unwrap()
        );
        assert_eq!(index.etag("scene"), Some("W/c".into()));
        assert_eq!(index.etag("files/mesh.pb"), None);
        assert!(index.replacement(&scene).unwrap().is_none());
        assert_eq!(scene.objects()[0].id, id);
        assert!(Rc::ptr_eq(
            scene.objects()[0].geometry().unwrap(),
            &geometry
        ));
        assert!(
            index
                .source(
                    "files/mesh.pb",
                    None,
                    &crate::specimen::precise_bytes()
                )
                .unwrap()
        );
        let changed = index.replacement(&scene).unwrap().unwrap();
        assert_ne!(changed.objects()[0].id, id);
        assert!(!Rc::ptr_eq(
            changed.objects()[0].geometry().unwrap(),
            &geometry
        ));
        assert!(Rc::ptr_eq(
            scene.objects()[0].geometry().unwrap(),
            &geometry
        ));
    }
    #[test]
    fn invalid_or_incomplete_live_updates_keep_the_last_display_and_retry_failed_bytes()
    {
        let mut index = Index::new(String::new());
        let manifest = b"items: [{file: mesh.pb}]";
        let raw = crate::specimen::bytes();
        index
            .manifest("scene", None, manifest)
            .unwrap();
        assert!(index.replacement(&Scene::empty()).is_err());
        index
            .source("mesh.pb", Some("good".into()), &raw)
            .unwrap();
        let scene = index
            .replacement(&Scene::empty())
            .unwrap()
            .unwrap();
        let id = scene.objects()[0].id;
        assert!(
            index
                .source("mesh.pb", Some("bad".into()), b"broken")
                .is_err()
        );
        assert_eq!(index.etag("mesh.pb"), None);
        assert!(index.replacement(&scene).unwrap().is_none());
        assert_eq!(scene.objects()[0].id, id);
        assert!(
            index
                .manifest("scene", None, b"invalid: manifest")
                .is_err()
        );
        assert_eq!(index.etag("scene"), None);
        assert!(
            index
                .manifest("scene", None, manifest)
                .unwrap()
        );
        assert!(index.replacement(&scene).unwrap().is_some());
        let mesh = Rc::downgrade(scene.objects()[0].geometry().unwrap());
        drop(scene);
        index
            .manifest("scene", None, b"items: []")
            .unwrap();
        assert!(mesh.upgrade().is_none());
        assert!(!index.cached("mesh.pb"));
    }
}
```

## <a id="checkpoint-66z-fetch-bounded-source-bodies"></a>38. Fetch Bounded Source Payloads

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 38 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/publication_fetch.rs, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">request validator</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded stream</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">decoded source bytes</text>
</svg>
</div>

**What this code does:** Fetch bounded source bodies.

### 1. `src/publication_fetch.rs`

Read cancellable HTTP responses with independent manifest and source byte limits.

Create the file and type:

```rust
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

pub struct Response {
    pub status: u16,
    pub etag: Option<String>,
    pub bytes: Vec<u8>,
}
pub async fn get(
    url: &str,
    etag: Option<String>,
    limit: usize,
    signal: &web_sys::AbortSignal,
) -> Result<Response, String> {
    let window = web_sys::window().ok_or("No browser window")?;
    let init = web_sys::RequestInit::new();
    init.set_signal(Some(signal));
    init.set_cache(web_sys::RequestCache::NoCache);
    let headers =
        web_sys::Headers::new().map_err(|_| "Cannot create request headers")?;
    if let Some(etag) = etag {
        headers
            .set("If-None-Match", &etag)
            .map_err(|_| "Cannot set HTTP validator")?;
    }
    init.set_headers(&headers);
    let started = crate::browser_phase::now();
    let response = JsFuture::from(window.fetch_with_str_and_init(url, &init))
        .await
        .map_err(|error| format!("Fetch failed: {error:?}"))?
        .dyn_into::<web_sys::Response>()
        .map_err(|_| "Invalid fetch response")?;
    let status = response.status();
    let etag = response
        .headers()
        .get("ETag")
        .map_err(|_| "Cannot read HTTP validator")?;
    if status == 304 {
        crate::browser_phase::finish("HTTP unchanged", started, 0, url);
        return Ok(Response {
            status,
            etag,
            bytes: vec![],
        });
    }
    if !response.ok() {
        return Ok(Response {
            status,
            etag,
            bytes: vec![],
        });
    }
    if let Some(size) = response
        .headers()
        .get("Content-Length")
        .map_err(|_| "Cannot read source length")?
    {
        if size
            .parse::<u64>()
            .map_err(|_| "Invalid source length")?
            > limit as u64
        {
            return Err("Response exceeds its byte limit".into());
        }
    }
    let reader = response
        .body()
        .ok_or("Source response has no body")?
        .get_reader()
        .dyn_into::<web_sys::ReadableStreamDefaultReader>()
        .map_err(|_| "Cannot read source stream")?;
    let result = read(&reader, limit).await;
    if result.is_err() {
        let _ = JsFuture::from(reader.cancel()).await;
    }
    reader.release_lock();
    let bytes = result?;
    crate::browser_phase::finish(
        "HTTP source",
        started,
        bytes.len() as u64,
        url,
    );
    Ok(Response {
        status,
        etag,
        bytes,
    })
}
async fn read(
    reader: &web_sys::ReadableStreamDefaultReader,
    limit: usize,
) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    loop {
        let part = JsFuture::from(reader.read())
            .await
            .map_err(|error| format!("Cannot read source: {error:?}"))?;
        if js_sys::Reflect::get(&part, &JsValue::from_str("done"))
            .map_err(|_| "Invalid stream reply")?
            .as_bool()
            == Some(true)
        {
            break;
        }
        let chunk = js_sys::Reflect::get(&part, &JsValue::from_str("value"))
            .map_err(|_| "Missing source chunk")?
            .dyn_into::<js_sys::Uint8Array>()
            .map_err(|_| "Invalid source chunk")?;
        if chunk.length() as usize > limit - bytes.len() {
            return Err("Response exceeds its byte limit".into());
        }
        bytes.extend_from_slice(&chunk.to_vec());
    }
    Ok(bytes)
}
```

### 2. `src/lib.rs`

Register the browser-only HTTP reader.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
pub mod publication_fetch;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
```

## <a id="checkpoint-66aa-read-a-complete-url-source-revision"></a>39. Read a Complete URL Source Revision

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 39 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser_publication.rs,…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">manifest response</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original files</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">complete source revision</text>
</svg>
</div>

**What this code does:** Read a complete URL source revision.

### 1. `src/browser_publication.rs`

Read complete original URL sources before publishing a replacement.

Create the file and type:

```rust
use crate::{publication_index::Index, publication_route::Route};
use std::{
    cell::RefCell,
    collections::HashSet,
    rc::{Rc, Weak},
};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

async fn read(
    route: &Route,
    index: &mut Index,
    signal: &web_sys::AbortSignal,
) -> Result<(), String> {
    let started = crate::browser_phase::now();
    let mut response = crate::publication_fetch::get(
        &route.manifest,
        index.etag(&route.manifest),
        4 * 1024 * 1024,
        signal,
    )
    .await?;
    if response.status == 404 && route.manifest.ends_with(".toml") {
        let yaml = format!(
            "{}.yaml",
            route.manifest.trim_end_matches(".toml")
        );
        response = crate::publication_fetch::get(
            &yaml,
            index.etag(&route.manifest),
            4 * 1024 * 1024,
            signal,
        )
        .await?;
    }
    match response.status {
        304 if index.has_manifest() => {}
        200..=299 => {
            index.manifest(
                &route.manifest,
                response.etag,
                &response.bytes,
            )?;
            crate::browser_phase::finish(
                "manifest",
                started,
                response.bytes.len() as u64,
                &route.manifest,
            );
        }
        status => return Err(format!("Manifest HTTP {status}")),
    }
    let mut seen = HashSet::new();
    for url in index.urls() {
        if !seen.insert(url.clone())
            || crate::manifest::immutable_key(&url) && index.cached(&url)
        {
            continue;
        }
        let response = crate::publication_fetch::get(
            &url,
            index.etag(&url),
            crate::archive::LIMIT,
            signal,
        )
        .await?;
        match response.status {
            304 if index.cached(&url) => {}
            200..=299 => {
                let started = crate::browser_phase::now();
                index.source(&url, response.etag, &response.bytes)?;
                crate::browser_phase::finish(
                    "decode/validation/display",
                    started,
                    response.bytes.len() as u64,
                    &url,
                );
            }
            status => {
                index.forget(&url);
                return Err(format!("Source HTTP {status}: {url}"));
            }
        }
    }
    Ok(())
}
```

### 2. `src/lib.rs`

Register the browser source loader.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
pub mod browser_publication;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
```

## <a id="checkpoint-66ab-own-cancel-the-url-loader"></a>40. Manage and Cancel the URL Loader

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 40 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser_publication.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">runtime owner</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">cancellation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source release</text>
</svg>
</div>

**What this code does:** Own and cancel the URL loader.

### 1. `src/browser_publication.rs`

Own the URL source cache, pending adoption and complete cancellation lifecycle.

**Locate this existing block:**

```rust
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

async fn read(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

struct State {
    active: bool,
    inflight: bool,
    again: bool,
    gate: crate::read_gate::ReadGate,
    abort: Option<web_sys::AbortController>,
    route: Route,
    index: Option<Index>,
    document: crate::drawing_document::Shared,
    fault: crate::gpu_fault::Fault,
    delivery: Option<Result<Box<crate::scene::Scene>, String>>,
    last_error: Option<String>,
    timer: Option<i32>,
    tick: Option<Closure<dyn FnMut()>>,
    notify: Option<web_sys::EventSource>,
    message: Option<Closure<dyn FnMut(web_sys::MessageEvent)>>,
}
pub struct Publication(Option<Rc<RefCell<State>>>);
#[derive(Clone)]
pub struct Control(Option<Weak<RefCell<State>>>);
impl Publication {
    pub fn new(
        document: crate::drawing_document::Shared,
        fault: crate::gpu_fault::Fault,
    ) -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No browser window")?;
        let url = web_sys::Url::new(&window.location().href()?)?;
        let q = url.search_params();
        let scene = q.get("scene");
        let data = q.get("data");
        let live = q.get("live");
        let poll = q.get("poll");
        let notify = q.get("notify");
        let Some(route) = crate::publication_route::select(
            scene.as_deref(),
            &window.location().pathname()?,
            data.as_deref(),
            live.as_deref(),
            poll.as_deref(),
            notify.as_deref(),
        ) else {
            return Ok(Self(None));
        };
        let index = Index::new(route.base.clone());
        Ok(Self(Some(Rc::new(RefCell::new(State {
            active: true,
            inflight: false,
            again: false,
            gate: Default::default(),
            abort: None,
            route,
            index: Some(index),
            document,
            fault,
            delivery: None,
            last_error: None,
            timer: None,
            tick: None,
            notify: None,
            message: None,
        })))))
    }
    pub fn control(&self) -> Control {
        Control(self.0.as_ref().map(Rc::downgrade))
    }
}
impl Drop for Publication {
    fn drop(&mut self) {
        self.control().cancel();
    }
}
impl Control {
    fn state(&self) -> Option<Rc<RefCell<State>>> {
        self.0.as_ref()?.upgrade()
    }
    pub fn cancel(&self) {
        let Some(state) = self.state() else {
            return;
        };
        let mut state = state.borrow_mut();
        state.active = false;
        state.gate.cancel();
        state.delivery = None;
        state.index = None;
        if let Some(abort) = state.abort.take() {
            abort.abort();
        }
        if let Some(id) = state.timer.take() {
            if let Some(window) = web_sys::window() {
                window.clear_interval_with_handle(id);
            }
        }
        if let Some(source) = state.notify.take() {
            source.set_onmessage(None);
            source.close();
        }
        state.message = None;
        state.tick = None;
    }
    pub fn take(&self) -> Result<Option<crate::editor::Action>, String> {
        let Some(state) = self.state() else {
            return Ok(None);
        };
        let mut state = state.borrow_mut();
        if !state.active {
            state.delivery = None;
            return Ok(None);
        }
        state
            .delivery
            .take()
            .transpose()
            .map(|scene| scene.map(crate::editor::Action::Live))
    }
}

async fn read(
```

## <a id="checkpoint-66ac-accept-only-current-url-completions"></a>41. Accept Current URL Completions

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 41 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser_publication.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one request</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">valid candidate</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">current runtime</text>
</svg>
</div>

**What this code does:** Accept only current URL completions.

### 1. `src/browser_publication.rs`

Accept only a current complete revision and coalesce announced rereads.

**Locate this existing block:**

```rust
}

async fn read(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

fn poll(state: Rc<RefCell<State>>) {
    let mut owner = state.borrow_mut();
    if !owner.active
        || !crate::browser_runtime::owns(&owner.fault)
        || owner.fault.message().is_some()
    {
        return;
    }
    if owner.inflight {
        owner.again = true;
        return;
    }
    let Ok(abort) = web_sys::AbortController::new() else {
        return;
    };
    let Ok(ticket) = owner.gate.begin() else {
        return;
    };
    let Some(mut index) = owner.index.take() else {
        return;
    };
    owner.inflight = true;
    owner.abort = Some(abort.clone());
    let route = owner.route.clone();
    drop(owner);
    wasm_bindgen_futures::spawn_local(async move {
        let result = read(&route, &mut index, &abort.signal()).await;
        let mut owner = state.borrow_mut();
        let current = owner.active
            && owner.gate.finish(ticket)
            && crate::browser_runtime::owns(&owner.fault)
            && owner.fault.message().is_none();
        owner.inflight = false;
        owner.abort = None;
        if !current {
            return;
        }
        let outcome =
            result.and_then(|()| index.replacement(&owner.document.borrow().scene));
        owner.index = Some(index);
        let changed = match outcome {
            Ok(Some(scene)) => {
                owner.last_error = None;
                owner.delivery = Some(Ok(Box::new(scene)));
                true
            }
            Ok(None) => false,
            Err(error) => {
                if owner.last_error.as_ref() == Some(&error) {
                    false
                } else {
                    let _ = crate::browser_report::observe("publication", &error);
                    owner.last_error = Some(error.clone());
                    owner.delivery = Some(Err(error));
                    true
                }
            }
        };
        let again = std::mem::take(&mut owner.again);
        drop(owner);
        if changed {
            if let Some(window) = web_sys::window() {
                if let Ok(event) = web_sys::Event::new("viewer-publication") {
                    let _ = window.dispatch_event(&event);
                }
            }
        }
        if again {
            poll(state);
        }
    });
}

async fn read(
```

## <a id="checkpoint-66ad-watch-published-source-changes"></a>42. Poll Published Source Updates

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 42 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser_publication.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">poll interval</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">coalesced read</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shutdown</text>
</svg>
</div>

**What this code does:** Watch published source changes.

### 1. `src/browser_publication.rs`

Start source polling and notification requests with explicit runtime cleanup.

**Locate this existing block:**

```rust
    }
    pub fn cancel(&self) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn begin(&self) {
        let Some(state) = self.state() else {
            return;
        };
        let Some(window) = web_sys::window() else {
            return;
        };
        let route = state.borrow().route.clone();
        if let Some(ms) = route.poll_ms {
            let weak = Rc::downgrade(&state);
            let callback = Closure::<dyn FnMut()>::new(move || {
                if let Some(state) = weak.upgrade() {
                    poll(state);
                }
            });
            match window.set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                ms,
            ) {
                Ok(id) => {
                    let mut state = state.borrow_mut();
                    state.timer = Some(id);
                    state.tick = Some(callback);
                }
                Err(error) => crate::browser::report(&format!(
                    "Cannot schedule live polling: {error:?}"
                )),
            }
            if let Some(url) = route.notify {
                if let Ok(source) = web_sys::EventSource::new(&url) {
                    let weak = Rc::downgrade(&state);
                    let callback =
                        Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |_| {
                            if let Some(state) = weak.upgrade() {
                                poll(state);
                            }
                        });
                    source.set_onmessage(Some(callback.as_ref().unchecked_ref()));
                    let mut state = state.borrow_mut();
                    state.notify = Some(source);
                    state.message = Some(callback);
                }
            }
        }
        poll(state);
    }
    pub fn cancel(&self) {
```

## <a id="checkpoint-66ae-inspect-published-source-identities"></a>43. Inspect Published Source IDs

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 43 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source row</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">compact diagnostics</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source assertions</text>
</svg>
</div>

**What this code does:** Inspect published source identities.

### 1. `src/browser.rs`

Report compact original source and presentation identities without serializing large coordinate columns.

**Locate this existing block:**

```rust
        )
        .chain(editor.scene.texts().iter().map(|row| row.id));
    let view_rows=view_ids.map(|id|serde_json::json!({"id":id.value(),"visible":editor.scene.visible(id),"locked":editor.scene.view.locked.contains(&id),"faces":editor.scene.view.faces.get(&id),"edges":editor.scene.view.edges.get(&id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-canvas-drag",&editor.canvas_drag.as_ref().map_or_else(||"null".into(),|d|serde_json::json!({"from":d.from,"to":d.to,"released":d.released,"missed":d.missed,"preview":d.preview.as_ref().map(|p|p.matrix())}).to_string()))?;
    canvas.set_attribute(
        "data-camera-frame",
        &serde_json::to_string(&editor.camera.frame).unwrap(),
    )?;
    canvas.set_attribute("data-view-flags",&serde_json::json!({"current":crate::layer_edit::current_key(&editor.scene),"rows":view_rows}).to_string())?;
    canvas.set_attribute("data-tree",&serde_json::json!(crate::hierarchy::rows(&editor.scene).iter().map(|r|serde_json::json!({"key":r.key,"label":r.label,"depth":r.depth,"end":r.end,"readonly":r.readonly,"ids":r.ids.iter().map(|id|id.value()).collect::<Vec<_>>()})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-gizmo",&serde_json::json!({"origin":editor.gizmo_origin(),"active":editor.gizmo_active(),"pointer":editor.gizmo_edit.as_ref().and_then(|e|e.pointer),"typing":editor.gizmo_edit.as_ref().and_then(|e|e.gizmo.typing).map(|h|h.title()),"matrix":editor.gizmo_edit.as_ref().map(|e|e.preview.matrix()),"gpu":renderer.gizmo_usage()}).to_string())?;
    canvas.set_attribute(
        "data-row-placements",
        &serde_json::to_string(&placements)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute(
        "data-pick-gpu",
        &serde_json::to_string(&renderer.pick_usage()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-point-gpu",
        &serde_json::to_string(&renderer.point_usage()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-cloud-cpu",
        &serde_json::to_string(&crate::memory::clouds(
            editor.presentation_scenes(),
        ))
        .unwrap(),
    )?;
    canvas.set_attribute(
        "data-cloud-gpu",
        &serde_json::to_string(&renderer.cloud_usage()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-cloud-error",
        &renderer.cloud_error().unwrap_or_default(),
    )?;
    canvas.set_attribute("data-cloud-sources",&serde_json::json!(editor.scene.clouds().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source_guid":row.prepared.source.guid(),"points":row.prepared.source.len(),"nodes":row.prepared.nodes.len(),"model":row.model.m})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        )
        .chain(editor.scene.texts().iter().map(|row| row.id))
        .chain(
            editor
                .scene
                .clouds()
                .iter()
                .map(|row| row.id),
        );
    let view_rows=view_ids.map(|id|serde_json::json!({"id":id.value(),"visible":editor.scene.visible(id),"locked":editor.scene.view.locked.contains(&id),"faces":editor.scene.view.faces.get(&id),"edges":editor.scene.view.edges.get(&id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-canvas-drag",&editor.canvas_drag.as_ref().map_or_else(||"null".into(),|d|serde_json::json!({"from":d.from,"to":d.to,"released":d.released,"missed":d.missed,"preview":d.preview.as_ref().map(|p|p.matrix())}).to_string()))?;
    canvas.set_attribute(
        "data-camera-frame",
        &serde_json::to_string(&editor.camera.frame).unwrap(),
    )?;
    canvas.set_attribute("data-view-flags",&serde_json::json!({"current":crate::layer_edit::current_key(&editor.scene),"rows":view_rows}).to_string())?;
    canvas.set_attribute("data-tree",&serde_json::json!(crate::hierarchy::rows(&editor.scene).iter().map(|r|serde_json::json!({"key":r.key,"label":r.label,"depth":r.depth,"end":r.end,"readonly":r.readonly,"ids":r.ids.iter().map(|id|id.value()).collect::<Vec<_>>()})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-gizmo",&serde_json::json!({"origin":editor.gizmo_origin(),"active":editor.gizmo_active(),"pointer":editor.gizmo_edit.as_ref().and_then(|e|e.pointer),"typing":editor.gizmo_edit.as_ref().and_then(|e|e.gizmo.typing).map(|h|h.title()),"matrix":editor.gizmo_edit.as_ref().map(|e|e.preview.matrix()),"gpu":renderer.gizmo_usage()}).to_string())?;
    canvas.set_attribute(
        "data-row-placements",
        &serde_json::to_string(&placements)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute(
        "data-pick-gpu",
        &serde_json::to_string(&renderer.pick_usage()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-point-gpu",
        &serde_json::to_string(&renderer.point_usage()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-cloud-cpu",
        &serde_json::to_string(&crate::memory::clouds(
            editor.presentation_scenes(),
        ))
        .unwrap(),
    )?;
    canvas.set_attribute(
        "data-cloud-gpu",
        &serde_json::to_string(&renderer.cloud_usage()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-cloud-error",
        &renderer.cloud_error().unwrap_or_default(),
    )?;
    canvas.set_attribute("data-cloud-sources",&serde_json::json!(editor.scene.clouds().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source_guid":row.prepared.source.guid(),"point_size":row.prepared.source.point_size,"point_px":row.point_px,"points":row.prepared.source.len(),"nodes":row.prepared.nodes.len(),"model":row.model.m})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute(
```

## <a id="checkpoint-66af-connect-browser-save-open-url-adoption"></a>44. Wire Browser Save, Open, and URL Loading

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 44 of 44</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser_runtime.rs, src…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original source archive</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">file or URL adoption</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/manifest.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">drawing and release</text>
</svg>
</div>

**What this code does:** Connect browser save open and URL adoption.

### 1. `src/browser_runtime.rs`

Connect original source publication through the current browser runtime and command flow.

**Locate this existing block:**

```rust
    _listeners: Listeners,
    request: Rc<RefCell<ReadGate>>,
    reload: Shared,
    fault: crate::gpu_fault::Fault,
    document: crate::drawing_document::Shared,
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.request.borrow_mut().cancel();
        self.reload.borrow_mut().cancel();
    }
}

thread_local! { static ACTIVE: RefCell<Option<Runtime>> = const { RefCell::new(None) }; }

pub fn install(
    listeners: Listeners,
    request: Rc<RefCell<ReadGate>>,
    reload: Shared,
    fault: crate::gpu_fault::Fault,
    document: crate::drawing_document::Shared,
) {
    ACTIVE.with(|slot| {
        slot.replace(Some(Runtime {
            _listeners: listeners,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    _listeners: Listeners,
    _publication: crate::browser_publication::Publication,
    request: Rc<RefCell<ReadGate>>,
    reload: Shared,
    fault: crate::gpu_fault::Fault,
    document: crate::drawing_document::Shared,
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.request.borrow_mut().cancel();
        self.reload.borrow_mut().cancel();
    }
}

thread_local! { static ACTIVE: RefCell<Option<Runtime>> = const { RefCell::new(None) }; }

pub fn install(
    listeners: Listeners,
    request: Rc<RefCell<ReadGate>>,
    reload: Shared,
    fault: crate::gpu_fault::Fault,
    document: crate::drawing_document::Shared,
    publication: crate::browser_publication::Publication,
) {
    ACTIVE.with(|slot| {
        slot.replace(Some(Runtime {
            _publication: publication,
            _listeners: listeners,
```

### 2. `src/file_input.rs`

Connect original source publication through the current browser runtime and command flow.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let location = Rc::new(crate::reload_url::ReloadUrl::from_file(
        &file,
    )?);
    Ok(Some(if replace {
        crate::editor::Action::ReplaceAt(array.to_vec(), location)
    } else {
        crate::editor::Action::ImportAt(array.to_vec(), location)
    }))
```

Replace it with:

```rust
    let location = Rc::new(crate::reload_url::ReloadUrl::from_file(
        &file,
    )?);
    let bytes = array.to_vec();
    let replace = replace || bytes.starts_with(crate::archive::MAGIC);
    let ready = crate::publication::prepare(scene, &bytes, replace, Some(location))
        .map_err(|error| JsValue::from_str(&error))?;
    Ok(Some(crate::editor::Action::Adopt {
        scene: Box::new(ready),
        replace,
    }))
```

**Locate this block:**

```rust
    else {
        return Ok(None);
    };
    if array.length() as usize > crate::document::MAX_BYTES {
        return Ok(None);
    }
    let Some(file) = delivery.borrow_mut().take() else {
```

Replace it with:

```rust
    else {
        return Ok(None);
    };
    if array.length() as usize > crate::archive::LIMIT {
        return Ok(None);
    }
    let Some(file) = delivery.borrow_mut().take() else {
```

**Locate this block:**

```rust
pub fn action(
    event: &web_sys::Event,
    delivery: &Delivery,
) -> Result<Option<crate::editor::Action>, JsValue> {
    let Some(event) = event.dyn_ref::<web_sys::CustomEvent>() else {
        return Ok(None);
```

Replace it with:

```rust
pub fn action(
    event: &web_sys::Event,
    delivery: &Delivery,
    scene: &crate::scene::Scene,
) -> Result<Option<crate::editor::Action>, JsValue> {
    let Some(event) = event.dyn_ref::<web_sys::CustomEvent>() else {
        return Ok(None);
```

**Locate this block:**

```rust
        }
        if file.size() > crate::document::MAX_BYTES as f64 {
            let accepted = request.borrow_mut().finish(id);
            if accepted {
                failure("This checkpoint accepts files up to 4 MiB");
            }
            return;
        }
```

Replace it with:

```rust
        }
        if file.size() > crate::archive::LIMIT as f64 {
            let accepted = request.borrow_mut().finish(id);
            if accepted {
                failure("Session exceeds the 512 MiB file limit");
            }
            return;
        }
```

### 3. `src/browser.rs`

Connect original source publication through the current browser runtime and command flow.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        owned_reload,
        fault,
        owned_editor,
    );
    report(&recovery.unwrap_or_else(|| {
        "Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag."
            .into()
    }));
    Ok(())
}
```

Replace it with:

```rust
        owned_reload,
        fault,
        owned_editor,
        publication,
    );
    report(&recovery.unwrap_or_else(|| {
        "Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag."
            .into()
    }));
    start_publication.begin();
    Ok(())
}

fn closing_action(action: &Action) -> bool {
    matches!(
        action,
        Action::Close
            | Action::Adopt { .. }
            | Action::Replace(_)
            | Action::ReplaceAt(_, _)
    )
}
```

**Locate this block:**

```rust
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
```

Replace it with:

```rust
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
```

**Locate this block:**

```rust
                        } else if closing {
                            report("Document closed.");
                            panel.result("Document closed.");
                        } else if event.type_() == "viewer-file" {
                            crate::browser_phase::revision(&editor.scene);
                            let message = if replacement {
```

Replace it with:

```rust
                        } else if closing {
                            report("Document closed.");
                            panel.result("Document closed.");
                        } else if event.type_() == "viewer-publication" {
                            crate::browser_phase::revision(&editor.scene);
                            let _ = crate::browser_report::observe(
                                "live-reload",
                                "source bytes changed",
                            );
                            report("URL scene replaced.");
                            panel.result("URL scene replaced.");
                        } else if event.type_() == "viewer-file" {
                            crate::browser_phase::revision(&editor.scene);
                            let message = if replacement {
```

**Locate this block:**

```rust
                    &action,
                    Action::Close
                        | Action::UnloadSources
                        | Action::Undo
                        | Action::Redo
                        | Action::Replace(_)
                        | Action::ReplaceAt(_, _)
                ) {
                    reload.borrow_mut().cancel();
                }
                let replacement = matches!(
                    &action,
                    Action::Replace(_) | Action::ReplaceAt(_, _)
                );
                let closing = matches!(&action, Action::Close);
                let unloading = matches!(&action, Action::UnloadSources);
```

Replace it with:

```rust
                    &action,
                    Action::Close
                        | Action::UnloadSources
                        | Action::Live(_)
                        | Action::Undo
                        | Action::Redo
                        | Action::Adopt { .. }
                        | Action::Replace(_)
                        | Action::ReplaceAt(_, _)
                ) {
                    reload.borrow_mut().cancel();
                }
                if closing_action(&action) {
                    publication_control.cancel();
                }
                let replacement = matches!(
                    &action,
                    Action::Adopt {
                        replace: true,
                        ..
                    } | Action::Replace(_)
                        | Action::ReplaceAt(_, _)
                );
                let closing = matches!(&action, Action::Close);
                let unloading = matches!(&action, Action::UnloadSources);
```

**Locate this block:**

```rust
                                    if waiting {
                                        Ok("Reloading editable sources…".to_owned())
                                    } else {
                                        crate::document::snapshot(&editor.scene)
                                            .map_err(str::to_owned)
                                            .and_then(|bytes| save_result(&bytes))
                                    }
                                });
```

Replace it with:

```rust
                                    if waiting {
                                        Ok("Reloading editable sources…".to_owned())
                                    } else {
                                        crate::scene_archive::save(&editor.scene)
                                            .and_then(|bytes| save_result(&bytes))
                                    }
                                });
```

**Locate this block:**

```rust
                                None
                            }
                            Host::Open | Host::Replace => {
                                let ticket = request.borrow_mut().begin();
                                if let Err(error) = ticket {
                                    report(error);
```

Replace it with:

```rust
                                None
                            }
                            Host::Open | Host::Replace => {
                                publication_control.cancel();
                                let ticket = request.borrow_mut().begin();
                                if let Err(error) = ticket {
                                    report(error);
```

**Locate this block:**

```rust
                    }
                    Ok(crate::commands::Command::Editor(mut action)) => {
                        if matches!(action, Action::Close) {
                            request.borrow_mut().cancel();
                            let _ = {
                                text_fonts.clear();
```

Replace it with:

```rust
                    }
                    Ok(crate::commands::Command::Editor(mut action)) => {
                        if matches!(action, Action::Close) {
                            publication_control.cancel();
                            request.borrow_mut().cancel();
                            let _ = {
                                text_fonts.clear();
```

**Locate this block:**

```rust
                    }
                }
                None
            } else if event.type_() == "viewer-file" {
                match crate::file_input::action(&event, &delivery) {
                    Ok(action) => action,
                    Err(error) => {
                        let message = format!("Cannot retain reload file: {error:?}");
                        report(&message);
                        panel.answer("Open", &message);
                        None
```

Replace it with:

```rust
                    }
                }
                None
            } else if event.type_() == "viewer-publication" {
                match publication_control.take() {
                    Ok(action) => action,
                    Err(error) => {
                        let message = format!("Cannot load scene: {error}");
                        report(&message);
                        panel.result(&message);
                        None
                    }
                }
            } else if event.type_() == "viewer-file" {
                match crate::file_input::action(&event, &delivery, &editor.scene) {
                    Ok(action) => action,
                    Err(error) => {
                        let message = format!("Cannot open file: {error:?}");
                        report(&message);
                        panel.answer("Open", &message);
                        None
```

**Locate this block:**

```rust
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
```

Replace it with:

```rust
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let publication = crate::browser_publication::Publication::new(
        std::rc::Rc::clone(&shared),
        fault.clone(),
    )?;
    let publication_control = publication.control();
    let start_publication = publication_control.clone();
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
```

## Build & Run

![Recorded visual output for Lesson 12.1 (66af-connect-browser-save-open-url-adoption)](../../../screenshots/journey/66af-connect-browser-save-open-url-adoption.png)

```sh
npm --prefix ../session_tests run course -- dependencies 12.1
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 12.1
npm --prefix ../session_tests run course -- save 12.1
```

Continue with [12.2 · Locate original arrays without downloading their bodies](../../consolidation/67-supply-checked-array-layout-readers.md).
