# session_viewer

WebAssembly 3D viewer for session geometry — wgpu + winit + Trunk.

## Run

```bash
cd session_viewer
trunk serve
```

Open http://localhost:8769

## Kill

```bash
# Git Bash (MINGW):
taskkill //F //IM trunk.exe

# PowerShell / cmd:
taskkill /F /IM trunk.exe
```

## Switch demo

Edit `src/demo.rs`, comment/uncomment one line in `active_scene()`:

```rust
pub fn active_scene() -> (Session, Vec<CylinderSegment>) {
    make_floor_scene()
    // make_cdt_scene()
}
```

## Install (once)

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```
