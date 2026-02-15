---
name: mini_test
description: Run and manage the custom MINI_TEST framework across C++, Python, and Rust
version: 1.0.0
author: session
type: skill
category: testing
tags:
  - test
  - mini_test
  - cpp
  - python
  - rust
  - geometry
---

# MINI_TEST Skill

> **Purpose**: Run and verify the custom MINI_TEST framework that tests geometry classes across C++, Python, and Rust implementations.

---

## What I Do

- Execute MINI_TEST framework in all 3 languages
- Verify test results against expected output
- Check JSON consistency across implementations
- Provide test status and failure diagnostics

---

## Command Reference

| Command | Description |
|---------|-------------|
| `run` | Run all tests in all languages |
| `run --py` | Run Python tests only |
| `run --rust` | Run Rust tests only |
| `run --cpp` | Run C++ tests only |
| `run <class>` | Run specific class tests |
| `fast` | Run in fast mode (skip deps) |
| `verify` | Verify JSON output consistency |

---

## Quick Start

```bash
# Run all tests + web viewer
./bash/minitest.sh

# All tests, no viewer
./bash/minitest.sh --no-web

# Single language (fast)
./bash/minitest.sh --py
./bash/minitest.sh --rust
./bash/minitest.sh --cpp

# Fast mode
./bash/minitest.sh --fast

# Single class
./bash/test_py.sh point
./bash/test_rust.sh
```

---

## Test Output

Tests write JSON to:

| Language | Path |
|----------|------|
| C++ | `session_tests/session_cpp/<class>_test.json` |
| Python | `session_tests/session_py/<class>_test.json` |
| Rust | `session_tests/session_rust/<class>_test.json` |

### JSON Format

```json
{
  "test_name": "Constructor",
  "passed": true,
  "time_ms": 0.123,
  "line": 10,
  "code": "let p = Point::new(1.0, 2.0, 3.0);\n",
  "checks": [
    { "line": 12, "code_line": "p[0] == 1.0", "passed": true }
  ],
  "failures": []
}
```

---

## Verification Checklist

When verifying tests:

- [ ] All `"passed": true` in JSON
- [ ] All `"failures": []` empty
- [ ] Same test names in all 3 languages
- [ ] Same test logic across languages
- [ ] JSON fields in alphabetical order
- [ ] Web viewer shows results at localhost:8769

---

## Troubleshooting

### Tests Fail

Check JSON `failures` array for error details:

```bash
# View failure details
cat session_tests/session_py/point_test.json | jq '.[] | select(.passed == false)'
```

### Missing Tests

Ensure test files are registered:
- C++: `CMakeLists.txt` → `MINITEST_SOURCES`
- Rust: `lib.rs` → `pub mod <name>_test`
- Python: Auto-discovered from `*_test.py`

### Build Errors

Check bash scripts in `bash/`:
- `test_py.sh` - Python test runner
- `test_cpp.sh` - C++ test runner
- `test_rust.sh` - Rust test runner

---

## Examples

### Check Point Tests

```bash
# Run Point tests in all languages
./bash/test_py.sh point
# Check results
cat session_tests/session_py/point_test.json
```

### Verify All Tests Pass

```bash
./bash/minitest.sh --no-web
# Check each language
jq '.[] | select(.passed == false)' session_tests/session_py/*_test.json
jq '.[] | select(.passed == false)' session_tests/session_rust/*_test.json
jq '.[] | select(.passed == false)' session_tests/session_cpp/*_test.json
```

### Fast Iteration

```bash
# Python only (instant)
./bash/test_py.sh point

# Rust only (~10-30 sec)
cd session_rust && cargo run --release --bin minitest
```

---

## Architecture

```
session_py/src/session_py/mini_test.py   # Python test framework
session_cpp/src/mini_test.h               # C++ test framework
session_rust/src/mini_test.rs             # Rust test framework

session_tests/session_{cpp,py,rust}/     # JSON output
bash/test_{py,cpp,rust}.sh               # Test runners
bash/minitest.sh                         # Orchestrator
```

---

## Key Concepts

### 1. MINI_TEST Macros

| Language | Test Definition | Assertion |
|----------|----------------|-----------|
| C++ | `MINI_TEST("Group", "Name") { }` | `MINI_CHECK(expr)` |
| Python | `@MINI_TEST("Group", "Name")` | `MINI_CHECK(expr)` |
| Rust | `MINI_TEST!("Name", { })` | `MINI_CHECK!(expr)` |

### 2. Test Registration

- C++: Auto-registers via static initialization
- Python: Decorator registers globally
- Rust: `REGISTER_MINI_TEST!` macro + inventory crate

### 3. JSON Output

- Alphabetically sorted fields
- Nested objects also sorted
- Used by Vue test viewer

---

## Integration with Agents

The main agent delegates to me for:
- Running full test suite
- Verifying test results
- Debugging failures
- Checking consistency

---

## Tips

1. **Use Python for fast iteration** - instant feedback
2. **Use Rust for mid-speed** - 10-30 sec rebuilds
3. **Use C++ only when needed** - slowest to build
4. **Check JSON output** - shows exact failure details
5. **Use --fast mode** after first build

---

## File Locations

- **Skill**: `.opencode/skills/mini_test/`
- **Test Runners**: `bash/test_{py,cpp,rust}.sh`
- **Orchestrator**: `bash/minitest.sh`
- **Output**: `session_tests/session_{cpp,py,rust}/`
- **Viewer**: localhost:8769

---

**MINI_TEST Skill** - Test geometry implementations across C++, Python, and Rust
