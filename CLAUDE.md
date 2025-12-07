# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Session is a multi-language geometry kernel (Python, C++, Rust) implementing 25+ geometric data structures including Point, Color, Vector, BoundingBox, Mesh, NURBS curves/surfaces, BVH, and more. The goal is to serialize geometry for web-based visualization via a Rust wgpu viewer, focused on code development sessions and learning engineering/math problems.

## Repository Structure

This is a **Git submodule-based monorepo**:
- `session_cpp/` - C++ kernel implementation (submodule)
- `session_py/` - Python kernel implementation (submodule)
- `session_rust/` - Rust kernel implementation (submodule)
- `session_proto/` - Shared Protocol Buffers schemas (submodule)
- `session_data/` - Shared geometry dataset (submodule)
- `session_tests/` - Vue 3 test viewer and test framework
- Root scripts for building and testing across all languages

## Build Commands

### Build All Languages
```bash
# From repository root
./build.sh
```
Builds all three implementations sequentially and reports success/failure for each.

### C++ Build
```bash
cd session_cpp
mkdir -p build && cd build
cmake ..
make tests -j$(nproc)
# Binary: build/tests
```
- Uses CMake with C++23 standard
- Protobuf integration enabled by default (disable with `-DENABLE_PROTOBUF=OFF`)
- External dependencies: Abseil, Protocol Buffers (auto-fetched via ExternalProject)
- Main executable target: `tests`
- Generates protobuf C++ bindings from `session_proto/` during build

### Python Setup & Test
```bash
# Create environment (first time only)
uv venv uvsession --python 3.11
source uvsession/bin/activate

# Install (from session_py/)
cd session_py
uv pip install -e .

# Run tests
pytest -v
```
- Uses `uv` for fast Python environment management
- Virtual environment: `uvsession/` in repository root
- Dependencies defined in `session_py/pyproject.toml`
- Test runner: pytest
- Protobuf Python bindings generated from `session_proto/` by `minitest.sh`

### Rust Build & Test
```bash
cd session_rust
cargo build --release
cargo test
cargo fmt                              # Format code
cargo clippy --fix --allow-dirty       # Lint and auto-fix
```
- Standard Cargo workflow
- Release binary: `target/release/session_rust`
- Protobuf feature enabled by default (uses `prost` and `prost-build`)
- Build script (`build.rs`) generates Rust protobuf bindings from `session_proto/`

## Test Framework & Viewer

The `session_tests/` directory contains a Vue 3 + Vite application that displays test results across all three language implementations.

### Run Mini Tests & Launch Viewer
```bash
# From repository root
./minitest.sh
```
This script:
1. Regenerates Python protobuf bindings from `session_proto/`
2. Runs Python mini tests (outputs JSON to `session_tests/session_py/`)
3. Builds and runs C++ mini tests (outputs JSON to `session_tests/session_cpp/`)
4. Builds and runs Rust mini tests (outputs JSON to `session_tests/session_rust/`)
5. Consolidates all JSON results into `session_tests/public/testData.js`
6. Runs `npm install` and `npm run build` in `session_tests/`
7. Starts Vite dev server on `http://localhost:8769/`
8. Opens browser to test viewer

### Viewer Architecture
- **Vue 3** with Composition API + **Vite** build tooling
- **Tabs**:
  - `/viewer` - 3D viewer placeholder (future wgpu viewer)
  - `/tests` - Test results table with Python/C++/Rust columns
- **CLI Interface**: ChatGPT-like command interface at bottom panel
  - Commands: `help`, `clear`, `info`, `search`, `stats`, `viewer`, `rag`
  - **RAG Command**: Semantic search over Point and Color source code across all languages (e.g., `rag how to create a Point in Python`)
- **Test Suite Selection**: Dropdown in Tests tab (e.g., `point_test`, `color_test`)
- **Data Loading**: Pre-processed `testData.js` loaded via global `window.TEST_DATA` (instant, no async fetch)
- **Syntax Highlighting**: highlight.js for code snippets

### Frontend-Only Development
```bash
cd session_tests
npm install          # First time only
npm run dev          # Starts dev server on port 8769
```

### Adding New Test Suites
1. Generate JSON output for new suite in each language:
   - Python: `session_tests/session_py/<suite_name>.json`
   - C++: `session_tests/session_cpp/<suite_name>.json`
   - Rust: `session_tests/session_rust/<suite_name>.json`
2. Edit `minitest.sh` to run the new tests
3. Register JSON files in `generate_test_data_js()` function's `SOURCES` array
4. Run `./minitest.sh` to regenerate `testData.js`
5. New suite appears in Tests tab dropdown automatically

## RAG Pipeline (Code Documentation Search)

The repository includes a **RAG (Retrieval Augmented Generation) pipeline** for semantic search over Point and Color source code across all three language implementations.

### Architecture
- **Data Ingestion**: `rag_pipeline.py` reads source files and chunks them by semantic units (classes, functions, methods)
- **Embedding Model**: HuggingFace sentence-transformers (`all-MiniLM-L6-v2`)
- **Vector Store**: ChromaDB with persistent storage at `./rag_db/`
- **API Server**: Flask REST API (`rag_api.py`) on port 8770
- **Frontend Integration**: Vue CLI `rag` command queries the API

### Setup RAG Environment
```bash
# Install RAG dependencies (large packages: ~900MB including PyTorch)
pip install -r rag_requirements.txt

# Ingest source code into vector database (creates ./rag_db/)
python3 rag_pipeline.py ingest
```

### RAG CLI Commands
```bash
# Query the RAG system (command line)
python3 rag_pipeline.py query --query "how to create a Point in Python" --results 5

# View database info
python3 rag_pipeline.py info

# Clear and reingest all files
python3 rag_pipeline.py clear
python3 rag_pipeline.py ingest
```

### Start RAG API Server
```bash
# Start Flask API server on port 8770
python3 rag_api.py

# The minitest.sh script automatically starts this server
```

### Using RAG in Vue Application
The Vue test viewer includes a `rag` command in the CLI interface:
```
rag how to create a Point in Python
rag Color methods in Rust
rag distance calculation between points
```

Results show:
- **Type**: class, function, or method
- **Name**: Code element name (e.g., `Point`, `Color::red`, `test_mid_point`)
- **Language**: python, cpp, or rust
- **File**: Source file name and line number
- **Relevance**: Similarity score (higher = more relevant)

### Adding Files to RAG
Edit `rag_pipeline.py` line 306-326 to add new files to the ingestion list:
```python
files_to_ingest = [
    # Python
    (self.repo_root / "session_py/src/session_py/point.py", "python"),
    (self.repo_root / "session_py/src/session_py/color.py", "python"),
    # Add your new files here
]
```

## MCP Server (Claude AI Integration)

The repository includes an **MCP (Model Context Protocol) server** that allows Claude AI to directly query the codebase during conversations.

### What is MCP?

MCP enables Claude to:
- Search your Session codebase semantically while helping you code
- Get detailed class information from any language
- Compare implementations across Python, C++, and Rust
- Provide context-aware coding assistance based on your actual code

### Setup

**See [MCP_SETUP.md](MCP_SETUP.md) for complete setup instructions.**

Quick setup:
1. Add to Claude Desktop config (`~/.config/Claude/claude_desktop_config.json`):
```json
{
  "mcpServers": {
    "session-codebase": {
      "command": "/home/pv/anaconda3/bin/python3",
      "args": ["/home/pv/brg/code_rust/code/code_session/session/mcp_server.py"]
    }
  }
}
```

2. Restart Claude Desktop

### Available MCP Tools

Once configured, Claude can use these tools automatically:

- **`search_code(query, n_results=5)`**: Semantic code search
  - Example: "Search for Point constructor in Python"

- **`get_class_details(class_name, language)`**: Get full class implementation
  - Example: "Get details about Color class in Rust"

- **`list_implementations()`**: List all Point and Color implementations
  - Shows which languages have implementations

- **`compare_implementations(feature)`**: Compare across languages
  - Example: "Compare distance calculation across Python, C++, and Rust"

### Example Usage with Claude

```
You: "How do I create a Point in Python?"
Claude: [Uses search_code tool internally]
        "To create a Point in Python:

        ```python
        from session_py import Point
        p = Point(x=1.0, y=2.0, z=3.0, name='my_point')
        ```

        The Point class is defined in point.py:10..."

You: "Compare Point constructors across all languages"
Claude: [Uses compare_implementations tool]
        "Here's how Point construction differs:

        Python: Point(x, y, z, name='my_point')
        C++: Point(double x, double y, double z, std::string name = 'my_point')
        Rust: Point::new(x: f64, y: f64, z: f64) -> Self

        [Shows actual code from each implementation]"
```

### Benefits

- **Context-aware help**: Claude knows your exact API when suggesting code
- **Cross-language insights**: Compare patterns across implementations
- **Accurate examples**: Get examples from your actual codebase
- **No copy-paste**: Claude queries the code directly

### Architecture

Both Flask API (Vue UI) and MCP Server (Claude AI) share the same RAG pipeline:

```
RAG Pipeline (ChromaDB + Embeddings)
    ├── Flask API :8770 → Vue Web UI
    ├── Flask API :8770 → ask_session.py CLI
    └── MCP Server       → Claude Desktop/API
```

### Command-Line Ask Tool

For quick terminal-based questions about the codebase, use `ask_session.py`:

```bash
# Start the RAG API server (in one terminal)
./start_ask_session.sh

# Or manually:
/home/pv/anaconda3/bin/python3 rag_api.py

# Then ask questions (in another terminal)
python3 ask_session.py "how to create a Point in Python"
python3 ask_session.py "what are the Color methods in Rust"
python3 ask_session.py "distance calculation between points"
```

The tool provides conversational LLM-like responses with:
- Natural language explanations
- Code examples
- Cross-language references
- Source file locations
- Relevance scores

## Protobuf Integration

All three implementations share common Protocol Buffers schemas in `session_proto/`:
- `point.proto`, `color.proto`, `xform.proto`, `mesh.proto`, etc.
- Each language generates bindings during build:
  - **C++**: CMake ExternalProject generates at build time → `build/generated/`
  - **Python**: `minitest.sh` uses `protoc --python_out` → `session_py/src/session_py/proto/`
  - **Rust**: `build.rs` uses `prost-build` → generated in `target/`

## Git Submodules

### Clone with Submodules
```bash
git clone --recurse-submodules https://github.com/petrasvestartas/session.git
```

### Update Submodules
```bash
git pull
git submodule update --init --recursive
```

### Commit & Push (All Submodules + Main Repo)
```bash
./git_push.sh "commit message"
```
This script commits and pushes all modified submodules, then updates and pushes the main repository.

## Key Architectural Patterns

### Mini Test Framework
Each language has a custom mini test framework (not pytest/Catch2/cargo test) that:
- Profiles execution time for performance comparison
- Serializes test results to JSON with structure: test name, status (pass/fail), timing, failing checks, code snippets
- Outputs to `session_tests/session_<lang>/<suite>.json`
- Located in:
  - C++: `src/mini_test.h`, `src/mini_test.cpp`
  - Python: `src/session_py/mini_test.py`
  - Rust: `src/mini_test.rs`

### Test File Naming Convention
- Implementation: `<geometry>.cpp`, `<geometry>.py`, `<geometry>.rs`
- Tests: `<geometry>_test.cpp`, `test_<geometry>.py`, `<geometry>_test.rs`
- Mini tests output: `test_<geometry>.json` (in language root during execution)

### Serialization Outputs
Tests generate both JSON (for viewer) and binary `.bin` files (protobuf serialization):
- JSON: Human-readable test results
- BIN: Protobuf binary format for cross-language geometry exchange

## Common Development Workflows

### Single Language Development
When working on one implementation (e.g., adding a Vector method):
1. Modify source in `session_<lang>/src/`
2. Build that language only (see Build Commands above)
3. Run language-specific tests
4. Optional: Run `./minitest.sh` to see cross-language comparison in viewer

### Cross-Language Feature Addition
When adding a new geometry type across all implementations:
1. Define `.proto` schema in `session_proto/`
2. Implement in each language: `session_<lang>/src/<geometry>.<ext>`
3. Add tests: `session_<lang>/src/<geometry>_test.<ext>`
4. Update `minitest.sh` to run new tests and register JSON outputs
5. Run `./minitest.sh` to verify consistency across languages

### Protobuf Schema Changes
After modifying `.proto` files in `session_proto/`:
1. **C++**: Re-run `cmake ..` and `make` (regenerates automatically)
2. **Python**: Run `minitest.sh` (includes proto regeneration step)
3. **Rust**: Run `cargo build` (build.rs regenerates automatically)

## Documentation

Documentation is built via `session_cpp/session_docs/`:
```bash
# Local documentation build
./session_cpp/session_docs/build_docs.sh           # Unix/macOS
./session_cpp/session_docs/build_docs.bat          # Windows

# Build and open in browser
./session_cpp/session_docs/build_docs.sh --open
```

Creates unified documentation portal in `session_cpp/session_docs/combined_docs/`:
- C++ docs (Doxygen with doxygen-awesome theme)
- Python docs (Sphinx)
- Rust docs (cargo doc)
- Unified landing page at `combined_docs/index.html`

GitHub Actions automatically builds and deploys documentation to GitHub Pages.

## Important Notes

### Python Protobuf Import Fixes
`minitest.sh` automatically fixes protobuf imports after generation:
- Generated: `import point_pb2 as`
- Fixed: `from . import point_pb2 as`
This makes imports work as relative imports within the `session_py.proto` package.

### C++ Compiler Requirements
- Requires C++23 support (uses `std::execution::par` for parallel algorithms)
- GCC/Clang: Automatically enables `-O3 -march=native` in Release mode
- MSVC: Uses `/utf-8` for fmt library compatibility

### Rust Feature Flags
- Protobuf support is optional: `cargo build --no-default-features` to disable
- Default features include `protobuf` (prost/prost-build)

### Test Viewer Port
The Vite dev server runs on **port 8769** (not the default 5173) - configured in `session_tests/vite.config.js`.

### Virtual Environment Location
Python virtual environment `uvsession/` is created at **repository root** (not in `session_py/`), so it can be shared across scripts like `minitest.sh`.
