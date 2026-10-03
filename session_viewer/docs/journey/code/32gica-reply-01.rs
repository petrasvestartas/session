use crate::{edit_intent::Intent, rehydrate::ReloadKey};

pub struct Reply {
    pub intent: Option<Intent>,
    pub result: Result<Vec<(ReloadKey, Vec<u8>)>, String>,
}

impl Reply {
    pub fn new(keys: Vec<ReloadKey>, intent: Option<Intent>, values: Result<Vec<Vec<u8>>, String>) -> Self {
        let result = values.and_then(|values| {
            if values.len() != keys.len() { return Err("Reload body count does not match its keys".into()); }
            Ok(keys.into_iter().zip(values).collect())
        });
        Self { intent, result }
    }
}
