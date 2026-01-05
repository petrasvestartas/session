# Plan: Optimize Rust Protoc Download

## Problem
Rust builds are slow because `protobuf-src` compiles protoc from C++ source (~5+ min on Windows).

## Solution
Download pre-built protoc binaries from GitHub releases (same approach as session_proto).

## Steps

### A) Update Rust build.rs to download pre-built protoc
- Download protoc v29.0 from protocolbuffers/protobuf releases
- Platform detection: Windows, Linux x64, macOS x64, macOS ARM64
- Extract and use for prost-build
- Remove protobuf-src dependency from Cargo.toml

### B) Test locally
- Run `cargo build --release --features protobuf` in session_rust
- Verify protoc downloads and proto compilation works
- Run minitest.sh to verify full integration

### C) Test on GitHub Actions
- Push changes
- Verify all 4 platforms pass: ubuntu-22.04, macos-15, macos-15-intel, windows-latest

### D) Review build-cpp.yml
- Check if `/Users/petras/brg/code_rust/session/.github/workflows/build-cpp.yml` is still needed
- It may be redundant if session_cpp is tested via minitest

### E) Check newer protobuf versions
- Test protobuf v30, v31, v32, v33
- Verify compatibility with session_proto (currently v29.0)
- Update if newer version works

## Files to modify
- `session_rust/Cargo.toml` - remove protobuf-src
- `session_rust/build.rs` - add protoc download logic

## Expected result
- Windows build time: ~5 min → ~30 sec
- All platforms use same protoc version (v29.0)
- Consistent with session_proto approach
