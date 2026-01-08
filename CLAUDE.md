# CLAUDE.md

Multi-language geometry kernel (Python, C++, Rust) with shared protobuf schemas and Vue test viewer.

## Style

- Be least verbose, without any print messages and exccesive comments
- When pushing code to github 

## Workflow

Run claude always: claude --dangerously-skip-permissions

## Structure
```
session_cpp/     # C++ (submodule)
session_py/      # Python (submodule)
session_rust/    # Rust (submodule)
session_proto/   # Protobuf schemas (submodule)
session_tests/   # Vue 3 test viewer
```

## Build

**C++:**
```bash
cd session_cpp && mkdir -p build && cd build && cmake .. && make tests -j$(nproc)
```

**Python:**
```bash
uv venv uvsession --python 3.11 && source uvsession/bin/activate
cd session_py && uv pip install -e . && pytest -v
```

**Rust:**
```bash
cd session_rust && cargo build --release && cargo test
```

## Test Viewer

```bash
./bash/minitest.sh            # Run all tests + launch viewer at localhost:8769
```

## Fast Development Workflow

### Build Time Reference
| Component | First Build | Incremental |
|-----------|-------------|-------------|
| C++ (with protobuf) | 15-25 min | 1-5 min |
| Rust | 5-10 min | 10-30 sec |
| Python | instant | instant |
| Vue | 1-2 min | 10-30 sec |

### Single-Language Development (FASTEST)
When working on one language, skip others:
```bash
./bash/minitest.sh --py --no-web      # Python only (instant)
./bash/minitest.sh --rust --no-web    # Rust only (fast)
./bash/minitest.sh --cpp --no-web     # C++ only
```

### Quick Single-Class Test
Test one class without full rebuild:
```bash
./bash/quicktest.sh point             # Test Point in all languages
./bash/quicktest.sh point --py        # Test Point in Python only
./bash/quicktest.sh mesh --rust       # Test Mesh in Rust only
```

### Fast Mode (Skip Dependencies)
After first build, use fast mode to skip pip/npm/protobuf:
```bash
./bash/minitest.sh --fast             # Skip dependency installs
./bash/minitest.sh --fast --py        # Fast Python only
```

### Development Order (Recommended)
1. **Prototype in Python** (instant feedback)
2. **Port to Rust** (fast incremental builds)
3. **Port to C++** (slowest, do last)
4. **Run full minitest** before commit

### Web Viewer Control
```bash
./bash/minitest.sh --no-web           # Skip Vue entirely
./bash/minitest.sh --kill             # Stop running dev server
```

### Pre-warm Builds (First Time Setup)
Run once to cache dependencies:
```bash
# Build C++ with protobuf (slow, but cached after)
cd session_cpp && cmake -B build -DENABLE_PROTOBUF=ON && cmake --build build --config Release

# Build Rust dependencies (slow, but cached after)
cd session_rust && cargo build --release --features protobuf
```

### IDE Integration Tips
- **VS Code:** Use language-specific tasks for single-language builds
- **Rust:** `cargo watch -x run` for auto-rebuild on save
- **Python:** Run tests directly: `python -m session_py.point_test`
- **C++:** Use ccache/sccache (auto-detected by CMakeLists.txt)

## Git

```bash
git clone --recurse-submodules <url>
git submodule update --init --recursive
./bash/git_push.sh "message"
```

## GitHub Actions

- after pushing with ./bash/git_push.sh, check GitHub Actions build status using: gh run list --limit 5
- if build fails, view logs with: gh run view <run-id> --log-failed
- fix the failing code locally, run ./bash/minitest.sh to verify, then push again
- all three languages (C++, Python, Rust) must pass CI before merge
- **macOS runners:** use `macos-15` for ARM64, `macos-15-intel` for Intel x64
- **Linux:** use `manylinux_2_28_x86_64` container for glibc compatibility

## MINITEST

### Adding New Datastructure to Test Viewer (3 Languages)

1. **Create implementation files:**
   - `session_rust/src/name.rs` - Rust implementation
   - `session_py/src/session_py/name.py` - Python implementation
   - `session_cpp/src/name.h` + `name.cpp` - C++ implementation

2. **Create minitest files:**
   - `session_rust/src/name_minitest.rs` - use `MINI_TEST!`, `MINI_CHECK!`, `REGISTER_MINI_TEST!`
   - `session_py/src/session_py/name_minitest.py` - use `@MINI_TEST`, `MINI_CHECK`
   - `session_cpp/src/name_minitest.cpp` - use `MINI_TEST`, `MINI_CHECK`

3. **Register in build system:**
   - **Rust:** Add `pub mod name_minitest;` to `session_rust/src/lib.rs`
   - **C++:** Add `src/name_minitest.cpp` to `MINITEST_SOURCES` in `session_cpp/CMakeLists.txt`
   - **Shell:** Add `"name"` to `CLASS_NAMES` array in `bash/minitest.sh`

4. **Verify:** Run `./bash/minitest.sh` - all tests must pass in all 3 languages

### Test Requirements

- datastructures name_test.py, name_test.rs, name_test.cpp must include separate tests for each class api method
- when using math pi constant, use it from tolerance class
- all api functions must be tested across all three languages (C++, Python, Rust)
- test names and test logic must be identical across languages
- each test should verify one specific api method or behavior
- api method order in all implementations: constructors/factory methods, accessors/getters, in-place mutators (*_self methods), copy-return operators (arithmetic returning new objects), utility methods (is_valid, distance_to, etc.), serialization (to_proto, from_proto, json_dump, json_load), string representation (str, repr)
- json serialization requires json_dump and json_load methods on all geometry classes
- protobuf serialization requires to_proto and from_proto methods on all geometry classes
- test files output to session_tests/session_{lang}/ as JSON for the Vue test viewer
- common methods across all geometry classes (Color, Point, Vector, Line, Plane, Polyline, Xform): constructor with default parameters, guid and name metadata fields, duplicate() for rust and python and cpp = operator duplicates the instance, /clone() creates new instance with new GUID, index operator [] for component access, equality operators == and !=, __str__/__repr__/to_string for string representation, __jsondump__/__jsonload__ for JSON dict conversion, json_dump(filepath)/json_load(filepath) for file I/O, to_proto/from_proto for protobuf serialization
- visual geometry classes (Point, Line, Plane, Polyline) have: width, color, xform fields, transform()/transformed() methods
- arithmetic classes (Point, Vector, Line, Polyline) have: in-place operators (+=, -=, *=, /=), copy operators (+, -, *, /)
- duplicate() copies all data (coordinates, name, visual properties) but generates a new GUID for the copy, in C++ the = operator and copy constructor behave the same way
- constructor test groups related functionality: default constructor, constructor overloads, index operator [], equality operators == !=, str() and repr() output, all tested together in single "constructor" test
- Vue test viewer shows serialized JSON output at bottom of each test result, showing exact JSON structure for each geometry class
- protobuf schemas defined in session_proto/*.proto files, defines binary serialization format for all geometry classes
- check if all tests passes in all languages
- check if you implemented minitest for json de/serialization and protobuf de/serialization
- check if all the operators minitests are part of constructor test not separate tests
- run ./bash/minitest.sh

### JSON Serialization Conventions

**Alphabetical Field Ordering:** All JSON serialization (`jsondump`/`__jsondump__`) must output fields in **alphabetical order** to match Rust's `serde_json` output. This ensures consistent JSON output across all three languages (C++, Python, Rust).

Example for Point:
```json
{
  "guid": "...",
  "name": "...",
  "pointcolor": {...},
  "type": "Point",
  "width": 1.0,
  "x": 0.0,
  "xform": {...},
  "y": 0.0,
  "z": 0.0
}
```

**Implementation:**
- **C++:** Use `nlohmann::ordered_json` and add fields in alphabetical order
- **Python:** Return dict with keys in alphabetical order
- **Rust:** Uses `serde_json::json!` which outputs alphabetically by default

**Nested Objects:** Also use alphabetical ordering for nested object fields (e.g., vertex data `attributes, x, y, z`).

### Code Style Rules

- **Python imports:** Each import must be on a separate line. Never use `from session_py import Mesh, Point`. Use separate lines instead.
- **C++ tolerance.h:** Never include `#include "tolerance.h"` in main source files. It is only for minitest files, not production code.

### Git Rules

- **NEVER add Claude/AI as a git contributor, author, or co-author.** All commits must be attributed to the human user only.
- Do not modify git author settings or add AI attribution to commits.
- Do not add Claude to CONTRIBUTORS, AUTHORS, or similar files.

