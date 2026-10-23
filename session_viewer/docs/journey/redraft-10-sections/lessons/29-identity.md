# 3.5 · Save, replace and asynchronous transaction results

**Section 3: Scene Documents, Transactions, and Browser GPU Lifetime** · **Lesson 3.5** (Redraft `3.5`) · **11 checkpoints (`29-identity` → `30e-recovery`)** · **Study time: 11–22 h**

> **What You Will Learn & Build:** Distinguish a scene object from the original geometry it shares. By the final checkpoint (`30e-recovery`), you will test partial replacement failure, preserved selection/Redo and placement upload limits.

## 1. Visual WebGPU Explanation & Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="256" viewBox="0 0 840 256" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrow-hollow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="4" y="4" width="832" height="248" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Chapter 3 Visual Explanation: Local Kernel Mesh → 4x4 World Placement, AABB Fit & Incremental GPU Sync</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Local f64 mesh placed in world space by 4x4 matrix M. Center: Transactional .pb decode gate. Right: Dirty-flag GPU sync.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <!-- Panel A: Local Space -> World Placement & AABB -->
    <rect x="18" y="64" width="275" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="155" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. Local Source → World Placement</text>
    <!-- Local mesh box at (0,0) -->
    <rect x="38" y="128" width="54" height="44" rx="2" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <line x1="38" y1="172" x2="98" y2="172" stroke="#111111" stroke-width="1" marker-end="url(#arrow)"/>
    <line x1="38" y1="172" x2="38" y2="115" stroke="#111111" stroke-width="1" marker-end="url(#arrow)"/>
    <text x="65" y="154" text-anchor="middle" font-size="9" font-weight="700" fill="#111111">Local</text>
    <text x="65" y="186" text-anchor="middle" font-size="9" fill="#111111">(0,0,0) f64</text>
    <!-- Transform arrow -->
    <line x1="102" y1="145" x2="152" y2="132" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>
    <text x="127" y="126" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">M (4x4)</text>
    <!-- Placed rotated box inside dashed World AABB -->
    <rect x="164" y="96" width="108" height="86" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1" stroke-dasharray="4,3"/>
    <polygon points="196,104 260,126 240,174 176,152" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
    <text x="218" y="194" text-anchor="middle" font-size="9" font-weight="600" fill="#111111">Dashed: World AABB (Fit)</text>
    <rect x="30" y="204" width="250" height="30" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1"/>
    <text x="155" y="218" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Cold CPU source dropped after GPU upload;</text>
    <text x="155" y="229" text-anchor="middle" font-size="9" fill="#111111">re-fetched by SHA/revision only when edited</text>

    <line x1="293" y1="154" x2="315" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel B: Transactional .pb Decode Gate -->
    <rect x="317" y="64" width="235" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="434" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. Transactional .pb Gate</text>
    <rect x="333" y="96" width="203" height="42" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <text x="434" y="113" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">Async Fetch Ticket #N</text>
    <text x="434" y="128" text-anchor="middle" font-size="9.5" fill="#111111">Cancels stale #N-1; checks byte cap</text>
    <line x1="434" y1="138" x2="434" y2="154" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <rect x="333" y="156" width="203" height="78" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="434" y="174" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">Atomic Validation Commit</text>
    <text x="345" y="192" font-size="9.5" fill="#111111">• Valid topology -&gt; Replace Scene</text>
    <text x="345" y="208" font-size="9.5" fill="#111111">• Corrupt bytes -&gt; Reject cleanly</text>
    <text x="345" y="223" font-size="9.5" font-weight="600" fill="#111111">  (Zero partial scene corruption!)</text>

    <line x1="552" y1="154" x2="574" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel C: Incremental GPU Sync -->
    <rect x="576" y="64" width="246" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="699" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. Incremental GPU Sync</text>
    <rect x="590" y="96" width="218" height="64" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="699" y="114" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">Path 1: Camera Orbit / Pan / Fit</text>
    <text x="602" y="131" font-size="9.5" fill="#111111">• sync_scene = false, redraw = true</text>
    <text x="602" y="147" font-size="9.5" font-weight="600" fill="#111111">• Writes 64-byte camera uniform ONLY</text>
    <rect x="590" y="170" width="218" height="64" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="699" y="188" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">Path 2: Mesh / Placement Edit</text>
    <text x="602" y="205" font-size="9.5" fill="#111111">• sync_scene = true, redraw = true</text>
    <text x="602" y="221" font-size="9.5" font-weight="600" fill="#111111">• Re-uploads dirty object buffers only</text>
    
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="214" viewBox="0 0 840 214" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrow-hollow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="4" y="4" width="832" height="206" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 3.5 Pipeline Schematic: Save, replace and asynchronous transaction results</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (11 Checkpoints: 29-identity → 30e-recovery)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="151" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">Source GUID</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="90" x2="316" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">inserted object</text>
    <text x="254" y="124" text-anchor="middle" font-size="11" fill="#111111">GUID</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="90" x2="481" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">cloned history</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="90" x2="646" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">stable file</text>
    <text x="584" y="124" text-anchor="middle" font-size="11" fill="#111111">identity</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">Malformed source or</text>
    <text x="749" y="124" text-anchor="middle" font-size="11" fill="#111111">failed insertion</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 3.5:</text>
    <text x="235" y="177" font-size="11" fill="#111111">Cargo.toml  ·  src/scene.rs  ·  src/file_identity_tests.rs  ·  src/lib.rs  ·  src/document.rs  ·  src/snapshot_tests.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">11 Checkpoints: 29-identity → 30e-recovery — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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
| **1** | [`29-identity`](#checkpoint-29-identity) | Distinguish a scene object from the original geometry it shares. | `Cargo.toml`, `src/scene.rs`, `src/file_identity_tests.rs`, `src/lib.rs` | `Source GUID → inserted object GUID → cloned history → stable file identity` |
| **2** | [`29a-snapshot`](#checkpoint-29a-snapshot) | Serialize live source meshes and their placements without changing the editor. | `src/document.rs`, `src/snapshot_tests.rs`, `src/lib.rs` | `Live objects → source protobuf copies + object GUIDs → flat tree and placements → bytes` |
| **3** | [`29b-placements`](#checkpoint-29b-placements) | Validate stored placements before reconstructing source-backed objects. | `src/placement.rs`, `src/lib.rs`, `src/scene.rs`, `src/prepared.rs` | `Bytes → validated mesh IDs and affine matrices → prepared local sources plus placement → inserted objects` |
| **4** | [`29c-roundtrip`](#checkpoint-29c-roundtrip) | Check exact local source data, attributes, identities and placement through saving and loading. | `src/save_roundtrip_tests.rs`, `src/lib.rs` | `Live source and placement → snapshot bytes → load → identical source values and object matrices` |
| **5** | [`29d-save`](#checkpoint-29d-save) | Connect Save to a browser download and reopen the actual downloaded document. | `Cargo.toml`, `src/file_output.rs`, `src/lib.rs`, `src/browser.rs` | `Typed Save → source snapshot → Blob URL → download → Open → restored source and placement` |
| **6** | [`30-feedback`](#checkpoint-30-feedback) | Keep failed reads and imports visible in command history without claiming success. | `src/browser.rs`, `src/file_input.rs` | `File read result → browser event → validated editor action → success or error in the command dock` |
| **7** | [`30a-tickets`](#checkpoint-30a-tickets) | Describe latest-read ownership, cancellation and one-shot completion in ordinary Rust. | `src/read_gate.rs`, `src/lib.rs` | `Begin read → pending ticket → supersede, cancel or finish once` |
| **8** | [`30b-cancel`](#checkpoint-30b-cancel) | Adopt one-shot tickets in the browser and give asynchronous results their own history entry. | `src/file_input.rs`, `src/browser.rs` | `Selected file → ticket-owned task → consume current ticket → deliver outcome; Cancel Open revokes ownership` |
| **9** | [`30c-replace`](#checkpoint-30c-replace) | Prepare a whole replacement before changing live rows, then commit it through existing history. | `src/scene.rs`, `src/editor.rs`, `src/replace_tests.rs`, `src/lib.rs` | `Replacement bytes → validated prepared sources → history transaction → new rows with fresh local IDs` |
| **10** | [`30d-bridge`](#checkpoint-30d-bridge) | Carry the selected Open operation through asynchronous file reading and cancel older work immediately. | `src/file_input.rs`, `src/browser.rs` | `Open or Open Replace → new ticket plus captured mode → file bytes → explicit editor action` |
| **11** | [`30e-recovery`](#checkpoint-30e-recovery) | Test partial replacement failure, preserved selection/Redo and placement upload limits. | `src/placement.rs`, `src/scene.rs`, `src/replace_tests.rs` | `Malformed source or failed insertion → retained old owners and history; double matrix → finite float upload check` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-29-identity"></a>

### Step 1 of 11 · Give each saved object a stable identity (`29-identity`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/11 Visual Trace (29-identity): Give each saved object a stable identity</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: Cargo.toml, src/scene.rs, src/file_identity_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Source GUID</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">inserted object GUID</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">cloned history</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">stable file identity</text>
    
</svg>
</div>

- **Goal:** Distinguish a scene object from the original geometry it shares.
- **Data Flow Trace:** `Source GUID → inserted object GUID → cloned history → stable file identity`
- **Files You Touch:** `Cargo.toml` (*Declare the UUID generator explicitly; browser randomness uses its js feature.*) · `src/scene.rs` (*Store the inserted object identity separately from kernel source identity and the local counter.*) · `src/file_identity_tests.rs` (*Optional authoring fixture. Exercise duplicate imports, row removal and history without mutating original source GUIDs.*) · `src/lib.rs` (*Compile the file-identity regression check.*)

Give each inserted object a stored GUID for saving. Local `ObjectId` still identifies the editor object; imported source GUID still identifies its original geometry.

Prefer the geometry GUID when unused. Duplicate imports receive a new UUID without changing their original source mesh or provenance. Check even generated candidates against live objects.

Assign identity once at insertion. History clones the stored `String`; Move and Save must not regenerate it.

The duplicate-import test checks distinct object GUIDs and unchanged source GUIDs, then removes an earlier row and travels through Undo/Redo. Download is connected later.

> **Self-Check Question:** *Why can two imports share a source GUID but need different saved GUIDs?*  
> **Answer:** The source GUID identifies geometry in an original file. Two imports create two scene objects with separate placements and histories. Each needs a unique stored identity. We preserve the source GUID when available, assign a fresh UUID on collision, and store that decision in the object so saving or row changes cannot rename it.

- **What You Should See:** Import `sample.pb` twice. Run the identity checks: copies may share a source GUID, but each scene object must have its own saved identity.
- **Hands-On Lab Experiment:** Inspect both imported copies’ source GUIDs and object GUIDs in the check. Predict which pair matches. Add another Undo/Redo cycle, then restore the check.

<details>
<summary>Implementation 29-identity: Give each saved object a stable identity</summary>

Distinguish a scene object from the original geometry it shares.

Data flow: Source GUID → inserted object GUID → cloned history → stable file identity.

### 1. `Cargo.toml`

Declare the UUID generator explicitly; browser randomness uses its js feature.

<details>
<summary>Locate the existing block</summary>

```toml
prost = "=0.14.4"
```

</details>

After that block, type:

```toml
--8<-- "journey/code/29-identity-01.toml" skip=18
```

### 2. `src/scene.rs`

Store the inserted object identity separately from kernel source identity and the local counter. Keep the source GUID when available; assign and check a fresh stored GUID for a duplicate import.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.objects.push(Object { id, mesh: Rc::new(prepared.display), geometry: prepared.geometry,
            model: session_rust::Xform::identity(), source: prepared.source });
        Ok(id)
    }
```

</details>

Replace it with:

```rust
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        let mut guid = prepared.geometry.guid().to_owned();
        while self.objects.iter().any(|object| object.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        self.objects.push(Object { id, guid, mesh: Rc::new(prepared.display), geometry: prepared.geometry,
            model: session_rust::Xform::identity(), source: prepared.source });
        Ok(id)
    }
```

<details>
<summary>Locate this block</summary>

```rust
#[derive(Clone)]
pub struct Object {
    pub id: ObjectId,
    pub mesh: Rc<Mesh>,
    pub geometry: Rc<session_rust::Mesh>,
    pub model: session_rust::Xform,
```

</details>

Replace it with:

```rust
#[derive(Clone)]
pub struct Object {
    pub id: ObjectId,
    pub guid: String,
    pub mesh: Rc<Mesh>,
    pub geometry: Rc<session_rust::Mesh>,
    pub model: session_rust::Xform,
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{mesh::Mesh, prepared::PreparedMesh};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjectId(u32);

#[derive(Clone)]
pub struct Object {
    pub id: ObjectId,
    pub guid: String,
    pub mesh: Rc<Mesh>,
    pub geometry: Rc<session_rust::Mesh>,
    pub model: session_rust::Xform,
    pub source: Option<crate::document::Source>,
}

impl Object {
    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
        let local = session_rust::Point::new(vertex[0] as f64, vertex[1] as f64, vertex[2] as f64);
        let world = self.model.transform_point(&local);
        [world[0], world[1], world[2]]
    }
}

#[derive(Clone)]
pub struct Scene {
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
}

impl Scene {
    pub fn demo() -> Self {
        let pink = PreparedMesh::triangle([
            [-0.7, -0.6, 0.25], [0.5, -0.6, 0.25], [-0.1, 0.6, 0.25],
        ], [0.9, 0.25, 0.45]).expect("Valid pink triangle");
        let turquoise = PreparedMesh::triangle([
            [-0.4, -0.2, 0.75], [0.8, -0.2, 0.75], [0.2, 0.8, 0.75],
        ], [0.05, 0.7, 0.7]).expect("Valid turquoise triangle");
        let mut scene = Self { objects: Vec::new(), next_id: 1, extra: None };
        scene.insert(pink).expect("ID available");
        scene.insert(turquoise).expect("ID available");
        scene
    }

    pub fn restore(&mut self, mut saved: Self) {
        saved.next_id = saved.next_id.max(self.next_id);
        *self = saved;
    }

    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for object in &self.objects {
            for vertex in object.mesh.vertices() {
                let point = object.world_point(*vertex);
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        bounds
    }

    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
        let object = self.objects.iter().find(|object| object.id == id)?;
        let mut vertices = object.mesh.vertices().iter();
        let first = vertices.next()?;
        let mut bounds = crate::bounds::Bounds::point(object.world_point(*first));
        for vertex in vertices {
            bounds.include(object.world_point(*vertex));
        }
        Some(bounds)
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        let mut guid = prepared.geometry.guid().to_owned();
        while self.objects.iter().any(|object| object.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        self.objects.push(Object { id, guid, mesh: Rc::new(prepared.display), geometry: prepared.geometry,
            model: session_rust::Xform::identity(), source: prepared.source });
        Ok(id)
    }

    pub fn import(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        for prepared in loaded.meshes {
            self.insert(prepared)?;
        }
        Ok(())
    }

    pub fn place(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !model.m.iter().all(|v| v.is_finite())
            || [model.m[3], model.m[7], model.m[11], model.m[15]] != [0.0, 0.0, 0.0, 1.0]
        {
            return Err("Placement must be finite and affine");
        }
        let object = self.objects.iter_mut().find(|o| o.id == id).ok_or("Object not found")?;
        object.model = model;
        Ok(())
    }

    pub fn remove(&mut self, id: ObjectId) -> bool {
        let Some(index) = self.objects.iter().position(|object| object.id == id) else { return false; };
        self.objects.remove(index);
        if self.extra == Some(id) {
            self.extra = None;
        }
        true
    }

    pub fn contains(&self, id: ObjectId) -> bool {
        self.objects.iter().any(|object| object.id == id)
    }

    pub fn next(&self, current: Option<ObjectId>) -> Option<ObjectId> {
        if self.objects.is_empty() {
            return None;
        }
        let index = self.objects.iter().position(|object| Some(object.id) == current)
            .map_or(0, |index| index + 1);
        Some(self.objects[index % self.objects.len()].id)
    }

    pub fn add_box(&mut self) -> Result<ObjectId, &'static str> {
        let mut source = session_rust::Mesh::create_box(0.8, 0.8, 0.8);
        source.set_objectcolor(session_rust::Color::new(0.65, 0.65, 0.65, 1.0));
        let id = self.insert(PreparedMesh::new(source)?)?;
        self.place(id, session_rust::Xform::translation(0.9, 0.0, 0.4))?;
        Ok(id)
    }

    pub fn toggle_extra(&mut self) {
        if let Some(id) = self.extra.take() {
            self.remove(id);
        } else {
            let extra = PreparedMesh::triangle([
                [-0.9, 0.3, 0.5], [-0.4, 0.3, 0.5], [-0.65, 0.9, 0.5],
            ], [0.2, 0.8, 0.3]).expect("Valid extra triangle");
            self.extra = Some(self.insert(extra).expect("ID available"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removal_moves_a_row_without_changing_its_identity() {
        let mut scene = Scene::demo();
        let first = scene.objects()[0].id;
        let second = scene.objects()[1].id;
        assert!(scene.remove(first));
        assert_eq!(scene.objects()[0].id, second);
        assert_eq!(scene.next(None), Some(second));
        assert!(!scene.contains(first));
        scene.toggle_extra();
        assert_ne!(scene.objects()[1].id, first);
        assert_ne!(scene.objects()[1].id, second);
    }

    #[test]
    fn toggling_the_extra_object_uses_its_id_after_other_removals() {
        let mut scene = Scene::demo();
        scene.toggle_extra();
        let extra = scene.objects()[2].id;
        scene.remove(scene.objects()[0].id);
        scene.toggle_extra();
        assert!(!scene.contains(extra));
        assert_eq!(scene.objects().len(), 1);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/29-identity-02.rs"
```

```text
--8<-- "journey/code/29-identity-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/file_identity_tests.rs`

Optional authoring fixture. Exercise duplicate imports, row removal and history without mutating original source GUIDs.

Copy this supplied file:

```text
--8<-- "journey/code/29-identity-04.rs"
```

</details>

### 4. `src/lib.rs`

Compile the file-identity regression check.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod source_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/29-identity-05.rs" skip=31
```

</details>

<a id="checkpoint-29a-snapshot"></a>

### Step 2 of 11 · Write a snapshot from the editable sources (`29a-snapshot`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/11 Visual Trace (29a-snapshot): Write a snapshot from the editable sources</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/document.rs, src/snapshot_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Live objects</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source protobuf copies +</text>
    <text x="316" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">object…</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">flat tree and placements</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">bytes</text>
    
</svg>
</div>

- **Goal:** Serialize live source meshes and their placements without changing the editor.
- **Data Flow Trace:** `Live objects → source protobuf copies + object GUIDs → flat tree and placements → bytes`
- **Files You Touch:** `src/document.rs` (*Copy live sources into one bounded flat document, write placements separately and return bytes without editor mutation.*) · `src/snapshot_tests.rs` (*Optional authoring fixture. Check live row membership, source topology/attributes, separate placement and untouched history.*) · `src/lib.rs` (*Compile the read-only snapshot checks.*)

Build Save bytes from live scene objects, including generated and edited objects and excluding deleted rows.

Copy each source mesh's protobuf message, preserving doubles, topology, colours, names and flags. Replace only the copy's GUID with the stored object identity. Keep geometry local and write placement separately, keyed by that same GUID; baking both would move it twice.

This snapshot writes a flat tree. Camera, selection, display arrays and history are application state and stay outside the file.

Match the current loader's bounds: 1–64 meshes and at most 4 MiB encoded. Reject unsupported source data. Today Rust checks inspect the snapshot; placement loading and browser downloading follow.

> **Self-Check Question:** *Why must saving copy source messages instead of modifying the retained session?*  
> **Answer:** The retained file can contain objects that have since been deleted, and imported source allocations are shared through history. Saving traverses current objects and copies their source messages. It applies the inserted object GUID to each copy and writes placement separately, leaving the original geometry, history and file provenance unchanged.

- **What You Should See:** Run the snapshot checks below. Serialization must read original source coordinates and separate placements without changing the live editor. Browser Save is connected in lesson 29d.
- **Hands-On Lab Experiment:** Change the box placement in the check, then predict whether mesh vertices or the XformEntry changes. Confirm the retained source GUID stays unchanged and restore the check.

<details>
<summary>Implementation 29a-snapshot: Write a snapshot from the editable sources</summary>

Serialize live source meshes and their placements without changing the editor.

Data flow: Live objects → source protobuf copies + object GUIDs → flat tree and placements → bytes.

### 1. `src/document.rs`

Copy live sources into one bounded flat document, write placements separately and return bytes without editor mutation.

<details>
<summary>Locate the existing block</summary>

```rust
fn validate(message: &proto::Session) -> Result<(), &'static str> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/29a-snapshot-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/snapshot_tests.rs`

Optional authoring fixture. Check live row membership, source topology/attributes, separate placement and untouched history.

Copy this supplied file:

```text
--8<-- "journey/code/29a-snapshot-02.rs"
```

</details>

### 3. `src/lib.rs`

Compile the read-only snapshot checks.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod file_identity_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/29a-snapshot-03.rs" skip=38
```

</details>

<a id="checkpoint-29b-placements"></a>

### Step 3 of 11 · Reopen source geometry with its placement (`29b-placements`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/11 Visual Trace (29b-placements): Reopen source geometry with its placement</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/placement.rs, src/lib.rs, src/scene.rs, src/prepared.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Bytes</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">validated mesh IDs and</text>
    <text x="316" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">affine…</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">prepared local sources</text>
    <text x="521" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">plus…</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">inserted objects</text>
    
</svg>
</div>

- **Goal:** Validate stored placements before reconstructing source-backed objects.
- **Data Flow Trace:** `Bytes → validated mesh IDs and affine matrices → prepared local sources plus placement → inserted objects`
- **Files You Touch:** `src/placement.rs` (*Use one affine validation rule for decoded matrices and direct edits.*) · `src/lib.rs` (*Expose the shared matrix validator.*) · `src/scene.rs` (*Make direct placement edits use the same rule as imported files.*) · `src/prepared.rs` (*Carry the validated object placement beside local source geometry.*) · `src/document.rs` (*Retain the validated stored placement without baking it into source vertices.*) · `src/placement_load_tests.rs` (*Optional authoring fixture. Reject all malformed placement families using actual encoded messages.*)

Accept the saved file's separate placement records. Each must identify an existing mesh, occur once, and contain sixteen finite values with an affine final row.

Share this rule through `placement::valid` for loading and scene placement. Validate before constructing `Xform`: its defaults must not silently fill a short file matrix.

`PreparedMesh` now carries a model matrix. Generated geometry starts at identity; loading copies the validated source placement; insertion retains it.

Snapshot and load now use the same complete validator. Tests reject short, non-finite or projective matrices and orphan or duplicate placement entries before scene mutation. Exact round-trip checks come next; the browser download follows those.

> **Self-Check Question:** *Where must a saved transform be applied when reopening?*  
> **Answer:** A saved transform belongs to the object placement. Load prepares the original local kernel mesh and attaches its validated matrix. Scene insertion uses that matrix once. It must not transform source vertices and also retain the placement, or the object moves twice.

- **What You Should See:** Run the reopen checks below. Valid stored placements restore the objects; an invalid matrix must reject the document before changing live rows.
- **Hands-On Lab Experiment:** Change the malformed-placement check’s projective matrix into a finite translation. Predict why validation accepts it, then restore the rejection case. Explain why a fifteen-value matrix remains invalid even if every supplied value is finite.

<details>
<summary>Implementation 29b-placements: Reopen source geometry with its placement</summary>

Validate stored placements before reconstructing source-backed objects.

Data flow: Bytes → validated mesh IDs and affine matrices → prepared local sources plus placement → inserted objects.

### 1. `src/placement.rs`

Use one affine validation rule for decoded matrices and direct edits.

Create the file and type:

```rust
--8<-- "journey/code/29b-placements-01.rs"
```

### 2. `src/lib.rs`

Expose the shared matrix validator.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod prepared;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/29b-placements-02.rs" skip=18
```

### 3. `src/scene.rs`

Make direct placement edits use the same rule as imported files.

<details>
<summary>Locate the existing block</summary>

```rust
        if !model.m.iter().all(|v| v.is_finite())
            || [model.m[3], model.m[7], model.m[11], model.m[15]] != [0.0, 0.0, 0.0, 1.0]
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/29b-placements-03.rs"
```

### 4. `src/prepared.rs`

Carry the validated object placement beside local source geometry. Generated geometry begins with identity placement.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{document::Source, mesh::Mesh};
use std::rc::Rc;

pub struct PreparedMesh {
    pub geometry: Rc<session_rust::Mesh>,
    pub display: Mesh,
    pub source: Option<Source>,
}

impl PreparedMesh {
    pub fn triangle(points: [[f64; 3]; 3], colour: [f32; 3]) -> Result<Self, &'static str> {
        let vertices = points.into_iter().map(|p| session_rust::Point::new(p[0], p[1], p[2])).collect();
        let mut geometry = session_rust::Mesh::from_vertices_and_faces(vertices, vec![vec![0, 1, 2]]);
        geometry.set_objectcolor(session_rust::Color::new(colour[0], colour[1], colour[2], 1.0));
        Self::new(geometry)
    }

    pub fn new(geometry: session_rust::Mesh) -> Result<Self, &'static str> {
        Self::from_shared(Rc::new(geometry))
    }

    pub fn from_shared(geometry: Rc<session_rust::Mesh>) -> Result<Self, &'static str> {
        // The kernel creates a GUID lazily; establish identity before sharing.
        let _ = geometry.guid();
        let display = Mesh::from_kernel(&geometry)?;
        Ok(Self { geometry, display, source: None })
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{document::Source, mesh::Mesh};
use std::rc::Rc;

pub struct PreparedMesh {
    pub geometry: Rc<session_rust::Mesh>,
    pub display: Mesh,
    pub source: Option<Source>,
    pub model: session_rust::Xform,
}

impl PreparedMesh {
    pub fn triangle(points: [[f64; 3]; 3], colour: [f32; 3]) -> Result<Self, &'static str> {
        let vertices = points.into_iter().map(|p| session_rust::Point::new(p[0], p[1], p[2])).collect();
        let mut geometry = session_rust::Mesh::from_vertices_and_faces(vertices, vec![vec![0, 1, 2]]);
        geometry.set_objectcolor(session_rust::Color::new(colour[0], colour[1], colour[2], 1.0));
        Self::new(geometry)
    }

    pub fn new(geometry: session_rust::Mesh) -> Result<Self, &'static str> {
        Self::from_shared(Rc::new(geometry))
    }

    pub fn from_shared(geometry: Rc<session_rust::Mesh>) -> Result<Self, &'static str> {
        // The kernel creates a GUID lazily; establish identity before sharing.
        let _ = geometry.guid();
        let display = Mesh::from_kernel(&geometry)?;
        Ok(Self { geometry, display, source: None, model: session_rust::Xform::identity() })
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/29b-placements-04.rs"
```

```text
--8<-- "journey/code/29b-placements-05.rs"
```

</details>

### 5. `src/document.rs`

Retain the validated stored placement without baking it into source vertices.

<details>
<summary>Locate the existing block</summary>

```rust
        let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/29b-placements-06.rs" skip=72
```

### 6. `src/scene.rs`

Insert each prepared model exactly once.

<details>
<summary>Locate the existing block</summary>

```rust
            model: session_rust::Xform::identity(), source: prepared.source });
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/29b-placements-07.rs"
```

### 7. `src/document.rs`

Remove the temporary source-only validation comment. Validate the complete saved message now that the loader accepts placements. Accept placements while keeping the other unsupported source families explicit. Reject unknown, duplicate, incomplete, non-finite or projective placement records before kernel construction.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        }
        validate_mesh(mesh)?;
    }
    if let Some(root) = message.tree.as_ref().and_then(|tree| tree.root.as_ref()) {
        let names: HashSet<_> = root.children.iter().map(|node| node.name.as_str()).collect();
        if names != guids || root.children.len() != guids.len()
```

</details>

Replace it with:

```rust
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
```

<details>
<summary>Locate this block</summary>

```rust

fn validate(message: &proto::Session) -> Result<(), &'static str> {
    let objects = message.objects.as_ref().ok_or("Session has no objects")?;
    if !message.xforms.is_empty() || message.definitions.is_some() || !message.interactions.is_empty() {
        return Err("Placements, definitions and interactions arrive in later checkpoints");
    }
    let other = objects.points.len() + objects.lines.len() + objects.planes.len()
        + objects.bboxes.len() + objects.polylines.len() + objects.pointclouds.len()
```

</details>

Replace it with:

```rust

fn validate(message: &proto::Session) -> Result<(), &'static str> {
    let objects = message.objects.as_ref().ok_or("Session has no objects")?;
    if message.definitions.is_some() || !message.interactions.is_empty() {
        return Err("Definitions and interactions arrive in later checkpoints");
    }
    let other = objects.points.len() + objects.lines.len() + objects.planes.len()
        + objects.bboxes.len() + objects.polylines.len() + objects.pointclouds.len()
```

<details>
<summary>Locate this block</summary>

```rust
            name: "Objects".into(), children, ..Default::default()
        }), ..Default::default() }), xforms, ..Default::default()
    };
    // Source validation; placements come next.
    let mut geometry = message.clone(); geometry.xforms.clear();
    validate(&geometry)?;
    let bytes = message.encode_to_vec();
    if bytes.len() > MAX_BYTES { return Err("The saved document exceeds 4 MiB"); }
    Ok(bytes)
```

</details>

Replace it with:

```rust
            name: "Objects".into(), children, ..Default::default()
        }), ..Default::default() }), xforms, ..Default::default()
    };
    validate(&message)?;
    let bytes = message.encode_to_vec();
    if bytes.len() > MAX_BYTES { return Err("The saved document exceeds 4 MiB"); }
    Ok(bytes)
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
}

pub struct Loaded {
    pub document: Rc<Session>,
    pub meshes: Vec<PreparedMesh>,
}

pub fn load(bytes: &[u8]) -> Result<Loaded, &'static str> {
    if bytes.len() > MAX_BYTES { return Err("This checkpoint accepts files up to 4 MiB"); }
    let message = proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")?;
    validate(&message)?;
    let document = Rc::new(Session::from_proto(message).map_err(|_| "Cannot construct session")?);
    let mut meshes = Vec::new();
    for mesh in document.objects.meshes.iter() {
        let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
        prepared.model = document.xforms.get(mesh.guid()).cloned().unwrap_or_else(session_rust::Xform::identity);
        prepared.source = Some(Source {
            document: Rc::clone(&document), guid: mesh.guid().to_owned(),
        });
        meshes.push(prepared);
    }
    Ok(Loaded { document, meshes })
}

pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    let mut objects = proto::Objects::default();
    let mut children = Vec::new();
    let mut xforms = Vec::new();
    for object in scene.objects() {
        let mut mesh = object.geometry.to_proto();
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
--8<-- "journey/code/29b-placements-09.rs"
```

```text
--8<-- "journey/code/29b-placements-10.rs"
```

```text
--8<-- "journey/code/29b-placements-11.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/placement_load_tests.rs`

Optional authoring fixture. Reject all malformed placement families using actual encoded messages.

Copy this supplied file:

```text
--8<-- "journey/code/29b-placements-12.rs"
```

</details>

### 9. `src/lib.rs`

Compile the malformed-input checks.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod snapshot_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/29b-placements-13.rs" skip=33
```

</details>

<a id="checkpoint-29c-roundtrip"></a>

### Step 4 of 11 · Prove the saved document reopens faithfully (`29c-roundtrip`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 4/11 Visual Trace (29c-roundtrip): Prove the saved document reopens faithfully</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/save_roundtrip_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Live source and placement</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">snapshot bytes</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">load</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">identical source values</text>
    <text x="726" y="81" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">and…</text>
    
</svg>
</div>

- **Goal:** Check exact local source data, attributes, identities and placement through saving and loading.
- **Data Flow Trace:** `Live source and placement → snapshot bytes → load → identical source values and object matrices`
- **Files You Touch:** `src/save_roundtrip_tests.rs` (*Optional authoring fixture. Compare exact source values, stored identities and object matrices after encoding and decoding.*) · `src/lib.rs` (*Compile exact document round-trip checks.*)

Prove snapshot and load preserve the editable document before adding browser downloading.

Save and reopen a triangle with a double coordinate that rounds in its display, a name, hide/lock flags and translation. Compare original source values exactly; derive display again and keep placement separate.

Then save duplicate imports. Reopened GUIDs must match their stored object GUIDs, including the duplicate's new identity; matrices must match too. Imported sources stay untouched.

Compare decoded structure rather than protobuf byte order. Pixels cannot establish source precision or identity. Flags are retained here; drawing and editing policies for hidden/locked objects come later.

> **Self-Check Question:** *What can a screenshot prove about saved precision and identity?*  
> **Answer:** A screenshot proves the drawing, not exact saved doubles or identifiers. Native round-trip checks compare original vertices, names, attributes, GUIDs and matrices. Duplicate-import checks also ensure separate objects survive one saved file even when they came from the same source GUID.

- **What You Should See:** Run the round-trip checks below. Reopened geometry must retain exact doubles, attributes, saved identities and placements. Compare the source values, not the float display.
- **Hands-On Lab Experiment:** Change the exact coordinate to 123456789.125. Predict why the reopened kernel coordinate matches exactly even though the display uses floats. Then inspect corresponding imported copies’ saved GUIDs. Restore the checks.

<details>
<summary>Implementation 29c-roundtrip: Prove the saved document reopens faithfully</summary>

Check exact local source data, attributes, identities and placement through saving and loading.

Data flow: Live source and placement → snapshot bytes → load → identical source values and object matrices.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/save_roundtrip_tests.rs`

Optional authoring fixture. Compare exact source values, stored identities and object matrices after encoding and decoding.

Copy this supplied file:

```text
--8<-- "journey/code/29c-roundtrip-01.rs"
```

</details>

### 2. `src/lib.rs`

Compile exact document round-trip checks.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod placement_load_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/29c-roundtrip-02.rs" skip=39
```

</details>

<a id="checkpoint-29d-save"></a>

### Step 5 of 11 · Download the editable document from the command line (`29d-save`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 5/11 Visual Trace (29d-save): Download the editable document from the command line</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: Cargo.toml, src/file_output.rs, src/lib.rs, src/browser.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="91" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Typed Save</text>
    <line x1="164" y1="68" x2="180" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="182" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="255" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source snapshot</text>
    <line x1="328" y1="68" x2="344" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="346" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">download</text>
    <line x1="492" y1="68" x2="508" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="510" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="583" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Open</text>
    <line x1="656" y1="68" x2="672" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="674" y="42" width="146" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="747" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">restored source and</text>
    <text x="747" y="81" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">placement</text>
    
</svg>
</div>

- **Goal:** Connect Save to a browser download and reopen the actual downloaded document.
- **Data Flow Trace:** `Typed Save → source snapshot → Blob URL → download → Open → restored source and placement`
- **Files You Touch:** `Cargo.toml` (*Enable the browser object URL and download anchor APIs. These features do not change dependency versions.*) · `src/file_output.rs` (*Create a temporary file owner, request a download and schedule one-shot URL cleanup.*) · `src/lib.rs` (*Expose the browser file-output adapter.*) · `src/browser.rs` (*Offer Save through command completion in the actual dock.*) · `index.html` (*Let the native file picker offer both original specimens and downloaded session files.*)

Connect typed `Save` to `snapshot` and a browser download. Snapshot errors go to command history and start no download.

Copy bytes into `Uint8Array`, wrap them in `Blob`, and create an object URL. A temporary hidden anchor downloads `viewer.session` and is removed; it is not a viewer feature control.

Keep the URL alive for ten seconds, then revoke it with a one-shot callback owning its `String`. If timer scheduling fails, clean up immediately and report the error.

Save reads the document without creating a history edit. The Chrome check downloads and reopens that file, preserving the prior Move's Undo/Redo and the reconstructed scene picture.

> **Self-Check Question:** *Why does Save belong outside document history?*  
> **Answer:** Save reads the current scene and creates bytes; it does not edit geometry, placement or selection. It must not consume an Undo step. The browser owns file downloading, while the native snapshot and loader own the document contract.

- **What You Should See:** Open `sample.pb`, select an object and move it. Type `Save`, then reopen the downloaded `viewer.session`. The placed scene should return; Save itself must not consume Undo.
- **Hands-On Lab Experiment:** Save after moving the box, then move it again and save to a second file. Reopen each into a cleared scene and predict which placement it restores. Saving should leave the next Undo step unchanged.

<details>
<summary>Implementation 29d-save: Download the editable document from the command line</summary>

Connect Save to a browser download and reopen the actual downloaded document.

Data flow: Typed Save → source snapshot → Blob URL → download → Open → restored source and placement.

### 1. `Cargo.toml`

Enable the browser object URL and download anchor APIs. These features do not change dependency versions.

<details>
<summary>Locate the existing block</summary>

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions"] }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```toml
--8<-- "journey/code/29d-save-01.toml"
```

### 2. `src/file_output.rs`

Create a temporary file owner, request a download and schedule one-shot URL cleanup.

Create the file and type:

```rust
--8<-- "journey/code/29d-save-02.rs"
```

### 3. `src/lib.rs`

Expose the browser file-output adapter.

<details>
<summary>Locate the existing block</summary>

```rust
mod file_input;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/29d-save-03.rs" skip=16
```

### 4. `src/browser.rs`

Offer Save through command completion in the actual dock. Read the scene for Save without creating a document action or consuming history.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
                    input.click();
                }
                None
            } else if line == "move" || line.starts_with("move ") {
                match crate::offset::parse(&line) {
                    Ok(offset) => Some(Action::Translate(offset)),
```

</details>

Replace it with:

```rust
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
```

<details>
<summary>Locate this block</summary>

```rust
        &[
            "Help",
            "Open",
            "Example Box",
            "Example Triangle",
            "Select Next",
```

</details>

Replace it with:

```rust
        &[
            "Help",
            "Open",
            "Save",
            "Example Box",
            "Example Triangle",
            "Select Next",
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
            if message.starts_with("Cannot") { let _ = status.remove_attribute("hidden"); }
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
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &[
            "Help",
            "Open",
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
    let request = std::rc::Rc::new(std::cell::Cell::new(0));
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

        let action = if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request));
            None
        } else if event.type_() == "viewer-file" {
            crate::file_input::bytes(&event).map(Action::Import)
        } else if let Some(line) = line {
            if line == "open" {
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
            match editor.apply(action) {
                Ok(Change::Scene) => renderer.set_scene(&editor.scene, editor.selected),
                Ok(Change::View) => {}
                Err(error) => {
                    report(error);
                    panel.result(error);
                }
            }
        }
        if let Err(error) = show_projection(&editor.camera) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
        if event.type_() == "viewer-file" {
            report("File imported. Undo removes the entire import.");
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
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}

fn show_projection(camera: &crate::camera::Camera) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    canvas.set_attribute("data-projection", &format!("{:?}", camera.projection))
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
--8<-- "journey/code/29d-save-04.rs"
```

```text
--8<-- "journey/code/29d-save-05.rs"
```

</details>

### 5. `index.html`

Let the native file picker offer both original specimens and downloaded session files.

<details>
<summary>Locate the existing block</summary>

```html
  <input id="open" type="file" accept=".pb" hidden>
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```html
--8<-- "journey/code/29d-save-06.html"
```

</details>

<a id="checkpoint-30-feedback"></a>

### Step 6 of 11 · Report the result that actually committed (`30-feedback`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 6/11 Visual Trace (30-feedback): Report the result that actually committed</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/browser.rs, src/file_input.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">File read result</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">browser event</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">validated editor action</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">success or error in the</text>
    <text x="726" y="81" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">command…</text>
    
</svg>
</div>

- **Goal:** Keep failed reads and imports visible in command history without claiming success.
- **Data Flow Trace:** `File read result → browser event → validated editor action → success or error in the command dock`
- **Files You Touch:** `src/browser.rs` (*Keep a startup failure visible when the GPU dock could not be created; file errors stay in command history.*) · `src/file_input.rs` (*Route the size refusal through the panel’s main event closure.*)

Report import success only after `Editor::apply` commits successfully. A failed decode retains the scene and records an error instead of saying “File imported.”

An asynchronous file adapter cannot borrow the callback's Panel. Dispatch `viewer-file-error` with text, then let the owning callback update history.

Queue oversized-file errors before reading. Immediate nested dispatch would re-enter the borrowed `FnMut` callback. After an awaited read, retain the latest-request check so stale work cannot publish an error.

Command history carries file feedback; the hidden status stays out of the drawing. Startup failure can still expose status when the GPU dock never became available.

> **Self-Check Question:** *Why must import success be reported inside the successful action branch?*  
> **Answer:** Receiving bytes does not mean they decoded or entered the scene. Editor::apply can reject a file while preserving scene, selection and history. Only a successful Scene change proves the import committed; an unconditional message after the match would erase the error.

- **What You Should See:** Try opening a text file renamed `.pb`. The command history must show the read/import failure and leave the current document unchanged.
- **Hands-On Lab Experiment:** Temporarily move the success report back outside the action match. Predict the final status after malformed bytes, reproduce the contradiction, then restore the successful-branch report.

<details>
<summary>Implementation 30-feedback: Report the result that actually committed</summary>

Keep failed reads and imports visible in command history without claiming success.

Data flow: File read result → browser event → validated editor action → success or error in the command dock.

### 1. `src/browser.rs`

Keep a startup failure visible when the GPU dock could not be created; file errors stay in command history.

<details>
<summary>Locate the existing block</summary>

```rust
            if message.starts_with("Cannot") { let _ = status.remove_attribute("hidden"); }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/30-feedback-01.rs"
```

### 2. `src/file_input.rs`

Route the size refusal through the panel’s main event closure. Deliver a read error only after the existing latest-read check. Pass asynchronous errors through a small custom event rather than borrowing the panel.

<details>
<summary>Locate the existing block</summary>

```rust
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};

pub fn choose(event: &web_sys::Event, request: Rc<Cell<u64>>) {
    let Some(input) = event.target().and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok()) else { return; };
    if input.id() != "open" { return; }
    let Some(file) = input.files().and_then(|files| files.get(0)) else { return; };
    input.set_value("");
    let id = request.get() + 1;
    request.set(id);
    if file.size() > crate::document::MAX_BYTES as f64 {
        super::browser::report("This checkpoint accepts files up to 4 MiB");
        return;
    }
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        if request.get() != id { return; }
        let result = result.and_then(deliver);
        if let Err(error) = result {
            super::browser::report(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn deliver(buffer: JsValue) -> Result<(), JsValue> {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&js_sys::Uint8Array::new(&buffer));
    let event = web_sys::CustomEvent::new_with_event_init_dict("viewer-file", &detail)?;
    web_sys::window().ok_or("No browser window")?.dispatch_event(&event)?;
    Ok(())
}

pub fn bytes(event: &web_sys::Event) -> Option<Vec<u8>> {
    let event = event.dyn_ref::<web_sys::CustomEvent>()?;
    let array = event.detail().dyn_into::<js_sys::Uint8Array>().ok()?;
    if array.length() as usize > crate::document::MAX_BYTES { return None; }
    Some(array.to_vec())
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};

pub fn choose(event: &web_sys::Event, request: Rc<Cell<u64>>) {
    let Some(input) = event.target().and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok()) else { return; };
    if input.id() != "open" { return; }
    let Some(file) = input.files().and_then(|files| files.get(0)) else { return; };
    input.set_value("");
    let id = request.get() + 1;
    request.set(id);
    if file.size() > crate::document::MAX_BYTES as f64 {
        failure("This checkpoint accepts files up to 4 MiB");
        return;
    }
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        if request.get() != id { return; }
        let result = result.and_then(deliver);
        if let Err(error) = result {
            failure(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn failure(message: &str) {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&JsValue::from_str(message));
    let result = web_sys::CustomEvent::new_with_event_init_dict("viewer-file-error", &detail)
        .and_then(|event| web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?.dispatch_event(&event));
    if result.is_err() { super::browser::report(message); }
}

fn deliver(buffer: JsValue) -> Result<(), JsValue> {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&js_sys::Uint8Array::new(&buffer));
    let event = web_sys::CustomEvent::new_with_event_init_dict("viewer-file", &detail)?;
    web_sys::window().ok_or("No browser window")?.dispatch_event(&event)?;
    Ok(())
}

pub fn bytes(event: &web_sys::Event) -> Option<Vec<u8>> {
    let event = event.dyn_ref::<web_sys::CustomEvent>()?;
    let array = event.detail().dyn_into::<js_sys::Uint8Array>().ok()?;
    if array.length() as usize > crate::document::MAX_BYTES { return None; }
    Some(array.to_vec())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/30-feedback-02.rs"
```

```text
--8<-- "journey/code/30-feedback-03.rs"
```

```text
--8<-- "journey/code/30-feedback-04.rs"
```

</details>

### 3. `src/browser.rs`

Receive read failures in the same closure that owns command history. A successful transaction is the only branch that may announce an imported file. Remove the unconditional success that overwrote decoding errors. Keep the error event connected for the viewer lifetime.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
```

</details>

Replace it with:

```rust
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file-error", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
```

<details>
<summary>Locate this block</summary>

```rust
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
        if event.type_() == "viewer-file" {
            report("File imported. Undo removes the entire import.");
        }
        if resize(
            &browser_window,
            &pointer_canvas,
```

</details>

Replace it with:

```rust
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
        if resize(
            &browser_window,
            &pointer_canvas,
```

<details>
<summary>Locate this block</summary>

```rust
        };
        if let Some(action) = action {
            match editor.apply(action) {
                Ok(Change::Scene) => renderer.set_scene(&editor.scene, editor.selected),
                Ok(Change::View) => {}
                Err(error) => {
                    report(error);
```

</details>

Replace it with:

```rust
        };
        if let Some(action) = action {
            match editor.apply(action) {
                Ok(Change::Scene) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if event.type_() == "viewer-file" {
                        report("File imported. Undo removes the entire import.");
                    }
                }
                Ok(Change::View) => {}
                Err(error) => {
                    report(error);
```

<details>
<summary>Locate this block</summary>

```rust
        let action = if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request));
            None
        } else if event.type_() == "viewer-file" {
            crate::file_input::bytes(&event).map(Action::Import)
        } else if let Some(line) = line {
```

</details>

Replace it with:

```rust
        let action = if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.result(&message);
            }
            None
        } else if event.type_() == "viewer-file" {
            crate::file_input::bytes(&event).map(Action::Import)
        } else if let Some(line) = line {
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
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &[
            "Help",
            "Open",
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
    let request = std::rc::Rc::new(std::cell::Cell::new(0));
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

        let action = if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.result(&message);
            }
            None
        } else if event.type_() == "viewer-file" {
            crate::file_input::bytes(&event).map(Action::Import)
        } else if let Some(line) = line {
            if line == "open" {
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
            match editor.apply(action) {
                Ok(Change::Scene) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if event.type_() == "viewer-file" {
                        report("File imported. Undo removes the entire import.");
                    }
                }
                Ok(Change::View) => {}
                Err(error) => {
                    report(error);
                    panel.result(error);
                }
            }
        }
        if let Err(error) = show_projection(&editor.camera) {
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
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file-error", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}

fn show_projection(camera: &crate::camera::Camera) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    canvas.set_attribute("data-projection", &format!("{:?}", camera.projection))
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
--8<-- "journey/code/30-feedback-05.rs"
```

```text
--8<-- "journey/code/30-feedback-06.rs"
```

```text
--8<-- "journey/code/30-feedback-08.rs"
```

</details>

### 4. `src/file_input.rs`

Move the size refusal out of the currently borrowed input callback. Check size in the queued task before reading; error delivery can now enter the main callback safely.

<details>
<summary>Locate the existing block</summary>

```rust
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};

pub fn choose(event: &web_sys::Event, request: Rc<Cell<u64>>) {
    let Some(input) = event.target().and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok()) else { return; };
    if input.id() != "open" { return; }
    let Some(file) = input.files().and_then(|files| files.get(0)) else { return; };
    input.set_value("");
    let id = request.get() + 1;
    request.set(id);
    if file.size() > crate::document::MAX_BYTES as f64 {
        failure("This checkpoint accepts files up to 4 MiB");
        return;
    }
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        if request.get() != id { return; }
        let result = result.and_then(deliver);
        if let Err(error) = result {
            failure(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn failure(message: &str) {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&JsValue::from_str(message));
    let result = web_sys::CustomEvent::new_with_event_init_dict("viewer-file-error", &detail)
        .and_then(|event| web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?.dispatch_event(&event));
    if result.is_err() { super::browser::report(message); }
}

fn deliver(buffer: JsValue) -> Result<(), JsValue> {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&js_sys::Uint8Array::new(&buffer));
    let event = web_sys::CustomEvent::new_with_event_init_dict("viewer-file", &detail)?;
    web_sys::window().ok_or("No browser window")?.dispatch_event(&event)?;
    Ok(())
}

pub fn bytes(event: &web_sys::Event) -> Option<Vec<u8>> {
    let event = event.dyn_ref::<web_sys::CustomEvent>()?;
    let array = event.detail().dyn_into::<js_sys::Uint8Array>().ok()?;
    if array.length() as usize > crate::document::MAX_BYTES { return None; }
    Some(array.to_vec())
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};

pub fn choose(event: &web_sys::Event, request: Rc<Cell<u64>>) {
    let Some(input) = event.target().and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok()) else { return; };
    if input.id() != "open" { return; }
    let Some(file) = input.files().and_then(|files| files.get(0)) else { return; };
    input.set_value("");
    let id = request.get() + 1;
    request.set(id);
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        if request.get() != id { return; }
        if file.size() > crate::document::MAX_BYTES as f64 {
            failure("This checkpoint accepts files up to 4 MiB");
            return;
        }
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        if request.get() != id { return; }
        let result = result.and_then(deliver);
        if let Err(error) = result {
            failure(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn failure(message: &str) {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&JsValue::from_str(message));
    let result = web_sys::CustomEvent::new_with_event_init_dict("viewer-file-error", &detail)
        .and_then(|event| web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?.dispatch_event(&event));
    if result.is_err() { super::browser::report(message); }
}

fn deliver(buffer: JsValue) -> Result<(), JsValue> {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&js_sys::Uint8Array::new(&buffer));
    let event = web_sys::CustomEvent::new_with_event_init_dict("viewer-file", &detail)?;
    web_sys::window().ok_or("No browser window")?.dispatch_event(&event)?;
    Ok(())
}

pub fn bytes(event: &web_sys::Event) -> Option<Vec<u8>> {
    let event = event.dyn_ref::<web_sys::CustomEvent>()?;
    let array = event.detail().dyn_into::<js_sys::Uint8Array>().ok()?;
    if array.length() as usize > crate::document::MAX_BYTES { return None; }
    Some(array.to_vec())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/30-feedback-10.rs"
```

</details>

</details>

<a id="checkpoint-30a-tickets"></a>

### Step 7 of 11 · Give a pending read an explicit ticket (`30a-tickets`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 7/11 Visual Trace (30a-tickets): Give a pending read an explicit ticket</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/read_gate.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Begin read</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">pending ticket</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">supersede, cancel or finish once</text>
    
</svg>
</div>

- **Goal:** Describe latest-read ownership, cancellation and one-shot completion in ordinary Rust.
- **Data Flow Trace:** `Begin read → pending ticket → supersede, cancel or finish once`
- **Files You Touch:** `src/read_gate.rs` (*Own one pending ticket and test stale, duplicate, cancelled and exhausted requests independently of browser APIs.*) · `src/lib.rs` (*Expose the native request model without changing the browser adapter yet.*)

The browser currently compares a loose counter before delivering a read. We need to express more than newest number: a cancelled or already completed read must no longer be allowed to publish a result.

Create ReadGate as ordinary Rust. begin issues a checked increasing ticket and makes it the sole pending read. finish returns true only for that pending ticket and consumes it. cancel removes pending ownership without reusing the issued number.

Option distinguishes no pending read from a particular ticket. The counter names requests, while pending determines which request may still commit. Those are separate responsibilities.

begin clears pending before checking the next number. If the counter is exhausted, it returns an error and does not leave an older read able to commit after a refused new request. It never wraps back to an ID a previous operation might still hold.

The checks simulate completions in the wrong order, cancellation, duplicate completion and counter exhaustion. They need no window or GPU. This checkpoint tests the ticket model; the next lesson adopts it in the running file adapter.

> **Self-Check Question:** *Why must a stale completion leave the newer pending ticket intact?*  
> **Answer:** The old operation owns only its own ticket. If rejecting it also cleared pending state, it would cancel the newer operation. finish compares IDs first, consumes only the matching current ticket, and rejects duplicate completions after it has been consumed.

- **What You Should See:** Run the ReadGate checks below. Start two tickets and complete the older one first: only the current ticket may be accepted. Browser integration follows next.
- **Hands-On Lab Experiment:** Start three tickets and finish them in reverse order. Predict why only the newest succeeds and why the rejected older completions cannot clear it. Restore the original checks.

<details>
<summary>Implementation 30a-tickets: Give a pending read an explicit ticket</summary>

Describe latest-read ownership, cancellation and one-shot completion in ordinary Rust.

Data flow: Begin read → pending ticket → supersede, cancel or finish once.

### 1. `src/read_gate.rs`

Own one pending ticket and test stale, duplicate, cancelled and exhausted requests independently of browser APIs.

Create the file and type:

```rust
--8<-- "journey/code/30a-tickets-01.rs"
```

### 2. `src/lib.rs`

Expose the native request model without changing the browser adapter yet.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod document;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/30a-tickets-02.rs" skip=18
```

</details>

<a id="checkpoint-30b-cancel"></a>

### Step 8 of 11 · Cancel reads without accepting their late result (`30b-cancel`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 8/11 Visual Trace (30b-cancel): Cancel reads without accepting their late result</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/file_input.rs, src/browser.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Selected file</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">ticket-owned task</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">consume current ticket</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">deliver outcome; Cancel</text>
    <text x="726" y="81" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">Open…</text>
    
</svg>
</div>

- **Goal:** Adopt one-shot tickets in the browser and give asynchronous results their own history entry.
- **Data Flow Trace:** `Selected file → ticket-owned task → consume current ticket → deliver outcome; Cancel Open revokes ownership`
- **Files You Touch:** `src/file_input.rs` (*Share the native gate through short interior mutable borrows.*) · `src/browser.rs` (*Expose the initial row count through hidden diagnostics.*)

Use `Rc<RefCell<ReadGate>>` to share a ticket gate between the command callback and read tasks. Keep mutable borrows short and release them before dispatching events.

A chosen file begins a ticket. Check it before reading; consume it once with `finish` after completion. Stale or cancelled tasks deliver neither bytes nor errors.

Typed `Cancel Open` revokes ownership without editing the document. It does not physically abort `File.arrayBuffer`; its eventual result is ignored.

Append asynchronous feedback as an Open history entry so it cannot overwrite a newer command's result. A hidden object-count attribute lets Chrome detect duplicate insertion even when its pixels overlap perfectly.

> **Self-Check Question:** *Does cancelling a read mean its promise stops running?*  
> **Answer:** No. File.arrayBuffer has no abort operation here. Cancellation revokes permission to deliver its result. The task may finish later, but finish rejects its ticket and neither scene nor command feedback changes. A separate result entry also prevents a read outcome from rewriting a later typed command.

- **What You Should See:** Start `Open`, then type `Cancel Open` before its read completes. A late result must not enter the document or replace a newer command’s history entry.
- **Hands-On Lab Experiment:** Hold a read in a debugger, type View Isometric and Cancel Open, then let the read finish. Predict why neither the object count nor command history should change when that cancelled task returns.

<details>
<summary>Implementation 30b-cancel: Cancel reads without accepting their late result</summary>

Adopt one-shot tickets in the browser and give asynchronous results their own history entry.

Data flow: Selected file → ticket-owned task → consume current ticket → deliver outcome; Cancel Open revokes ownership.

### 1. `src/file_input.rs`

Share the native gate through short interior mutable borrows. Receive the same pending-read owner as the command callback. Issue a bounded ticket; even an exhaustion error must be delivered after the input callback returns. Avoid work for a ticket already superseded or cancelled before its queued task starts. Consume the current ticket before reporting a size refusal, releasing the RefCell borrow before dispatch. Only a current one-shot completion may deliver bytes or a read error.

<details>
<summary>Locate the existing block</summary>

```rust
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};

pub fn choose(event: &web_sys::Event, request: Rc<Cell<u64>>) {
    let Some(input) = event.target().and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok()) else { return; };
    if input.id() != "open" { return; }
    let Some(file) = input.files().and_then(|files| files.get(0)) else { return; };
    input.set_value("");
    let id = request.get() + 1;
    request.set(id);
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        if request.get() != id { return; }
        if file.size() > crate::document::MAX_BYTES as f64 {
            failure("This checkpoint accepts files up to 4 MiB");
            return;
        }
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        if request.get() != id { return; }
        let result = result.and_then(deliver);
        if let Err(error) = result {
            failure(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn failure(message: &str) {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&JsValue::from_str(message));
    let result = web_sys::CustomEvent::new_with_event_init_dict("viewer-file-error", &detail)
        .and_then(|event| web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?.dispatch_event(&event));
    if result.is_err() { super::browser::report(message); }
}

fn deliver(buffer: JsValue) -> Result<(), JsValue> {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&js_sys::Uint8Array::new(&buffer));
    let event = web_sys::CustomEvent::new_with_event_init_dict("viewer-file", &detail)?;
    web_sys::window().ok_or("No browser window")?.dispatch_event(&event)?;
    Ok(())
}

pub fn bytes(event: &web_sys::Event) -> Option<Vec<u8>> {
    let event = event.dyn_ref::<web_sys::CustomEvent>()?;
    let array = event.detail().dyn_into::<js_sys::Uint8Array>().ok()?;
    if array.length() as usize > crate::document::MAX_BYTES { return None; }
    Some(array.to_vec())
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};

pub fn choose(event: &web_sys::Event, request: Rc<RefCell<crate::read_gate::ReadGate>>) {
    let Some(input) = event.target().and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok()) else { return; };
    if input.id() != "open" { return; }
    let Some(file) = input.files().and_then(|files| files.get(0)) else { return; };
    input.set_value("");
    let id = match request.borrow_mut().begin() {
        Ok(id) => id,
        Err(error) => {
            wasm_bindgen_futures::spawn_local(async move { failure(error); });
            return;
        }
    };
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        if request.borrow().pending() != Some(id) { return; }
        if file.size() > crate::document::MAX_BYTES as f64 {
            let accepted = request.borrow_mut().finish(id);
            if accepted { failure("This checkpoint accepts files up to 4 MiB"); }
            return;
        }
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        let accepted = request.borrow_mut().finish(id);
        if !accepted { return; }
        let result = result.and_then(deliver);
        if let Err(error) = result {
            failure(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn failure(message: &str) {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&JsValue::from_str(message));
    let result = web_sys::CustomEvent::new_with_event_init_dict("viewer-file-error", &detail)
        .and_then(|event| web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?.dispatch_event(&event));
    if result.is_err() { super::browser::report(message); }
}

fn deliver(buffer: JsValue) -> Result<(), JsValue> {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&js_sys::Uint8Array::new(&buffer));
    let event = web_sys::CustomEvent::new_with_event_init_dict("viewer-file", &detail)?;
    web_sys::window().ok_or("No browser window")?.dispatch_event(&event)?;
    Ok(())
}

pub fn bytes(event: &web_sys::Event) -> Option<Vec<u8>> {
    let event = event.dyn_ref::<web_sys::CustomEvent>()?;
    let array = event.detail().dyn_into::<js_sys::Uint8Array>().ok()?;
    if array.length() as usize > crate::document::MAX_BYTES { return None; }
    Some(array.to_vec())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/30b-cancel-01.rs"
```

```text
--8<-- "journey/code/30b-cancel-02.rs"
```

```text
--8<-- "journey/code/30b-cancel-03.rs"
```

```text
--8<-- "journey/code/30b-cancel-04.rs"
```

```text
--8<-- "journey/code/30b-cancel-05.rs"
```

```text
--8<-- "journey/code/30b-cancel-06.rs"
```

</details>

### 2. `src/browser.rs`

Expose the initial row count through hidden diagnostics. Offer read cancellation as a command rather than a keyboard feature shortcut. Let the input callback and queued tasks share one native gate. An asynchronous read error gets its own history entry instead of rewriting a later command. Revoke pending delivery without touching the scene or edit history. Record asynchronous success separately from subsequent typed commands. Distinguish a file result from a synchronous error on the current command. Update hidden diagnostics after every handled event. Keep scene count and projection in one diagnostic boundary. Report rows independently of overlapping drawing pixels.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    Ok(())
}

fn show_projection(camera: &crate::camera::Camera) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    canvas.set_attribute("data-projection", &format!("{:?}", camera.projection))
}

fn navigation_action(
```

</details>

Replace it with:

```rust
    Ok(())
}

fn inspect(editor: &Editor) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    canvas.set_attribute("data-projection", &format!("{:?}", editor.camera.projection))
}

fn navigation_action(
```

<details>
<summary>Locate this block</summary>

```rust
                    renderer.set_scene(&editor.scene, editor.selected);
                    if event.type_() == "viewer-file" {
                        report("File imported. Undo removes the entire import.");
                    }
                }
                Ok(Change::View) => {}
                Err(error) => {
                    report(error);
                    panel.result(error);
                }
            }
        }
        if let Err(error) = show_projection(&editor.camera) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
```

</details>

Replace it with:

```rust
                    renderer.set_scene(&editor.scene, editor.selected);
                    if event.type_() == "viewer-file" {
                        report("File imported. Undo removes the entire import.");
                        panel.answer("Open", "File imported. Undo removes the entire import.");
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
        if let Err(error) = inspect(&editor) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
```

<details>
<summary>Locate this block</summary>

```rust
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.result(&message);
            }
            None
        } else if event.type_() == "viewer-file" {
            crate::file_input::bytes(&event).map(Action::Import)
        } else if let Some(line) = line {
            if line == "open" {
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
```

</details>

Replace it with:

```rust
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-file" {
            crate::file_input::bytes(&event).map(Action::Import)
        } else if let Some(line) = line {
            if line == "cancel open" {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            } else if line == "open" {
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
```

<details>
<summary>Locate this block</summary>

```rust
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let request = std::rc::Rc::new(std::cell::Cell::new(0));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
```

</details>

Replace it with:

```rust
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
```

<details>
<summary>Locate this block</summary>

```rust
        &[
            "Help",
            "Open",
            "Save",
            "Example Box",
            "Example Triangle",
```

</details>

Replace it with:

```rust
        &[
            "Help",
            "Open",
            "Cancel Open",
            "Save",
            "Example Box",
            "Example Triangle",
```

<details>
<summary>Locate this block</summary>

```rust
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = Editor::default();
    let mut renderer = Renderer::new(
        device,
        queue,
```

</details>

Replace it with:

```rust
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = Editor::default();
    inspect(&editor)?;
    let mut renderer = Renderer::new(
        device,
        queue,
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
    inspect(&editor)?;
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    renderer.resize(Viewport { width, height });
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &[
            "Help",
            "Open",
            "Cancel Open",
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

        let action = if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-file" {
            crate::file_input::bytes(&event).map(Action::Import)
        } else if let Some(line) = line {
            if line == "cancel open" {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            } else if line == "open" {
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
            match editor.apply(action) {
                Ok(Change::Scene) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if event.type_() == "viewer-file" {
                        report("File imported. Undo removes the entire import.");
                        panel.answer("Open", "File imported. Undo removes the entire import.");
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
        if let Err(error) = inspect(&editor) {
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
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file-error", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}

fn inspect(editor: &Editor) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
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
--8<-- "journey/code/30b-cancel-07.rs"
```

```text
--8<-- "journey/code/30b-cancel-08.rs"
```

```text
--8<-- "journey/code/30b-cancel-09.rs"
```

```text
--8<-- "journey/code/30b-cancel-10.rs"
```

```text
--8<-- "journey/code/30b-cancel-11.rs"
```

```text
--8<-- "journey/code/30b-cancel-12.rs"
```

```text
--8<-- "journey/code/30b-cancel-13.rs"
```

```text
--8<-- "journey/code/30b-cancel-14.rs"
```

```text
--8<-- "journey/code/30b-cancel-15.rs"
```

```text
--8<-- "journey/code/30b-cancel-16.rs"
```

</details>

</details>

<a id="checkpoint-30c-replace"></a>

### Step 9 of 11 · Replace a document as one reversible change (`30c-replace`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 9/11 Visual Trace (30c-replace): Replace a document as one reversible change</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/scene.rs, src/editor.rs, src/replace_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Replacement bytes</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">validated prepared sources</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">history transaction</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">new rows with fresh local</text>
    <text x="726" y="81" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">IDs</text>
    
</svg>
</div>

- **Goal:** Prepare a whole replacement before changing live rows, then commit it through existing history.
- **Data Flow Trace:** `Replacement bytes → validated prepared sources → history transaction → new rows with fresh local IDs`
- **Files You Touch:** `src/scene.rs` (*Replace rows while preserving the issued local-ID counter; the caller supplies the transaction.*) · `src/editor.rs` (*Name replacement explicitly instead of giving Import a hidden destructive mode.*) · `src/replace_tests.rs` (*Optional authoring fixture. Prove replacement/Undo/Redo preserve owners, camera state and non-reused local IDs.*) · `src/lib.rs` (*Compile the replacement contract.*)

Add `Action::Replace` for one reversible document replacement. First prepare the entire file with `document::load`; only then enter `History::try_edit` and replace the rows.

`Scene::replace` clears objects and the demo marker, then inserts prepared objects. Keep the ID counter advancing. The transaction restores the old scene if insertion fails partway through.

Repair selection after success. Camera, projection and background remain outside document history. Undo restores the old shared owners; Redo restores the replacement objects.

This step tests the editor operation; browser Open is still append-only until the next lesson. Old objects retained by Undo remain alive and are accounted for later.

> **Self-Check Question:** *Why must the scene counter survive replacing all its rows?*  
> **Answer:** A replacement must not reuse local IDs held by old selection or delayed work. Clearing rows leaves the increasing counter intact. History restoration also retains the highest issued value, while Undo restores the old objects and shared source allocations.

- **What You Should See:** Run the replacement checks below. A valid replacement is one Undo step; an invalid candidate must leave the existing scene unchanged. The browser command is connected next.
- **Hands-On Lab Experiment:** In the replacement check, change camera distance and aspect before replacing. Predict why Undo and Redo preserve both values. Explain why setting next_id back to one would make an old selection unsafe.

<details>
<summary>Implementation 30c-replace: Replace a document as one reversible change</summary>

Prepare a whole replacement before changing live rows, then commit it through existing history.

Data flow: Replacement bytes → validated prepared sources → history transaction → new rows with fresh local IDs.

### 1. `src/scene.rs`

Replace rows while preserving the issued local-ID counter; the caller supplies the transaction.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn import(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/30c-replace-01.rs"
```

### 2. `src/editor.rs`

Name replacement explicitly instead of giving Import a hidden destructive mode. Validate every source first, then replace as one undoable transaction.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
                self.selected = self.camera.ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
```

</details>

Replace it with:

```rust
                self.selected = self.camera.ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
            }
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
```

<details>
<summary>Locate this block</summary>

```rust
pub enum Action {
    Pick([f32; 2]),
    Import(Vec<u8>),
    AddBox,
    Translate([f64; 3]),
    ToggleExtra,
```

</details>

Replace it with:

```rust
pub enum Action {
    Pick([f32; 2]),
    Import(Vec<u8>),
    Replace(Vec<u8>),
    AddBox,
    Translate([f64; 3]),
    ToggleExtra,
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    Pick([f32; 2]),
    Import(Vec<u8>),
    Replace(Vec<u8>),
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
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scene: Scene::demo(),
            selected: None,
            camera: Camera::default(),
            background: Background::default(),
            history: History::default(),
        }
    }
}

impl Editor {
    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
                self.selected = self.camera.ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
            }
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
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
            Action::Delete => {
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
--8<-- "journey/code/30c-replace-02.rs"
```

```text
--8<-- "journey/code/30c-replace-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/replace_tests.rs`

Optional authoring fixture. Prove replacement/Undo/Redo preserve owners, camera state and non-reused local IDs.

Copy this supplied file:

```text
--8<-- "journey/code/30c-replace-04.rs"
```

</details>

### 4. `src/lib.rs`

Compile the replacement contract.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod save_roundtrip_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/30c-replace-05.rs" skip=39
```

</details>

<a id="checkpoint-30d-bridge"></a>

### Step 10 of 11 · Choose append or replace before opening the picker (`30d-bridge`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 10/11 Visual Trace (30d-bridge): Choose append or replace before opening the picker</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/file_input.rs, src/browser.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Open or Open Replace</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">new ticket plus captured</text>
    <text x="316" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">mode</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">file bytes</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">explicit editor action</text>
    
</svg>
</div>

- **Goal:** Carry the selected Open operation through asynchronous file reading and cancel older work immediately.
- **Data Flow Trace:** `Open or Open Replace → new ticket plus captured mode → file bytes → explicit editor action`
- **Files You Touch:** `src/file_input.rs` (*Capture a typed operation value with the selected file instead of consulting mutable state after reading.*) · `src/browser.rs` (*Offer replacement as a typed command.*)

Add typed `Open Replace`, retaining `Open` for append. Opening either picker begins a ticket and stores its operation as a `Mode` enum.

The selected-file task captures that ticket and mode. `choose` reuses the pending ticket; a newer picker revokes earlier delivery before a file is chosen.

The completion event carries a replace flag and `Uint8Array`. Validate shape and size, then translate to `Action::Import` or `Action::Replace`. Editor never receives browser values.

Picker cancellation revokes its ticket and appends an Open notice. Report append or replacement only after successful commit. A failed replacement retains the document; successful replacement is one Undo change.

> **Self-Check Question:** *Why must a newer Open revoke old work before the new file is chosen?*  
> **Answer:** Opening a new picker expresses a new request even if the user cancels it. If the ticket were issued only after choosing a file, an older pending read could still commit behind the newer picker. Begin at the command, capture its mode at selection, and consume that ticket before delivering an outcome.

- **What You Should See:** Type `Open Replace` and choose `sample.pb`, then type `Undo`. The preceding document returns as one transaction. `Open Append` instead keeps existing objects.
- **Hands-On Lab Experiment:** Hold a previous read, open Open Replace, cancel the new picker and then let the old read finish. Predict why the old file must stay rejected despite the new picker having no chosen file.

<details>
<summary>Implementation 30d-bridge: Choose append or replace before opening the picker</summary>

Carry the selected Open operation through asynchronous file reading and cancel older work immediately.

Data flow: Open or Open Replace → new ticket plus captured mode → file bytes → explicit editor action.

### 1. `src/file_input.rs`

Capture a typed operation value with the selected file instead of consulting mutable state after reading. A choice with no file revokes pending ownership without editing the scene. Use the ticket already issued by the Open command. Keep the captured mode beside the accepted bytes. Include the selected operation in the private byte event. Use an explicit replace flag and byte array at the browser bridge. Translate checked browser payloads into ordinary editor actions.

<details>
<summary>Locate the existing block</summary>

```rust
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};

pub fn choose(event: &web_sys::Event, request: Rc<RefCell<crate::read_gate::ReadGate>>) {
    let Some(input) = event.target().and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok()) else { return; };
    if input.id() != "open" { return; }
    let Some(file) = input.files().and_then(|files| files.get(0)) else { return; };
    input.set_value("");
    let id = match request.borrow_mut().begin() {
        Ok(id) => id,
        Err(error) => {
            wasm_bindgen_futures::spawn_local(async move { failure(error); });
            return;
        }
    };
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        if request.borrow().pending() != Some(id) { return; }
        if file.size() > crate::document::MAX_BYTES as f64 {
            let accepted = request.borrow_mut().finish(id);
            if accepted { failure("This checkpoint accepts files up to 4 MiB"); }
            return;
        }
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        let accepted = request.borrow_mut().finish(id);
        if !accepted { return; }
        let result = result.and_then(deliver);
        if let Err(error) = result {
            failure(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn failure(message: &str) {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&JsValue::from_str(message));
    let result = web_sys::CustomEvent::new_with_event_init_dict("viewer-file-error", &detail)
        .and_then(|event| web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?.dispatch_event(&event));
    if result.is_err() { super::browser::report(message); }
}

fn deliver(buffer: JsValue) -> Result<(), JsValue> {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&js_sys::Uint8Array::new(&buffer));
    let event = web_sys::CustomEvent::new_with_event_init_dict("viewer-file", &detail)?;
    web_sys::window().ok_or("No browser window")?.dispatch_event(&event)?;
    Ok(())
}

pub fn bytes(event: &web_sys::Event) -> Option<Vec<u8>> {
    let event = event.dyn_ref::<web_sys::CustomEvent>()?;
    let array = event.detail().dyn_into::<js_sys::Uint8Array>().ok()?;
    if array.length() as usize > crate::document::MAX_BYTES { return None; }
    Some(array.to_vec())
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode { Append, Replace }

pub fn choose(event: &web_sys::Event, request: Rc<RefCell<crate::read_gate::ReadGate>>, mode: Mode) {
    let Some(input) = event.target().and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok()) else { return; };
    if input.id() != "open" { return; }
    let Some(file) = input.files().and_then(|files| files.get(0)) else { request.borrow_mut().cancel(); return; };
    input.set_value("");
    let Some(id) = request.borrow().pending() else { return; };
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        if request.borrow().pending() != Some(id) { return; }
        if file.size() > crate::document::MAX_BYTES as f64 {
            let accepted = request.borrow_mut().finish(id);
            if accepted { failure("This checkpoint accepts files up to 4 MiB"); }
            return;
        }
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        let accepted = request.borrow_mut().finish(id);
        if !accepted { return; }
        let result = result.and_then(|buffer| deliver(buffer, mode));
        if let Err(error) = result {
            failure(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn failure(message: &str) {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&JsValue::from_str(message));
    let result = web_sys::CustomEvent::new_with_event_init_dict("viewer-file-error", &detail)
        .and_then(|event| web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?.dispatch_event(&event));
    if result.is_err() { super::browser::report(message); }
}

fn deliver(buffer: JsValue, mode: Mode) -> Result<(), JsValue> {
    let detail = web_sys::CustomEventInit::new();
    let payload = js_sys::Array::new();
    payload.push(&JsValue::from_bool(mode == Mode::Replace));
    payload.push(&js_sys::Uint8Array::new(&buffer));
    detail.set_detail(&payload);
    let event = web_sys::CustomEvent::new_with_event_init_dict("viewer-file", &detail)?;
    web_sys::window().ok_or("No browser window")?.dispatch_event(&event)?;
    Ok(())
}

pub fn action(event: &web_sys::Event) -> Option<crate::editor::Action> {
    let event = event.dyn_ref::<web_sys::CustomEvent>()?;
    let payload = event.detail().dyn_into::<js_sys::Array>().ok()?;
    if payload.length() != 2 { return None; }
    let replace = payload.get(0).as_bool()?;
    let array = payload.get(1).dyn_into::<js_sys::Uint8Array>().ok()?;
    if array.length() as usize > crate::document::MAX_BYTES { return None; }
    Some(if replace { crate::editor::Action::Replace(array.to_vec()) }
        else { crate::editor::Action::Import(array.to_vec()) })
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/30d-bridge-01.rs"
```

```text
--8<-- "journey/code/30d-bridge-02.rs"
```

```text
--8<-- "journey/code/30d-bridge-03.rs"
```

```text
--8<-- "journey/code/30d-bridge-04.rs"
```

```text
--8<-- "journey/code/30d-bridge-05.rs"
```

```text
--8<-- "journey/code/30d-bridge-06.rs"
```

```text
--8<-- "journey/code/30d-bridge-07.rs"
```

</details>

### 2. `src/browser.rs`

Offer replacement as a typed command. Hold the picker operation in the event closure; each task copies it at selection. Revoke native picker cancellation and capture the operation when a file is selected. Keep browser values outside the editor transaction. A newer request revokes older work at picker opening, not only after file selection. Remember which successful file action was committed before consuming it. Report the committed operation without overwriting another command. Receive the native file-input cancellation event for the viewer lifetime.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    window.add_event_listener_with_callback("resize", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file-error", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
```

</details>

Replace it with:

```rust
    window.add_event_listener_with_callback("resize", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("cancel", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file-error", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
```

<details>
<summary>Locate this block</summary>

```rust
            None
        };
        if let Some(action) = action {
            match editor.apply(action) {
                Ok(Change::Scene) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if event.type_() == "viewer-file" {
                        report("File imported. Undo removes the entire import.");
                        panel.answer("Open", "File imported. Undo removes the entire import.");
                    }
                }
                Ok(Change::View) => {}
```

</details>

Replace it with:

```rust
            None
        };
        if let Some(action) = action {
            let replacement = matches!(&action, Action::Replace(_));
            match editor.apply(action) {
                Ok(Change::Scene) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if event.type_() == "viewer-file" {
                        let message = if replacement { "Document replaced. Undo restores the previous document." }
                            else { "File imported. Undo removes the entire import." };
                        report(message);
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Change::View) => {}
```

<details>
<summary>Locate this block</summary>

```rust
            }
            None
        } else if event.type_() == "viewer-file" {
            crate::file_input::bytes(&event).map(Action::Import)
        } else if let Some(line) = line {
            if line == "cancel open" {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            } else if line == "open" {
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
```

</details>

Replace it with:

```rust
            }
            None
        } else if event.type_() == "viewer-file" {
            crate::file_input::action(&event)
        } else if let Some(line) = line {
            if line == "cancel open" {
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
```

<details>
<summary>Locate this block</summary>

```rust
            }
        } };

        let action = if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
```

</details>

Replace it with:

```rust
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
            crate::file_input::choose(&event, std::rc::Rc::clone(&request), read_mode.get());
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
```

<details>
<summary>Locate this block</summary>

```rust
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
```

</details>

Replace it with:

```rust
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
```

<details>
<summary>Locate this block</summary>

```rust
        &[
            "Help",
            "Open",
            "Cancel Open",
            "Save",
            "Example Box",
```

</details>

Replace it with:

```rust
        &[
            "Help",
            "Open",
            "Open Replace",
            "Cancel Open",
            "Save",
            "Example Box",
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
    inspect(&editor)?;
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    renderer.resize(Viewport { width, height });
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &[
            "Help",
            "Open",
            "Open Replace",
            "Cancel Open",
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
            crate::file_input::choose(&event, std::rc::Rc::clone(&request), read_mode.get());
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-file" {
            crate::file_input::action(&event)
        } else if let Some(line) = line {
            if line == "cancel open" {
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
            let replacement = matches!(&action, Action::Replace(_));
            match editor.apply(action) {
                Ok(Change::Scene) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if event.type_() == "viewer-file" {
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
        if let Err(error) = inspect(&editor) {
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
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
    report("Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.");
    Ok(())
}

fn inspect(editor: &Editor) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
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
--8<-- "journey/code/30d-bridge-08.rs"
```

```text
--8<-- "journey/code/30d-bridge-09.rs"
```

```text
--8<-- "journey/code/30d-bridge-10.rs"
```

```text
--8<-- "journey/code/30d-bridge-11.rs"
```

```text
--8<-- "journey/code/30d-bridge-12.rs"
```

```text
--8<-- "journey/code/30d-bridge-13.rs"
```

```text
--8<-- "journey/code/30d-bridge-14.rs"
```

```text
--8<-- "journey/code/30d-bridge-15.rs"
```

</details>

</details>

<a id="checkpoint-30e-recovery"></a>

### Step 11 of 11 · Prove recovery at the transaction and display boundaries (`30e-recovery`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 11/11 Visual Trace (30e-recovery): Prove recovery at the transaction and display boundaries</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/placement.rs, src/scene.rs, src/replace_tests.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Malformed source or failed insertion</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">retained old owners and history;</text>
    <text x="420" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">double…</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">finite float upload check</text>
    
</svg>
</div>

- **Goal:** Test partial replacement failure, preserved selection/Redo and placement upload limits.
- **Data Flow Trace:** `Malformed source or failed insertion → retained old owners and history; double matrix → finite float upload check`
- **Files You Touch:** `src/placement.rs` (*Reject a finite double coefficient that cannot enter the current float uniform.*) · `src/scene.rs` (*State the full placement boundary in its error.*) · `src/replace_tests.rs` (*Optional authoring fixture. Test refusal before mutation, selection/Redo retention and the double-to-float upload boundary.*)

Test a replacement failure after insertion starts, not only malformed bytes before mutation. Put the object-ID counter one below its limit: one insertion succeeds and the next fails.

History must restore every old row and allocation while retaining the highest issued counter. IDs from failed work must not be reused. A separate malformed-replacement test preserves selection and Redo.

Also reject placement coefficients that are finite as doubles but become infinity in the current float upload. `f64::MAX` demonstrates this boundary for direct placement and encoded files. Keep the previous placement untouched. This limits the display adapter without reducing retained source precision.

> **Self-Check Question:** *Why do we need a failure after the first new row was inserted?*  
> **Answer:** Malformed decoding fails before a transaction starts. It does not test rollback after mutation. Exhausting the ID counter during the second insertion proves History::try_edit restores old rows and shared owners after a partial replacement, while retaining the highest issued counter so failed work cannot cause ID reuse.

- **What You Should See:** Run the recovery checks below. A replacement that fails on its second mesh must preserve the whole previous scene and its Redo branch.
- **Hands-On Lab Experiment:** Temporarily remove History::try_edit from the partial-replacement check. Predict why the old scene disappears when the second insertion fails. Restore the transaction. Then compare f64::MAX with its float cast before reading the range-check answer.

<details>
<summary>Implementation 30e-recovery: Prove recovery at the transaction and display boundaries</summary>

Test partial replacement failure, preserved selection/Redo and placement upload limits.

Data flow: Malformed source or failed insertion → retained old owners and history; double matrix → finite float upload check.

### 1. `src/placement.rs`

Reject a finite double coefficient that cannot enter the current float uniform.

<details>
<summary>Locate the existing block</summary>

```rust
    matrix.len() == 16 && matrix.iter().all(|value| value.is_finite())
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    matrix.len() == 16 && matrix.iter().all(|value| value.is_finite() && (*value as f32).is_finite())
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/30e-recovery-01.rs"
```

</details>

### 2. `src/scene.rs`

State the full placement boundary in its error. Fail during the second insertion, then prove rollback restores the old allocations without recycling issued IDs.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    use super::*;

    #[test]
    fn removal_moves_a_row_without_changing_its_identity() {
        let mut scene = Scene::demo();
        let first = scene.objects()[0].id;
```

</details>

Replace it with:

```rust
    use super::*;

    #[test]
    fn partial_replacement_failure_restores_owners_without_reusing_ids() {
        let mut scene = Scene::demo();
        let before = scene.objects.clone();
        scene.next_id = u32::MAX - 1;
        let loaded = crate::document::load(&crate::specimen::bytes()).unwrap();
        let mut history = crate::history::History::default();
        assert!(history.try_edit(&mut scene, |scene| scene.replace(loaded)).is_err());
        assert_eq!(scene.next_id, u32::MAX);
        assert_eq!(scene.objects.len(), before.len());
        for (object, old) in scene.objects.iter().zip(&before) {
            assert_eq!(object.id, old.id);
            assert!(Rc::ptr_eq(&object.geometry, &old.geometry));
            assert!(Rc::ptr_eq(&object.mesh, &old.mesh));
        }
        assert!(!history.undo(&mut scene));
    }

    #[test]
    fn removal_moves_a_row_without_changing_its_identity() {
        let mut scene = Scene::demo();
        let first = scene.objects()[0].id;
```

<details>
<summary>Locate this block</summary>

```rust
    pub fn place(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m)
        {
            return Err("Placement must be finite and affine");
        }
        let object = self.objects.iter_mut().find(|o| o.id == id).ok_or("Object not found")?;
        object.model = model;
```

</details>

Replace it with:

```rust
    pub fn place(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m)
        {
            return Err("Placement must be finite, affine and fit the display range");
        }
        let object = self.objects.iter_mut().find(|o| o.id == id).ok_or("Object not found")?;
        object.model = model;
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{mesh::Mesh, prepared::PreparedMesh};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjectId(u32);

#[derive(Clone)]
pub struct Object {
    pub id: ObjectId,
    pub guid: String,
    pub mesh: Rc<Mesh>,
    pub geometry: Rc<session_rust::Mesh>,
    pub model: session_rust::Xform,
    pub source: Option<crate::document::Source>,
}

impl Object {
    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
        let local = session_rust::Point::new(vertex[0] as f64, vertex[1] as f64, vertex[2] as f64);
        let world = self.model.transform_point(&local);
        [world[0], world[1], world[2]]
    }
}

#[derive(Clone)]
pub struct Scene {
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
}

impl Scene {
    pub fn demo() -> Self {
        let pink = PreparedMesh::triangle([
            [-0.7, -0.6, 0.25], [0.5, -0.6, 0.25], [-0.1, 0.6, 0.25],
        ], [0.9, 0.25, 0.45]).expect("Valid pink triangle");
        let turquoise = PreparedMesh::triangle([
            [-0.4, -0.2, 0.75], [0.8, -0.2, 0.75], [0.2, 0.8, 0.75],
        ], [0.05, 0.7, 0.7]).expect("Valid turquoise triangle");
        let mut scene = Self { objects: Vec::new(), next_id: 1, extra: None };
        scene.insert(pink).expect("ID available");
        scene.insert(turquoise).expect("ID available");
        scene
    }

    pub fn restore(&mut self, mut saved: Self) {
        saved.next_id = saved.next_id.max(self.next_id);
        *self = saved;
    }

    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for object in &self.objects {
            for vertex in object.mesh.vertices() {
                let point = object.world_point(*vertex);
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        bounds
    }

    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
        let object = self.objects.iter().find(|object| object.id == id)?;
        let mut vertices = object.mesh.vertices().iter();
        let first = vertices.next()?;
        let mut bounds = crate::bounds::Bounds::point(object.world_point(*first));
        for vertex in vertices {
            bounds.include(object.world_point(*vertex));
        }
        Some(bounds)
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        let mut guid = prepared.geometry.guid().to_owned();
        while self.objects.iter().any(|object| object.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        self.objects.push(Object { id, guid, mesh: Rc::new(prepared.display), geometry: prepared.geometry,
            model: prepared.model, source: prepared.source });
        Ok(id)
    }

    pub fn replace(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        self.objects.clear();
        self.extra = None;
        self.import(loaded)
    }

    pub fn import(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        for prepared in loaded.meshes {
            self.insert(prepared)?;
        }
        Ok(())
    }

    pub fn place(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m)
        {
            return Err("Placement must be finite, affine and fit the display range");
        }
        let object = self.objects.iter_mut().find(|o| o.id == id).ok_or("Object not found")?;
        object.model = model;
        Ok(())
    }

    pub fn remove(&mut self, id: ObjectId) -> bool {
        let Some(index) = self.objects.iter().position(|object| object.id == id) else { return false; };
        self.objects.remove(index);
        if self.extra == Some(id) {
            self.extra = None;
        }
        true
    }

    pub fn contains(&self, id: ObjectId) -> bool {
        self.objects.iter().any(|object| object.id == id)
    }

    pub fn next(&self, current: Option<ObjectId>) -> Option<ObjectId> {
        if self.objects.is_empty() {
            return None;
        }
        let index = self.objects.iter().position(|object| Some(object.id) == current)
            .map_or(0, |index| index + 1);
        Some(self.objects[index % self.objects.len()].id)
    }

    pub fn add_box(&mut self) -> Result<ObjectId, &'static str> {
        let mut source = session_rust::Mesh::create_box(0.8, 0.8, 0.8);
        source.set_objectcolor(session_rust::Color::new(0.65, 0.65, 0.65, 1.0));
        let id = self.insert(PreparedMesh::new(source)?)?;
        self.place(id, session_rust::Xform::translation(0.9, 0.0, 0.4))?;
        Ok(id)
    }

    pub fn toggle_extra(&mut self) {
        if let Some(id) = self.extra.take() {
            self.remove(id);
        } else {
            let extra = PreparedMesh::triangle([
                [-0.9, 0.3, 0.5], [-0.4, 0.3, 0.5], [-0.65, 0.9, 0.5],
            ], [0.2, 0.8, 0.3]).expect("Valid extra triangle");
            self.extra = Some(self.insert(extra).expect("ID available"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_replacement_failure_restores_owners_without_reusing_ids() {
        let mut scene = Scene::demo();
        let before = scene.objects.clone();
        scene.next_id = u32::MAX - 1;
        let loaded = crate::document::load(&crate::specimen::bytes()).unwrap();
        let mut history = crate::history::History::default();
        assert!(history.try_edit(&mut scene, |scene| scene.replace(loaded)).is_err());
        assert_eq!(scene.next_id, u32::MAX);
        assert_eq!(scene.objects.len(), before.len());
        for (object, old) in scene.objects.iter().zip(&before) {
            assert_eq!(object.id, old.id);
            assert!(Rc::ptr_eq(&object.geometry, &old.geometry));
            assert!(Rc::ptr_eq(&object.mesh, &old.mesh));
        }
        assert!(!history.undo(&mut scene));
    }

    #[test]
    fn removal_moves_a_row_without_changing_its_identity() {
        let mut scene = Scene::demo();
        let first = scene.objects()[0].id;
        let second = scene.objects()[1].id;
        assert!(scene.remove(first));
        assert_eq!(scene.objects()[0].id, second);
        assert_eq!(scene.next(None), Some(second));
        assert!(!scene.contains(first));
        scene.toggle_extra();
        assert_ne!(scene.objects()[1].id, first);
        assert_ne!(scene.objects()[1].id, second);
    }

    #[test]
    fn toggling_the_extra_object_uses_its_id_after_other_removals() {
        let mut scene = Scene::demo();
        scene.toggle_extra();
        let extra = scene.objects()[2].id;
        scene.remove(scene.objects()[0].id);
        scene.toggle_extra();
        assert!(!scene.contains(extra));
        assert_eq!(scene.objects().len(), 1);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/30e-recovery-02.rs"
```

```text
--8<-- "journey/code/30e-recovery-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/replace_tests.rs`

Optional authoring fixture. Test refusal before mutation, selection/Redo retention and the double-to-float upload boundary.

<details>
<summary>Locate the existing block</summary>

```text
    assert_eq!((editor.camera.distance, editor.camera.aspect), (7.0, 2.0));
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/30e-recovery-04.rs" skip=78
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 30e-recovery
npm --prefix ../session_tests run course -- check 30e-recovery
npm --prefix ../session_tests run course -- save 30e-recovery
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

![Recorded visual output for Lesson 3.5 (30e-recovery)](../../../screenshots/journey/30e-recovery-browser.png)

Run the recovery checks below. A replacement that fails on its second mesh must preserve the whole previous scene and its Redo branch.

- **Final Lesson Experiment:** Temporarily remove History::try_edit from the partial-replacement check. Predict why the old scene disappears when the second insertion fails. Restore the transaction. Then compare f64::MAX with its float cast before reading the range-check answer.

Continue with [Shared GPU geometry and releasable source ownership](../../consolidation/31-settings.md).
