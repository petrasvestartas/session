# Tutorial 14 · Shared `Op` API, browser `execute_op` and FastMCP server

## Goal

Unify typed JSON operations (`49` `Op` variants addressed by stable `(doc, guid)` `ObjectRef` and `(doc, name)` `LayerRef`) across `CommandHost`, browser `#[wasm_bindgen] execute_op` + `postMessage`, `session-viewer-cli serve --stdio`, and the Python `session_mcp` FastMCP server.

![Double-layer parabolic 3D space-truss with 35 joint blocks generated 100% through viewer_core::op::execute](../screenshots/tutorials/t14-mcp-truss.png)

**What we achieve:** Typed `49`-variant JSON `Op` automation (`viewer-core/src/op.rs`) shared across browser `execute_op` (`postMessage`), native CLI, and the Python `session_mcp` FastMCP server (`test_mcp_parity.py`).

## How it works

`viewer_core::op::execute` runs against `&mut dyn CommandHost` (`src/state/host.rs` in WASM vs `HeadlessHost` in CLI/stdio), addressing objects by pointer-free `ObjectRef { doc, guid }`, classifying failures into 8 machine `ErrorCode`s, and producing SHA-256-identical `.session` state across CLI, JSON Ops, and Python FastMCP tools.

![Three-transport Op::execute architecture (postMessage, CLI, Python FastMCP), Request/Response JSONL wire envelope, 8 ErrorCodes, and ObjectRef](../screenshots/tutorials/explain-14-fastmcp.png)

## Prerequisites and files

Requires [Tutorial 13](tutorial-thirteen-commands.md). Owns `viewer-core/src/command/{host,headless_tests}.rs`, `src/state/host.rs`, `viewer-core/src/op.rs`, and the complete `session_mcp/` Python package.

Owned files in build order (`10` files):

1. `viewer-core/src/command/host.rs`
2. `src/state/host.rs`
3. `viewer-core/src/command/headless_tests.rs`
4. `viewer-core/src/op.rs`
5. `../session_mcp/pyproject.toml`
6. `../session_mcp/README.md`
7. `../session_mcp/src/session_mcp/__init__.py`
8. `../session_mcp/src/session_mcp/client.py`
9. `../session_mcp/src/session_mcp/server.py`
10. `../session_mcp/tests/test_mcp_parity.py`

## Build

### Step 1 · `viewer-core/src/command/host.rs`

`viewer-core/src/command/host.rs` (1830 lines) defines `enum ViewFlag`, `struct DisplaySettings`, `struct SelectionState`, `selected_rows()`, `ordered_rows()`. It provides `scene()`, `scene_mut()`, `camera()`.

```rust
--8<-- "session_viewer/viewer-core/src/command/host.rs"
```

### Step 2 · `src/state/host.rs`

`src/state/host.rs` (566 lines) implements `host` (`scene()`, `scene_mut()`, `camera()`) inside `state`. It connects `State`, `Mode`, `host`, `Tool` within `state` across native and `wasm32` targets.

```rust
--8<-- "session_viewer/src/state/host.rs"
```

### Step 3 · `viewer-core/src/command/headless_tests.rs`

`viewer-core/src/command/headless_tests.rs` (321 lines) verifies `headless_host_executes_every_registered_verb_without_gpu_or_window`. It provides `run()`, `run_follow()`; and locked by `cargo xtest` (`headless_host_executes_every_registered_verb_without_gpu_or_window`).

```rust
--8<-- "session_viewer/viewer-core/src/command/headless_tests.rs"
```

### Step 4 · `viewer-core/src/op.rs`

`viewer-core/src/op.rs` defines the serde-serializable `Op` API (`Request`, `Response`, `Reply`, `OpError`, `ObjectRef`, `LayerRef` and `execute`) shared by typed command verbs, browser `execute_op`, the native CLI and the Python FastMCP server.

~~~~rust
--8<-- "session_viewer/viewer-core/src/op.rs"
~~~~

### Step 5 · `../session_mcp/pyproject.toml`

`session_mcp/pyproject.toml` defines the Python >= 3.11 package metadata, the `session-mcp` console entry point (`session_mcp.server:main`), and optional `fastmcp` + `pytest` dependencies. Trap: keep core runtime imports stdlib-compatible so `session_mcp` runs offline inside `uvsession` even when external wheels are unavailable.

```toml
--8<-- "session_mcp/pyproject.toml"
```

### Step 6 · `../session_mcp/README.md`

`session_mcp/README.md` documents launching `session-mcp` over stdio, overriding `SESSION_VIEWER_CLI`, and verifying 3-way `.session` parity with `pytest`. Trap: MCP stdio servers must reserve `stdout` strictly for newline-delimited JSON-RPC 2.0 frames and send any diagnostic logs to `stderr`.

~~~~markdown
--8<-- "session_mcp/README.md"
~~~~

### Step 7 · `../session_mcp/src/session_mcp/__init__.py`

`session_mcp/src/session_mcp/__init__.py` exports `ViewerCliClient`, `ViewerOpError`, `find_cli_binary`, `create_server`, and `main` as the public Python package surface. Trap: keep package import side-effect-free so importing `session_mcp` in unit tests never spawns `session-viewer-cli` until a `ViewerCliClient` context opens.

```python
--8<-- "session_mcp/src/session_mcp/__init__.py"
```

### Step 8 · `../session_mcp/src/session_mcp/client.py`

`session_mcp/src/session_mcp/client.py` manages a persistent `session-viewer-cli serve --stdio` child process, framing `Request` dicts as single-line JSON and raising `ViewerOpError` with structured `ErrorCode`s on failure. Trap: flush `stdin` after every JSONL line and close `stdin` cleanly in `close()` so the Rust headless host exits without zombie subprocesses.

```python
--8<-- "session_mcp/src/session_mcp/client.py"
```

### Step 9 · `../session_mcp/src/session_mcp/server.py`

`session_mcp/src/session_mcp/server.py` exposes 31 typed `@mcp.tool()` geometry/layer/view/I/O operations and 3 `scene://` resources (`inspect`, `objects`, `perf`) over `ViewerCliClient`, falling back to `_BuiltinFastMCP` when `fastmcp` is absent. Trap: never duplicate geometry math in Python—every tool forwards a typed `Op` dictionary to `viewer_core::op::execute` so CLI, browser, and MCP stay bit-identical.

```python
--8<-- "session_mcp/src/session_mcp/server.py"
```

### Step 10 · `../session_mcp/tests/test_mcp_parity.py`

`session_mcp/tests/test_mcp_parity.py` verifies MCP JSON-RPC `initialize`/`tools/list`/`tools/call`/`resources/read`, `session_py` `.pb` round-tripping, and 3-way byte-identical `.session` saves across `--cmd`, `--op`, and FastMCP tools. Trap: compare raw `.session` byte slices by index rather than letting `pytest` diff multi-megabyte binary blobs on failure.

```python
--8<-- "session_mcp/tests/test_mcp_parity.py"
```

## Integration

Typed command-line actions (`Delete`, `Undo`, `Redo`, `Move`, `Copy`) delegate directly to `viewer_core::op::execute`, `src/app/runtime.rs` exports `#[wasm_bindgen] pub fn execute_op(request_json: &str) -> String`, `index.html` bridges `{type: "session-viewer:op", id, request}` -> `session-viewer:op-reply`, and `session-viewer-cli schema --markdown` generates `docs/operations.md` directly from `op_catalog()`.

## Run and observe

Run `PYTHONPATH=../session_mcp/src ../uvsession/bin/python -m pytest ../session_mcp/tests/test_mcp_parity.py -v` and `cargo xtest -p viewer-core op::tests -- --nocapture`: headless throughput reaches `4,076 ops/s` (`400` create+transform ops in `98.1 ms`), and `--cmd`, `--op`, and FastMCP tools produce byte-for-byte identical `.session` archives on `timber-floor.pb`.

## Predict and try

Predict what happens if an MCP client sends `{"id":"99","op":"object_delete","targets":"selection"}` when nothing is selected: `execute_json` never panics or mutates the scene—it returns `{"id":"99","ok":false,"error":{"code":"no_selection","message":"nothing is selected"}}`, which `ViewerCliClient` raises as a typed `ViewerOpError`.

## Question and answer

**Why address objects by `ObjectRef { doc: DocRef, guid: String }` in `Op` instead of passing `u32` GPU row indices across MCP and `postMessage`?**

??? tip "Think first — click to reveal how to work it out and the answer"
    **How to work it out:** Trace what happens to `u32` row indices after `LayerDelete`, `SceneImport`, `Explode`, or `Scene::rebuild` compacts tombstoned rows.

    **The answer:** `u32` row indices are transient internal GPU table slots that renumber on compaction or rebuild, and raw GUID strings alone can collide when the same `.pb` component is imported into two documents. Addressing every `Op` target by `(DocRef, guid)` resolves unambiguously through `Scene::row_of(doc, &guid)` across undo/redo, `.session` save/open, and multi-document assemblies.

## Validation

Verified via `cargo xtest -p viewer-core op::tests` (`4/4` catalog, CRUD/transform/measure/undo, 2-way `.session` byte parity, and `4,076 ops/s` throughput benchmark), `command::headless_tests` (`86/86` specs headless), and `pytest test_mcp_parity.py` (`3/3` passed in `6.25 s`).
