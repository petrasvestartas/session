# Minitest Scripts

Test runner system for Python, C++, and Rust geometry implementations with Vue test viewer.

## Quick Reference

| Command | Description | Time |
|---------|-------------|------|
| `./test_py.sh --fast` | All Python tests | ~13s |
| `./test_py.sh --fast point` | Single class test | instant |
| `./test_cpp.sh` | All C++ tests | 1-5 min |
| `./test_rust.sh` | All Rust tests | 10-30s |
| `./minitest.sh --fast` | All languages | 2-6 min |
| `./minitest.sh --kill` | Stop dev server | instant |

## Single Language Scripts

### Python (Fastest)

```bash
./test_py.sh                    # Run all Python tests
./test_py.sh point              # Run only Point tests
./test_py.sh --fast             # Skip pip install (use after first run)
./test_py.sh --fast point       # Fast single class
./test_py.sh --no-viewer        # Don't update testData.js
```

### C++

```bash
./test_cpp.sh                   # Build and run all C++ tests
./test_cpp.sh --no-viewer       # Don't update testData.js
```

### Rust

```bash
./test_rust.sh                  # Build and run all Rust tests
./test_rust.sh --no-viewer      # Don't update testData.js
```

## Main Orchestrator

```bash
./minitest.sh                   # Run all languages + start server
./minitest.sh --py              # Python only
./minitest.sh --cpp             # C++ only
./minitest.sh --rust            # Rust only
./minitest.sh --fast            # Skip dependency installs
./minitest.sh --no-web          # Skip Vue server
./minitest.sh --kill            # Stop dev server only (safe, no cleanup)
```

Combine flags:
```bash
./minitest.sh --fast --py       # Fast Python only
./minitest.sh --fast --no-web   # Fast, no server
```

## Server Management

```bash
./lib/server.sh start           # Start Vue dev server
./lib/server.sh start --fast    # Start, skip npm install
./lib/server.sh stop            # Stop server
./lib/server.sh restart         # Restart server
```

Server runs on http://localhost:8769/session/tests

## Utility Scripts

```bash
./lib/consolidate.sh            # Regenerate testData.js from existing JSON
```

## Windows Batch Equivalents

All `.sh` scripts have `.bat` equivalents:

```cmd
.\test_py.bat --fast point
.\test_cpp.bat
.\test_rust.bat
.\minitest.bat --fast --py
.\minitest.bat --kill
.\lib\server.bat start
.\lib\consolidate.bat
```

## Development Workflow

### Recommended: Single Language Iteration

1. Start server once:
   ```bash
   ./lib/server.sh start
   ```

2. Edit code, then run single language test:
   ```bash
   ./test_py.sh --fast point    # Instant feedback
   ```

3. Browser auto-refreshes via Vite HMR

4. When done:
   ```bash
   ./minitest.sh --kill
   ```

### Full Validation Before Commit

```bash
./minitest.sh --fast            # All languages, starts server
```

## Architecture

```
bash/
├── minitest.sh/.bat            # Main orchestrator
├── test_py.sh/.bat             # Python-only runner
├── test_cpp.sh/.bat            # C++-only runner
├── test_rust.sh/.bat           # Rust-only runner
├── lib/
│   ├── common.sh/.bat          # Shared functions
│   ├── consolidate.sh/.bat     # JSON -> testData.js
│   └── server.sh/.bat          # Dev server management
└── README.md                   # This file
```

## Key Design Principles

1. **No destructive cleanup** - Each language only writes its own JSON
2. **Incremental updates** - Run Python, C++/Rust data preserved
3. **Persistent server** - Start once, Vite hot-reloads changes
4. **Safe --kill** - Only stops server, never deletes data

## Output Locations

| Language | JSON Output |
|----------|-------------|
| Python | `session_tests/session_py/*.json` |
| C++ | `session_tests/session_cpp/*.json` |
| Rust | `session_tests/session_rust/*.json` |
| Consolidated | `session_tests/public/testData.js` |

## Troubleshooting

### Tests hang
```bash
# Run directly to see errors
./uvsession/Scripts/python.exe -m session_py.tolerance_test
```

### Server won't start
```bash
./minitest.sh --kill            # Kill orphaned process
./lib/server.sh start           # Try again
```

### Missing dependencies
```bash
./test_py.sh                    # Without --fast, reinstalls deps
./minitest.sh                   # Full install
```

### JSON not updating in viewer
```bash
./lib/consolidate.sh            # Force regenerate testData.js
```
