//! Check CAD edge ids survive upload for `cad_fixture` files.
use session_viewer::selftest::check_cad_edges;

/// Check the source and affine file of each solid.
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
