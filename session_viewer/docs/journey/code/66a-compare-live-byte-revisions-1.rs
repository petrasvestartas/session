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

