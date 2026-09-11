Port a class implementation from Python to Rust and/or C++.

Usage: /port <class> [--rust|--cpp]  (default: both targets)

## Read first
- `session_py/src/session_py/$ARGUMENTS.py`
- `session_py/src/session_py/$ARGUMENTS_minitest.py`
- Existing Rust/C++ files if present (partial port detection)
- `.claude/skills/` templates for target language patterns

## Porting rules (enforced)
- Variable names, method names, test names: identical to Python
- Coordinate access: ONLY p[0], p[1], p[2] — NEVER p.x, p.y, p.z
- Tests: explicit for loops only — no iterators, no map/collect, no comprehensions
- Tests: inline counts — never `let nv = mesh.number_of_vertices(); ... nv`
- JSON fields: alphabetically ordered
- Line count: match Python density — no extra comments, no verbosity
- Imports: Rust `use` inside MINI_TEST block; C++ `#include` at top of file

## Rust output files
- `session_rust/src/$ARGUMENTS.rs`
- `session_rust/src/$ARGUMENTS_minitest.rs`
- Register: `pub mod $ARGUMENTS; pub mod $ARGUMENTS_minitest;` in `session_rust/src/lib.rs`

## C++ output files
- `session_cpp/src/$ARGUMENTS.h` + `session_cpp/src/$ARGUMENTS.cpp`
- `session_cpp/src/$ARGUMENTS_minitest.cpp`
- Register: add `src/$ARGUMENTS_minitest.cpp` to MINITEST_SOURCES in `session_cpp/CMakeLists.txt`

## Verify
```bash
./bash/quicktest.sh $ARGUMENTS --rust   # Rust
./bash/quicktest.sh $ARGUMENTS --cpp    # C++
```
