use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Phase {
    pub name: String,
    pub duration_ms: f64,
    pub bytes: u64,
    pub source: String,
    pub elapsed_ms: f64,
}

impl Phase {
    pub fn valid(&self) -> bool {
        !self.name.is_empty() && self.name.chars().count() <= 64
            && self.source.chars().count() <= 1024 && !self.source.contains(['?', '#'])
            && self.duration_ms.is_finite() && self.duration_ms >= 0.0
            && self.elapsed_ms.is_finite() && self.elapsed_ms >= self.duration_ms
    }
}

pub fn zero(value: &u64) -> bool { *value == 0 }
