fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        print!("{}", session_viewer::selftest::run());
    }
}
