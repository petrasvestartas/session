use super::manifest::{Item, TextItem};
use serde::Deserialize;
use std::collections::HashMap;

/// A slide deck beside its geometry files, loaded with `?presentation=`.
#[derive(Clone, Debug, Deserialize)]
pub struct Presentation {
    pub name: String,
    #[serde(default = "slide_width")]
    pub width: f64,
    #[serde(default = "slide_height")]
    pub height: f64,
    #[serde(default = "stroke_scale")]
    pub stroke_scale: f64,
    #[serde(default = "head_scale")]
    pub head_scale: f64,
    #[serde(default = "spatial_split")]
    pub spatial_split: f64,
    #[serde(default)]
    pub spatial: Vec<String>,
    #[serde(default)]
    pub non_pickable: Vec<String>,
    pub topics: Vec<Topic>,
    #[serde(default)]
    pub dark: bool,
    #[serde(default)]
    pub links: Vec<Vec<String>>,
    #[serde(default)]
    pub gradients: HashMap<String, Gradient>,
    #[serde(default)]
    pub colored: HashMap<String, Vec<u32>>,
}

/// A group of steps that accumulate on one canvas.
#[derive(Clone, Debug, Deserialize)]
pub struct Topic {
    pub title: String,
    #[serde(default)]
    pub preserve_spatial: bool,
    pub steps: Vec<Step>,
}

/// One click's worth of geometry, text and effects.
#[derive(Clone, Debug, Deserialize)]
pub struct Step {
    pub name: String,
    #[serde(default)]
    pub caption: String,
    #[serde(default)]
    pub items: Vec<Item>,
    #[serde(default)]
    pub texts: Vec<TextItem>,
    #[serde(default)]
    pub effects: Vec<Effect>,
    #[serde(default)]
    pub highlight: Vec<String>,
}

/// An effect played when its step appears.
#[derive(Clone, Debug, Deserialize)]
pub struct Effect {
    #[serde(alias = "target")]
    pub name: String,
    #[serde(rename = "kind", default = "grow_effect")]
    pub effect: EffectKind,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Option<[f64; 3]>,
    #[serde(default)]
    pub about: Option<[f64; 3]>,
    #[serde(default)]
    pub factor: Option<f64>,
    #[serde(default = "effect_duration")]
    pub duration_ms: f64,
    #[serde(default)]
    pub delay_ms: f64,
}

/// Effects understood by the ordinary geometry lanes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EffectKind {
    Grow,
    Pulse,
    Highlight,
    Cells,
    Morph,
    Move,
    Scale,
}

/// End color of a line; its start color is in the session geometry.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Gradient {
    pub from: [u8; 4],
    pub to: [u8; 4],
}

/// A named object, a matrix row, a contiguous cell span, or one matrix entry.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Target {
    pub name: String,
    pub cells: Option<([u32; 2], [u32; 2])>,
}

impl Target {
    /// Parse `edge0`, `C/0`, `C/0:2` or `C[0,1]`.
    pub fn parse(text: &str) -> Result<Self, String> {
        let text = text.trim();
        if text.is_empty() {
            return Err("an object name cannot be empty".into());
        }
        if let Some((name, row)) = text.split_once('/') {
            if row.contains('/') || name.is_empty() {
                return Err(format!("invalid matrix target '{text}'"));
            }
            let (first, last) = row.split_once(':').unwrap_or((row, row));
            let row = number(first, text)?;
            let end = number(last, text)?;
            if end < row {
                return Err(format!("invalid matrix row span '{text}'"));
            }
            return Ok(Self {
                name: name.to_string(),
                cells: Some(([row, 0], [end, u32::MAX])),
            });
        }
        if let Some(prefix) = text.strip_suffix(']')
            && let Some((name, entry)) = prefix.split_once('[')
        {
            let (row, col) = entry
                .split_once(',')
                .ok_or_else(|| format!("matrix entry '{text}' must have row,column indices"))?;
            if name.is_empty() {
                return Err(format!("invalid matrix target '{text}'"));
            }
            let cell = [number(row, text)?, number(col, text)?];
            return Ok(Self {
                name: name.to_string(),
                cells: Some((cell, cell)),
            });
        }
        if text.contains(['[', ']']) {
            return Err(format!("invalid matrix target '{text}'"));
        }
        Ok(Self {
            name: text.to_string(),
            cells: None,
        })
    }

    /// The whole named object, dropping a cell qualifier.
    pub fn object(&self) -> Self {
        Self {
            name: self.name.clone(),
            cells: None,
        }
    }

    /// Resolve a row's `u32::MAX` column against the actual matrix width.
    pub fn cell_range(&self, shape: (u32, u32)) -> Option<([u32; 2], [u32; 2])> {
        let (first, mut last) = self.cells?;
        if first[0] >= shape.0 || last[0] >= shape.0 {
            return None;
        }
        if last[1] == u32::MAX {
            last[1] = shape.1.checked_sub(1)?;
        }
        if first[1] > last[1] || last[1] >= shape.1 || last[0] > 0xffff || last[1] > 0xffff {
            return None;
        }
        Some((first, last))
    }

    /// The row-major packed span read by matrix.wgsl.
    pub fn packed_range(&self, shape: (u32, u32)) -> Option<(u32, u32)> {
        let (first, last) = self.cell_range(shape)?;
        Some((pack_cell(first), pack_cell(last)))
    }
}

/// A cell index packed into 16 bits each, row first.
pub fn pack_cell(cell: [u32; 2]) -> u32 {
    (cell[0] << 16) | cell[1]
}

impl Presentation {
    /// Parse and validate YAML (JSON is a YAML subset too).
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > 4 * 1024 * 1024 {
            return Err("presentation exceeds 4 MiB".into());
        }
        let text =
            std::str::from_utf8(bytes).map_err(|e| format!("presentation is not UTF-8: {e}"))?;
        let deck: Self =
            serde_yaml_ng::from_str(text).map_err(|e| format!("invalid presentation: {e}"))?;
        deck.validate()?;
        Ok(deck)
    }

    /// Reject incomplete decks before any scene rows are loaded.
    fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("presentation needs a name".into());
        }
        if !self.width.is_finite()
            || !self.height.is_finite()
            || self.width <= 0.0
            || self.height <= 0.0
        {
            return Err("presentation width and height must be positive and finite".into());
        }
        if !self.stroke_scale.is_finite() || self.stroke_scale <= 0.0 {
            return Err("presentation stroke scale must be positive and finite".into());
        }
        if !self.head_scale.is_finite() || self.head_scale <= 0.0 {
            return Err("presentation head scale must be positive and finite".into());
        }
        if !self.spatial_split.is_finite() || !(0.2..=0.8).contains(&self.spatial_split) {
            return Err("presentation spatial split must be between 0.2 and 0.8".into());
        }
        for name in &self.spatial {
            if name.trim().is_empty() {
                return Err("a spatial object needs a nonempty name".into());
            }
        }
        if self.topics.is_empty() || self.step_count() > 4096 {
            return Err("presentation needs 1..4096 steps".into());
        }
        for topic in &self.topics {
            if topic.title.trim().is_empty() || topic.steps.is_empty() {
                return Err("every topic needs a title and at least one step".into());
            }
            for step in &topic.steps {
                if step.name.trim().is_empty() {
                    return Err("every step needs a name".into());
                }
                for item in &step.items {
                    if item.file.trim().is_empty() || item.file.split('/').any(|part| part == "..")
                    {
                        return Err(format!("step '{}': invalid geometry file", step.name));
                    }
                }
                for effect in &step.effects {
                    Target::parse(&effect.name)?;
                    if !effect.duration_ms.is_finite()
                        || !(0.0..=60_000.0).contains(&effect.duration_ms)
                        || !effect.delay_ms.is_finite()
                        || !(0.0..=60_000.0).contains(&effect.delay_ms)
                    {
                        return Err(format!(
                            "effect '{}': duration and delay must be 0..60000 ms",
                            effect.name
                        ));
                    }
                    match effect.effect {
                        EffectKind::Morph => {
                            let Some(source) = &effect.from else {
                                return Err(format!(
                                    "effect '{}': morph needs a source",
                                    effect.name
                                ));
                            };
                            Target::parse(source)?;
                        }
                        EffectKind::Move => {
                            if effect.to.is_none_or(|at| at.iter().any(|v| !v.is_finite())) {
                                return Err(format!(
                                    "effect '{}': move needs a finite destination",
                                    effect.name
                                ));
                            }
                        }
                        EffectKind::Scale => {
                            if effect.factor.is_none_or(|v| !v.is_finite() || v <= 0.0)
                                || effect
                                    .about
                                    .is_none_or(|at| at.iter().any(|v| !v.is_finite()))
                            {
                                return Err(format!(
                                    "effect '{}': scale needs a positive factor and finite center",
                                    effect.name
                                ));
                            }
                        }
                        EffectKind::Grow
                        | EffectKind::Pulse
                        | EffectKind::Highlight
                        | EffectKind::Cells => {}
                    }
                }
                for name in &step.highlight {
                    Target::parse(name)?;
                }
            }
        }
        for (name, gradient) in &self.gradients {
            if name.trim().is_empty() {
                return Err("a gradient needs an object name".into());
            }
            if gradient.from[3] == 0 && gradient.to[3] == 0 {
                return Err(format!("gradient '{name}' is transparent at both ends"));
            }
        }
        for (name, cells) in &self.colored {
            if name.trim().is_empty() || cells.len() > 64 || cells.iter().any(|cell| *cell >= 64) {
                return Err(format!("invalid colored cells for matrix '{name}'"));
            }
        }
        for group in &self.links {
            if group.len() < 2 {
                return Err("a highlight link needs at least two objects".into());
            }
            for name in group {
                Target::parse(name)?;
            }
        }
        Ok(())
    }

    /// Number of steps across all topics.
    pub fn step_count(&self) -> usize {
        self.topics.iter().map(|topic| topic.steps.len()).sum()
    }

    /// The first flat step index of `topic`.
    pub fn topic_start(&self, topic: usize) -> usize {
        self.topics
            .iter()
            .take(topic)
            .map(|topic| topic.steps.len())
            .sum()
    }

    /// A step's topic and local index from its flat index.
    pub fn locate(&self, mut index: usize) -> Option<(usize, usize)> {
        for (topic, group) in self.topics.iter().enumerate() {
            if index < group.steps.len() {
                return Some((topic, index));
            }
            index -= group.steps.len();
        }
        None
    }

    /// A step by flat index.
    pub fn step(&self, index: usize) -> Option<&Step> {
        let (topic, step) = self.locate(index)?;
        self.topics[topic].steps.get(step)
    }

    /// The position to show after `delta` clicks, clamped at both ends.
    pub fn navigate(&self, index: usize, topic: usize, delta: i32) -> (usize, usize) {
        let next = (index as i32 + delta).clamp(0, self.step_count() as i32 - 1) as usize;
        (next, self.locate(next).map_or(topic, |(topic, _)| topic))
    }
}

fn number(text: &str, target: &str) -> Result<u32, String> {
    text.parse::<u32>()
        .map_err(|_| format!("invalid matrix target '{target}'"))
}

fn slide_width() -> f64 {
    1600.0
}

fn slide_height() -> f64 {
    760.0
}

fn stroke_scale() -> f64 {
    1.0
}

fn head_scale() -> f64 {
    1.0
}

fn spatial_split() -> f64 {
    0.56
}

fn grow_effect() -> EffectKind {
    EffectKind::Grow
}

fn effect_duration() -> f64 {
    650.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_topics_steps_effects_gradients_and_links() {
        let deck = Presentation::parse(
            r#"
name: Force density
width: 1600
height: 760
topics:
  - title: Equilibrium
    steps:
      - name: node
        caption: One free node
        items: [{file: steps/node.pb}]
        texts: [{name: n0-label, text: n₀, at: [10, 20, 0], height: 24, plate: false, outline: 2}]
        effects: [{name: n0, kind: pulse, duration_ms: 800}]
  - title: The force density trick
    steps:
      - name: trick
        items: [{file: steps/trick.pb}]
        effects: [{name: C, kind: cells}]
links: [[edge0, C/0], [n0, "C[0,1]"]]
gradients:
  force: {from: [20, 60, 200, 255], to: [220, 50, 40, 255]}
"#
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(deck.step_count(), 2);
        assert_eq!(deck.topics[0].steps[0].texts[0].outline, 2.0);
        assert_eq!(deck.gradients["force"].to, [220, 50, 40, 255]);
        assert_eq!(deck.navigate(0, 0, 1), (1, 1));
        assert_eq!(deck.navigate(1, 1, -1), (0, 0));
        assert_eq!(deck.navigate(1, 1, 1), (1, 1));
    }

    #[test]
    fn matrix_names_address_rows_spans_and_entries() {
        let object = Target::parse("edge0").unwrap();
        assert_eq!(object.name, "edge0");
        assert_eq!(object.cells, None);
        let row = Target::parse("C/0").unwrap();
        assert_eq!(row.packed_range((4, 3)), Some((0, 2)));
        let span = Target::parse("C/1:2").unwrap();
        assert_eq!(span.packed_range((4, 3)), Some((0x10000, 0x20002)));
        let cell = Target::parse("C[2,1]").unwrap();
        assert_eq!(cell.packed_range((4, 3)), Some((0x20001, 0x20001)));
        assert_eq!(cell.object().name, "C");
        assert!(Target::parse("C[2]").is_err());
        assert!(
            Target::parse("C[2,9]")
                .unwrap()
                .packed_range((4, 3))
                .is_none()
        );
    }

    #[test]
    fn spatial_object_names_may_contain_slashes() {
        let deck = Presentation::parse(
            br#"{
                "name": "3D",
                "spatial": ["net/grid/0/-1"],
                "topics": [{"title": "Net", "steps": [{"name": "net"}]}]
            }"#,
        )
        .expect("a slash in a grid name is not a matrix row address");
        assert_eq!(deck.spatial[0], "net/grid/0/-1");
    }

    #[test]
    fn parses_spatial_math_items() {
        let deck = Presentation::parse(
            br#"{
                "name": "3D",
                "topics": [{
                    "title": "Net",
                    "steps": [{
                        "name": "net",
                        "texts": [{
                            "text": "n_0", "height": 0.42,
                            "math": true, "camera_facing": true, "spatial": true
                        }]
                    }]
                }]
            }"#,
        )
        .unwrap();
        assert!(deck.topics[0].steps[0].texts[0].spatial);
        assert!(deck.topics[0].steps[0].texts[0].camera_facing);
    }

    #[test]
    fn validates_morph_move_scale_and_delay() {
        let deck = Presentation::parse(
            br#"{
                "name": "Derivation",
                "topics": [{"title": "Weights", "steps": [{"name": "solve", "effects": [
                    {"name": "weighted", "from": "average", "kind": "morph", "duration_ms": 900},
                    {"name": "node", "kind": "move", "to": [7.5, -2.5, 0], "delay_ms": 800},
                    {"name": "force", "kind": "scale", "factor": 3, "about": [5, -2.5, 0]}
                ]}]}]
            }"#,
        )
        .unwrap();
        let effects = &deck.topics[0].steps[0].effects;
        assert_eq!(effects[0].from.as_deref(), Some("average"));
        assert_eq!(effects[1].to, Some([7.5, -2.5, 0.0]));
        assert_eq!(effects[1].delay_ms, 800.0);
        assert_eq!(effects[2].factor, Some(3.0));
    }

    #[test]
    fn rejects_unsafe_files_empty_topics_and_bad_effects() {
        for source in [
            "name: x\ntopics: []",
            "name: x\ntopics: [{title: t, steps: []}]",
            "name: x\ntopics: [{title: t, steps: [{name: s, items: [{file: '../secret.pb'}]}]}]",
            "name: x\ntopics: [{title: t, steps: [{name: s, effects: [{name: x, duration_ms: -1}]}]}]",
            "name: x\ntopics: [{title: t, steps: [{name: s, effects: [{name: x, kind: morph}]}]}]",
            "name: x\ntopics: [{title: t, steps: [{name: s, effects: [{name: x, kind: move}]}]}]",
            "name: x\ntopics: [{title: t, steps: [{name: s, effects: [{name: x, kind: scale, factor: -2, about: [0, 0, 0]}]}]}]",
        ] {
            assert!(Presentation::parse(source.as_bytes()).is_err(), "{source}");
        }
    }
}
