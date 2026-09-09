# 16 · Resource accounting

## You are building

```mermaid
flowchart TB
    D["Scene docs<br/>Rc&lt;Session&gt;"] -- "Rc::downgrade" --> C["SourceCache<br/>Weak identities + Payload"]
    C -- "same Rc pointers → cached" --> P["Payload<br/>known bytes"]
    C -- "identity changed → session_payload walk" --> P
    G["Gpu::allocated_bytes"] --> I["inspection snapshot<br/>?inspect=1"]
    P --> I
```

## Starting point

- Checkpoint 15: the inspection snapshot reports owned GPU buffer and texture bytes.
- This lesson adds a known-payload figure for retained source documents, kept in a cache that cannot extend their lifetime, and declares the native tooling the production crate ships with.

## Step 1 · Native tooling the crate declares

`Cargo.toml` will name native examples; their sources and the offscreen harness are supplied, not taught. Install them now so the manifest can refer to them.

```mermaid
flowchart LR
    S["supplied examples/ · tests/"] --> C["Cargo.toml<br/>[[example]] entries"]
    C --> N["native tooling builds"]
    style C fill:#1a1eb2,color:#fff
```

<!-- supplied: 16 -->

<!-- file: 16 session_viewer/Cargo.toml copy -->

## Step 2 · Count what is knowable, name what is not

- The number is a lower bound: exact `Vec`/`String` capacities, occupied map entries and exposed slice lengths, never allocator overhead or RSS.
- Shared values are counted once: each `Rc` object is recorded by pointer in a `seen` set, so a document listed twice or a geometry in both a typed list and the lookup adds nothing twice.

```mermaid
flowchart LR
    D["Doc · Rc&lt;Session&gt;"] -- "walk once per Rc" --> P["Payload<br/>known_bytes"]
    P -- "seen set" --> U["no double count"]
    style P fill:#1a1eb2,color:#fff
```

<!-- file: 16 session_viewer/src/app/inspection/source_memory.rs type lines=1-52 -->

- A `Weak<Session>` recognizes a document without keeping it alive; if every `Rc` pointer matches the last snapshot, the cached payload is returned without a walk.
- In-place editing of a document would make this cache stale; replacement and append change identity, which is what the cache keys on.

```mermaid
flowchart LR
    C["SourceCache<br/>Weak&lt;Session&gt; ids"] -- "same Rc pointers" --> H["cached Payload"]
    C -- "identity changed" --> S["snapshot(docs) walk"]
    style C fill:#1a1eb2,color:#fff
```

<!-- file: 16 session_viewer/src/app/inspection/source_memory.rs type lines=53-95 -->

<!-- file: 16 session_viewer/src/app/inspection/source_memory.rs type lines=96-200 -->

Per-type payload walks, one function per geometry kind:

<!-- file: 16 session_viewer/src/app/inspection/source_memory.rs copy lines=201-432 -->

Unit tests, part of the file:

<!-- file: 16 session_viewer/src/app/inspection/source_memory.rs copy lines=433-510 -->

<!-- check: 16 -->

## Step 3 · Report it beside the GPU figures

- The snapshot names its scope and exclusions in the JSON itself, so a reader of `?inspect=1` cannot mistake the payload for total heap.

```mermaid
flowchart LR
    K["known_bytes()"] --> J["?inspect=1 JSON<br/>source_cpu_known_payload"]
    G["Gpu::allocated_bytes"] --> J
    J --> X["scope + exclusions named"]
    style J fill:#1a1eb2,color:#fff
```

<!-- file: 16 session_viewer/src/app/inspection.rs type -->

## Check

<!-- checkpoint: 16 -->

Expected:

- The local scene renders as before.
- The canvas `data-viewer-inspection` attribute now carries `source_cpu_known_payload_bytes`, `source_cpu_known_payload`, `source_cpu_scope` and `source_cpu_exclusions`.
- Reload the same scene: `scans` in the payload stays at one per document identity change, not one per frame.

## What changed

<!-- tree: 16 session_viewer/src/app -->

- `SourceCache` (weak identities) → `Payload` (known bytes) → inspection snapshot.
- Four separate measurements now sit side by side and mean different things: retained source payload, owned GPU buffers, estimated texture bytes, and whatever the browser reports for WASM memory.

**Production equivalent:** `src/app/inspection.rs`, `src/app/inspection/source_memory.rs`, `Cargo.toml`.

## Next

[17 · Faces, text objects and silhouettes](17-source-presentation.md): source-face selection, selectable authored text and one black outline.
