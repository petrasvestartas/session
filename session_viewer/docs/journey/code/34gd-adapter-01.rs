use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AdapterInfo {
    pub vendor: String,
    pub architecture: String,
    pub device: String,
    pub description: String,
}

impl AdapterInfo {
    pub fn valid(&self) -> bool {
        [&self.vendor, &self.architecture, &self.device, &self.description]
            .into_iter().all(|text| text.chars().count() <= 256)
    }
}
