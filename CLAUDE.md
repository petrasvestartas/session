# CLAUDE.md

Multi-language geometry kernel (Python, C++, Rust) with shared protobuf schemas and Vue test viewer.

## Structure
```
session_cpp/     # C++ (submodule)
session_py/      # Python (submodule)
session_rust/    # Rust (submodule)
session_proto/   # Protobuf schemas (submodule)
session_tests/   # Vue 3 test viewer
```

## Build

```bash
./build.sh                    # All languages
```

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
./minitest.sh                 # Run all tests + launch viewer at localhost:8769
```

## Add New Test Suite (e.g., bbox)

### 1. minitest.sh - Add to these arrays:
- `cleanup_json()` - add JSON paths
- Python test execution section
- Rust test execution section
- `SOURCES` array in `generate_test_data_js()`
- `ARTIFACTS` array (optional)

### 2. Create test files:

| Language | File | Template |
|----------|------|----------|
| Python | `session_py/src/session_py/bbox_test.py` | Use `@MINI_TEST` decorator |
| C++ | `session_cpp/src/bbox_test.cpp` | Use `MINI_TEST` macro |
| Rust | `session_rust/src/bbox_test.rs` | Use `MINI_TEST!` macro |

### 3. Wire up:
- **C++**: Add `bbox_test.cpp` to `MINITEST_SOURCES` in `CMakeLists.txt`
- **Rust**: Add `pub mod bbox_test;` to `lib.rs`, create `src/bin/bbox_minitest.rs`

### 4. MCP/Browser index - Add files to `SOURCE_FILES` in:
- `session_tests/mcp/session_api_server.py`
- `session_tests/mcp/generate_browser_index.py`

### 5. Run
```bash
./minitest.sh
cd session_tests/mcp && python3 generate_browser_index.py  # Regenerate search index
```

## MCP Server (Claude Desktop)

Config: `~/.config/Claude/claude_desktop_config.json`
```json
{
  "mcpServers": {
    "session-api": {
      "command": "python3",
      "args": ["/path/to/session/session_tests/mcp/session_api_server.py"]
    }
  }
}
```

Tools available to Claude:
- `search_api(query)` - Search methods across all languages
- `get_method(name, language)` - Get method implementation
- `list_classes()` - List all classes

## Git

```bash
git clone --recurse-submodules <url>            # Clone
git submodule update --init --recursive         # Update submodules
./git_push.sh "message"                         # Push all
```

## Notes

- Python venv: `uvsession/` at repo root
- Viewer port: 8769
- C++ requires C++23
- Protobuf bindings auto-generated during build

## Test Consistency Patterns

Tests across Python, C++, and Rust must be **EXACTLY identical** in structure, line count, and variable names.

### Include Comments
C++ tests must have **only one** `// uncomment` comment for the main header:
```cpp
// uncomment #include "vector.h"
```
**DO NOT** add `// uncomment #include "tolerance.h"` or `// uncomment #include <cmath>`.

### Variable Consistency
If C++ requires a variable (e.g., reference parameter), **all languages must have it**:
```cpp
// C++: get_leveled_vector takes double& reference
double vertical_height = 1.0;
Vector v_leveled = v.get_leveled_vector(vertical_height);
```
```python
# Python: same variable for consistency
vertical_height = 1.0
v_leveled = v.get_leveled_vector(vertical_height)
```
```rust
// Rust: same variable for consistency
let vertical_height = 1.0;
let v_leveled = v.get_leveled_vector(vertical_height);
```

### Comment Formatting
Use **ASCII characters** in comments (not Unicode):
- Use `.` not `•` for dot product
- Use `^2` not `²` for squared
- Use `deg` not `°` for degrees
- Use `alpha, beta, gamma` not `α, β, γ`

### Blank Line Consistency
Each test block should have the same number of blank lines in all languages.

### Serialization File Paths
- **Python**: Use `Path(__file__).resolve().parents[2] / "filename.json"` to save to `session_py/`
- **C++/Rust**: Use simple filename `"filename.json"` (runs from their directories)

### MINI_CHECK Assertions
Keep assertions on separate lines with same logic:
```python
MINI_CHECK(x == 1.0 and y == 2.0 and z == 3.0)
```
```cpp
MINI_CHECK(x == 1.0 && y == 2.0 && z == 3.0);
```
```rust
MINI_CHECK!(x == 1.0 && y == 1.0 && z == 1.0);
```

### Floating Point Comparisons
Use `TOLERANCE.is_close()` instead of rounding:
```python
# GOOD - use is_close
MINI_CHECK(TOLERANCE.is_close(d, 3.741657))

# BAD - don't use rounding
d = round(value, Tolerance.ROUNDING)
MINI_CHECK(d == 3.741657)
```
```cpp
// GOOD - use is_close
MINI_CHECK(TOLERANCE.is_close(d, 3.741657));

// BAD - don't use rounding
double d = Tolerance::round_to(value, Tolerance::ROUNDING);
MINI_CHECK(d == 3.741657);
```
```rust
// GOOD - use is_close
use crate::tolerance::TOLERANCE;
MINI_CHECK!(TOLERANCE.is_close(d, 3.741657));

// BAD - don't use rounding
let d = Tolerance::round_to(value, Tolerance::ROUNDING);
MINI_CHECK!(d == 3.741657);
```
