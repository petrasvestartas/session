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
