//! Check retained CAD edge IDs after upload for files from `cad_fixture`.
use session_viewer::selftest::check_cad_edges;

/// Exercise source and affine instances through the native production scene coordinator.
fn main() {
    let directory = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/cad-fixture".into());
    let mut paths = Vec::new();
    for kind in ["cylinder", "sphere", "hole"] {
        for transform in ["source", "affine"] {
            paths.push(format!("{directory}/{kind}-{transform}.pb"));
        }
    }
    check_cad_edges(&paths);
}
