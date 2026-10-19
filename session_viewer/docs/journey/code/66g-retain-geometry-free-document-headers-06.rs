pub fn versions(scene: &crate::scene::Scene) -> Vec<String> {
    let mut versions: Vec<_> = scene.objects().iter().filter_map(|object|
        object.origin().map(|origin| origin.version.hex())).chain(scene.forest.documents.iter().filter_map(|doc|doc.header.as_ref().map(|origin|origin.version.hex()))).collect();
    versions.sort(); versions.dedup(); versions
}
