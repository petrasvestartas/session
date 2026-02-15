---
name: build
description: Build C++, Python, and Rust components
version: 1.0.0
author: session
type: skill
category: development
tags:
  - build
  - cpp
  - python
  - rust
  - cmake
  - cargo
---

# Build Skill

> **Purpose**: Build and compile the session geometry kernel components in C++, Python, and Rust.

---

## What I Do

- Build C++ with CMake
- Set up Python environment
- Compile Rust with Cargo
- Manage incremental builds

---

## Build Times

| Component | First Build | Incremental |
|-----------|-------------|-------------|
| C++ | 15-25 min | 1-5 min |
| Rust | 5-10 min | 10-30 sec |
| Python | instant | instant |

---

## C++ Build

### Full Build
```bash
cd session_cpp
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release --parallel $(nproc)
```

### Incremental
```bash
cd session_cpp/build
cmake --build . --config Release
```

### Windows
```bash
cd session_cpp
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release --parallel %NUMBER_OF_PROCESSORS%
```

---

## Python Setup

### One-Time Setup
```bash
# Create virtual environment
uv venv uvsession --python 3.11

# Activate (Linux/Mac)
source uvsession/bin/activate

# Activate (Windows)
uvsession\Scripts\activate.bat
```

### Install Package
```bash
cd session_py
uv pip install -e .
```

---

## Rust Build

```bash
cd session_rust
cargo build --release

# Run tests
cargo run --release --bin minitest
```

---

## Quick Scripts

### All Languages
```bash
./bash/minitest.sh
```

### Python Only (Fast)
```bash
./bash/test_py.sh              # All
./bash/test_py.sh point        # Single class
./bash/test_py.sh --fast       # Skip install
```

### Rust Only
```bash
./bash/test_rust.sh
```

### C++ Only
```bash
./bash/test_cpp.sh
./bash/test_cpp.sh --clean    # Force rebuild
```

---

## Fast Development

### Python (Instant)
```bash
python -m session_py.point_test
```

### Rust (10-30 sec)
```bash
cd session_rust
cargo run --release --bin minitest
```

### C++ (Slowest)
```bash
cd session_cpp/build
cmake --build . --config Release
```

---

## Troubleshooting

### C++ Build Fails
- Check CMake configuration
- Ensure protobuf is available
- Try: `./bash/test_cpp.sh --clean`

### Python Import Fails
- Ensure venv is activated
- Try: `uv pip install -e session_py`

### Rust Build Fails
- Check Cargo.toml dependencies
- Try: `cargo clean && cargo build --release`

---

## Pre-Warm (First Time)

Run once to cache:

```bash
# C++ with protobuf
cd session_cpp
cmake -B build -DENABLE_PROTOBUF=ON
cmake --build build --config Release

# Rust with protobuf
cd session_rust
cargo build --release --features protobuf
```

---

## File Locations

| Component | Build Path |
|-----------|------------|
| C++ | `session_cpp/build/` |
| Python | `uvsession/` |
| Rust | `session_rust/target/` |

---

## Tips

1. **Use Python** for fastest iteration
2. **Use Rust** for mid-speed development
3. **Use C++** only when necessary
4. **Use --fast** mode after first build
5. **Use ccache** for C++ (auto-detected)

---

**Build Skill** - Compile session geometry kernel
