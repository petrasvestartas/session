# 3.7 · Restore exact sources and replay the captured edit

**Section 3: Scene Documents, Transactions, and Browser GPU Lifetime** · **Lesson 3.7** (Redraft `3.7`) · **26 checkpoints (`32g-keys` → `32gja-failures`)** · **Study time: 26–52 h**

> **What You Will Learn & Build:** Borrow the current imported release identity and epoch before asking for source data. By the final checkpoint (`32gja-failures`), you will verify automatic command failures preserve cold drawing, placement and history, including all-or-nothing multiple-source Save.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 3.7 Execution Flow: Restore exact sources and replay the captured edit</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (26 Checkpoints: 32g-keys → 32gja-failures)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="151" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">Active cold rows</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="92" x2="316" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">ReloadKey::of</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="92" x2="481" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">unique imported</text>
    <text x="419" y="124" text-anchor="middle" font-size="11" fill="#111111">Origin + release</text>
    <text x="419" y="140" text-anchor="middle" font-size="11" fill="#111111">epoch</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="92" x2="646" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">Captured operation</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">complete body set</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 3.7:</text>
    <text x="215" y="181" font-size="11" fill="#111111">src/lib.rs  ·  src/rehydrate.rs  ·  src/editor.rs  ·  src/document.rs  ·  src/scene.rs  ·  src/rehydrate_tests.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">26 Checkpoints: 32g-keys → 32gja-failures — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Chapter 3 Architecture: Transactional Protobuf (.pb) Decode, World Placements & Incremental GPU Sync</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Separating exact session_rust kernel geometry from GPU display buffers so view changes never re-upload unchanged meshes.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="185" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="110" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. Async .pb File / URL I/O</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• Ticket #N (cancels #N-1)</text>
    <text x="28" y="126" font-size="10.5" fill="#111111">• Bounded Protobuf decode</text>
    <text x="28" y="144" font-size="10.5" font-weight="600" fill="#111111">• Topological validation</text>
    <text x="28" y="162" font-size="10.5" fill="#111111">• Corrupt file? Reject with</text>
    <text x="28" y="177" font-size="10.5" fill="#111111">  ZERO partial scene mutation</text>

    <line x1="203" y1="138" x2="233" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="235" y="68" width="275" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="372" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. Editor Owner (Browser-Independent)</text>
    <text x="247" y="108" font-size="10.5" fill="#111111">• Kernel Mesh (local-space f64 coordinates)</text>
    <text x="247" y="126" font-size="10.5" fill="#111111">• Placement 4x4 Matrix (world transform)</text>
    <text x="247" y="144" font-size="10.5" font-weight="600" fill="#111111">• Releasable Source Ownership:</text>
    <text x="247" y="160" font-size="10" fill="#111111">  Drop cold CPU source after GPU upload;</text>
    <text x="247" y="175" font-size="10" fill="#111111">  re-fetch exact SHA/revision on edit!</text>
    <text x="247" y="195" font-size="10.5" fill="#111111">• Emits Change { sync_scene, redraw }</text>

    <line x1="510" y1="115" x2="542" y2="115" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <line x1="510" y1="175" x2="542" y2="175" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="544" y="68" width="276" height="68" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="556" y="86" font-size="11" font-weight="700" fill="#111111">3A. Geometry Changed (sync_scene = true)</text>
    <text x="556" y="104" font-size="10.5" fill="#111111">• Upload changed mesh vertex/index buffers</text>
    <text x="556" y="120" font-size="10.5" fill="#111111">• Retain unchanged object GPU allocations</text>

    <rect x="544" y="145" width="276" height="68" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="556" y="163" font-size="11" font-weight="700" fill="#111111">3B. View / Camera Only (sync_scene = false)</text>
    <text x="556" y="181" font-size="10.5" fill="#111111">• queue.write_buffer(camera_uniform) ONLY</text>
    <text x="556" y="197" font-size="10.5" fill="#111111">• Zero geometry re-uploads during Orbit/Fit!</text>
    
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **Protobuf (`.pb`)** | `session_proto` Binary Wire Format | Compact binary serialization for `Session` models; decoded with strict byte/count limits and topological validation before touching the live scene. |
| **Rust (`session_rust`)** | Kernel `Mesh` + World Placement `4x4` Matrix | Separates exact `f64` kernel geometry from placed world instances; cold CPU source bytes can be released after GPU upload and re-fetched on edit. |
| **Rust (`wgpu`)** | `Change { sync_scene, redraw }` | Incremental dirty-flag contract: camera orbit/fit sets `sync_scene = false` (updating only the 64-byte camera uniform), while geometry edits upload only changed buffers. |

**Authoritative production source (`session_viewer/src`):** `session_viewer/src`

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`32g-keys`](#checkpoint-32g-keys) | Borrow the current imported release identity and epoch before asking for source data. | `src/lib.rs`, `src/rehydrate.rs`, `src/editor.rs` | `Active cold rows → ReloadKey::of → unique imported Origin + release epoch` |
| **2** | [`32ga-prepare`](#checkpoint-32ga-prepare) | Validate an immutable source version and restore its Session into a private reload candidate. | `src/document.rs`, `src/rehydrate.rs` | `Original bytes → bounded decode/version check → candidate Session → original source-GUID lookup` |
| **3** | [`32gb-adopt`](#checkpoint-32gb-adopt) | Validate current releases and all matching history rows before adopting any kernel candidate. | `src/scene.rs`, `src/editor.rs` | `Current keys → prepare all candidates → validate all matching roots → adopt editable owners` |
| **4** | [`32gc-roundtrip`](#checkpoint-32gc-roundtrip) | Restore exact source coordinates and editing without reallocating retained display owners. | `src/lib.rs`, `src/rehydrate_tests.rs` | `Move → Unload Sources → checked hydrate → exact Save → retained Undo/Redo and GPU owners` |
| **5** | [`32gd-rejections`](#checkpoint-32gd-rejections) | Keep rows cold on changed versions, missing source identity and obsolete release keys. | `src/rehydrate_tests.rs` | `Prepare two origins → reject changed second source → no adoption → successful retry` |
| **6** | [`32ge-stale`](#checkpoint-32ge-stale) | Reject results for closed imports, loaded rows, duplicate keys and previous release epochs. | `src/rehydrate_tests.rs` | `Old import/release key → active-key check → false before payload decoding` |
| **7** | [`32gf-request`](#checkpoint-32gf-request) | Give each source request its own owner. | `src/lib.rs`, `src/reload_job.rs` | `Active release keys → ReloadJob → ticket and URL strings → one accepted completion` |
| **8** | [`32gfa-ownership`](#checkpoint-32gfa-ownership) | Prove cancelled work releases its source owners. | `src/reload_job.rs` | `Pending job holds keys → Close drops rows → cancel drops keys → abandoned request is inert` |
| **9** | [`32gg-fetch`](#checkpoint-32gg-fetch) | Read a source response within its byte limit. | `Cargo.toml`, `src/lib.rs`, `src/source_fetch.rs` | `Recorded URL → fetch with AbortSignal → HTTP/length checks → bounded stream chunks` |
| **10** | [`32gga-flight`](#checkpoint-32gga-flight) | Pair reload ownership with a browser abort controller. | `src/lib.rs`, `src/browser_reload.rs` | `ReloadJob + AbortController → captured signal → cancel or current finish` |
| **11** | [`32ggb-delivery`](#checkpoint-32ggb-delivery) | Deliver only the current completed reload batch. | `src/browser_reload.rs` | `Flight begin → await URL reads → finish ticket once → scoped Rust reply → viewer event` |
| **12** | [`32gh-command`](#checkpoint-32gh-command) | Restore editable sources through the command line. | `src/browser.rs` | `Reload Sources → active keys → owned fetch → atomic hydrate → same GPU display` |
| **13** | [`32gha-cancel`](#checkpoint-32gha-cancel) | Cancel reloads when their document context changes. | `src/browser.rs`, `examples/reload_check.rs` | `Close / replacement / Undo / release → cancel flight → late result is inert` |
| **14** | [`32gi-capture`](#checkpoint-32gi-capture) | Retain the original object and arguments before any source request starts. | `src/lib.rs`, `src/edit_intent.rs` | `Command + original selection → value-only Intent → unchanged target while waiting` |
| **15** | [`32gia-scope`](#checkpoint-32gia-scope) | Select source requests from the original edit target, with Save covering the active document. | `src/edit_intent.rs`, `src/lib.rs`, `src/edit_scope_tests.rs` | `Intent → original current row → cold origin + epoch; Save → unique active cold origins` |
| **16** | [`32gib-move`](#checkpoint-32gib-move) | Apply a captured Move to its original target, composing with the placement that exists at replay time. | `src/editor.rs`, `src/lib.rs`, `src/edit_move_tests.rs` | `Captured ObjectId + offset → current row.model → world shift × model → one history edit` |
| **17** | [`32giba-delete`](#checkpoint-32giba-delete) | Delete a captured target by identity without redirecting the command to a later selection. | `src/editor.rs`, `src/lib.rs`, `src/edit_delete_tests.rs` | `Captured ObjectId → current editable row → one history removal → preserve other selection` |
| **18** | [`32gibb-reply`](#checkpoint-32gibb-reply) | Return a scene change or original-precision save bytes from a captured intent. | `src/edit_replay.rs`, `src/lib.rs` | `Intent → explicit-target edit or original source snapshot → Reply::Changed or Reply::Saved` |
| **19** | [`32gic-owner`](#checkpoint-32gic-owner) | Store source keys and captured intent in one pending owner that current completion can consume once. | `src/reload_job.rs`, `src/lib.rs`, `src/edit_owner_tests.rs` | `ReloadJob<Intent> → Pending { ticket, keys, payload } → current finish_with takes both once` |
| **20** | [`32gica-reply`](#checkpoint-32gica-reply) | Build a completion value that retains captured intent and rejects incomplete source-body pairings. | `src/reload_reply.rs`, `src/lib.rs`, `src/reload_reply_tests.rs` | `current ticket → keys + Intent → equal body count → complete pairs or error` |
| **21** | [`32gicb-bridge`](#checkpoint-32gicb-bridge) | Carry optional captured intent through the abortable flight and deliver it beside complete source bodies or a current failure. | `src/browser_reload.rs`, `src/browser.rs` | `Flight → ReloadJob<Option<Intent>> → current finish_with → Reply → synchronous browser mailbox` |
| **22** | [`32gid-complete`](#checkpoint-32gid-complete) | Complete source restoration and replay the captured operation only after current document validation. | `src/reload_reply.rs`, `src/lib.rs`, `src/reload_complete_tests.rs` | `Reply → complete bodies → current keys and versions → hydrate → captured Intent → edit result` |
| **23** | [`32gie-response`](#checkpoint-32gie-response) | Route validated scene edits and original-precision downloads back to the captured command history. | `src/edit_intent.rs`, `src/browser.rs` | `Current completion → captured command name → complete → Changed or Saved → dock answer` |
| **24** | [`32gif-auto`](#checkpoint-32gif-auto) | Connect cold-source edits and Save to captured reload and validated replay without requiring Reload Sources first. | `src/browser_reload.rs`, `src/browser.rs` | `Typed command → capture original intent → required cold keys → fetch → validated completion → one edit or download` |
| **25** | [`32gj-precision`](#checkpoint-32gj-precision) | Verify the automatic browser Save download retains exact source coordinates after display-only unloading. | `src/specimen.rs`, `examples/sample.rs`, `src/lib.rs`, `src/reload_precision_tests.rs` | `Original f64 sentinel → derived f32 display → unload kernel → reload original bytes → Save → exact source doubles` |
| **26** | [`32gja-failures`](#checkpoint-32gja-failures) | Verify automatic command failures preserve cold drawing, placement and history, including all-or-nothing multiple-source Save. | `src/lib.rs`, `src/reload_failure_tests.rs` | `Captured operation → complete body set → validate every source → all restored or none → no edit/download on failure` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-32g-keys"></a>

### Step 1 of 26 · Identify the source release a reload belongs to (`32g-keys`)

- **Goal:** Borrow the current imported release identity and epoch before asking for source data.
- **Data Flow Trace:** `Active cold rows → ReloadKey::of → unique imported Origin + release epoch`
- **Files You Touch:** `src/lib.rs` (*Give reload identity and preparation their own Rust boundary.*) · `src/rehydrate.rs` (*Match an imported allocation and checked release epoch instead of only a file GUID.*) · `src/editor.rs` (*Request each active imported release once without duplicating its geometry.*)

Define `ReloadKey` as an Origin owner and one release epoch. matches compares the actual imported Origin Rc and the row’s current epoch. That is the residency identity; local/saved object identity and source GUID remain separate.

Collect distinct keys from active cold rows. A later explicit reload can use them, while source adoption will also visit matching history roots. Do not fetch a history-only source without a current active request. If history later restores that source, a new request can obtain its current key.

`Rc::clone` creates another owner of the same value; it does not copy the Origin. `Rc::ptr_eq` asks whether two owners refer to that same allocation. `Option<Self>` means a row might have no reload key: the question marks return None immediately for a loaded or unlocated row. `is_some_and` runs the comparison only when an origin exists.

> **Self-Check Question:** *Why is a file GUID insufficient to accept a reload result?*  
> **Answer:** Duplicate imports can share the same original GUID and byte version. A result must belong to this imported Origin allocation and its current release epoch. Loaded rows, closed rows and another release cycle must not match it.

- **What You Should See:** Run the reload-key checks below. A key from a closed import must not match a new import of identical bytes; changing the release epoch must also reject it. No fetch is connected yet.
- **Hands-On Lab Experiment:** Compare two imports of the same file. Explain why sharing the byte version must not make a result for one import eligible for the other.

<details>
<summary>Implementation 32g-keys: Identify the source release a reload belongs to</summary>

Borrow the current imported release identity and epoch before asking for source data.

Data flow: Active cold rows → ReloadKey::of → unique imported Origin + release epoch.

### 1. `src/lib.rs`

Give reload identity and preparation their own Rust boundary.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod reload_url;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32g-keys-01.rs" skip=20
```

### 2. `src/rehydrate.rs`

Match an imported allocation and checked release epoch instead of only a file GUID.

Create the file and type:

```rust
--8<-- "journey/code/32g-keys-02.rs"
```

### 3. `src/editor.rs`

Request each active imported release once without duplicating its geometry.

<details>
<summary>Locate the existing block</summary>

```rust
    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/32g-keys-03.rs"
```

</details>

<a id="checkpoint-32ga-prepare"></a>

### Step 2 of 26 · Prepare original kernel data without rebuilding its display (`32ga-prepare`)

- **Goal:** Validate an immutable source version and restore its Session into a private reload candidate.
- **Data Flow Trace:** `Original bytes → bounded decode/version check → candidate Session → original source-GUID lookup`
- **Files You Touch:** `src/document.rs` (*Share bounded decoding without sharing display preparation.*) · `src/rehydrate.rs` (*Keep candidate kernel ownership private until every matched row has a valid original source.*)

Introduce `document::decode` as the common bounded protobuf/schema boundary. Ordinary `load_at` still creates an Origin and prepares display meshes. `document::restore` instead validates the expected immutable byte version, constructs the kernel `Session` and returns its Rc directly.

`PreparedReload` owns one key and one candidate `Session`. It checks each matching row’s original source GUID, name and visibility/locking against the decoded kernel mesh. Missing source identity or metadata drift refuses adoption; retained displays are never treated as editable geometry.

A candidate does not change a row. This separation makes it possible to validate every candidate and history root before mutation. SHA-256 here identifies the immutable selected File; mutable published HTTP source policies remain later work.

The ampersand borrows bytes and the Origin for this call. `Result<Rc<Session>, &'static str>` returns either an owned `Session` handle or an error message; the messages are string literals that exist for the whole program. map wraps only a successful `Session` in Rc. A failed candidate cannot change the `Scene` because preparation never receives a mutable `Scene`.

> **Self-Check Question:** *Why should reloading editable sources avoid preparing another display mesh?*  
> **Answer:** The displayed rows and GPU buffers are already retained. Reload needs original kernel ownership for editing, not a second float approximation or a new upload. Decode and validate the exact source version, then keep a candidate Session until every affected row is checked.

- **What You Should See:** Run the preparation checks below. Matching immutable bytes produce private source candidates; a wrong version is rejected without restoring any live row.
- **Hands-On Lab Experiment:** Follow the difference between load_at and restore. Identify where display conversion occurs, and why the restored candidate should not invoke it.

<details>
<summary>Implementation 32ga-prepare: Prepare original kernel data without rebuilding its display</summary>

Validate an immutable source version and restore its Session into a private reload candidate.

Data flow: Original bytes → bounded decode/version check → candidate Session → original source-GUID lookup.

### 1. `src/document.rs`

Share bounded decoding without sharing display preparation. Restore only original kernel data after validating the immutable source version.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    Ok(Loaded { document, meshes })
}

pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    let mut objects = proto::Objects::default();
    let mut children = Vec::new();
```

</details>

Replace it with:

```rust
    Ok(Loaded { document, meshes })
}

fn decode(bytes: &[u8]) -> Result<proto::Session, &'static str> {
    if bytes.len() > MAX_BYTES { return Err("This checkpoint accepts files up to 4 MiB"); }
    let message = proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")?;
    validate(&message)?;
    Ok(message)
}

pub(crate) fn restore(bytes: &[u8], origin: &crate::origin::Origin) -> Result<Rc<Session>, &'static str> {
    if bytes.len() > MAX_BYTES { return Err("This checkpoint accepts files up to 4 MiB"); }
    if crate::origin::FileVersion::of(bytes) != origin.version { return Err("Reloaded file version changed"); }
    Session::from_proto(decode(bytes)?).map(Rc::new).map_err(|_| "Cannot restore source session")
}

pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    let mut objects = proto::Objects::default();
    let mut children = Vec::new();
```

<details>
<summary>Locate this block</summary>

```rust
pub fn load(bytes: &[u8]) -> Result<Loaded, &'static str> { load_at(bytes, None) }

pub fn load_at(bytes: &[u8], location: Option<Rc<crate::reload_url::ReloadUrl>>) -> Result<Loaded, &'static str> {
    if bytes.len() > MAX_BYTES { return Err("This checkpoint accepts files up to 4 MiB"); }
    let message = proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")?;
    validate(&message)?;
    let mut origin = crate::origin::Origin::new(&message, bytes); origin.location = location;
    let origin = Rc::new(origin);
    let document = Rc::new(Session::from_proto(message).map_err(|_| "Cannot construct session")?);
```

</details>

Replace it with:

```rust
pub fn load(bytes: &[u8]) -> Result<Loaded, &'static str> { load_at(bytes, None) }

pub fn load_at(bytes: &[u8], location: Option<Rc<crate::reload_url::ReloadUrl>>) -> Result<Loaded, &'static str> {
    let message = decode(bytes)?;
    let mut origin = crate::origin::Origin::new(&message, bytes); origin.location = location;
    let origin = Rc::new(origin);
    let document = Rc::new(Session::from_proto(message).map_err(|_| "Cannot construct session")?);
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::prepared::PreparedMesh;
use prost::Message;
use session_rust::{proto, Session};
use std::{collections::HashSet, rc::Rc};

pub const MAX_BYTES: usize = 4 * 1024 * 1024;

#[derive(Clone)]
pub struct Source {
    pub document: Rc<Session>,
    pub guid: String,
    pub origin: Rc<crate::origin::Origin>,
}

pub struct Loaded {
    pub document: Rc<Session>,
    pub meshes: Vec<PreparedMesh>,
}

pub fn load(bytes: &[u8]) -> Result<Loaded, &'static str> { load_at(bytes, None) }

pub fn load_at(bytes: &[u8], location: Option<Rc<crate::reload_url::ReloadUrl>>) -> Result<Loaded, &'static str> {
    let message = decode(bytes)?;
    let mut origin = crate::origin::Origin::new(&message, bytes); origin.location = location;
    let origin = Rc::new(origin);
    let document = Rc::new(Session::from_proto(message).map_err(|_| "Cannot construct session")?);
    let mut meshes = Vec::new();
    for mesh in document.objects.meshes.iter() {
        let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
        prepared.model = document.xforms.get(mesh.guid()).cloned().unwrap_or_else(session_rust::Xform::identity);
        prepared.source = Some(Source {
            document: Rc::clone(&document), guid: mesh.guid().to_owned(), origin: Rc::clone(&origin),
        });
        meshes.push(prepared);
    }
    Ok(Loaded { document, meshes })
}

fn decode(bytes: &[u8]) -> Result<proto::Session, &'static str> {
    if bytes.len() > MAX_BYTES { return Err("This checkpoint accepts files up to 4 MiB"); }
    let message = proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")?;
    validate(&message)?;
    Ok(message)
}

pub(crate) fn restore(bytes: &[u8], origin: &crate::origin::Origin) -> Result<Rc<Session>, &'static str> {
    if bytes.len() > MAX_BYTES { return Err("This checkpoint accepts files up to 4 MiB"); }
    if crate::origin::FileVersion::of(bytes) != origin.version { return Err("Reloaded file version changed"); }
    Session::from_proto(decode(bytes)?).map(Rc::new).map_err(|_| "Cannot restore source session")
}

pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    let mut objects = proto::Objects::default();
    let mut children = Vec::new();
    let mut xforms = Vec::new();
    for object in scene.objects() {
        let geometry = object.geometry().ok_or("Reload editable sources before saving")?;
        let mut mesh = geometry.to_proto();
        mesh.guid = object.guid.clone();
        objects.meshes.push(mesh);
        children.push(proto::TreeNode { name: object.guid.clone(), ..Default::default() });
        if object.model.m != session_rust::Xform::identity().m {
            xforms.push(proto::XformEntry {
                guid: object.guid.clone(), xform: Some(object.model.to_proto()),
            });
        }
    }
    let message = proto::Session {
        name: "Viewer document".into(), objects: Some(objects),
        tree: Some(proto::Tree { root: Some(proto::TreeNode {
            name: "Objects".into(), children, ..Default::default()
        }), ..Default::default() }), xforms, ..Default::default()
    };
    validate(&message)?;
    let bytes = message.encode_to_vec();
    if bytes.len() > MAX_BYTES { return Err("The saved document exceeds 4 MiB"); }
    Ok(bytes)
}

fn validate(message: &proto::Session) -> Result<(), &'static str> {
    let objects = message.objects.as_ref().ok_or("Session has no objects")?;
    if message.definitions.is_some() || !message.interactions.is_empty() {
        return Err("Definitions and interactions arrive in later checkpoints");
    }
    let other = objects.points.len() + objects.lines.len() + objects.planes.len()
        + objects.bboxes.len() + objects.polylines.len() + objects.pointclouds.len()
        + objects.nurbscurves.len() + objects.nurbssurfaces.len() + objects.breps.len()
        + objects.elements.len() + objects.components.len() + objects.sheets.len() + objects.instances.len();
    if other != 0 || objects.meshes.is_empty() || objects.meshes.len() > 64 {
        return Err("This checkpoint accepts 1–64 meshes and no other geometry types");
    }
    let mut guids = HashSet::new();
    for mesh in &objects.meshes {
        if mesh.guid.is_empty() || !guids.insert(mesh.guid.as_str()) {
            return Err("Every mesh needs a distinct source GUID");
        }
        validate_mesh(mesh)?;
    }
    let mut placed = HashSet::new();
    for entry in &message.xforms {
        let matrix = &entry.xform.as_ref().ok_or("Missing placement matrix")?.matrix;
        if !guids.contains(entry.guid.as_str()) || !placed.insert(entry.guid.as_str())
            || !crate::placement::valid(matrix)
        {
            return Err("Invalid mesh placement");
        }
    }
    if let Some(root) = message.tree.as_ref().and_then(|tree| tree.root.as_ref()) {
        let names: HashSet<_> = root.children.iter().map(|node| node.name.as_str()).collect();
        if names != guids || root.children.len() != guids.len()
            || root.children.iter().any(|node| !node.children.is_empty()) {
            return Err("This checkpoint needs one flat tree row per mesh");
        }
    }
    Ok(())
}

fn validate_mesh(mesh: &proto::Mesh) -> Result<(), &'static str> {
    if mesh.vertices.len() > 10_000 || mesh.faces.is_empty() || mesh.faces.len() > 10_000 {
        return Err("A mesh needs faces and at most 10,000 vertices and faces");
    }
    if mesh.color_mode != 0 { return Err("Per-face and per-vertex colours arrive later"); }
    for (&key, vertex) in &mesh.vertices {
        if key > u32::MAX as u64 || [vertex.x, vertex.y, vertex.z].iter().any(|v| !v.is_finite()) {
            return Err("Invalid vertex key or coordinate");
        }
    }
    for (&key, face) in &mesh.faces {
        if key > u32::MAX as u64 || !(3..=4).contains(&face.vertices.len()) || !face.holes.is_empty()
            || face.vertices.iter().any(|key| !mesh.vertices.contains_key(key)) {
            return Err("This checkpoint needs triangles or quads with existing vertices and no holes");
        }
    }
    for (key, triangles) in &mesh.triangulation {
        if !mesh.faces.contains_key(key) || triangles.vertices.len() % 3 != 0
            || triangles.vertices.iter().any(|key| !mesh.vertices.contains_key(key)) {
            return Err("Invalid stored triangulation");
        }
    }
    Ok(())
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/32ga-prepare-01.rs"
```

```text
--8<-- "journey/code/32ga-prepare-02.rs"
```

</details>

### 2. `src/rehydrate.rs`

Keep candidate kernel ownership private until every matched row has a valid original source.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod tests {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/32ga-prepare-03.rs"
```

</details>

<a id="checkpoint-32gb-adopt"></a>

### Step 3 of 26 · Adopt restored source owners as one residency change (`32gb-adopt`)

- **Goal:** Validate current releases and all matching history rows before adopting any kernel candidate.
- **Data Flow Trace:** `Current keys → prepare all candidates → validate all matching roots → adopt editable owners`
- **Files You Touch:** `src/scene.rs` (*Restore editable owners without replacing display, metadata, identity or placement.*) · `src/editor.rs` (*Check current release ownership and all source rows before any candidate is adopted.*)

Restore unloaded sources as one transaction. Validate every reply first; one invalid reply must leave all sources unchanged.

hydrate accepts key/byte pairs from a completed request. First check that each distinct key still matches an active cold row; a stale or duplicate key cannot start preparation. Next build every candidate and validate all matching active, Undo and Redo rows. Only after every check succeeds does `Scene` adopt the prepared source owners.

Adoption changes only `EditSource`: each matching row receives the restored mesh and Source sharing one candidate `Session` and the original Origin. Keep local ID, saved GUID, metadata Rc, display Rc and model matrix exactly as they were.

This is a residency pass, not a document edit. It must not clear history, append an Undo entry or alter camera/selection. The next endpoint adds explicit ownership, precision and rejection checks. Browser fetching and command replay are not wired yet.

`collect::<Result<_, _>>()` turns many candidate results into one result: an error stops collection and drops candidates already prepared. The question mark returns that error before adoption starts. `flat_map` visits rows from each active/history `Scene` without copying them. The final mutable pass replaces only editable ownership; it is not a History edit.

> **Self-Check Question:** *What prevents a valid first candidate from committing when a later source fails?*  
> **Answer:** Prepare every candidate and validate every affected row before the adoption loop. If any version, source GUID or metadata check fails, the function returns with all rows still cold and no new history entry.

- **What You Should See:** Run the adoption checks below. A bad second candidate must restore neither source. Valid candidates restore editable owners without changing display, placement or history.
- **Hands-On Lab Experiment:** Move the adoption loop ahead of candidate validation in a scratch copy. Explain what a failure in the second origin would leave behind.

<details>
<summary>Implementation 32gb-adopt: Adopt restored source owners as one residency change</summary>

Validate current releases and all matching history rows before adopting any kernel candidate.

Data flow: Current keys → prepare all candidates → validate all matching roots → adopt editable owners.

### 1. `src/scene.rs`

Restore editable owners without replacing display, metadata, identity or placement.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/32gb-adopt-01.rs"
```

### 2. `src/editor.rs`

Check current release ownership and all source rows before any candidate is adopted.

<details>
<summary>Locate the existing block</summary>

```rust
    fn unload_sources(&mut self) -> Result<(), &'static str> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/32gb-adopt-02.rs"
```

</details>

<a id="checkpoint-32gc-roundtrip"></a>

### Step 4 of 26 · Prove source restoration preserves display and history (`32gc-roundtrip`)

- **Goal:** Restore exact source coordinates and editing without reallocating retained display owners.
- **Data Flow Trace:** `Move → Unload Sources → checked hydrate → exact Save → retained Undo/Redo and GPU owners`
- **Files You Touch:** `src/lib.rs` (*Keep source restoration proofs in typed native state checks.*) · `src/rehydrate_tests.rs` (*Optional authoring fixture. Check exact restored coordinates and retained display/metadata/placement history.*)

Add a fixture with a deliberately precise double coordinate and original flags. Import through a located source, Move, unload, then hydrate the captured key with the original bytes. The old kernel value stays expired; restored source geometry has the exact original coordinate.

The row’s display and metadata Rcs are retained, as are its IDs, placement, camera and history roots. Snapshot can again serialize the exact editable source. Undo/Redo still return the earlier and moved placements with available restored kernel owners.

The native GPU fixture adds the same boundary: source hydration must preserve the existing geometry/settings allocations and drawing. The browser still demonstrates unloading at this endpoint; actual fetch and replay come after the native rejection checks.

The protobuf field uses `Some(true)` because its representation distinguishes an explicitly stored flag from an omitted one. This test checks flag preservation, not the later selection/locking policy. `Rc::downgrade` observes a value without keeping it alive; upgrade returning None proves the old kernel allocation is gone. `Rc::ptr_eq` then distinguishes a retained display from a newly copied display with equal coordinates.

> **Self-Check Question:** *How can the test distinguish rehydration from rebuilding the whole scene?*  
> **Answer:** Record row IDs, saved GUIDs, metadata/display Rc identities, model matrices, camera and history-root count. After hydration these must be unchanged, while kernel geometry becomes available again from the restored original bytes.

- **What You Should See:** Run the cold-source round-trip checks below. Restoring and saving must retain the original double coordinates through Undo and Redo.
- **Hands-On Lab Experiment:** Replace the restored kernel coordinate with a value reconstructed from the retained display and predict which assertion fails.

<details>
<summary>Implementation 32gc-roundtrip: Prove source restoration preserves display and history</summary>

Restore exact source coordinates and editing without reallocating retained display owners.

Data flow: Move → Unload Sources → checked hydrate → exact Save → retained Undo/Redo and GPU owners.

### 1. `src/lib.rs`

Keep source restoration proofs in typed native state checks.

<details>
<summary>Locate the existing block</summary>

```rust
mod unload_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gc-roundtrip-01.rs" skip=18
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/rehydrate_tests.rs`

Optional authoring fixture. Check exact restored coordinates and retained display/metadata/placement history.

Copy this supplied file:

```text
--8<-- "journey/code/32gc-roundtrip-02.rs"
```

</details>

</details>

<a id="checkpoint-32gd-rejections"></a>

### Step 5 of 26 · Reject stale or inconsistent source batches atomically (`32gd-rejections`)

- **Goal:** Keep rows cold on changed versions, missing source identity and obsolete release keys.
- **Data Flow Trace:** `Prepare two origins → reject changed second source → no adoption → successful retry`
- **Files You Touch:** `src/rehydrate_tests.rs` (*Optional authoring fixture. Reject the entire candidate batch before adopting any source and preserve displayed owners.*)

Test atomic source restoration across independent imported origins. Change the second source version and require both origins to remain cold. Then give a retained row the wrong original source GUID and require refusal.

`Rc::make_mut` gives the test row its own metadata when history shares the original. This simulates row inconsistency without modifying the imported file.

Version mismatch and row-metadata mismatch are separate rejection boundaries. Validate all candidates and rows before adopting any source. The next lesson tests stale and duplicate delivery.

> **Self-Check Question:** *Should a late result after Close be decoded before deciding that it is obsolete?*  
> **Answer:** No. First confirm the captured key still matches an active cold row. An obsolete result returns false before source construction or mutation, whether its payload is valid or malformed.

- **What You Should See:** Source-restoration rejection is covered by optional fixtures. The browser still shows the existing Unload Sources behavior; automatic restoration is connected later.
- **Hands-On Lab Experiment:** Unload an imported source and confirm its cached drawing stays visible. In the supplied two-origin example, trace why a malformed second candidate is rejected before the first origin is adopted. This is a code-reading exercise; native execution is optional.

<details>
<summary>Implementation 32gd-rejections: Reject stale or inconsistent source batches atomically</summary>

Keep rows cold on changed versions, missing source identity and obsolete release keys.

Data flow: Prepare two origins → reject changed second source → no adoption → successful retry.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/rehydrate_tests.rs`

Optional authoring fixture. Reject the entire candidate batch before adopting any source and preserve displayed owners.

<details>
<summary>Locate the existing block</summary>

```text
use crate::editor::{Action, Editor};
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/32gd-rejections-01.rs" skip=37
```

</details>

</details>

<a id="checkpoint-32ge-stale"></a>

### Step 6 of 26 · Ignore old reload results before decoding them (`32ge-stale`)

- **Goal:** Reject results for closed imports, loaded rows, duplicate keys and previous release epochs.
- **Data Flow Trace:** `Old import/release key → active-key check → false before payload decoding`
- **Files You Touch:** `src/rehydrate_tests.rs` (*Optional authoring fixture. Show that obsolete identities are rejected before version checks, decoding or mutation.*)

Close and reimport the same file with the same original GUIDs. The import Origin is a new allocation, so the previous key must be ignored. Then hydrate the current release, deliver it twice, release again and deliver the previous epoch. Every obsolete completion returns false.

A request must contain at most one candidate per origin. Duplicate keys are rejected before preparing either candidate. An empty batch also has no effect. Keep the current rows and their display identities throughout.

These checks establish the native adoption contract. Browser request cancellation and automatic Move/Delete/Save replay are still pending; the browser at this endpoint continues to demonstrate Unload Sources.

`Ok(false)` means this result no longer belongs to a current request; it is not a document error. `Err(message)` means a current candidate failed validation. Keeping those outcomes separate lets a later browser callback silently discard obsolete completions while reporting failures for the current reload. A captured key owns its Origin, so the next request-lifetime lesson must also drop that owner when cancelling work.

> **Self-Check Question:** *Why deliver deliberately invalid bytes with an obsolete reload key?*  
> **Answer:** A false result instead of a decode error proves the identity check happens first. Invalid stale payloads cannot revive a closed import or disturb the current source release.

- **What You Should See:** The supplied cases distinguish an obsolete reply from an invalid current candidate. They add no browser request or restoration behavior at this checkpoint.
- **Hands-On Lab Experiment:** Read the stale-key example with the request owner from the preceding lesson. Identify which identity changes after Close/reimport and which epoch changes after another release. Then contrast an obsolete reply with invalid bytes for the current key.

<details>
<summary>Implementation 32ge-stale: Ignore old reload results before decoding them</summary>

Reject results for closed imports, loaded rows, duplicate keys and previous release epochs.

Data flow: Old import/release key → active-key check → false before payload decoding.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/rehydrate_tests.rs`

Optional authoring fixture. Show that obsolete identities are rejected before version checks, decoding or mutation.

<details>
<summary>Locate the existing block</summary>

```text
use crate::editor::{Action, Editor};
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/32ge-stale-01.rs" skip=37
```

</details>

</details>

<a id="checkpoint-32gf-request"></a>

### Step 7 of 26 · Give each source request its own owner (`32gf-request`)

- **Goal:** Give each source request its own owner.
- **Data Flow Trace:** `Active release keys → ReloadJob → ticket and URL strings → one accepted completion`
- **Files You Touch:** `src/lib.rs` (*Expose the native request owner without adding browser APIs to it.*) · `src/reload_job.rs` (*Issue checked tickets and retain source keys only in current pending work.*)

Give each source reload a ticket. Only the current ticket can complete its batch, and it can complete once.

Add a browser-independent `ReloadJob`. It owns only one pending set of release keys, issues checked tickets and gives the caller a `Request` containing a ticket and URL strings. A new request cancels older keys before validating its own candidates. Empty requests, duplicate origins and missing locations return errors.

`finish` first checks the ticket, then takes the matching keys once. `Option<Vec<ReloadKey>>` distinguishes accepted keys from obsolete work. `take` replaces the pending option with `None`; the removed owners drop when the caller finishes with them. Cancellation takes the keys without handing them to abandoned work.

The request ticket identifies this operation; each key still identifies the imported source and its release epoch. Both checks are needed. This native owner is not yet connected to fetch or a new browser command.

> **Self-Check Question:** *Why does the asynchronous request receive URL strings instead of cloned Origins?*  
> **Answer:** A cloned Origin would retain its URL owner after cancellation or Close. Keep the keys in the pending job, clear them on cancellation, and let asynchronous work carry plain URL strings and a ticket. A late result then has no source ownership to revive.

- **What You Should See:** Run the request checks below. A reload ticket must identify one current batch and permit its completion once. Browser fetching follows next.
- **Hands-On Lab Experiment:** Keep a returned Request alive after cancelling the job. Explain why its strings cannot keep the Origin or ReloadUrl alive.

<details>
<summary>Implementation 32gf-request: Give each source request its own owner</summary>

Give each source request its own owner.

Data flow: Active release keys → ReloadJob → ticket and URL strings → one accepted completion.

### 1. `src/lib.rs`

Expose the native request owner without adding browser APIs to it.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod rehydrate;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gf-request-01.rs" skip=19
```

### 2. `src/reload_job.rs`

Issue checked tickets and retain source keys only in current pending work.

Create the file and type:

```rust
--8<-- "journey/code/32gf-request-02.rs"
```

</details>

<a id="checkpoint-32gfa-ownership"></a>

### Step 8 of 26 · Prove cancelled work releases its source owners (`32gfa-ownership`)

- **Goal:** Prove cancelled work releases its source owners.
- **Data Flow Trace:** `Pending job holds keys → Close drops rows → cancel drops keys → abandoned request is inert`
- **Files You Touch:** `src/reload_job.rs` (*Check ordering, exhaustion and actual owner expiration while abandoned work remains alive.*)

Add three native checks. The first starts requests out of order and proves that an older ticket cannot consume current keys; a matching ticket consumes them once. Cancellation, empty requests and duplicate origins leave no pending work.

The ownership check closes the editor while a pending job still owns the source. Then it cancels the job and keeps a simulated abandoned asynchronous owner alive. `Weak::upgrade` must return `None` for the Origin and URL while the request strings remain readable. This directly tests the reason for separating URL strings from source owners.

Finally force ticket exhaustion. A checked increment must fail, never wrap, and leave older keys cancelled. These tests do not claim the browser has aborted a network read yet; that controller belongs to the later browser flight.

> **Self-Check Question:** *Can a held reference to the job keep the source alive after cancellation?*  
> **Answer:** It can keep the empty job alive, but not its removed keys. Weak observers of the Origin and URL must expire even while an abandoned caller still holds the job and its request strings.

- **What You Should See:** Run the request-owner checks below. Cancelling a batch releases its request metadata; a late completion must not revive that owner.
- **Hands-On Lab Experiment:** Change Request to store an `Rc<Origin>` in a scratch copy and predict which Weak assertion would fail after cancellation.

<details>
<summary>Implementation 32gfa-ownership: Prove cancelled work releases its source owners</summary>

Prove cancelled work releases its source owners.

Data flow: Pending job holds keys → Close drops rows → cancel drops keys → abandoned request is inert.

### 1. `src/reload_job.rs`

Check ordering, exhaustion and actual owner expiration while abandoned work remains alive.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn cancel(&mut self) -> bool { self.pending.take().is_some() }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gfa-ownership-01.rs" skip=73
```

</details>

<a id="checkpoint-32gg-fetch"></a>

### Step 9 of 26 · Read a source response within its byte limit (`32gg-fetch`)

- **Goal:** Read a source response within its byte limit.
- **Data Flow Trace:** `Recorded URL → fetch with AbortSignal → HTTP/length checks → bounded stream chunks`
- **Files You Touch:** `Cargo.toml` (*Enable only the browser types used by streaming and cancellation.*) · `src/lib.rs` (*Keep browser promises and streams out of the native editor.*) · `src/source_fetch.rs` (*Read original byte chunks without exceeding the per-source limit.*)

Fetch source bytes with cancellation and a four-MiB limit. Reject oversized or incomplete bodies before they reach the scene.

Enable the browser fetch, response, stream-reader and abort types. `source_fetch::fetch` uses a borrowed URL and AbortSignal, checks the HTTP status and optional length, then reads the response body as a stream. The current tutorial limit remains 4 MiB per source; it is not a measurement of browser staging or total process memory.

`JsFuture::from` lets Rust await a browser promise. `dyn_into` checks that the returned JavaScript value has the expected browser type. Each reader reply contains `done` and a byte-array `value`; `Reflect::get` reads those fields. Check remaining capacity before converting and appending a chunk.

On read failure, cancel the reader and release its lock. On success, release the lock and return original bytes. Version/schema checks still belong to source preparation, before adoption. This helper is compiled here; the browser command will exercise it after the flight and delivery boundaries exist.

> **Self-Check Question:** *Why is checking only Content-Length insufficient?*  
> **Answer:** The header can be missing or wrong. Check it before reading when present, then enforce the same limit on every received chunk. Never append a chunk that would exceed the per-source byte budget.

- **What You Should See:** Run the bounded-fetch checks below. Oversized or incomplete bodies must fail before source adoption. The fetch is not a viewer command yet.
- **Hands-On Lab Experiment:** Remove the incremental limit in a scratch copy. Explain what a response without a length header could allocate before the final check.

<details>
<summary>Implementation 32gg-fetch: Read a source response within its byte limit</summary>

Read a source response within its byte limit.

Data flow: Recorded URL → fetch with AbortSignal → HTTP/length checks → bounded stream chunks.

### 1. `Cargo.toml`

Enable only the browser types used by streaming and cancellation.

<details>
<summary>Locate the existing block</summary>

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement"] }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/32gg-fetch-01.toml"
```

</details>

### 2. `src/lib.rs`

Keep browser promises and streams out of the native editor.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(target_arch = "wasm32")]
mod browser;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/32gg-fetch-02.rs"
```

### 3. `src/source_fetch.rs`

Read original byte chunks without exceeding the per-source limit.

Create the file and type:

```rust
--8<-- "journey/code/32gg-fetch-03.rs"
```

</details>

<a id="checkpoint-32gga-flight"></a>

### Step 10 of 26 · Pair reload ownership with a browser abort controller (`32gga-flight`)

- **Goal:** Pair reload ownership with a browser abort controller.
- **Data Flow Trace:** `ReloadJob + AbortController → captured signal → cancel or current finish`
- **Files You Touch:** `src/lib.rs` (*Keep the browser flight beside the fetch adapter.*) · `src/browser_reload.rs` (*Cancel keys and I/O together without letting old results touch a new flight.*)

Define `Flight` with the native job and one AbortController. `begin` cancels the previous flight, creates the browser controller and gives the fetch task its signal together with URL strings. The controller stays in the current owner.

`cancel` removes keys and takes the controller before calling abort. `finish` checks the ticket through the job first. A matching failure aborts any remaining response work; a successful complete batch simply drops its controller. An older completion cannot reach either path.

`Shared` is an `Rc<RefCell<Flight>>` because one browser event callback and its asynchronous task need the same owner. Borrow only for short synchronous operations. Never keep a RefCell borrow across await: another event must be able to cancel while a fetch is waiting.

> **Self-Check Question:** *Why must an old failed request not abort the newer controller?*  
> **Answer:** Only a matching current ticket may finish and take the current controller. A late result first fails the job ticket check and cannot touch newer work.

- **What You Should See:** Run the checks below and build the browser bundle. The flight now pairs its ticket with an AbortController; cancelling it removes authority and aborts its fetch.
- **Hands-On Lab Experiment:** Move controller.take before the job ticket check in a scratch copy. Trace how a late failure could remove the newer request controller.

<details>
<summary>Implementation 32gga-flight: Pair reload ownership with a browser abort controller</summary>

Pair reload ownership with a browser abort controller.

Data flow: ReloadJob + AbortController → captured signal → cancel or current finish.

### 1. `src/lib.rs`

Keep the browser flight beside the fetch adapter.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(target_arch = "wasm32")]
mod source_fetch;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/32gga-flight-01.rs"
```

### 2. `src/browser_reload.rs`

Cancel keys and I/O together without letting old results touch a new flight.

Create the file and type:

```rust
--8<-- "journey/code/32gga-flight-02.rs"
```

</details>

<a id="checkpoint-32ggb-delivery"></a>

### Step 11 of 26 · Deliver only the current completed reload batch (`32ggb-delivery`)

- **Goal:** Deliver only the current completed reload batch.
- **Data Flow Trace:** `Flight begin → await URL reads → finish ticket once → scoped Rust reply → viewer event`
- **Files You Touch:** `src/browser_reload.rs` (*Spawn bounded source reads and deliver only one current owned completion.*)

Add a `Delivery` slot and spawn one local asynchronous task. Fetch each requested URL with the captured signal. A failure stops the batch; successful bytes are kept in request order. No Scene changes while any read is pending.

After awaiting, ask the shared owner to finish this ticket. A cancelled or replaced request returns immediately, before reporting an error or dispatching a viewer event. For the current request, zip the accepted keys with its bytes and place that Result in the Rust slot for synchronous dispatch. Clear any unclaimed result after dispatch.

The asynchronous task holds the shared flight, plain strings, signal and delivery slot. Cancellation empties the flight keys, so a delayed task does not keep the old Origin alive. Event callbacks will take the accepted slot and call the atomic hydration API; the command is connected next.

> **Self-Check Question:** *Why is the restored source data kept in a Rust delivery slot rather than event.detail?*  
> **Answer:** The slot carries the accepted release keys with their bytes. The event only wakes the viewer; an external event without accepted Rust data cannot fabricate a source completion.

- **What You Should See:** Run the delivery checks below. Only the current completed batch can fill the reply slot. Stale or duplicate replies must leave that slot unchanged.
- **Hands-On Lab Experiment:** Dispatch viewer-reload from the browser console without an accepted slot. Predict why it cannot restore any source.

<details>
<summary>Implementation 32ggb-delivery: Deliver only the current completed reload batch</summary>

Deliver only the current completed reload batch.

Data flow: Flight begin → await URL reads → finish ticket once → scoped Rust reply → viewer event.

### 1. `src/browser_reload.rs`

Spawn bounded source reads and deliver only one current owned completion.

<details>
<summary>Locate the existing block</summary>

```rust
pub type Shared = Rc<RefCell<Flight>>;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32ggb-delivery-01.rs" skip=39
```

</details>

<a id="checkpoint-32gh-command"></a>

### Step 12 of 26 · Restore editable sources through the command line (`32gh-command`)

- **Goal:** Restore editable sources through the command line.
- **Data Flow Trace:** `Reload Sources → active keys → owned fetch → atomic hydrate → same GPU display`
- **Files You Touch:** `src/browser.rs` (*Make source residency requests typed commands without feature buttons.*)

Add `Reload Sources` and `Cancel Reload` to the same production-styled dock. Reload collects active release keys and starts the browser flight. Immediate setup errors belong to the typed command; later success or failure gets its own `Reload Sources` history reply.

A `viewer-reload` event takes only the accepted Rust delivery slot. Atomic hydrate validates keys, immutable bytes and every affected history row before restoring source owners. Synchronize the renderer afterward: retained display Rcs and object settings must produce zero new GPU allocations or writes.

> **Self-Check Question:** *Does source restoration need a new Undo entry?*  
> **Answer:** No. Restoring editable ownership changes residency, not the document. Keep the existing drawing, placement and history; only the later requested edit should create a document transaction.

- **What You Should See:** Open `sample.pb`, then type `Unload Sources` and `Reload Sources`. Wait for “Editable sources restored; display retained.” The picture stays the same and Move works again.
- **Hands-On Lab Experiment:** Keep a command such as View Isometric between the reload request and its reply. Explain why panel.answer must append its own result instead of replacing that newer command.

<details>
<summary>Implementation 32gh-command: Restore editable sources through the command line</summary>

Restore editable sources through the command line.

Data flow: Reload Sources → active keys → owned fetch → atomic hydrate → same GPU display.

### 1. `src/browser.rs`

Make source residency requests typed commands without feature buttons. Own one current reload flight and one scoped accepted reply. Adopt only an accepted current result and give asynchronous replies their own history entry. Start or cancel an owned request from the actual command field. Wake the viewer for a scoped accepted result.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    document.add_event_listener_with_callback("cancel", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file-error", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
```

</details>

Replace it with:

```rust
    document.add_event_listener_with_callback("cancel", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file-error", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-reload", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
```

<details>
<summary>Locate this block</summary>

```rust
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            if line == "unload sources" {
                Some(Action::UnloadSources)
            } else if line == "close" {
                request.borrow_mut().cancel();
```

</details>

Replace it with:

```rust
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            if line == "reload sources" {
                let result = crate::browser_reload::start(std::rc::Rc::clone(&reload), editor.reload_keys(), std::rc::Rc::clone(&reload_delivery));
                match result {
                    Ok(()) => panel.result("Reloading editable sources…"),
                    Err(error) => { report(&error); panel.result(&error); }
                }
                None
            } else if line == "cancel reload" {
                reload.borrow_mut().cancel();
                report("Reload cancelled."); panel.result("Reload cancelled.");
                None
            } else if line == "unload sources" {
                Some(Action::UnloadSources)
            } else if line == "close" {
                request.borrow_mut().cancel();
```

<details>
<summary>Locate this block</summary>

```rust
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery) {
                Ok(action) => action,
```

</details>

Replace it with:

```rust
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-reload" {
            if let Some(result) = reload_delivery.borrow_mut().take() {
                match result.and_then(|values| editor.hydrate(values).map_err(str::to_owned)) {
                    Ok(true) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        let message = "Editable sources restored; display retained.";
                        report(message); panel.answer("Reload Sources", message);
                    }
                    Ok(false) => {}
                    Err(error) => { report(&error); panel.answer("Reload Sources", &error); }
                }
            }
            None
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery) {
                Ok(action) => action,
```

<details>
<summary>Locate this block</summary>

```rust
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
```

</details>

Replace it with:

```rust
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
```

<details>
<summary>Locate this block</summary>

```rust
            "Cancel Open",
            "Close",
            "Unload Sources",
            "Save",
            "Example Box",
            "Example Triangle",
```

</details>

Replace it with:

```rust
            "Cancel Open",
            "Close",
            "Unload Sources",
            "Reload Sources",
            "Cancel Reload",
            "Save",
            "Example Box",
            "Example Triangle",
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::background::Background;
use crate::editor::{Action, Change, Editor};
use crate::gesture::Gesture;
use crate::renderer::Renderer;
use crate::viewport::Viewport;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot draw:") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = Editor::default();
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    renderer.resize(Viewport { width, height });
    inspect(&editor, &renderer)?;
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &[
            "Help",
            "Open",
            "Open Replace",
            "Cancel Open",
            "Close",
            "Unload Sources",
            "Reload Sources",
            "Cancel Reload",
            "Save",
            "Example Box",
            "Example Triangle",
            "Select Next",
            "Delete",
            "Undo",
            "Redo",
            "Background",
            "Zoom In",
            "Zoom Out",
            "Pan Left",
            "Pan Right",
            "Orbit Right",
            "Orbit Up",
            "View Isometric",
            "View Perspective",
            "View Orthographic",
            "Fit",
            "Fit Selected",
            "Move",
            "View Reset",
        ],
    );
    panel.update(None, &canvas)?;
    if resize(
        &window,
        &canvas,
        &surface,
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            &mut panel,
        )?;
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
            event.prevent_default();
            panel.answer(shortcut.command(), "");
            Some(shortcut.command().to_ascii_lowercase())
        } else { match panel.update(Some(&event), &input_canvas) {
            Ok(line) => line,
            Err(error) => {
                report(&format!("Cannot read command: {error:?}"));
                return;
            }
        } };

        let action = if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
        {
            request.borrow_mut().cancel();
            report("Open cancelled.");
            panel.answer("Open", "Open cancelled.");
            None
        } else if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request), read_mode.get(), std::rc::Rc::clone(&delivery));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-reload" {
            if let Some(result) = reload_delivery.borrow_mut().take() {
                match result.and_then(|values| editor.hydrate(values).map_err(str::to_owned)) {
                    Ok(true) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        let message = "Editable sources restored; display retained.";
                        report(message); panel.answer("Reload Sources", message);
                    }
                    Ok(false) => {}
                    Err(error) => { report(&error); panel.answer("Reload Sources", &error); }
                }
            }
            None
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery) {
                Ok(action) => action,
                Err(error) => { let message = format!("Cannot retain reload file: {error:?}");
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            if line == "reload sources" {
                let result = crate::browser_reload::start(std::rc::Rc::clone(&reload), editor.reload_keys(), std::rc::Rc::clone(&reload_delivery));
                match result {
                    Ok(()) => panel.result("Reloading editable sources…"),
                    Err(error) => { report(&error); panel.result(&error); }
                }
                None
            } else if line == "cancel reload" {
                reload.borrow_mut().cancel();
                report("Reload cancelled."); panel.result("Reload cancelled.");
                None
            } else if line == "unload sources" {
                Some(Action::UnloadSources)
            } else if line == "close" {
                request.borrow_mut().cancel();
                Some(Action::Close)
            } else if line == "cancel open" {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            } else if line == "open" || line == "open replace" {
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                read_mode.set(if line == "open replace" { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
                    .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                }
                None
            } else if line == "save" {
                match crate::document::snapshot(&editor.scene) {
                    Ok(bytes) => match crate::file_output::download(&bytes) {
                        Ok(()) => panel.result(&format!("Saved editable document ({} bytes)", bytes.len())),
                        Err(error) => panel.result(&format!("Save failed: {error:?}")),
                    },
                    Err(error) => panel.result(error),
                }
                None
            } else if line == "move" || line.starts_with("move ") {
                match crate::offset::parse(&line) {
                    Ok(offset) => Some(Action::Translate(offset)),
                    Err(error) => { panel.result(error); None }
                }
            } else {
                let action = match line.as_str() {
                    "example box" => Action::AddBox,
                    "example triangle" => Action::ToggleExtra,
                    "select next" => Action::SelectNext,
                    "delete" => Action::Delete,
                    "undo" => Action::Undo,
                    "redo" => Action::Redo,
                    "background" => Action::Background,
                    "zoom in" => Action::Zoom(2.0),
                    "zoom out" => Action::Zoom(0.5),
                    "pan left" => Action::Pan(-0.25, 0.0),
                    "pan right" => Action::Pan(0.25, 0.0),
                    "orbit right" => Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0),
                    "orbit up" => Action::Orbit(0.0, std::f64::consts::FRAC_PI_6),
                    "view isometric" => Action::Isometric,
                    "view perspective" => {
                        Action::Projection(crate::camera::Projection::Perspective)
                    }
                    "view orthographic" => {
                        Action::Projection(crate::camera::Projection::Orthographic)
                    }
                    "fit" if shortcut.is_some() && editor.selected.is_some() => Action::FitSelected,
                    "fit" => Action::Fit,

                    "fit selected" => Action::FitSelected,
                    "view reset" => Action::ResetView,
                    _ => return,
                };
                Some(action)
            }
        } else if !panel.consumed {
            navigation_action(&event, &pointer_canvas, &mut gesture)
        } else {
            None
        };
        if let Some(action) = action {
            let replacement = matches!(&action, Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            match editor.apply(action) {
                Ok(Change::Scene) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if unloading {
                        report("Editable sources unloaded; display retained.");
                        panel.result("Editable sources unloaded; display retained.");
                    } else if closing {
                        report("Document closed.");
                        panel.result("Document closed.");
                    } else if event.type_() == "viewer-file" {
                        let message = if replacement { "Document replaced. Undo restores the previous document." }
                            else { "File imported. Undo removes the entire import." };
                        report(message);
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Change::View) => {}
                Err(error) => {
                    report(error);
                    if event.type_() == "viewer-file" { panel.answer("Open", error); }
                    else { panel.result(error); }
                }
            }
        }
        if let Err(error) = inspect(&editor, &renderer) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
        if resize(
            &browser_window,
            &pointer_canvas,
            &surface,
            &mut config,
            &mut renderer,
            &mut editor,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
                return;
            }
            if let Err(error) = present(
                &surface,
                &renderer,
                &editor.background,
                &editor.camera.uniform(),
                &mut panel,
            ) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        }
    });
    for name in [
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
        "lostpointercapture",
        "contextmenu",
        "keydown",
        "keyup",
        "blur",
    ] {
        canvas.add_event_listener_with_callback(name, update.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        update.as_ref().unchecked_ref(),
        &options,
    )?;
    window.add_event_listener_with_callback("resize", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("cancel", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file-error", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-reload", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}

fn inspect(editor: &Editor, renderer: &Renderer) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
        (&row.guid, &m.source_guid, &m.name, m.is_visible, m.is_locked)
    }).collect();
    let origins: Vec<_> = editor.scene.objects().iter().filter_map(|row| row.origin().map(|origin|
        (&row.guid, origin.id.to_string(), origin.version.hex(),
            origin.location.as_ref().map(|location| location.value())))).collect();
    let residency: Vec<_> = editor.scene.objects().iter().map(|row|
        (&row.guid, row.geometry().is_some(), row.release_epoch())).collect();
    canvas.set_attribute("data-source-residency", &serde_json::to_string(&residency)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-source-origins", &serde_json::to_string(&origins)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-row-metadata", &serde_json::to_string(&metadata)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-gpu-stats", &format!("{:?}", renderer.stats()))?;
    let cpu = crate::memory::cpu(editor.scenes(), renderer.geometry_owners().map(|owner| &owner.source));
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
    canvas.set_attribute("data-gpu-usage", &format!("{:?}", renderer.usage()))?;
    canvas.set_attribute("data-projection", &format!("{:?}", editor.camera.projection))
}

fn navigation_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    let action = match event.type_().as_str() {
        "wheel" => {
            let wheel = event.dyn_ref::<web_sys::WheelEvent>()?;
            let mode = wheel.delta_mode();
            crate::navigation::wheel(
                wheel.delta_y(),
                mode,
                canvas.get_bounding_client_rect().height(),
            )
        }
        "keydown" => {
            let key = event.dyn_ref::<web_sys::KeyboardEvent>()?;
            if key.is_composing() || key.alt_key() {
                return None;
            }
            if key.key() == "Escape" {
                gesture.cancel();
                event.prevent_default();
                return None;
            }
            return None;
        }
        _ => return pointer_action(event, canvas, gesture),
    }?;
    event.prevent_default();
    Some(action)
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    match event.type_().as_str() {
        "contextmenu" => {
            event.prevent_default();
            return None;
        }
        "blur" | "resize" => {
            gesture.cancel();
            return None;
        }
        _ => {}
    }
    let pointer = event.dyn_ref::<web_sys::PointerEvent>()?;
    let id = pointer.pointer_id();
    let position = [pointer.client_x() as f64, pointer.client_y() as f64];
    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary() && gesture.press(id, pointer.button(), position) {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err()
                    || canvas.set_pointer_capture(id).is_err()
                {
                    gesture.cancel();
                }
            }
            None
        }
        "pointermove" => gesture.move_to(id, position),
        "pointerup" => {
            let motion = gesture.release(id, position);
            let _ = canvas.release_pointer_capture(id);
            motion
        }
        "pointercancel" | "lostpointercapture" => {
            gesture.cancel_pointer(id);
            None
        }
        _ => None,
    }?;
    let rect = canvas.get_bounding_client_rect();
    motion.action([rect.left(), rect.top(), rect.width(), rect.height()])
}

fn resize(
    window: &web_sys::Window,
    canvas: &web_sys::HtmlCanvasElement,
    surface: &wgpu::Surface<'_>,
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        window.device_pixel_ratio(),
        limit,
    ) else {
        return false;
    };
    if size.width != config.width || size.height != config.height {
        canvas.set_width(size.width);
        canvas.set_height(size.height);
        config.width = size.width;
        config.height = size.height;
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    editor.camera.aspect = size.aspect();
    true
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 16],
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => {
            return Err(JsValue::from_str(&format!(
                "Surface unavailable: {other:?}"
            )));
        }
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view, background, transform);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/32gh-command-01.rs"
```

```text
--8<-- "journey/code/32gh-command-02.rs"
```

```text
--8<-- "journey/code/32gh-command-03.rs"
```

```text
--8<-- "journey/code/32gh-command-04.rs"
```

```text
--8<-- "journey/code/32gh-command-05.rs"
```

</details>

</details>

<a id="checkpoint-32gha-cancel"></a>

### Step 13 of 26 · Cancel reloads when their document context changes (`32gha-cancel`)

- **Goal:** Cancel reloads when their document context changes.
- **Data Flow Trace:** `Close / replacement / Undo / release → cancel flight → late result is inert`
- **Files You Touch:** `src/browser.rs` (*Invalidate the old reload as soon as replacement intent begins.*) · `examples/reload_check.rs` (*Supply this optional course-author fixture generator; browser exercises use the linked data files or recorded comparison.*)

Cancel the reload before Close, another Unload Sources, Undo/Redo or replacement adoption. Open Replace also cancels at picker opening, matching the existing file-read boundary. Camera and selection remain available while a source request waits.

`matches!` recognizes actions that invalidate pending work. Borrow the reload owner, cancel it before applying the document action, then end that borrow. Late delivery cannot regain the old ticket’s authority.

> **Self-Check Question:** *Why cancel Open Replace at picker opening rather than after its file finishes reading?*  
> **Answer:** The user has already chosen to replace the document context. Cancel the older reload before waiting for another picker or file read, so its delayed result cannot adopt into that context.

- **What You Should See:** Open `sample.pb` and unload its sources. Request `Reload Sources`, then `Cancel Reload` while it is pending. Its late reply must not restore the cancelled batch.
- **Hands-On Lab Experiment:** Cancel only after replacement adoption in a scratch copy. Explain the interval in which an older reload could still finish while the replacement picker or read is pending.

<details>
<summary>Implementation 32gha-cancel: Cancel reloads when their document context changes</summary>

Cancel reloads when their document context changes.

Data flow: Close / replacement / Undo / release → cancel flight → late result is inert.

### 1. `src/browser.rs`

Invalidate the old reload as soon as replacement intent begins. Remove reload keys and abort I/O before changing the current document context.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            None
        };
        if let Some(action) = action {
            let replacement = matches!(&action, Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
```

</details>

Replace it with:

```rust
            None
        };
        if let Some(action) = action {
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Undo | Action::Redo
                | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
            let replacement = matches!(&action, Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
```

<details>
<summary>Locate this block</summary>

```rust
            } else if line == "open" || line == "open replace" {
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                read_mode.set(if line == "open replace" { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
```

</details>

Replace it with:

```rust
            } else if line == "open" || line == "open replace" {
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                if line == "open replace" { reload.borrow_mut().cancel(); }
                read_mode.set(if line == "open replace" { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::background::Background;
use crate::editor::{Action, Change, Editor};
use crate::gesture::Gesture;
use crate::renderer::Renderer;
use crate::viewport::Viewport;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot draw:") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = Editor::default();
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    renderer.resize(Viewport { width, height });
    inspect(&editor, &renderer)?;
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &[
            "Help",
            "Open",
            "Open Replace",
            "Cancel Open",
            "Close",
            "Unload Sources",
            "Reload Sources",
            "Cancel Reload",
            "Save",
            "Example Box",
            "Example Triangle",
            "Select Next",
            "Delete",
            "Undo",
            "Redo",
            "Background",
            "Zoom In",
            "Zoom Out",
            "Pan Left",
            "Pan Right",
            "Orbit Right",
            "Orbit Up",
            "View Isometric",
            "View Perspective",
            "View Orthographic",
            "Fit",
            "Fit Selected",
            "Move",
            "View Reset",
        ],
    );
    panel.update(None, &canvas)?;
    if resize(
        &window,
        &canvas,
        &surface,
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            &mut panel,
        )?;
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
            event.prevent_default();
            panel.answer(shortcut.command(), "");
            Some(shortcut.command().to_ascii_lowercase())
        } else { match panel.update(Some(&event), &input_canvas) {
            Ok(line) => line,
            Err(error) => {
                report(&format!("Cannot read command: {error:?}"));
                return;
            }
        } };

        let action = if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
        {
            request.borrow_mut().cancel();
            report("Open cancelled.");
            panel.answer("Open", "Open cancelled.");
            None
        } else if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request), read_mode.get(), std::rc::Rc::clone(&delivery));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-reload" {
            if let Some(result) = reload_delivery.borrow_mut().take() {
                match result.and_then(|values| editor.hydrate(values).map_err(str::to_owned)) {
                    Ok(true) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        let message = "Editable sources restored; display retained.";
                        report(message); panel.answer("Reload Sources", message);
                    }
                    Ok(false) => {}
                    Err(error) => { report(&error); panel.answer("Reload Sources", &error); }
                }
            }
            None
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery) {
                Ok(action) => action,
                Err(error) => { let message = format!("Cannot retain reload file: {error:?}");
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            if line == "reload sources" {
                let result = crate::browser_reload::start(std::rc::Rc::clone(&reload), editor.reload_keys(), std::rc::Rc::clone(&reload_delivery));
                match result {
                    Ok(()) => panel.result("Reloading editable sources…"),
                    Err(error) => { report(&error); panel.result(&error); }
                }
                None
            } else if line == "cancel reload" {
                reload.borrow_mut().cancel();
                report("Reload cancelled."); panel.result("Reload cancelled.");
                None
            } else if line == "unload sources" {
                Some(Action::UnloadSources)
            } else if line == "close" {
                request.borrow_mut().cancel();
                Some(Action::Close)
            } else if line == "cancel open" {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            } else if line == "open" || line == "open replace" {
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                if line == "open replace" { reload.borrow_mut().cancel(); }
                read_mode.set(if line == "open replace" { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
                    .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                }
                None
            } else if line == "save" {
                match crate::document::snapshot(&editor.scene) {
                    Ok(bytes) => match crate::file_output::download(&bytes) {
                        Ok(()) => panel.result(&format!("Saved editable document ({} bytes)", bytes.len())),
                        Err(error) => panel.result(&format!("Save failed: {error:?}")),
                    },
                    Err(error) => panel.result(error),
                }
                None
            } else if line == "move" || line.starts_with("move ") {
                match crate::offset::parse(&line) {
                    Ok(offset) => Some(Action::Translate(offset)),
                    Err(error) => { panel.result(error); None }
                }
            } else {
                let action = match line.as_str() {
                    "example box" => Action::AddBox,
                    "example triangle" => Action::ToggleExtra,
                    "select next" => Action::SelectNext,
                    "delete" => Action::Delete,
                    "undo" => Action::Undo,
                    "redo" => Action::Redo,
                    "background" => Action::Background,
                    "zoom in" => Action::Zoom(2.0),
                    "zoom out" => Action::Zoom(0.5),
                    "pan left" => Action::Pan(-0.25, 0.0),
                    "pan right" => Action::Pan(0.25, 0.0),
                    "orbit right" => Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0),
                    "orbit up" => Action::Orbit(0.0, std::f64::consts::FRAC_PI_6),
                    "view isometric" => Action::Isometric,
                    "view perspective" => {
                        Action::Projection(crate::camera::Projection::Perspective)
                    }
                    "view orthographic" => {
                        Action::Projection(crate::camera::Projection::Orthographic)
                    }
                    "fit" if shortcut.is_some() && editor.selected.is_some() => Action::FitSelected,
                    "fit" => Action::Fit,

                    "fit selected" => Action::FitSelected,
                    "view reset" => Action::ResetView,
                    _ => return,
                };
                Some(action)
            }
        } else if !panel.consumed {
            navigation_action(&event, &pointer_canvas, &mut gesture)
        } else {
            None
        };
        if let Some(action) = action {
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Undo | Action::Redo
                | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
            let replacement = matches!(&action, Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            match editor.apply(action) {
                Ok(Change::Scene) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if unloading {
                        report("Editable sources unloaded; display retained.");
                        panel.result("Editable sources unloaded; display retained.");
                    } else if closing {
                        report("Document closed.");
                        panel.result("Document closed.");
                    } else if event.type_() == "viewer-file" {
                        let message = if replacement { "Document replaced. Undo restores the previous document." }
                            else { "File imported. Undo removes the entire import." };
                        report(message);
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Change::View) => {}
                Err(error) => {
                    report(error);
                    if event.type_() == "viewer-file" { panel.answer("Open", error); }
                    else { panel.result(error); }
                }
            }
        }
        if let Err(error) = inspect(&editor, &renderer) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
        if resize(
            &browser_window,
            &pointer_canvas,
            &surface,
            &mut config,
            &mut renderer,
            &mut editor,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
                return;
            }
            if let Err(error) = present(
                &surface,
                &renderer,
                &editor.background,
                &editor.camera.uniform(),
                &mut panel,
            ) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        }
    });
    for name in [
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
        "lostpointercapture",
        "contextmenu",
        "keydown",
        "keyup",
        "blur",
    ] {
        canvas.add_event_listener_with_callback(name, update.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        update.as_ref().unchecked_ref(),
        &options,
    )?;
    window.add_event_listener_with_callback("resize", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("cancel", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file-error", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-reload", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}

fn inspect(editor: &Editor, renderer: &Renderer) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
        (&row.guid, &m.source_guid, &m.name, m.is_visible, m.is_locked)
    }).collect();
    let origins: Vec<_> = editor.scene.objects().iter().filter_map(|row| row.origin().map(|origin|
        (&row.guid, origin.id.to_string(), origin.version.hex(),
            origin.location.as_ref().map(|location| location.value())))).collect();
    let residency: Vec<_> = editor.scene.objects().iter().map(|row|
        (&row.guid, row.geometry().is_some(), row.release_epoch())).collect();
    canvas.set_attribute("data-source-residency", &serde_json::to_string(&residency)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-source-origins", &serde_json::to_string(&origins)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-row-metadata", &serde_json::to_string(&metadata)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-gpu-stats", &format!("{:?}", renderer.stats()))?;
    let cpu = crate::memory::cpu(editor.scenes(), renderer.geometry_owners().map(|owner| &owner.source));
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
    canvas.set_attribute("data-gpu-usage", &format!("{:?}", renderer.usage()))?;
    canvas.set_attribute("data-projection", &format!("{:?}", editor.camera.projection))
}

fn navigation_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    let action = match event.type_().as_str() {
        "wheel" => {
            let wheel = event.dyn_ref::<web_sys::WheelEvent>()?;
            let mode = wheel.delta_mode();
            crate::navigation::wheel(
                wheel.delta_y(),
                mode,
                canvas.get_bounding_client_rect().height(),
            )
        }
        "keydown" => {
            let key = event.dyn_ref::<web_sys::KeyboardEvent>()?;
            if key.is_composing() || key.alt_key() {
                return None;
            }
            if key.key() == "Escape" {
                gesture.cancel();
                event.prevent_default();
                return None;
            }
            return None;
        }
        _ => return pointer_action(event, canvas, gesture),
    }?;
    event.prevent_default();
    Some(action)
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    match event.type_().as_str() {
        "contextmenu" => {
            event.prevent_default();
            return None;
        }
        "blur" | "resize" => {
            gesture.cancel();
            return None;
        }
        _ => {}
    }
    let pointer = event.dyn_ref::<web_sys::PointerEvent>()?;
    let id = pointer.pointer_id();
    let position = [pointer.client_x() as f64, pointer.client_y() as f64];
    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary() && gesture.press(id, pointer.button(), position) {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err()
                    || canvas.set_pointer_capture(id).is_err()
                {
                    gesture.cancel();
                }
            }
            None
        }
        "pointermove" => gesture.move_to(id, position),
        "pointerup" => {
            let motion = gesture.release(id, position);
            let _ = canvas.release_pointer_capture(id);
            motion
        }
        "pointercancel" | "lostpointercapture" => {
            gesture.cancel_pointer(id);
            None
        }
        _ => None,
    }?;
    let rect = canvas.get_bounding_client_rect();
    motion.action([rect.left(), rect.top(), rect.width(), rect.height()])
}

fn resize(
    window: &web_sys::Window,
    canvas: &web_sys::HtmlCanvasElement,
    surface: &wgpu::Surface<'_>,
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        window.device_pixel_ratio(),
        limit,
    ) else {
        return false;
    };
    if size.width != config.width || size.height != config.height {
        canvas.set_width(size.width);
        canvas.set_height(size.height);
        config.width = size.width;
        config.height = size.height;
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    editor.camera.aspect = size.aspect();
    true
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 16],
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => {
            return Err(JsValue::from_str(&format!(
                "Surface unavailable: {other:?}"
            )));
        }
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view, background, transform);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/32gha-cancel-01.rs"
```

```text
--8<-- "journey/code/32gha-cancel-02.rs"
```

</details>

<details>
<summary>Optional authoring helper</summary>

Copy this supplied authoring helper once. Use the linked data or recorded comparison for the browser exercise; you do not need to compile or run this native helper.

### 2. `examples/reload_check.rs`

Supply this optional course-author fixture generator; browser exercises use the linked data files or recorded comparison.

Copy this supplied file:

```text
--8<-- "journey/code/32gha-cancel-03.rs"
```

</details>

</details>

<a id="checkpoint-32gi-capture"></a>

### Step 14 of 26 · Capture the requested edit before waiting (`32gi-capture`)

- **Goal:** Retain the original object and arguments before any source request starts.
- **Data Flow Trace:** `Command + original selection → value-only Intent → unchanged target while waiting`
- **Files You Touch:** `src/lib.rs` (*Register the value-only requested-edit module.*) · `src/edit_intent.rs` (*Capture the requested target and arguments without retaining any source owners.*)

A delayed edit needs a small record of the user’s request. Intent contains only a stable ObjectId and copied Move arguments, or Save. It owns no geometry, origin or URL. Changing selection cannot redirect that value.

Unselected Delete, zero-offset Move and ordinary view commands do not create an edit intent; invalid Move values are refused. Save is represented explicitly because it will need every active cold origin instead of one selected target. Source selection and replay are added in the following checkpoints.

> **Self-Check Question:** *Why store ObjectId and Move offset rather than selection or a precomputed model?*  
> **Answer:** Selection can change while I/O waits. The original ObjectId names the requested target; the offset expresses the requested change. A later replay must read that target’s current placement rather than replace it with a stale matrix.

- **What You Should See:** Run the intent checks below. Capture Move for one selected object, change selection, and compare the stored target and offset. Automatic browser replay comes later.
- **Hands-On Lab Experiment:** Trace the supplied captured-Move fixture: capture one object, change selection, then inspect the retained ObjectId. Explain which origin the later request must load.

<details>
<summary>Implementation 32gi-capture: Capture the requested edit before waiting</summary>

Retain the original object and arguments before any source request starts.

Data flow: Command + original selection → value-only Intent → unchanged target while waiting.

### 1. `src/lib.rs`

Register the value-only requested-edit module.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod reload_job;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gi-capture-01.rs" skip=20
```

### 2. `src/edit_intent.rs`

Capture the requested target and arguments without retaining any source owners.

Create the file and type:

```rust
--8<-- "journey/code/32gi-capture-02.rs"
```

</details>

<a id="checkpoint-32gia-scope"></a>

### Step 15 of 26 · Load only the sources the requested edit needs (`32gia-scope`)

- **Goal:** Select source requests from the original edit target, with Save covering the active document.
- **Data Flow Trace:** `Intent → original current row → cold origin + epoch; Save → unique active cold origins`
- **Files You Touch:** `src/edit_intent.rs` (*Bring the editor and existing release-key type into this module.*) · `src/lib.rs` (*Register the independent source-scope tests.*) · `src/edit_scope_tests.rs` (*Optional authoring fixture. Distinguish target imports from later selection, active Save sources from history, and missing targets from loaded ones.*)

Move and Delete follow the captured ObjectId, even if selection now belongs to a different import. A loaded target needs no source request. A missing target fails before any fetch begins. For a cold row, ReloadKey::of returns its exact origin and release epoch.

Save reuses Editor::reload_keys: it gathers each active cold origin once, even when several rows share it. It excludes loaded rows and sources held only by Undo/Redo history. Three native tests distinguish two imports, later selection, deduplication, history-only sources and missing or loaded targets.

The method borrows Editor only while collecting keys. The returned keys temporarily retain the origins needed by a future ReloadJob; the Intent itself still holds values only. Browser interception and replay are not connected yet.

> **Self-Check Question:** *Why should Save ignore a released source that exists only in Undo history?*  
> **Answer:** Save serializes the active scene. History-only rows are not in that file, so fetching their source delays Save without supplying any required data. Undo can request that source later when its rows become active.

- **What You Should See:** Run the scope checks below. Move/Delete request only their captured target; Save requests active cold sources, excluding history-only rows.
- **Hands-On Lab Experiment:** In the first scope test, point selection at a row from the second import. Explain why Move and Delete still request the first import, while Save requests both.

<details>
<summary>Implementation 32gia-scope: Load only the sources the requested edit needs</summary>

Select source requests from the original edit target, with Save covering the active document.

Data flow: Intent → original current row → cold origin + epoch; Save → unique active cold origins.

### 1. `src/edit_intent.rs`

Bring the editor and existing release-key type into this module. Resolve dependencies from the captured target or active Save scene.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
}

impl Intent {
    pub fn capture(action: &Action, selected: Option<ObjectId>) -> Result<Option<Self>, &'static str> {
        Ok(match action {
            Action::Translate(offset) => {
```

</details>

Replace it with:

```rust
}

impl Intent {
    pub fn keys(&self, editor: &Editor) -> Result<Vec<ReloadKey>, &'static str> {
        match self {
            Self::Save => Ok(editor.reload_keys()),
            Self::Move { id, .. } | Self::Delete { id } => {
                let row = editor.scene.objects().iter().find(|row| row.id == *id).ok_or("Object not found")?;
                Ok(ReloadKey::of(row).into_iter().collect())
            }
        }
    }

    pub fn capture(action: &Action, selected: Option<ObjectId>) -> Result<Option<Self>, &'static str> {
        Ok(match action {
            Action::Translate(offset) => {
```

<details>
<summary>Locate this block</summary>

```rust
use crate::{editor::Action, scene::ObjectId};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Intent {
```

</details>

Replace it with:

```rust
use crate::{editor::{Action, Editor}, rehydrate::ReloadKey, scene::ObjectId};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Intent {
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{editor::{Action, Editor}, rehydrate::ReloadKey, scene::ObjectId};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Intent {
    Move { id: ObjectId, offset: [f64; 3] },
    Delete { id: ObjectId },
    Save,
}

impl Intent {
    pub fn keys(&self, editor: &Editor) -> Result<Vec<ReloadKey>, &'static str> {
        match self {
            Self::Save => Ok(editor.reload_keys()),
            Self::Move { id, .. } | Self::Delete { id } => {
                let row = editor.scene.objects().iter().find(|row| row.id == *id).ok_or("Object not found")?;
                Ok(ReloadKey::of(row).into_iter().collect())
            }
        }
    }

    pub fn capture(action: &Action, selected: Option<ObjectId>) -> Result<Option<Self>, &'static str> {
        Ok(match action {
            Action::Translate(offset) => {
                let id = selected.ok_or("Select an object before Move")?;
                if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
                if offset.iter().all(|&value| value == 0.0) { None }
                else { Some(Self::Move { id, offset: *offset }) }
            }
            Action::Delete => selected.map(|id| Self::Delete { id }),
            _ => None,
        })
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::Editor, reload_url::ReloadUrl};
    use std::rc::Rc;

    fn cold() -> Editor {
        let mut editor = Editor::default();
        editor.apply(Action::ReplaceAt(crate::specimen::bytes(), Rc::new(ReloadUrl::new("test:intent".into())))).unwrap();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        editor
    }

    #[test]
    fn move_keeps_original_target_and_arguments_after_selection_changes() {
        let mut editor = cold(); let id = editor.selected.unwrap();
        let origin = Rc::downgrade(editor.scene.objects()[0].origin().unwrap());
        let intent = Intent::capture(&Action::Translate([0.125, -0.25, 0.5]), editor.selected).unwrap().unwrap();
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Orbit(0.1, 0.2)).unwrap();
        assert_ne!(editor.selected, Some(id));
        assert_eq!(intent, Intent::Move { id, offset: [0.125, -0.25, 0.5] });
        editor.apply(Action::Close).unwrap();
        assert!(origin.upgrade().is_none(), "intent owns no source or URL");
    }

    #[test]
    fn delete_keeps_its_original_target() {
        let mut editor = cold(); let id = editor.selected.unwrap();
        let intent = Intent::capture(&Action::Delete, editor.selected).unwrap().unwrap();
        editor.apply(Action::SelectNext).unwrap();
        assert_eq!(intent, Intent::Delete { id });
        assert_ne!(editor.selected, Some(id));
    }

    #[test]
    fn empty_edits_do_not_request_io_and_invalid_moves_are_rejected() {
        assert_eq!(Intent::capture(&Action::Delete, None).unwrap(), None);
        assert!(Intent::capture(&Action::Translate([0.0; 3]), None).is_err());
        let editor = Editor::default(); let id = editor.scene.objects()[0].id;
        assert_eq!(Intent::capture(&Action::Translate([0.0; 3]), Some(id)).unwrap(), None);
        assert!(Intent::capture(&Action::Translate([f64::NAN, 0.0, 0.0]), Some(id)).is_err());
        assert_eq!(Intent::capture(&Action::Fit, Some(id)).unwrap(), None);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/32gia-scope-01.rs"
```

```text
--8<-- "journey/code/32gia-scope-02.rs"
```

</details>

### 2. `src/lib.rs`

Register the independent source-scope tests.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod edit_intent;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gia-scope-03.rs" skip=21
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/edit_scope_tests.rs`

Optional authoring fixture. Distinguish target imports from later selection, active Save sources from history, and missing targets from loaded ones.

Copy this supplied file:

```text
--8<-- "journey/code/32gia-scope-04.rs"
```

</details>

</details>

<a id="checkpoint-32gib-move"></a>

### Step 16 of 26 · Move the original target from its current placement (`32gib-move`)

- **Goal:** Apply a captured Move to its original target, composing with the placement that exists at replay time.
- **Data Flow Trace:** `Captured ObjectId + offset → current row.model → world shift × model → one history edit`
- **Files You Touch:** `src/editor.rs` (*Resolve an explicit target from current state and commit its world-space delta once.*) · `src/lib.rs` (*Register the original-target Move checks.*) · `src/edit_move_tests.rs` (*Optional authoring fixture. Prove current placement, later selection, camera and one history transaction survive replay.*)

Extract Editor::move_object with an explicit ObjectId. It validates arguments, finds that current row, composes the world shift with its current model and makes one History::try_edit transaction. It leaves selection and camera untouched. Normal Action::Translate calls the same method after resolving current selection.

This is the native Move replay operation. One-shot asynchronous authority is added in the ticket/intent lesson, and automatic browser interception follows it. Delete and Save are taught separately to keep each change small.

> **Self-Check Question:** *Why must a delayed Move compose with the current placement rather than a matrix saved when the request began?*  
> **Answer:** The request describes an offset, not a replacement matrix. Reading the current model preserves any placement that now exists. The captured ObjectId chooses the target while current selection remains available for other interaction.

- **What You Should See:** Run the captured-Move checks below. Change selection while waiting: replay must still move the original target and compose with its current placement.
- **Hands-On Lab Experiment:** Read the supplied placement fixture. Predict what would be lost if replay replaced the newer model with the model that existed at capture time.

<details>
<summary>Implementation 32gib-move: Move the original target from its current placement</summary>

Apply a captured Move to its original target, composing with the placement that exists at replay time.

Data flow: Captured ObjectId + offset → current row.model → world shift × model → one history edit.

### 1. `src/editor.rs`

Resolve an explicit target from current state and commit its world-space delta once. Keep ordinary Move on the same implementation used by captured edits.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            }
            Action::Translate(offset) => {
                let id = self.selected.ok_or("Select an object before Move")?;
                if offset.iter().all(|&v| v == 0.0) { return Ok(Change::View); }
                let object = self.scene.objects().iter().find(|o| o.id == id).ok_or("Object not found")?;
                let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
                let model = &shift * &object.model;
                self.history.try_edit(&mut self.scene, |scene| scene.place(id, model))?;
            }
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => self.selected = self.scene.next(self.selected),
```

</details>

Replace it with:

```rust
            }
            Action::Translate(offset) => {
                let id = self.selected.ok_or("Select an object before Move")?;
                return self.move_object(id, offset);
            }
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => self.selected = self.scene.next(self.selected),
```

<details>
<summary>Locate this block</summary>

```rust
        Ok(())
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
```

</details>

Replace it with:

```rust
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        let object = self.scene.objects().iter().find(|row| row.id == id).ok_or("Object not found")?;
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        let model = &shift * &object.model;
        self.history.try_edit(&mut self.scene, |scene| scene.place(id, model))?;
        Ok(Change::Scene)
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    Pick([f32; 2]),
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    ReplaceAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    Close,
    UnloadSources,
    AddBox,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
}

pub struct Editor {
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub camera: Camera,
    pub background: Background,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scene: Scene::demo(),
            selected: None,
            camera: Camera::default(),
            background: Background::default(),
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene.objects().iter().filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id)).collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(&mut self, values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>) -> Result<bool, &'static str> {
        if values.is_empty() { return Ok(false); }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id) || !self.scene.objects().iter().any(|row| key.matches(row)) {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values.into_iter().map(|(key, bytes)|
            crate::rehydrate::PreparedReload::new(key, &bytes)).collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self.scenes().flat_map(|scene| scene.objects()).filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row)) { return Err("Reloaded source GUID or metadata does not match"); }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared { scene.hydrate(plan); }
        }
        Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self.scenes().flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload()).filter_map(|row| row.origin().map(|origin| origin.id)).collect();
        for row in self.scenes().flat_map(|scene| scene.objects()) {
            if let Some(origin) = row.origin() {
                if !row.can_unload() { origins.remove(&origin.id); }
            }
        }
        if origins.is_empty() { return Err("No eligible imported sources to unload"); }
        let epoch = self.last_release.checked_add(1).ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() { scene.unload(&origins, epoch); }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        let object = self.scene.objects().iter().find(|row| row.id == id).ok_or("Object not found")?;
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        let model = &shift * &object.model;
        self.history.try_edit(&mut self.scene, |scene| scene.place(id, model))?;
        Ok(Change::Scene)
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
                self.selected = self.camera.ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.selected = None;
            }
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => {
                let id = self.selected.ok_or("Select an object before Move")?;
                return self.move_object(id, offset);
            }
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => self.selected = self.scene.next(self.selected),
            Action::Delete => {
                if let Some(id) = self.selected {
                    let row = self.scene.objects().iter().find(|row| row.id == id).ok_or("Object not found")?;
                    if row.geometry().is_none() { return Err("Reload editable sources before Delete"); }
                }
                if let Some(id) = self.selected.take() {
                    self.history.edit(&mut self.scene, |scene| { scene.remove(id); });
                }
            }
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.scene.bounds() {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.selected
                            .and_then(|id| self.scene.selected_bounds(id))
                        {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::ResetView => {
                        let aspect = self.camera.aspect;
                        self.camera = Camera { aspect, ..Camera::default() };
                    }
                    _ => unreachable!("Document actions were handled above"),
                }
                return Ok(Change::View);
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.selected = self.selected.filter(|id| self.scene.contains(*id));
        Ok(Change::Scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new("test:first".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), first)).unwrap();
        let origin = editor.scene.objects()[0].origin().unwrap().id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new("test:second".into()));
        editor.apply(Action::ImportAt(bytes, second)).unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(saved.objects()[0].origin().unwrap().id, origin);
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some()); assert!(copy.upgrade().is_none());
        assert!(editor.scene.objects()[0].geometry().is_some());
        assert!(editor.scene.objects()[3].geometry().is_none());
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err()); assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:epoch".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(editor.apply(Action::UnloadSources), Err("Release epochs exhausted"));
        assert!(editor.scene.objects()[0].geometry().is_some());
        editor.last_release = 0; editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(2));
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
        editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap(); editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform(); let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view); assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(crate::memory::cpu(editor.scenes(), std::iter::empty()), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(&row.mesh))).collect();
        let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(row.geometry().unwrap()))).collect();
        let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document)))).collect();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
        assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
        assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(editor.scene.objects().iter().all(|row| !old.contains(&row.id)));
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
        let mut editor = Editor::default();
        editor.camera.aspect = 2.0;
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::ResetView).unwrap();
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
    }

    #[test]
    fn undo_repairs_selection_and_empty_delete_preserves_redo() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        for _ in 0..3 {
            editor.apply(Action::SelectNext).unwrap();
        }
        let box_id = editor.selected.unwrap();
        editor.apply(Action::Undo).unwrap();
        assert!(editor.selected.is_none());
        editor.apply(Action::Delete).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(box_id));
    }

    #[test]
    fn picking_a_moved_view_uses_the_editors_current_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        let point = session_rust::Point::new(0.2, 0.3, 0.75);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert_eq!(editor.apply(Action::Pick([screen[0] as f32, screen[1] as f32])).unwrap(), Change::Scene);
        assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
        assert_eq!(editor.apply(Action::Zoom(2.0)).unwrap(), Change::View);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/32gib-move-01.rs"
```

```text
--8<-- "journey/code/32gib-move-02.rs"
```

</details>

### 2. `src/lib.rs`

Register the original-target Move checks.

<details>
<summary>Locate the existing block</summary>

```rust
mod edit_scope_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gib-move-03.rs" skip=22
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/edit_move_tests.rs`

Optional authoring fixture. Prove current placement, later selection, camera and one history transaction survive replay.

Copy this supplied file:

```text
--8<-- "journey/code/32gib-move-04.rs"
```

</details>

</details>

<a id="checkpoint-32giba-delete"></a>

### Step 17 of 26 · Delete the original target while keeping later selection (`32giba-delete`)

- **Goal:** Delete a captured target by identity without redirecting the command to a later selection.
- **Data Flow Trace:** `Captured ObjectId → current editable row → one history removal → preserve other selection`
- **Files You Touch:** `src/editor.rs` (*Remove the original target once and clear only its own selection.*) · `src/lib.rs` (*Register the captured-target Delete checks.*) · `src/edit_delete_tests.rs` (*Optional authoring fixture. Check delayed selection, one Undo and refusal without losing Redo.*)

Extract Editor::delete_object with an explicit ObjectId. Look up the current row and require editable geometry before creating history. A missing or cold target returns an error and preserves Redo. Normal Delete with no selection changes only the view.

History::edit records one removal. Clear selection only if it still names the deleted target; otherwise keep the newer selection. The camera is untouched.

The immutable borrow of row is last used in the geometry guard. Rust ends that borrow before History mutably borrows scene. Do not take selection to choose the replay target: taking it would clear a later selection before the command succeeds.

This lesson adds the native Delete operation. Automatic asynchronous replay remains a later lesson.

> **Self-Check Question:** *Why does a delayed Delete compare selection with its captured target before clearing it?*  
> **Answer:** The user may have selected another row while loading the target. The captured identity decides what to delete; current selection is cleared only when that exact row disappears.

- **What You Should See:** Run the captured-Delete checks below. Later selection must not change the deleted target. One Undo restores the captured object.
- **Hands-On Lab Experiment:** Trace deletion with another row selected in the supplied fixture. Explain why unconditional selection clearing would discard that later interaction.

<details>
<summary>Implementation 32giba-delete: Delete the original target while keeping later selection</summary>

Delete a captured target by identity without redirecting the command to a later selection.

Data flow: Captured ObjectId → current editable row → one history removal → preserve other selection.

### 1. `src/editor.rs`

Remove the original target once and clear only its own selection. Use the same helper for ordinary Delete and preserve history when nothing is selected.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => self.selected = self.scene.next(self.selected),
            Action::Delete => {
                if let Some(id) = self.selected {
                    let row = self.scene.objects().iter().find(|row| row.id == id).ok_or("Object not found")?;
                    if row.geometry().is_none() { return Err("Reload editable sources before Delete"); }
                }
                if let Some(id) = self.selected.take() {
                    self.history.edit(&mut self.scene, |scene| { scene.remove(id); });
                }
            }
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
```

</details>

Replace it with:

```rust
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => self.selected = self.scene.next(self.selected),
            Action::Delete => {
                return match self.selected {
                    Some(id) => self.delete_object(id),
                    None => Ok(Change::View),
                };
            }
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
```

<details>
<summary>Locate this block</summary>

```rust
        Ok(Change::Scene)
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
```

</details>

Replace it with:

```rust
        Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        let row = self.scene.objects().iter().find(|row| row.id == id).ok_or("Object not found")?;
        if row.geometry().is_none() { return Err("Reload editable sources before Delete"); }
        self.history.edit(&mut self.scene, |scene| { scene.remove(id); });
        if self.selected == Some(id) { self.selected = None; }
        Ok(Change::Scene)
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    Pick([f32; 2]),
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    ReplaceAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    Close,
    UnloadSources,
    AddBox,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
}

pub struct Editor {
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub camera: Camera,
    pub background: Background,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scene: Scene::demo(),
            selected: None,
            camera: Camera::default(),
            background: Background::default(),
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene.objects().iter().filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id)).collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(&mut self, values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>) -> Result<bool, &'static str> {
        if values.is_empty() { return Ok(false); }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id) || !self.scene.objects().iter().any(|row| key.matches(row)) {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values.into_iter().map(|(key, bytes)|
            crate::rehydrate::PreparedReload::new(key, &bytes)).collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self.scenes().flat_map(|scene| scene.objects()).filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row)) { return Err("Reloaded source GUID or metadata does not match"); }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared { scene.hydrate(plan); }
        }
        Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self.scenes().flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload()).filter_map(|row| row.origin().map(|origin| origin.id)).collect();
        for row in self.scenes().flat_map(|scene| scene.objects()) {
            if let Some(origin) = row.origin() {
                if !row.can_unload() { origins.remove(&origin.id); }
            }
        }
        if origins.is_empty() { return Err("No eligible imported sources to unload"); }
        let epoch = self.last_release.checked_add(1).ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() { scene.unload(&origins, epoch); }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        let object = self.scene.objects().iter().find(|row| row.id == id).ok_or("Object not found")?;
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        let model = &shift * &object.model;
        self.history.try_edit(&mut self.scene, |scene| scene.place(id, model))?;
        Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        let row = self.scene.objects().iter().find(|row| row.id == id).ok_or("Object not found")?;
        if row.geometry().is_none() { return Err("Reload editable sources before Delete"); }
        self.history.edit(&mut self.scene, |scene| { scene.remove(id); });
        if self.selected == Some(id) { self.selected = None; }
        Ok(Change::Scene)
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
                self.selected = self.camera.ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.selected = None;
            }
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => {
                let id = self.selected.ok_or("Select an object before Move")?;
                return self.move_object(id, offset);
            }
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => self.selected = self.scene.next(self.selected),
            Action::Delete => {
                return match self.selected {
                    Some(id) => self.delete_object(id),
                    None => Ok(Change::View),
                };
            }
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.scene.bounds() {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.selected
                            .and_then(|id| self.scene.selected_bounds(id))
                        {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::ResetView => {
                        let aspect = self.camera.aspect;
                        self.camera = Camera { aspect, ..Camera::default() };
                    }
                    _ => unreachable!("Document actions were handled above"),
                }
                return Ok(Change::View);
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.selected = self.selected.filter(|id| self.scene.contains(*id));
        Ok(Change::Scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new("test:first".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), first)).unwrap();
        let origin = editor.scene.objects()[0].origin().unwrap().id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new("test:second".into()));
        editor.apply(Action::ImportAt(bytes, second)).unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(saved.objects()[0].origin().unwrap().id, origin);
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some()); assert!(copy.upgrade().is_none());
        assert!(editor.scene.objects()[0].geometry().is_some());
        assert!(editor.scene.objects()[3].geometry().is_none());
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err()); assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:epoch".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(editor.apply(Action::UnloadSources), Err("Release epochs exhausted"));
        assert!(editor.scene.objects()[0].geometry().is_some());
        editor.last_release = 0; editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(2));
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
        editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap(); editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform(); let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view); assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(crate::memory::cpu(editor.scenes(), std::iter::empty()), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(&row.mesh))).collect();
        let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(row.geometry().unwrap()))).collect();
        let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document)))).collect();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
        assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
        assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(editor.scene.objects().iter().all(|row| !old.contains(&row.id)));
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
        let mut editor = Editor::default();
        editor.camera.aspect = 2.0;
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::ResetView).unwrap();
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
    }

    #[test]
    fn undo_repairs_selection_and_empty_delete_preserves_redo() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        for _ in 0..3 {
            editor.apply(Action::SelectNext).unwrap();
        }
        let box_id = editor.selected.unwrap();
        editor.apply(Action::Undo).unwrap();
        assert!(editor.selected.is_none());
        editor.apply(Action::Delete).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(box_id));
    }

    #[test]
    fn picking_a_moved_view_uses_the_editors_current_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        let point = session_rust::Point::new(0.2, 0.3, 0.75);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert_eq!(editor.apply(Action::Pick([screen[0] as f32, screen[1] as f32])).unwrap(), Change::Scene);
        assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
        assert_eq!(editor.apply(Action::Zoom(2.0)).unwrap(), Change::View);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/32giba-delete-01.rs"
```

```text
--8<-- "journey/code/32giba-delete-02.rs"
```

</details>

### 2. `src/lib.rs`

Register the captured-target Delete checks.

<details>
<summary>Locate the existing block</summary>

```rust
mod edit_move_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32giba-delete-03.rs" skip=21
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/edit_delete_tests.rs`

Optional authoring fixture. Check delayed selection, one Undo and refusal without losing Redo.

Copy this supplied file:

```text
--8<-- "journey/code/32giba-delete-04.rs"
```

</details>

</details>

<a id="checkpoint-32gibb-reply"></a>

### Step 18 of 26 · Route captured Move, Delete and Save results (`32gibb-reply`)

- **Goal:** Return a scene change or original-precision save bytes from a captured intent.
- **Data Flow Trace:** `Intent → explicit-target edit or original source snapshot → Reply::Changed or Reply::Saved`
- **Files You Touch:** `src/edit_replay.rs` (*Return changed-state feedback or save bytes without confusing a download with a history edit.*) · `src/lib.rs` (*Register the captured-intent result module.*)

Add edit_replay as a separate module. A second impl Intent block adds replay without changing capture or scope discovery. Reply carries either a Change or saved bytes, letting the future browser completion handler decide whether to update the renderer or start a download.

Move and Delete call the explicit-target editor methods already proved. Save calls document::snapshot on the current scene; that function reads original kernel doubles and current placements. It neither changes selection/camera nor adds history. Result::map wraps successful values while preserving existing errors. The enum is a Rust tagged choice: matching Saved exposes bytes; Changed exposes a renderer change.

Replay itself does not authorize an asynchronous completion. The next lesson pairs the intent with its pending ticket; only a current completion may take and execute it.

> **Self-Check Question:** *Why must Save return bytes without adding a document history entry?*  
> **Answer:** Saving observes the current document. Undo must still undo the last modeling operation; returning original-source bytes preserves precision without turning a download into an edit.

- **What You Should See:** Run the reply checks below. Move/Delete return a scene change; Save returns original-source bytes without adding an Undo step.
- **Hands-On Lab Experiment:** Trace Save through the supplied history fixture. Explain why calling History::edit would add an unwanted Undo step.

<details>
<summary>Implementation 32gibb-reply: Route captured Move, Delete and Save results</summary>

Return a scene change or original-precision save bytes from a captured intent.

Data flow: Intent → explicit-target edit or original source snapshot → Reply::Changed or Reply::Saved.

### 1. `src/edit_replay.rs`

Return changed-state feedback or save bytes without confusing a download with a history edit.

Create the file and type:

```rust
--8<-- "journey/code/32gibb-reply-01.rs"
```

### 2. `src/lib.rs`

Register the captured-intent result module.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod edit_intent;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gibb-reply-02.rs" skip=21
```

</details>

<a id="checkpoint-32gic-owner"></a>

### Step 19 of 26 · Own the intent with its pending source ticket (`32gic-owner`)

- **Goal:** Store source keys and captured intent in one pending owner that current completion can consume once.
- **Data Flow Trace:** `ReloadJob<Intent> → Pending { ticket, keys, payload } → current finish_with takes both once`
- **Files You Touch:** `src/reload_job.rs` (*Keep the intent and source keys inside the same ticket-owned value.*) · `src/lib.rs` (*Register native ownership checks without exposing test code to the browser.*) · `src/edit_owner_tests.rs` (*Optional authoring fixture. Prove stale/duplicate/cancelled replies cannot consume an intent or retain its source metadata.*)

Generalize ReloadJob with a payload type P. Pending stores that payload beside its ticket and keys; begin_with installs them together, finish_with returns them together once, and cancel drops them together. The request sent to a future still owns only URL strings and a ticket.

P is a Rust type parameter: ReloadJob<Intent> owns a captured modeling request, while ReloadJob<()> carries the unit value, which means no additional data. The default type keeps existing explicit Reload Sources callers unchanged. A manual Default implementation starts empty without requiring Intent to implement Default.

The normal begin/finish methods are thin wrappers for the unit specialization. Existing cancellation, ticket exhaustion, duplicate-origin and browser fetch checks still run. New native checks prove an old reply cannot consume the newer Save, the current reply returns its exact intent once, and cancellation drops pending metadata without retaining a kernel document.

A matching ticket establishes current request ownership, not document validity. The next browser lesson must still hydrate with the captured keys and reject changed origin/epoch context before replay. Success and failure both consume only their own current ticket; a stale failure must never abort newer work.

> **Self-Check Question:** *Why does a stale completion check the current ticket before taking the pending value?*  
> **Answer:** Taking first would destroy a newer request. Checking first preserves newer keys and intent; only the matching current completion can consume the pair. Cancellation drops the whole pending owner.

- **What You Should See:** Run the owner checks below. A pending batch keeps its keys and captured intent together; cancellation and one-shot delivery must release that ownership.
- **Hands-On Lab Experiment:** Move pending.take before the ticket comparison in finish_with. Run the stale completion test and explain which newer request was lost.

<details>
<summary>Implementation 32gic-owner: Own the intent with its pending source ticket</summary>

Store source keys and captured intent in one pending owner that current completion can consume once.

Data flow: ReloadJob<Intent> → Pending { ticket, keys, payload } → current finish_with takes both once.

### 1. `src/reload_job.rs`

Keep the intent and source keys inside the same ticket-owned value. Allow any payload without requiring that payload to have a default value. Capture the requested operation when beginning its scoped source request. Install keys and payload together after request validation succeeds. Take the current payload once; stale replies leave newer work untouched. Keep explicit Reload Sources using a unit payload and its existing interface.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        }
        let ticket = self.issued.checked_add(1).ok_or("Reload tickets exhausted; reload the viewer")?;
        self.issued = ticket;
        self.pending = Some(Pending { ticket, keys });
        Ok(Request { ticket, urls })
    }

    pub fn pending(&self) -> Option<u64> { self.pending.as_ref().map(|pending| pending.ticket) }

    pub fn finish(&mut self, ticket: u64) -> Option<Vec<ReloadKey>> {
        if self.pending() != Some(ticket) { return None; }
        self.pending.take().map(|pending| pending.keys)
    }

    pub fn cancel(&mut self) -> bool { self.pending.take().is_some() }
}

#[cfg(test)]
mod tests {
    use super::*;
```

</details>

Replace it with:

```rust
        }
        let ticket = self.issued.checked_add(1).ok_or("Reload tickets exhausted; reload the viewer")?;
        self.issued = ticket;
        self.pending = Some(Pending { ticket, keys, payload });
        Ok(Request { ticket, urls })
    }

    pub fn pending(&self) -> Option<u64> { self.pending.as_ref().map(|pending| pending.ticket) }

    pub fn finish_with(&mut self, ticket: u64) -> Option<(Vec<ReloadKey>, P)> {
        if self.pending() != Some(ticket) { return None; }
        self.pending.take().map(|pending| (pending.keys, pending.payload))
    }

    pub fn cancel(&mut self) -> bool { self.pending.take().is_some() }
}

impl ReloadJob<()> {
    pub fn begin(&mut self, keys: Vec<ReloadKey>) -> Result<Request, &'static str> {
        self.begin_with(keys, ())
    }
    pub fn finish(&mut self, ticket: u64) -> Option<Vec<ReloadKey>> {
        self.finish_with(ticket).map(|(keys, ())| keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
```

<details>
<summary>Locate this block</summary>

```rust
    pub urls: Vec<String>,
}

struct Pending {
    ticket: u64,
    keys: Vec<ReloadKey>,
}

#[derive(Default)]
pub struct ReloadJob {
    issued: u64,
    pending: Option<Pending>,
}

impl ReloadJob {
    pub fn begin(&mut self, keys: Vec<ReloadKey>) -> Result<Request, &'static str> {
        self.cancel();
        if keys.is_empty() { return Err("No released sources to reload"); }
        let mut origins = std::collections::HashSet::new();
```

</details>

Replace it with:

```rust
    pub urls: Vec<String>,
}

struct Pending<P> {
    ticket: u64,
    keys: Vec<ReloadKey>,
    payload: P,
}

pub struct ReloadJob<P = ()> {
    issued: u64,
    pending: Option<Pending<P>>,
}

impl<P> Default for ReloadJob<P> {
    fn default() -> Self { Self { issued: 0, pending: None } }
}

impl<P> ReloadJob<P> {
    pub fn begin_with(&mut self, keys: Vec<ReloadKey>, payload: P) -> Result<Request, &'static str> {
        self.cancel();
        if keys.is_empty() { return Err("No released sources to reload"); }
        let mut origins = std::collections::HashSet::new();
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::rehydrate::ReloadKey;

pub struct Request {
    pub ticket: u64,
    pub urls: Vec<String>,
}

struct Pending<P> {
    ticket: u64,
    keys: Vec<ReloadKey>,
    payload: P,
}

pub struct ReloadJob<P = ()> {
    issued: u64,
    pending: Option<Pending<P>>,
}

impl<P> Default for ReloadJob<P> {
    fn default() -> Self { Self { issued: 0, pending: None } }
}

impl<P> ReloadJob<P> {
    pub fn begin_with(&mut self, keys: Vec<ReloadKey>, payload: P) -> Result<Request, &'static str> {
        self.cancel();
        if keys.is_empty() { return Err("No released sources to reload"); }
        let mut origins = std::collections::HashSet::new();
        let mut urls = Vec::new();
        for key in &keys {
            if !origins.insert(key.origin.id) { return Err("Duplicate reload origin"); }
            let location = key.origin.location.as_ref().ok_or("Source has no reload location")?;
            urls.push(location.value().to_owned());
        }
        let ticket = self.issued.checked_add(1).ok_or("Reload tickets exhausted; reload the viewer")?;
        self.issued = ticket;
        self.pending = Some(Pending { ticket, keys, payload });
        Ok(Request { ticket, urls })
    }

    pub fn pending(&self) -> Option<u64> { self.pending.as_ref().map(|pending| pending.ticket) }

    pub fn finish_with(&mut self, ticket: u64) -> Option<(Vec<ReloadKey>, P)> {
        if self.pending() != Some(ticket) { return None; }
        self.pending.take().map(|pending| (pending.keys, pending.payload))
    }

    pub fn cancel(&mut self) -> bool { self.pending.take().is_some() }
}

impl ReloadJob<()> {
    pub fn begin(&mut self, keys: Vec<ReloadKey>) -> Result<Request, &'static str> {
        self.begin_with(keys, ())
    }
    pub fn finish(&mut self, ticket: u64) -> Option<Vec<ReloadKey>> {
        self.finish_with(ticket).map(|(keys, ())| keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::{Action, Editor}, reload_url::ReloadUrl};
    use std::{cell::RefCell, rc::Rc};

    fn cold() -> (Editor, Vec<ReloadKey>) {
        let mut editor = Editor::default();
        let location = Rc::new(ReloadUrl::new("test:request".into()));
        editor.apply(Action::ReplaceAt(crate::specimen::bytes(), location)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        let keys = editor.reload_keys(); (editor, keys)
    }

    #[test]
    fn only_current_work_can_finish_and_it_finishes_once() {
        let (_, keys) = cold(); let mut job = ReloadJob::default();
        let first = job.begin(keys.clone()).unwrap();
        let second = job.begin(keys.clone()).unwrap();
        assert!(second.ticket > first.ticket);
        assert!(job.finish(first.ticket).is_none()); assert_eq!(job.pending(), Some(second.ticket));
        let accepted = job.finish(second.ticket).unwrap(); assert_eq!(accepted.len(), 1);
        assert!(Rc::ptr_eq(&accepted[0].origin, &keys[0].origin));
        assert!(job.finish(second.ticket).is_none());
        let third = job.begin(keys.clone()).unwrap(); assert!(job.cancel());
        assert!(!job.cancel()); assert!(job.finish(third.ticket).is_none());
        assert!(job.begin(vec![keys[0].clone(), keys[0].clone()]).is_err());
        assert!(job.begin(Vec::new()).is_err()); assert_eq!(job.pending(), None);
    }

    #[test]
    fn cancellation_drops_keys_even_when_abandoned_work_holds_the_job() {
        let (mut editor, keys) = cold();
        let origin = Rc::downgrade(&keys[0].origin);
        let location = Rc::downgrade(keys[0].origin.location.as_ref().unwrap());
        let job = Rc::new(RefCell::new(ReloadJob::default()));
        let request = job.borrow_mut().begin(keys).unwrap();
        let abandoned = Rc::clone(&job);
        editor.apply(Action::Close).unwrap(); assert!(origin.upgrade().is_some());
        assert!(job.borrow_mut().cancel());
        assert!(origin.upgrade().is_none() && location.upgrade().is_none());
        assert_eq!(request.urls, vec!["test:request".to_owned()]);
        assert!(abandoned.borrow_mut().finish(request.ticket).is_none());
    }

    #[test]
    fn exhaustion_cancels_old_keys_and_never_wraps() {
        let (_, keys) = cold(); let mut job = ReloadJob::default();
        job.begin(keys.clone()).unwrap(); job.issued = u64::MAX;
        assert!(job.begin(keys).is_err()); assert_eq!(job.pending(), None);
        assert_eq!(job.issued, u64::MAX);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/32gic-owner-01.rs"
```

```text
--8<-- "journey/code/32gic-owner-02.rs"
```

```text
--8<-- "journey/code/32gic-owner-03.rs"
```

```text
--8<-- "journey/code/32gic-owner-04.rs"
```

```text
--8<-- "journey/code/32gic-owner-05.rs"
```

```text
--8<-- "journey/code/32gic-owner-06.rs"
```

</details>

### 2. `src/lib.rs`

Register native ownership checks without exposing test code to the browser.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod edit_replay;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gic-owner-07.rs" skip=21
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/edit_owner_tests.rs`

Optional authoring fixture. Prove stale/duplicate/cancelled replies cannot consume an intent or retain its source metadata.

Copy this supplied file:

```text
--8<-- "journey/code/32gic-owner-08.rs"
```

</details>

</details>

<a id="checkpoint-32gica-reply"></a>

### Step 20 of 26 · Pair captured intent with complete source bodies (`32gica-reply`)

- **Goal:** Build a completion value that retains captured intent and rejects incomplete source-body pairings.
- **Data Flow Trace:** `current ticket → keys + Intent → equal body count → complete pairs or error`
- **Files You Touch:** `src/reload_reply.rs` (*Keep the captured operation beside either its complete source bodies or its failure.*) · `src/lib.rs` (*Register the completion value for native checks and browser delivery.*) · `src/reload_reply_tests.rs` (*Optional authoring fixture. Check all captured operations, exact bodies, failures, partial counts and the explicit no-operation path.*)

A completion must keep its captured operation beside every returned source body, or beside its failure. Reply stores an optional Intent and one Result. None describes explicit Reload Sources; Some describes captured Move, Delete or Save.

Before pairing bodies and keys, the constructor checks equal lengths. zip stops when either iterator ends; without this check a missing or extra body could silently become a partial restoration. A failure retains the operation and creates no source-body pairs.

This checkpoint does not change the browser flight or connect automatic commands. The next lesson gives the browser this completion value.

> **Self-Check Question:** *Why must body-count validation happen before pairing bodies with source keys?*  
> **Answer:** zip stops at the shorter vector. Without checking equal lengths first, a partial response could look like a complete restoration request. Failure keeps the operation but exposes no partial body pairs.

- **What You Should See:** Run the completed-body checks below. The reply must contain every requested body paired with its key and retain the captured intent until adoption.
- **Hands-On Lab Experiment:** Remove the body-count check and inspect the missing/extra body tests. Explain why zip alone cannot establish a complete response.

<details>
<summary>Implementation 32gica-reply: Pair captured intent with complete source bodies</summary>

Build a completion value that retains captured intent and rejects incomplete source-body pairings.

Data flow: current ticket → keys + Intent → equal body count → complete pairs or error.

### 1. `src/reload_reply.rs`

Keep the captured operation beside either its complete source bodies or its failure.

Create the file and type:

```rust
--8<-- "journey/code/32gica-reply-01.rs"
```

### 2. `src/lib.rs`

Register the completion value for native checks and browser delivery.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod edit_replay;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gica-reply-02.rs" skip=21
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/reload_reply_tests.rs`

Optional authoring fixture. Check all captured operations, exact bodies, failures, partial counts and the explicit no-operation path.

Copy this supplied file:

```text
--8<-- "journey/code/32gica-reply-03.rs"
```

</details>

</details>

<a id="checkpoint-32gicb-bridge"></a>

### Step 21 of 26 · Carry captured intent through browser completion (`32gicb-bridge`)

- **Goal:** Carry optional captured intent through the abortable flight and deliver it beside complete source bodies or a current failure.
- **Data Flow Trace:** `Flight → ReloadJob<Option<Intent>> → current finish_with → Reply → synchronous browser mailbox`
- **Files You Touch:** `src/browser_reload.rs` (*Use the same completion value in the browser mailbox.*) · `src/browser.rs` (*Keep the existing explicit source-restoration receiver reading the reply body.*)

The browser flight now uses ReloadJob<Option<Intent>>. None means explicit Reload Sources; Some carries the captured operation that an automatic command will request next. start keeps its existing interface and calls start_with using None.

The async future owns URL strings, an abort signal and a ticket. Current completion takes keys and optional intent together, constructs the previously taught Reply and delivers it through the synchronous browser mailbox. A stale reply returns before touching a newer flight or its controller; cancellation retains the existing abort path.

The existing receiver reads reply.result and restores sources. Automatic commands will call start_with and replay reply.intent only after document-context validation in the next lesson. This checkpoint wires the completion boundary, not automatic replay.

> **Self-Check Question:** *Why does the asynchronous future retain a ticket instead of owning the pending intent and keys?*  
> **Answer:** The flight can cancel and release pending ownership while a delayed future remains alive. Only a matching current ticket can take the intent and keys; a stale completion cannot disturb newer work.

- **What You Should See:** Build the browser bundle and run the checks below. Accepted delivery now carries the intent; an aborted older flight cannot deliver it. Automatic commands are not routed yet.
- **Hands-On Lab Experiment:** Return the intent from the future rather than finish_with. Explain why cancelling the pending owner would no longer release that future’s captured request.

<details>
<summary>Implementation 32gicb-bridge: Carry captured intent through browser completion</summary>

Carry optional captured intent through the abortable flight and deliver it beside complete source bodies or a current failure.

Data flow: Flight → ReloadJob<Option<Intent>> → current finish_with → Reply → synchronous browser mailbox.

### 1. `src/browser_reload.rs`

Use the same completion value in the browser mailbox. Preserve explicit Reload Sources while allowing a captured operation to accompany its keys. Only a current ticket may collect its captured operation and deliver the complete reply. Let the existing abortable flight own an optional operation. Capture the operation when starting the abortable source flight. Install the optional operation and keys into one ticket owner. Return both keys and operation only after checking completion ownership.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{rehydrate::ReloadKey, reload_job::{ReloadJob, Request}};
use std::{cell::RefCell, rc::Rc};

pub type Shared = Rc<RefCell<Flight>>;

pub type Reply = Result<Vec<(ReloadKey, Vec<u8>)>, String>;
pub type Delivery = Rc<RefCell<Option<Reply>>>;

pub fn start(shared: Shared, keys: Vec<ReloadKey>, delivery: Delivery) -> Result<(), String> {
    let (request, signal) = shared.borrow_mut().begin(keys)?;
    crate::browser::report("Reloading editable sources…");
    wasm_bindgen_futures::spawn_local(async move {
        let result: Result<Vec<Vec<u8>>, String> = async {
            let mut values = Vec::new();
            for url in request.urls { values.push(crate::source_fetch::fetch(&url, &signal).await?); }
            Ok(values)
        }.await;
        let Some(keys) = shared.borrow_mut().finish(request.ticket, result.is_err()) else { return; };
        let result = result.map(|values| keys.into_iter().zip(values).collect());
        if let Err(error) = deliver(result, &delivery) {
            crate::browser::report(&format!("Cannot deliver reload: {error:?}"));
        }
    });
    Ok(())
}

fn deliver(result: Reply, delivery: &Delivery) -> Result<(), wasm_bindgen::JsValue> {
    let event = web_sys::Event::new("viewer-reload")?;
    *delivery.borrow_mut() = Some(result);
    let result = web_sys::window().ok_or_else(|| wasm_bindgen::JsValue::from_str("No browser window"))
        .and_then(|window| window.dispatch_event(&event));
    delivery.borrow_mut().take();
    result.map(|_| ())
}

#[derive(Default)]
pub struct Flight {
    job: ReloadJob,
    controller: Option<web_sys::AbortController>,
}

impl Flight {
    pub fn cancel(&mut self) -> bool {
        let cancelled = self.job.cancel();
        if let Some(controller) = self.controller.take() { controller.abort(); }
        cancelled
    }

    fn begin(&mut self, keys: Vec<ReloadKey>) -> Result<(Request, web_sys::AbortSignal), String> {
        self.cancel();
        let controller = web_sys::AbortController::new().map_err(|error| format!("Cannot start reload: {error:?}"))?;
        let request = self.job.begin(keys)?;
        let signal = controller.signal();
        self.controller = Some(controller);
        Ok((request, signal))
    }

    fn finish(&mut self, ticket: u64, failed: bool) -> Option<Vec<ReloadKey>> {
        let keys = self.job.finish(ticket)?;
        if let Some(controller) = self.controller.take() { if failed { controller.abort(); } }
        Some(keys)
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{rehydrate::ReloadKey, reload_job::{ReloadJob, Request}};
use std::{cell::RefCell, rc::Rc};

pub type Shared = Rc<RefCell<Flight>>;

pub type Reply = crate::reload_reply::Reply;
pub type Delivery = Rc<RefCell<Option<Reply>>>;

pub fn start(shared: Shared, keys: Vec<ReloadKey>, delivery: Delivery) -> Result<(), String> {
    start_with(shared, keys, None, delivery)
}

pub fn start_with(shared: Shared, keys: Vec<ReloadKey>, intent: Option<crate::edit_intent::Intent>, delivery: Delivery) -> Result<(), String> {
    let (request, signal) = shared.borrow_mut().begin(keys, intent)?;
    crate::browser::report("Reloading editable sources…");
    wasm_bindgen_futures::spawn_local(async move {
        let result: Result<Vec<Vec<u8>>, String> = async {
            let mut values = Vec::new();
            for url in request.urls { values.push(crate::source_fetch::fetch(&url, &signal).await?); }
            Ok(values)
        }.await;
        let Some((keys, intent)) = shared.borrow_mut().finish(request.ticket, result.is_err()) else { return; };
        let reply = Reply::new(keys, intent, result);
        if let Err(error) = deliver(reply, &delivery) {
            crate::browser::report(&format!("Cannot deliver reload: {error:?}"));
        }
    });
    Ok(())
}

fn deliver(result: Reply, delivery: &Delivery) -> Result<(), wasm_bindgen::JsValue> {
    let event = web_sys::Event::new("viewer-reload")?;
    *delivery.borrow_mut() = Some(result);
    let result = web_sys::window().ok_or_else(|| wasm_bindgen::JsValue::from_str("No browser window"))
        .and_then(|window| window.dispatch_event(&event));
    delivery.borrow_mut().take();
    result.map(|_| ())
}

#[derive(Default)]
pub struct Flight {
    job: ReloadJob<Option<crate::edit_intent::Intent>>,
    controller: Option<web_sys::AbortController>,
}

impl Flight {
    pub fn cancel(&mut self) -> bool {
        let cancelled = self.job.cancel();
        if let Some(controller) = self.controller.take() { controller.abort(); }
        cancelled
    }

    fn begin(&mut self, keys: Vec<ReloadKey>, intent: Option<crate::edit_intent::Intent>) -> Result<(Request, web_sys::AbortSignal), String> {
        self.cancel();
        let controller = web_sys::AbortController::new().map_err(|error| format!("Cannot start reload: {error:?}"))?;
        let request = self.job.begin_with(keys, intent)?;
        let signal = controller.signal();
        self.controller = Some(controller);
        Ok((request, signal))
    }

    fn finish(&mut self, ticket: u64, failed: bool) -> Option<(Vec<ReloadKey>, Option<crate::edit_intent::Intent>)> {
        let keys = self.job.finish_with(ticket)?;
        if let Some(controller) = self.controller.take() { if failed { controller.abort(); } }
        Some(keys)
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/32gicb-bridge-01.rs"
```

```text
--8<-- "journey/code/32gicb-bridge-02.rs"
```

```text
--8<-- "journey/code/32gicb-bridge-03.rs"
```

```text
--8<-- "journey/code/32gicb-bridge-04.rs"
```

```text
--8<-- "journey/code/32gicb-bridge-05.rs"
```

```text
--8<-- "journey/code/32gicb-bridge-06.rs"
```

```text
--8<-- "journey/code/32gicb-bridge-07.rs"
```

</details>

### 2. `src/browser.rs`

Keep the existing explicit source-restoration receiver reading the reply body.

<details>
<summary>Locate the existing block</summary>

```rust
            if let Some(result) = reload_delivery.borrow_mut().take() {
                match result.and_then(|values| editor.hydrate(values).map_err(str::to_owned)) {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            if let Some(reply) = reload_delivery.borrow_mut().take() {
                match reply.result.and_then(|values| editor.hydrate(values).map_err(str::to_owned)) {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/32gicb-bridge-08.rs"
```

</details>

</details>

<a id="checkpoint-32gid-complete"></a>

### Step 22 of 26 · Validate restoration before replaying the command (`32gid-complete`)

- **Goal:** Complete source restoration and replay the captured operation only after current document validation.
- **Data Flow Trace:** `Reply → complete bodies → current keys and versions → hydrate → captured Intent → edit result`
- **Files You Touch:** `src/reload_reply.rs` (*Validate every current source key and body before replay; a stale context produces no reply.*) · `src/lib.rs` (*Register native completion checks separately from the body-pairing checks.*) · `src/reload_complete_tests.rs` (*Optional authoring fixture. Prove automatic native Move/Delete/Save completion, original target, later selection/camera, Undo, and stale or failed completion.*)

Restore the required sources before replaying a captured command. A failed or stale reply must perform no edit or download.

The reply already pairs complete bodies with the operation captured before fetching. complete now gives it one synchronous editor boundary. Hydration validates current source identity, release epoch, file version and metadata before it restores any source. A stale context returns None; a failed fetch or invalid body returns an error. Neither path replays the operation.

After successful hydration, Some(intent) calls the explicit-target replay taught earlier. Move and Delete each create one edit transaction; Save produces the original-precision document bytes without creating history. None represents explicit Reload Sources and returns a scene result. The browser will consume these results in the next checkpoint.

> **Self-Check Question:** *Why does a stale restoration return None instead of replaying its captured Save or edit?*  
> **Answer:** The source keys no longer match the current document and release epoch. The reply has lost authority; neither a scene change nor a download belongs to the replacement context.

- **What You Should See:** Run the completion checks below. A valid batch restores sources before replaying its captured edit. Failed or stale batches must perform no edit or download.
- **Hands-On Lab Experiment:** Move intent.replay above hydrate and run the failed/stale completion checks. Explain which edits or downloads could escape document validation.

<details>
<summary>Implementation 32gid-complete: Validate restoration before replaying the command</summary>

Complete source restoration and replay the captured operation only after current document validation.

Data flow: Reply → complete bodies → current keys and versions → hydrate → captured Intent → edit result.

### 1. `src/reload_reply.rs`

Validate every current source key and body before replay; a stale context produces no reply.

<details>
<summary>Locate the existing block</summary>

```rust
impl Reply {
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gid-complete-01.rs" skip=13
```

### 2. `src/lib.rs`

Register native completion checks separately from the body-pairing checks.

<details>
<summary>Locate the existing block</summary>

```rust
mod reload_reply_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gid-complete-02.rs" skip=24
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/reload_complete_tests.rs`

Optional authoring fixture. Prove automatic native Move/Delete/Save completion, original target, later selection/camera, Undo, and stale or failed completion.

Copy this supplied file:

```text
--8<-- "journey/code/32gid-complete-03.rs"
```

</details>

</details>

<a id="checkpoint-32gie-response"></a>

### Step 23 of 26 · Deliver restored edit and Save results to the dock (`32gie-response`)

- **Goal:** Route validated scene edits and original-precision downloads back to the captured command history.
- **Data Flow Trace:** `Current completion → captured command name → complete → Changed or Saved → dock answer`
- **Files You Touch:** `src/edit_intent.rs` (*Name the captured operation independently of the current input field or selection.*) · `src/browser.rs` (*Consume validated edit or Save results and associate success or failure with the original command.*)

Connect completed source replies to drawing and Save downloads. Report success or failure under the original command name.

The browser receiver now calls complete instead of hydrating only the body. It records the command name from the captured intent before consuming the reply. Changed synchronizes the renderer; Saved downloads the bytes produced by the original-precision snapshot. A stale None does nothing. Errors use the same original command name.

Explicit Reload Sources still uses None intent and keeps its existing success message. save_result converts either the browser download failure or its success into one dock message. Actual automatic commands are connected next; this checkpoint prepares their result handling.

> **Self-Check Question:** *Why is the command name read from the reply before completion consumes it?*  
> **Answer:** The reply owns the operation submitted before fetching. Later selection and typing do not identify that operation. Completion consumes its source bodies and intent, so capture the name first.

- **What You Should See:** Build and run the viewer. Explicit `Reload Sources` still restores without an Undo step; the browser can now consume scene or Save replies. Automatic command routing follows next.
- **Hands-On Lab Experiment:** Use the current command field as the completion label. Type a camera command while a fetch is held and explain why that label would attach the result to the wrong operation.

<details>
<summary>Implementation 32gie-response: Deliver restored edit and Save results to the dock</summary>

Route validated scene edits and original-precision downloads back to the captured command history.

Data flow: Current completion → captured command name → complete → Changed or Saved → dock answer.

### 1. `src/edit_intent.rs`

Name the captured operation independently of the current input field or selection.

<details>
<summary>Locate the existing block</summary>

```rust
impl Intent {
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gie-response-01.rs" skip=14
```

### 2. `src/browser.rs`

Consume validated edit or Save results and associate success or failure with the original command. Keep download handling shared by immediate and restored Save results.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            None
        } else if event.type_() == "viewer-reload" {
            if let Some(reply) = reload_delivery.borrow_mut().take() {
                match reply.result.and_then(|values| editor.hydrate(values).map_err(str::to_owned)) {
                    Ok(true) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        let message = "Editable sources restored; display retained.";
                        report(message); panel.answer("Reload Sources", message);
                    }
                    Ok(false) => {}
                    Err(error) => { report(&error); panel.answer("Reload Sources", &error); }
                }
            }
            None
```

</details>

Replace it with:

```rust
            None
        } else if event.type_() == "viewer-reload" {
            if let Some(reply) = reload_delivery.borrow_mut().take() {
                let command = reply.intent.map_or("Reload Sources", |intent| intent.command());
                match reply.complete(&mut editor) {
                    Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        let message = if command == "Reload Sources" {
                            "Editable sources restored; display retained.".to_owned()
                        } else { format!("{command} completed after source reload.") };
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(Some(crate::edit_replay::Reply::Saved(bytes))) => {
                        let message = save_result(&bytes).unwrap_or_else(|error| error);
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(None) => {}
                    Err(error) => { report(&error); panel.answer(command, &error); }
                }
            }
            None
```

<details>
<summary>Locate this block</summary>

```rust
    }
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
```

</details>

Replace it with:

```rust
    }
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| format!("Saved editable document ({} bytes)", bytes.len()))
        .map_err(|error| format!("Save failed: {error:?}"))
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::background::Background;
use crate::editor::{Action, Change, Editor};
use crate::gesture::Gesture;
use crate::renderer::Renderer;
use crate::viewport::Viewport;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot draw:") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| format!("Saved editable document ({} bytes)", bytes.len()))
        .map_err(|error| format!("Save failed: {error:?}"))
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = Editor::default();
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    renderer.resize(Viewport { width, height });
    inspect(&editor, &renderer)?;
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &[
            "Help",
            "Open",
            "Open Replace",
            "Cancel Open",
            "Close",
            "Unload Sources",
            "Reload Sources",
            "Cancel Reload",
            "Save",
            "Example Box",
            "Example Triangle",
            "Select Next",
            "Delete",
            "Undo",
            "Redo",
            "Background",
            "Zoom In",
            "Zoom Out",
            "Pan Left",
            "Pan Right",
            "Orbit Right",
            "Orbit Up",
            "View Isometric",
            "View Perspective",
            "View Orthographic",
            "Fit",
            "Fit Selected",
            "Move",
            "View Reset",
        ],
    );
    panel.update(None, &canvas)?;
    if resize(
        &window,
        &canvas,
        &surface,
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            &mut panel,
        )?;
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
            event.prevent_default();
            panel.answer(shortcut.command(), "");
            Some(shortcut.command().to_ascii_lowercase())
        } else { match panel.update(Some(&event), &input_canvas) {
            Ok(line) => line,
            Err(error) => {
                report(&format!("Cannot read command: {error:?}"));
                return;
            }
        } };

        let action = if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
        {
            request.borrow_mut().cancel();
            report("Open cancelled.");
            panel.answer("Open", "Open cancelled.");
            None
        } else if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request), read_mode.get(), std::rc::Rc::clone(&delivery));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-reload" {
            if let Some(reply) = reload_delivery.borrow_mut().take() {
                let command = reply.intent.map_or("Reload Sources", |intent| intent.command());
                match reply.complete(&mut editor) {
                    Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        let message = if command == "Reload Sources" {
                            "Editable sources restored; display retained.".to_owned()
                        } else { format!("{command} completed after source reload.") };
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(Some(crate::edit_replay::Reply::Saved(bytes))) => {
                        let message = save_result(&bytes).unwrap_or_else(|error| error);
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(None) => {}
                    Err(error) => { report(&error); panel.answer(command, &error); }
                }
            }
            None
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery) {
                Ok(action) => action,
                Err(error) => { let message = format!("Cannot retain reload file: {error:?}");
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            if line == "reload sources" {
                let result = crate::browser_reload::start(std::rc::Rc::clone(&reload), editor.reload_keys(), std::rc::Rc::clone(&reload_delivery));
                match result {
                    Ok(()) => panel.result("Reloading editable sources…"),
                    Err(error) => { report(&error); panel.result(&error); }
                }
                None
            } else if line == "cancel reload" {
                reload.borrow_mut().cancel();
                report("Reload cancelled."); panel.result("Reload cancelled.");
                None
            } else if line == "unload sources" {
                Some(Action::UnloadSources)
            } else if line == "close" {
                request.borrow_mut().cancel();
                Some(Action::Close)
            } else if line == "cancel open" {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            } else if line == "open" || line == "open replace" {
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                if line == "open replace" { reload.borrow_mut().cancel(); }
                read_mode.set(if line == "open replace" { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
                    .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                }
                None
            } else if line == "save" {
                match crate::document::snapshot(&editor.scene) {
                    Ok(bytes) => match crate::file_output::download(&bytes) {
                        Ok(()) => panel.result(&format!("Saved editable document ({} bytes)", bytes.len())),
                        Err(error) => panel.result(&format!("Save failed: {error:?}")),
                    },
                    Err(error) => panel.result(error),
                }
                None
            } else if line == "move" || line.starts_with("move ") {
                match crate::offset::parse(&line) {
                    Ok(offset) => Some(Action::Translate(offset)),
                    Err(error) => { panel.result(error); None }
                }
            } else {
                let action = match line.as_str() {
                    "example box" => Action::AddBox,
                    "example triangle" => Action::ToggleExtra,
                    "select next" => Action::SelectNext,
                    "delete" => Action::Delete,
                    "undo" => Action::Undo,
                    "redo" => Action::Redo,
                    "background" => Action::Background,
                    "zoom in" => Action::Zoom(2.0),
                    "zoom out" => Action::Zoom(0.5),
                    "pan left" => Action::Pan(-0.25, 0.0),
                    "pan right" => Action::Pan(0.25, 0.0),
                    "orbit right" => Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0),
                    "orbit up" => Action::Orbit(0.0, std::f64::consts::FRAC_PI_6),
                    "view isometric" => Action::Isometric,
                    "view perspective" => {
                        Action::Projection(crate::camera::Projection::Perspective)
                    }
                    "view orthographic" => {
                        Action::Projection(crate::camera::Projection::Orthographic)
                    }
                    "fit" if shortcut.is_some() && editor.selected.is_some() => Action::FitSelected,
                    "fit" => Action::Fit,

                    "fit selected" => Action::FitSelected,
                    "view reset" => Action::ResetView,
                    _ => return,
                };
                Some(action)
            }
        } else if !panel.consumed {
            navigation_action(&event, &pointer_canvas, &mut gesture)
        } else {
            None
        };
        if let Some(action) = action {
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Undo | Action::Redo
                | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
            let replacement = matches!(&action, Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            match editor.apply(action) {
                Ok(Change::Scene) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if unloading {
                        report("Editable sources unloaded; display retained.");
                        panel.result("Editable sources unloaded; display retained.");
                    } else if closing {
                        report("Document closed.");
                        panel.result("Document closed.");
                    } else if event.type_() == "viewer-file" {
                        let message = if replacement { "Document replaced. Undo restores the previous document." }
                            else { "File imported. Undo removes the entire import." };
                        report(message);
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Change::View) => {}
                Err(error) => {
                    report(error);
                    if event.type_() == "viewer-file" { panel.answer("Open", error); }
                    else { panel.result(error); }
                }
            }
        }
        if let Err(error) = inspect(&editor, &renderer) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
        if resize(
            &browser_window,
            &pointer_canvas,
            &surface,
            &mut config,
            &mut renderer,
            &mut editor,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
                return;
            }
            if let Err(error) = present(
                &surface,
                &renderer,
                &editor.background,
                &editor.camera.uniform(),
                &mut panel,
            ) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        }
    });
    for name in [
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
        "lostpointercapture",
        "contextmenu",
        "keydown",
        "keyup",
        "blur",
    ] {
        canvas.add_event_listener_with_callback(name, update.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        update.as_ref().unchecked_ref(),
        &options,
    )?;
    window.add_event_listener_with_callback("resize", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("cancel", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file-error", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-reload", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}

fn inspect(editor: &Editor, renderer: &Renderer) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
        (&row.guid, &m.source_guid, &m.name, m.is_visible, m.is_locked)
    }).collect();
    let origins: Vec<_> = editor.scene.objects().iter().filter_map(|row| row.origin().map(|origin|
        (&row.guid, origin.id.to_string(), origin.version.hex(),
            origin.location.as_ref().map(|location| location.value())))).collect();
    let residency: Vec<_> = editor.scene.objects().iter().map(|row|
        (&row.guid, row.geometry().is_some(), row.release_epoch())).collect();
    canvas.set_attribute("data-source-residency", &serde_json::to_string(&residency)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-source-origins", &serde_json::to_string(&origins)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-row-metadata", &serde_json::to_string(&metadata)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-gpu-stats", &format!("{:?}", renderer.stats()))?;
    let cpu = crate::memory::cpu(editor.scenes(), renderer.geometry_owners().map(|owner| &owner.source));
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
    canvas.set_attribute("data-gpu-usage", &format!("{:?}", renderer.usage()))?;
    canvas.set_attribute("data-projection", &format!("{:?}", editor.camera.projection))
}

fn navigation_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    let action = match event.type_().as_str() {
        "wheel" => {
            let wheel = event.dyn_ref::<web_sys::WheelEvent>()?;
            let mode = wheel.delta_mode();
            crate::navigation::wheel(
                wheel.delta_y(),
                mode,
                canvas.get_bounding_client_rect().height(),
            )
        }
        "keydown" => {
            let key = event.dyn_ref::<web_sys::KeyboardEvent>()?;
            if key.is_composing() || key.alt_key() {
                return None;
            }
            if key.key() == "Escape" {
                gesture.cancel();
                event.prevent_default();
                return None;
            }
            return None;
        }
        _ => return pointer_action(event, canvas, gesture),
    }?;
    event.prevent_default();
    Some(action)
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    match event.type_().as_str() {
        "contextmenu" => {
            event.prevent_default();
            return None;
        }
        "blur" | "resize" => {
            gesture.cancel();
            return None;
        }
        _ => {}
    }
    let pointer = event.dyn_ref::<web_sys::PointerEvent>()?;
    let id = pointer.pointer_id();
    let position = [pointer.client_x() as f64, pointer.client_y() as f64];
    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary() && gesture.press(id, pointer.button(), position) {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err()
                    || canvas.set_pointer_capture(id).is_err()
                {
                    gesture.cancel();
                }
            }
            None
        }
        "pointermove" => gesture.move_to(id, position),
        "pointerup" => {
            let motion = gesture.release(id, position);
            let _ = canvas.release_pointer_capture(id);
            motion
        }
        "pointercancel" | "lostpointercapture" => {
            gesture.cancel_pointer(id);
            None
        }
        _ => None,
    }?;
    let rect = canvas.get_bounding_client_rect();
    motion.action([rect.left(), rect.top(), rect.width(), rect.height()])
}

fn resize(
    window: &web_sys::Window,
    canvas: &web_sys::HtmlCanvasElement,
    surface: &wgpu::Surface<'_>,
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        window.device_pixel_ratio(),
        limit,
    ) else {
        return false;
    };
    if size.width != config.width || size.height != config.height {
        canvas.set_width(size.width);
        canvas.set_height(size.height);
        config.width = size.width;
        config.height = size.height;
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    editor.camera.aspect = size.aspect();
    true
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 16],
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => {
            return Err(JsValue::from_str(&format!(
                "Surface unavailable: {other:?}"
            )));
        }
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view, background, transform);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/32gie-response-02.rs"
```

```text
--8<-- "journey/code/32gie-response-03.rs"
```

</details>

</details>

<a id="checkpoint-32gif-auto"></a>

### Step 24 of 26 · Automatically restore sources for Move, Delete and Save (`32gif-auto`)

- **Goal:** Connect cold-source edits and Save to captured reload and validated replay without requiring Reload Sources first.
- **Data Flow Trace:** `Typed command → capture original intent → required cold keys → fetch → validated completion → one edit or download`
- **Files You Touch:** `src/browser_reload.rs` (*A new edit supersedes older pending work. Warm edits need no fetch; cold edits capture their required keys and operation.*) · `src/browser.rs` (*Save reloads every active cold source before producing its original-precision download; warm Save remains immediate.*)

Restore only the sources required by a captured command: Move/Delete need the target; Save needs all active cold sources. Cancel older pending work, return immediately for warm work, or start the abortable flight with its intent.

`Result<Option<Change>, String>` distinguishes an immediate change, waiting, and failure. `map(Some)` wraps an immediate result; `and_then` preserves earlier errors. Waiting leaves placement, drawing and history untouched.

On completion, replay against the captured target and current placement, preserving later camera and selection changes. Save downloads only after required sources return. Zero Move or Delete without selection fetches nothing.

Close, Undo, Redo, unloading and replacement cancel work. A newer real edit or Save supersedes an earlier intent, even when the newer operation is warm.

> **Self-Check Question:** *Which commands should keep working while source restoration is pending?*  
> **Answer:** Camera and selection commands execute immediately. The pending operation retains its original target and arguments. A new real edit or Save supersedes earlier pending work; Close, Undo, Redo, Unload Sources and replacement already cancel its authority.

- **What You Should See:** Open `sample.pb`, select an object and type `Unload Sources`, then `Move 0.25,0,0.15`. Move restores its source automatically and runs once. `Undo` returns the object to its previous placement.
- **Hands-On Lab Experiment:** Hold a source response, submit Move, select another object and orbit, then release it. Explain why replay must use the captured object identity but preserve the later selection and camera.

<details>
<summary>Implementation 32gif-auto: Automatically restore sources for Move, Delete and Save</summary>

Connect cold-source edits and Save to captured reload and validated replay without requiring Reload Sources first.

Data flow: Typed command → capture original intent → required cold keys → fetch → validated completion → one edit or download.

### 1. `src/browser_reload.rs`

A new edit supersedes older pending work. Warm edits need no fetch; cold edits capture their required keys and operation.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn start(shared: Shared, keys: Vec<ReloadKey>, delivery: Delivery) -> Result<(), String> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn restore_for(intent: crate::edit_intent::Intent, editor: &crate::editor::Editor, shared: Shared, delivery: Delivery) -> Result<bool, String> {
    let keys = intent.keys(editor).map_err(str::to_owned)?;
    shared.borrow_mut().cancel();
    if keys.is_empty() { return Ok(false); }
    start_with(shared, keys, Some(intent), delivery)?;
    Ok(true)
}

pub fn start(shared: Shared, keys: Vec<ReloadKey>, delivery: Delivery) -> Result<(), String> {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/32gif-auto-01.rs"
```

</details>

### 2. `src/browser.rs`

Save reloads every active cold source before producing its original-precision download; warm Save remains immediate. Capture Move/Delete before fetching; camera and selection commands remain immediate while pending work keeps its original target. A deferred command has no scene change yet; failures retain normal dock reporting. Expose read-only placement, selection and camera values so Chrome checks the original target and later view exactly. These values add no controls.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
```

</details>

Replace it with:

```rust
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    let placements: Vec<_> = editor.scene.objects().iter().map(|row| (format!("{:?}", row.id), row.model.m)).collect();
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-selected-id", &editor.selected.map(|id| format!("{id:?}")).unwrap_or_default())?;
    canvas.set_attribute("data-camera-matrix", &serde_json::to_string(&editor.camera.view_projection().m).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
```

<details>
<summary>Locate this block</summary>

```rust
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Change::View) => {}
                Err(error) => {
                    report(error);
                    if event.type_() == "viewer-file" { panel.answer("Open", error); }
                    else { panel.result(error); }
                }
            }
        }
```

</details>

Replace it with:

```rust
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Some(Change::View)) | Ok(None) => {}
                Err(error) => {
                    report(&error);
                    if event.type_() == "viewer-file" { panel.answer("Open", &error); }
                    else { panel.result(&error); }
                }
            }
        }
```

<details>
<summary>Locate this block</summary>

```rust
            let replacement = matches!(&action, Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            match editor.apply(action) {
                Ok(Change::Scene) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if unloading {
                        report("Editable sources unloaded; display retained.");
```

</details>

Replace it with:

```rust
            let replacement = matches!(&action, Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            let waiting = crate::edit_intent::Intent::capture(&action, editor.selected)
                .map_err(str::to_owned).and_then(|intent| match intent {
                    Some(intent) => crate::browser_reload::restore_for(intent, &editor,
                        std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery)),
                    None => Ok(false),
                });
            let change = waiting.and_then(|waiting| {
                if waiting { panel.result("Reloading editable sources…"); Ok(None) }
                else { editor.apply(action).map(Some).map_err(str::to_owned) }
            });
            match change {
                Ok(Some(Change::Scene)) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if unloading {
                        report("Editable sources unloaded; display retained.");
```

<details>
<summary>Locate this block</summary>

```rust
                }
                None
            } else if line == "save" {
                match crate::document::snapshot(&editor.scene) {
                    Ok(bytes) => match crate::file_output::download(&bytes) {
                        Ok(()) => panel.result(&format!("Saved editable document ({} bytes)", bytes.len())),
                        Err(error) => panel.result(&format!("Save failed: {error:?}")),
                    },
                    Err(error) => panel.result(error),
                }
                None
            } else if line == "move" || line.starts_with("move ") {
                match crate::offset::parse(&line) {
```

</details>

Replace it with:

```rust
                }
                None
            } else if line == "save" {
                let result = crate::browser_reload::restore_for(crate::edit_intent::Intent::Save,
                    &editor, std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery))
                    .and_then(|waiting| {
                        if waiting { Ok("Reloading editable sources…".to_owned()) }
                        else { crate::document::snapshot(&editor.scene).map_err(str::to_owned)
                            .and_then(|bytes| save_result(&bytes)) }
                    });
                panel.result(&result.unwrap_or_else(|error| error));
                None
            } else if line == "move" || line.starts_with("move ") {
                match crate::offset::parse(&line) {
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::background::Background;
use crate::editor::{Action, Change, Editor};
use crate::gesture::Gesture;
use crate::renderer::Renderer;
use crate::viewport::Viewport;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot draw:") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| format!("Saved editable document ({} bytes)", bytes.len()))
        .map_err(|error| format!("Save failed: {error:?}"))
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = Editor::default();
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    renderer.resize(Viewport { width, height });
    inspect(&editor, &renderer)?;
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &[
            "Help",
            "Open",
            "Open Replace",
            "Cancel Open",
            "Close",
            "Unload Sources",
            "Reload Sources",
            "Cancel Reload",
            "Save",
            "Example Box",
            "Example Triangle",
            "Select Next",
            "Delete",
            "Undo",
            "Redo",
            "Background",
            "Zoom In",
            "Zoom Out",
            "Pan Left",
            "Pan Right",
            "Orbit Right",
            "Orbit Up",
            "View Isometric",
            "View Perspective",
            "View Orthographic",
            "Fit",
            "Fit Selected",
            "Move",
            "View Reset",
        ],
    );
    panel.update(None, &canvas)?;
    if resize(
        &window,
        &canvas,
        &surface,
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            &mut panel,
        )?;
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
            event.prevent_default();
            panel.answer(shortcut.command(), "");
            Some(shortcut.command().to_ascii_lowercase())
        } else { match panel.update(Some(&event), &input_canvas) {
            Ok(line) => line,
            Err(error) => {
                report(&format!("Cannot read command: {error:?}"));
                return;
            }
        } };

        let action = if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
        {
            request.borrow_mut().cancel();
            report("Open cancelled.");
            panel.answer("Open", "Open cancelled.");
            None
        } else if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request), read_mode.get(), std::rc::Rc::clone(&delivery));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-reload" {
            if let Some(reply) = reload_delivery.borrow_mut().take() {
                let command = reply.intent.map_or("Reload Sources", |intent| intent.command());
                match reply.complete(&mut editor) {
                    Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        let message = if command == "Reload Sources" {
                            "Editable sources restored; display retained.".to_owned()
                        } else { format!("{command} completed after source reload.") };
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(Some(crate::edit_replay::Reply::Saved(bytes))) => {
                        let message = save_result(&bytes).unwrap_or_else(|error| error);
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(None) => {}
                    Err(error) => { report(&error); panel.answer(command, &error); }
                }
            }
            None
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery) {
                Ok(action) => action,
                Err(error) => { let message = format!("Cannot retain reload file: {error:?}");
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            if line == "reload sources" {
                let result = crate::browser_reload::start(std::rc::Rc::clone(&reload), editor.reload_keys(), std::rc::Rc::clone(&reload_delivery));
                match result {
                    Ok(()) => panel.result("Reloading editable sources…"),
                    Err(error) => { report(&error); panel.result(&error); }
                }
                None
            } else if line == "cancel reload" {
                reload.borrow_mut().cancel();
                report("Reload cancelled."); panel.result("Reload cancelled.");
                None
            } else if line == "unload sources" {
                Some(Action::UnloadSources)
            } else if line == "close" {
                request.borrow_mut().cancel();
                Some(Action::Close)
            } else if line == "cancel open" {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            } else if line == "open" || line == "open replace" {
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                if line == "open replace" { reload.borrow_mut().cancel(); }
                read_mode.set(if line == "open replace" { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
                    .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                }
                None
            } else if line == "save" {
                let result = crate::browser_reload::restore_for(crate::edit_intent::Intent::Save,
                    &editor, std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery))
                    .and_then(|waiting| {
                        if waiting { Ok("Reloading editable sources…".to_owned()) }
                        else { crate::document::snapshot(&editor.scene).map_err(str::to_owned)
                            .and_then(|bytes| save_result(&bytes)) }
                    });
                panel.result(&result.unwrap_or_else(|error| error));
                None
            } else if line == "move" || line.starts_with("move ") {
                match crate::offset::parse(&line) {
                    Ok(offset) => Some(Action::Translate(offset)),
                    Err(error) => { panel.result(error); None }
                }
            } else {
                let action = match line.as_str() {
                    "example box" => Action::AddBox,
                    "example triangle" => Action::ToggleExtra,
                    "select next" => Action::SelectNext,
                    "delete" => Action::Delete,
                    "undo" => Action::Undo,
                    "redo" => Action::Redo,
                    "background" => Action::Background,
                    "zoom in" => Action::Zoom(2.0),
                    "zoom out" => Action::Zoom(0.5),
                    "pan left" => Action::Pan(-0.25, 0.0),
                    "pan right" => Action::Pan(0.25, 0.0),
                    "orbit right" => Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0),
                    "orbit up" => Action::Orbit(0.0, std::f64::consts::FRAC_PI_6),
                    "view isometric" => Action::Isometric,
                    "view perspective" => {
                        Action::Projection(crate::camera::Projection::Perspective)
                    }
                    "view orthographic" => {
                        Action::Projection(crate::camera::Projection::Orthographic)
                    }
                    "fit" if shortcut.is_some() && editor.selected.is_some() => Action::FitSelected,
                    "fit" => Action::Fit,

                    "fit selected" => Action::FitSelected,
                    "view reset" => Action::ResetView,
                    _ => return,
                };
                Some(action)
            }
        } else if !panel.consumed {
            navigation_action(&event, &pointer_canvas, &mut gesture)
        } else {
            None
        };
        if let Some(action) = action {
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Undo | Action::Redo
                | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
            let replacement = matches!(&action, Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            let waiting = crate::edit_intent::Intent::capture(&action, editor.selected)
                .map_err(str::to_owned).and_then(|intent| match intent {
                    Some(intent) => crate::browser_reload::restore_for(intent, &editor,
                        std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery)),
                    None => Ok(false),
                });
            let change = waiting.and_then(|waiting| {
                if waiting { panel.result("Reloading editable sources…"); Ok(None) }
                else { editor.apply(action).map(Some).map_err(str::to_owned) }
            });
            match change {
                Ok(Some(Change::Scene)) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if unloading {
                        report("Editable sources unloaded; display retained.");
                        panel.result("Editable sources unloaded; display retained.");
                    } else if closing {
                        report("Document closed.");
                        panel.result("Document closed.");
                    } else if event.type_() == "viewer-file" {
                        let message = if replacement { "Document replaced. Undo restores the previous document." }
                            else { "File imported. Undo removes the entire import." };
                        report(message);
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Some(Change::View)) | Ok(None) => {}
                Err(error) => {
                    report(&error);
                    if event.type_() == "viewer-file" { panel.answer("Open", &error); }
                    else { panel.result(&error); }
                }
            }
        }
        if let Err(error) = inspect(&editor, &renderer) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
        if resize(
            &browser_window,
            &pointer_canvas,
            &surface,
            &mut config,
            &mut renderer,
            &mut editor,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
                return;
            }
            if let Err(error) = present(
                &surface,
                &renderer,
                &editor.background,
                &editor.camera.uniform(),
                &mut panel,
            ) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        }
    });
    for name in [
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
        "lostpointercapture",
        "contextmenu",
        "keydown",
        "keyup",
        "blur",
    ] {
        canvas.add_event_listener_with_callback(name, update.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        update.as_ref().unchecked_ref(),
        &options,
    )?;
    window.add_event_listener_with_callback("resize", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("cancel", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file-error", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-reload", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}

fn inspect(editor: &Editor, renderer: &Renderer) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    let placements: Vec<_> = editor.scene.objects().iter().map(|row| (format!("{:?}", row.id), row.model.m)).collect();
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-selected-id", &editor.selected.map(|id| format!("{id:?}")).unwrap_or_default())?;
    canvas.set_attribute("data-camera-matrix", &serde_json::to_string(&editor.camera.view_projection().m).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
        (&row.guid, &m.source_guid, &m.name, m.is_visible, m.is_locked)
    }).collect();
    let origins: Vec<_> = editor.scene.objects().iter().filter_map(|row| row.origin().map(|origin|
        (&row.guid, origin.id.to_string(), origin.version.hex(),
            origin.location.as_ref().map(|location| location.value())))).collect();
    let residency: Vec<_> = editor.scene.objects().iter().map(|row|
        (&row.guid, row.geometry().is_some(), row.release_epoch())).collect();
    canvas.set_attribute("data-source-residency", &serde_json::to_string(&residency)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-source-origins", &serde_json::to_string(&origins)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-row-metadata", &serde_json::to_string(&metadata)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-gpu-stats", &format!("{:?}", renderer.stats()))?;
    let cpu = crate::memory::cpu(editor.scenes(), renderer.geometry_owners().map(|owner| &owner.source));
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
    canvas.set_attribute("data-gpu-usage", &format!("{:?}", renderer.usage()))?;
    canvas.set_attribute("data-projection", &format!("{:?}", editor.camera.projection))
}

fn navigation_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    let action = match event.type_().as_str() {
        "wheel" => {
            let wheel = event.dyn_ref::<web_sys::WheelEvent>()?;
            let mode = wheel.delta_mode();
            crate::navigation::wheel(
                wheel.delta_y(),
                mode,
                canvas.get_bounding_client_rect().height(),
            )
        }
        "keydown" => {
            let key = event.dyn_ref::<web_sys::KeyboardEvent>()?;
            if key.is_composing() || key.alt_key() {
                return None;
            }
            if key.key() == "Escape" {
                gesture.cancel();
                event.prevent_default();
                return None;
            }
            return None;
        }
        _ => return pointer_action(event, canvas, gesture),
    }?;
    event.prevent_default();
    Some(action)
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    match event.type_().as_str() {
        "contextmenu" => {
            event.prevent_default();
            return None;
        }
        "blur" | "resize" => {
            gesture.cancel();
            return None;
        }
        _ => {}
    }
    let pointer = event.dyn_ref::<web_sys::PointerEvent>()?;
    let id = pointer.pointer_id();
    let position = [pointer.client_x() as f64, pointer.client_y() as f64];
    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary() && gesture.press(id, pointer.button(), position) {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err()
                    || canvas.set_pointer_capture(id).is_err()
                {
                    gesture.cancel();
                }
            }
            None
        }
        "pointermove" => gesture.move_to(id, position),
        "pointerup" => {
            let motion = gesture.release(id, position);
            let _ = canvas.release_pointer_capture(id);
            motion
        }
        "pointercancel" | "lostpointercapture" => {
            gesture.cancel_pointer(id);
            None
        }
        _ => None,
    }?;
    let rect = canvas.get_bounding_client_rect();
    motion.action([rect.left(), rect.top(), rect.width(), rect.height()])
}

fn resize(
    window: &web_sys::Window,
    canvas: &web_sys::HtmlCanvasElement,
    surface: &wgpu::Surface<'_>,
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        window.device_pixel_ratio(),
        limit,
    ) else {
        return false;
    };
    if size.width != config.width || size.height != config.height {
        canvas.set_width(size.width);
        canvas.set_height(size.height);
        config.width = size.width;
        config.height = size.height;
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    editor.camera.aspect = size.aspect();
    true
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 16],
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => {
            return Err(JsValue::from_str(&format!(
                "Surface unavailable: {other:?}"
            )));
        }
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view, background, transform);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/32gif-auto-02.rs"
```

```text
--8<-- "journey/code/32gif-auto-03.rs"
```

```text
--8<-- "journey/code/32gif-auto-04.rs"
```

```text
--8<-- "journey/code/32gif-auto-05.rs"
```

</details>

</details>

<a id="checkpoint-32gj-precision"></a>

### Step 25 of 26 · Prove restored Save keeps the source doubles (`32gj-precision`)

- **Goal:** Verify the automatic browser Save download retains exact source coordinates after display-only unloading.
- **Data Flow Trace:** `Original f64 sentinel → derived f32 display → unload kernel → reload original bytes → Save → exact source doubles`
- **Files You Touch:** `src/specimen.rs` (*Add a source coordinate that cannot survive an f64-to-f32 round trip; keep the original specimen available.*) · `examples/sample.rs` (*Supply this optional course-author fixture generator; browser exercises use the linked data files or recorded comparison.*) · `src/lib.rs` (*Register the exact-coordinate restoration acceptance check.*) · `src/reload_precision_tests.rs` (*Optional authoring fixture. Verify exact source-coordinate restoration with a real precision sentinel and unchanged edit history.*)

Add `precise_bytes` to the specimen builder with one coordinate that cannot survive an `f32` round trip. Keep the ordinary specimen function for existing checks.

Save while sources are warm, then unload and trigger restoration through Save. Decode both actual downloads and compare the source coordinate exactly. The derived display is unsuitable evidence because it intentionally rounds to floats.

The native and browser checks use the same uploaded specimen. They compare original coordinates and preserve the moved placement; the native test also checks Undo/Redo.

> **Self-Check Question:** *Why is equal drawing insufficient proof that Save preserved source coordinates?*  
> **Answer:** The GPU display already rounds coordinates to f32. Two different f64 values may draw identically. Compare the actual downloaded source coordinates and include a value whose f32 round trip differs.

- **What You Should See:** Open `sample.pb`, type `Unload Sources`, then `Save`. Run the precision checks below: the restored Save must retain the exact source double rather than its rounded display value.
- **Hands-On Lab Experiment:** Predict what would happen if snapshot geometry came from the display floats. Compare the supplied exact-coordinate expectations and the browser download: screenshots alone cannot reveal the lost precision.

<details>
<summary>Implementation 32gj-precision: Prove restored Save keeps the source doubles</summary>

Verify the automatic browser Save download retains exact source coordinates after display-only unloading.

Data flow: Original f64 sentinel → derived f32 display → unload kernel → reload original bytes → Save → exact source doubles.

### 1. `src/specimen.rs`

Add a source coordinate that cannot survive an f64-to-f32 round trip; keep the original specimen available.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn bytes() -> Vec<u8> {
    use session_rust::{Color, Mesh, Session, Xform};
    let mut session = Session::new("Three-piece frame");
    for (name, size, centre) in [
        ("Left post", [0.25, 0.35, 1.0], [-0.65, 0.0, 0.5]),
        ("Right post", [0.25, 0.35, 1.0], [0.65, 0.0, 0.5]),
        ("Top beam", [1.8, 0.35, 0.25], [0.0, 0.0, 1.125]),
    ] {
        let mut mesh = Mesh::create_box(size[0], size[1], size[2]);
        mesh.name = name.to_owned();
        mesh.transform(&Xform::translation(centre[0], centre[1], centre[2]));
        mesh.set_objectcolor(Color::new(0.75, 0.35, 0.1, 1.0));
        assert!(session.add_mesh(mesh, None).is_some());
    }
    session.pb_dumps()
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gj-precision-01.rs" skip=697
```

<details>
<summary>Optional authoring helper</summary>

Copy this supplied authoring helper once. Use the linked data or recorded comparison for the browser exercise; you do not need to compile or run this native helper.

### 2. `examples/sample.rs`

Supply this optional course-author fixture generator; browser exercises use the linked data files or recorded comparison.

<details>
<summary>Locate the existing block</summary>

```text
viewer_journey::specimen::bytes()
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/32gj-precision-02.rs"
```

</details>

### 3. `src/lib.rs`

Register the exact-coordinate restoration acceptance check.

<details>
<summary>Locate the existing block</summary>

```rust
mod reload_complete_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gj-precision-03.rs" skip=27
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/reload_precision_tests.rs`

Optional authoring fixture. Verify exact source-coordinate restoration with a real precision sentinel and unchanged edit history.

Copy this supplied file:

```text
--8<-- "journey/code/32gj-precision-04.rs"
```

</details>

</details>

<a id="checkpoint-32gja-failures"></a>

### Step 26 of 26 · Prove failed restoration cannot partly commit (`32gja-failures`)

- **Goal:** Verify automatic command failures preserve cold drawing, placement and history, including all-or-nothing multiple-source Save.
- **Data Flow Trace:** `Captured operation → complete body set → validate every source → all restored or none → no edit/download on failure`
- **Files You Touch:** `src/lib.rs` (*Register multiple-source completion acceptance checks.*) · `src/reload_failure_tests.rs` (*Optional authoring fixture. A failure, missing body, changed file, duplicate key or stale epoch must not partially restore a multi-source Save.*)

Test restoration as one transaction across two imported origins. If the second body is invalid, neither source may be restored and the captured Save must not download anything.

Use missing bodies, changed versions and stale keys as rejected examples. A late reply after Close must find its source URL already released. These checks exercise the existing implementation; they add no new viewer command.

> **Self-Check Question:** *Why must the first valid source remain cold when a second source fails?*  
> **Answer:** Save represents one document operation. Installing the first source before checking the remaining responses would leave a partially restored editor after the operation failed. Validate the complete set before installation.

- **What You Should See:** Run the failed-restoration checks below. An invalid or incomplete reply must leave the scene, source residency and history unchanged, with no edit or Save download.
- **Hands-On Lab Experiment:** Trace a successful first fetch followed by a failed second fetch. Explain how the browser residency checks distinguish atomic adoption from a partial commit.

<details>
<summary>Implementation 32gja-failures: Prove failed restoration cannot partly commit</summary>

Verify automatic command failures preserve cold drawing, placement and history, including all-or-nothing multiple-source Save.

Data flow: Captured operation → complete body set → validate every source → all restored or none → no edit/download on failure.

### 1. `src/lib.rs`

Register multiple-source completion acceptance checks.

<details>
<summary>Locate the existing block</summary>

```rust
mod reload_precision_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/32gja-failures-01.rs" skip=28
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/reload_failure_tests.rs`

Optional authoring fixture. A failure, missing body, changed file, duplicate key or stale epoch must not partially restore a multi-source Save.

Copy this supplied file:

```text
--8<-- "journey/code/32gja-failures-02.rs"
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 32gja-failures
npm --prefix ../session_tests run course -- check 32gja-failures
npm --prefix ../session_tests run course -- save 32gja-failures
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

Run the failed-restoration checks below. An invalid or incomplete reply must leave the scene, source residency and history unchanged, with no edit or Save download.

- **Final Lesson Experiment:** Trace a successful first fetch followed by a failed second fetch. Explain how the browser residency checks distinguish atomic adoption from a partial commit.
