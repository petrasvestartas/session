use session_rust::proto;
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileVersion([u8; 32]);

impl FileVersion {
    pub fn of(bytes: &[u8]) -> Self { Self(Sha256::digest(bytes).into()) }

    pub fn hex(&self) -> String { self.0.iter().map(|b| format!("{b:02x}")).collect() }
}

pub struct Origin {
    pub id: uuid::Uuid,
    pub header: proto::Session,
    pub version: FileVersion,
}

impl Origin {
    pub fn new(message: &proto::Session, bytes: &[u8]) -> Self {
        let header = proto::Session { name: message.name.clone(), guid: message.guid.clone(),
            tree: message.tree.clone(), graph: message.graph.clone(),
            bvh_boxes: message.bvh_boxes.clone(), xforms: message.xforms.clone(), ..Default::default() };
        Self { id: uuid::Uuid::new_v4(), header, version: FileVersion::of(bytes) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_version_matches_a_known_vector_and_changed_bytes_differ() {
        assert_eq!(FileVersion::of(b"abc").hex(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(FileVersion::of(b"abc"), FileVersion::of(b"abc"));
        assert_ne!(FileVersion::of(b"abc"), FileVersion::of(b"abd"));
    }

    #[test]
    fn identical_files_have_distinct_origins_and_geometry_free_headers() {
        use prost::Message;
        let bytes = crate::specimen::bytes(); let message = proto::Session::decode(bytes.as_slice()).unwrap();
        let first = Origin::new(&message, &bytes); let second = Origin::new(&message, &bytes);
        assert_ne!(first.id, second.id); assert_eq!(first.version, second.version);
        assert_eq!(first.header.name, message.name); assert_eq!(first.header.guid, message.guid);
        assert_eq!(first.header.tree, message.tree); assert_eq!(first.header.xforms, message.xforms);
        assert!(first.header.objects.is_none() && first.header.definitions.is_none());
    }
}
