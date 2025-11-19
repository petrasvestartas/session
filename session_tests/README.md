# Mini test overview

This repo has small cross-language mini tests for comparing implementations.

To add a new class (for example `Color`) to the mini-test + website pipeline:

1. **Python**  
   Add `@MINI_TEST("ClassName", "test_name")` functions in `session_py/<class>_test.py` and call `run_all(language="python")` from `if __name__ == "__main__"`.

2. **C++**  
   Add `MINI_TEST(ClassName, test_name)` blocks in `session_cpp/src/<class>_test.cpp`, exclude that file from Catch2 in `CMakeLists.txt`, and add it to the `point_minitest` (or similar) executable.

3. **Rust**  
   Extend `session_rust/src/bin/point_minitest.rs` (or a new bin) with functions that build `TestResult` values for the new class and write `<class>_test.json` into `session_tests/session_rust/`.

4. **session_tests**  
   Ensure `session_tests/minitest.sh` runs the new Python module and Rust/C++ binaries so the JSON files are produced.

5. **Website**  
   In `session_tests/website/index.html`, append the new JSON paths to the `sources` array and, if needed, add a new suite name so you can switch between `<class>_test` groups in the UI.
