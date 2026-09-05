//! Native GPU lifecycle regression: empty face attachments, runtime toggles, uploads and picking.
//! check_hidden_line_lifecycle /tmp/lifecycle [first.pb second.pb ...]

/// Run the native-only harness without exposing the renderer's private implementation modules.
fn main() {
    session_viewer::selftest::lifecycle::run();
}
