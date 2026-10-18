fn main() -> std::io::Result<()> {
    std::fs::write("sample.pb", viewer_journey::interaction_example::bytes())
}
