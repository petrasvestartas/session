//! Run the three existing CAD mini-test registries without starting the viewer or a GPU.

/// Preserve each registered test's assertions and fail if a selected suite disappears.
fn main() {
    std::fs::create_dir_all("serialization").expect("create mini-test serialization directory");
    // Legacy file-roundtrip tests write beneath the shared crate's manifest directory.
    let shared_output =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../session_rust/serialization");
    std::fs::create_dir_all(shared_output).expect("create shared roundtrip output directory");
    let groups = ["RemeshNurbsSurfaceGrid", "NurbsSurfaceTrimmed", "BRep"];
    let mut counts = [0; 3];
    let mut failures = 0;
    for test in session_rust::mini_test::get_all_tests() {
        for (index, group) in groups.iter().enumerate() {
            if test.group == *group {
                let result = (test.func)();
                counts[index] += 1;
                if !result.passed {
                    eprintln!(
                        "FAILED {}::{}: {:?}",
                        test.group, test.name, result.failures
                    );
                    failures += 1;
                }
            }
        }
    }
    let mut total = 0;
    for (group, count) in groups.iter().zip(counts) {
        assert!(count > 0, "missing CAD mini-test group: {group}");
        println!("{group}: {count} tests");
        total += count;
    }
    println!("[rust-minitest] {}/{total} passed", total - failures);
    assert_eq!(failures, 0, "shared CAD mini-tests failed");
}
