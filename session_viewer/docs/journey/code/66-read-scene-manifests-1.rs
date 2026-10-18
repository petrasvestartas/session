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
