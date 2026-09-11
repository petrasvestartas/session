Build and test commands reference.

## Quick test (single language)
```bash
./bash/minitest.sh --py --no-web      # Python only (instant)
./bash/minitest.sh --rust --no-web    # Rust only
./bash/minitest.sh --cpp --no-web     # C++ only
./bash/minitest.sh --fast --py        # Skip dependency installs
```

## Quick single-class test
```bash
./bash/quicktest.sh $ARGUMENTS             # All languages
./bash/quicktest.sh $ARGUMENTS --py        # Python only
./bash/quicktest.sh $ARGUMENTS --rust      # Rust only
```

## Full build
```bash
./bash/minitest.sh                    # All tests + viewer at localhost:8769
./bash/minitest.sh --no-web           # Skip Vue viewer
./bash/minitest.sh --kill             # Stop dev server
```

## Manual builds
- C++: `cd session_cpp && cmake -B build && cmake --build build --config Release`
- Rust: `cd session_rust && cargo build --release && cargo test`
- Python: `source uvsession/bin/activate && cd session_py && uv pip install -e . && pytest -v`


