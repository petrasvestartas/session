fn original_documents(scene: &Scene) -> Vec<session_rust::proto::Session> {
    use prost::Message;
    crate::archive::Bundle::decode(&crate::scene_archive::save(scene).unwrap())
        .unwrap()
        .documents.into_iter()
        .map(|bytes| session_rust::proto::Session::decode(bytes.as_slice()).unwrap())
        .collect()
}
