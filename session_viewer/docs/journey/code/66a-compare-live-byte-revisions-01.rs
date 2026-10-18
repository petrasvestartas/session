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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_bytes_survive_strong_weak_missing_and_new_etags() {
        let mut revisions = Revisions::default();
        assert!(revisions.changed("scene", Some("a".into()), b"one"));
        for tag in [Some("W/a".into()), None, Some("b".into()), Some("b".into())] {
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
