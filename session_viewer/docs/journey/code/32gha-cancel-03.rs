use prost::Message;
use session_rust::proto::Session;

fn read(path: &str) -> Session {
    let bytes = std::fs::read(path).expect("Read the browser file");
    Session::decode(bytes.as_slice()).expect("Valid session protobuf")
}

fn main() {
    let paths: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(paths.len(), 2, "Give original and browser-saved files");
    let original = read(&paths[0]); let saved = read(&paths[1]);
    let original = original.objects.unwrap().meshes; let restored = saved.objects.unwrap().meshes;
    assert_eq!(original.len(), restored.len());
    for source in original {
        let row = restored.iter().find(|row| row.guid == source.guid).expect("Saved original GUID");
        assert_eq!(row.vertices, source.vertices, "Exact original double coordinates");
        assert_eq!(row.faces, source.faces, "Original topology");
        assert_eq!(row.name, source.name);
        assert_eq!(row.is_visible.unwrap_or(true), source.is_visible.unwrap_or(true));
        assert_eq!(row.is_locked.unwrap_or(false), source.is_locked.unwrap_or(false));
    }
    assert!(!saved.xforms.is_empty(), "Current placements are saved separately");
    println!("Browser reload/save preserves original geometry, flags, names and saved GUIDs.");
}
