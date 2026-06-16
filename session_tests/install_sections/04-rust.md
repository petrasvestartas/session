# Rust

## Install & download

**Windows:**

```bash
winget install Rustlang.Rustup
```

**macOS / Linux:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Build & run tests — same on all OS

```bash
cd session_rust
cargo run --release --bin minitest
```
