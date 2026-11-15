# Decode a document

Read a real session file, inspect its decoded contents, and recover from malformed bytes with a filename-specific error.

This eighth cumulative checkpoint is a candidate. Compilation, native execution and browser evidence are recorded separately below.

## Prerequisites and ownership

Keep the seven earlier subjects. This subject owns `app/decode.rs`, `app/validate.rs` and the browser scheduler in `app/yielding.rs`. It uses the retained painter and canvas to inspect parser results. The protobuf fixture stays in the shared assets directory; it is not copied into each build cache.

The decoder needs to yield to the browser, but does not need HTTP or command panels. The scheduler therefore has its own module. The fetch module reexports the same public functions for existing callers. Loading later registers HTTP and UI integration without changing the decoder or its scheduler.

## Build

Add the decoder, validator, scheduler and inspector registration while retaining every earlier implementation.

### Bytes, ownership and limits

`Body::Bytes` owns a Rust byte vector. `Body::Js` retains a JavaScript typed array and copies requested ranges into WebAssembly memory. The latter avoids keeping an entire fetched body in both JavaScript and Rust while converting it.

`Reader` borrows that body. Its byte-range operation uses checked addition before slicing: an overflowing length must become an error, not an out-of-bounds read. A JavaScript body normally uses a one-MiB window. One larger object can require a larger window; after decoding that object, the oversized allocation is released. This is a bounded normal working set, not a claim that every individual object fits one MiB.

The whole-file decoder rejects bodies over 512 MiB and directs larger cloud data to streaming. Validation also limits source objects to two million and hierarchy depth to 64. These are explicit resource bounds checked before an unrestricted conversion can consume the application.

### Protobuf fields and validation

A protobuf field header contains a field number and wire type encoded as a varint. `validate::varint` returns a decoded value and consumed length only for a complete, representable encoding. `Reader::field` then calculates where the value begins and the next field starts. Fixed-width values, varints and length-delimited messages need different advances. Unsupported wire types, invalid field numbers and lengths beyond the enclosing message are errors.

Unknown valid fields are skipped using that next offset. This allows a reader to tolerate newer fields without understanding their contents. It does not allow arbitrary trailing bytes: the E experiment appends an unfinished varint header, which has no valid next field boundary.

For each geometry record, the decoder first uses prost to decode that record, validates its structure, and only then constructs the kernel object. Mesh indices, point/cloud column lengths and NURBS data require checks appropriate to their representation. A successfully parsed wire message can still contain invalid geometry. The fallible conversion branch also preserves a kernel conversion error rather than publishing a partial success.

The decoder counts records and reserves their collections before conversion. Reserving the required capacity avoids repeated doubling while old and new allocations coexist. `Rc` then shares a constructed object between the typed collection and GUID lookup; those are two references to one object, not two geometry copies.

### Trees, graphs and placements

The document tree is reconstructed in file order with an explicit stack containing node, next field and end offset. That makes the depth bound visible and avoids unbounded recursive calls. A node's transparent stored colour is treated as no colour, matching the kernel's loading semantics.

The relationship graph is separate from this tree. Graph edges are stored in both directions; the decoder preserves the stored GUID on both copies because interaction records refer to that identity. Numeric edge attributes and vertex attributes are restored alongside connectivity.

The `vertices` argument controls whether independent graph vertices and their attributes are retained. The public `session_from_bytes` path uses true, preserving them. A display-only caller may request false and retain connectivity through the edges; such a reduced document is not suitable for saving as an equivalent original.

Instance definitions and instance references have their own lookup. Stored placements are folded into the central transform table using the same convention as kernel loaders. Those transforms remain local to the parent. Scene composition and hierarchy panels consume this document in their later subject; this inspector does not claim to implement those interfaces.

### Yielding without threads

Converting 25,000 objects triggers a yield. An async function can pause without moving the parser to another thread. A promise resolved by a zero-delay browser timer lets the browser process other work before conversion continues. Awaiting that promise resumes the same logical operation.

```rust
--8<-- "session_viewer/src/app/yielding.rs:browser-yield"
```

`spawn_local` runs the inspector's future on the browser thread. Its result is shared with the drawing object through `Rc<RefCell<_>>`: `Rc` shares ownership, and `RefCell` checks mutable access at runtime. No borrow is held across the await. A pending operation refuses another request, and completion explicitly requests a redraw. The window is retained through its existing `Arc` handle.

The parser returns `Result<Session, String>`. The inspector matches both outcomes and adds the source filename to an error. It replaces its pending state only after the complete operation returns. The R key starts a new decode of the original bytes, so recovery does not depend on repairing a partially constructed document.

```rust
--8<-- "session_viewer/examples/course/decoding.rs:course-decoding"
```

## Run and observe

Materialize `decoding` with the completion-checkpoint command in [Maintaining](../maintaining.md), build it, and open `?tutorial=decoding`.

The page shows `viewer comparison`, four BReps, and zero meshes, curves, components and stored placements. Press E: the counts are replaced by a red error naming `course-comparison.pb`. Press R: the original document summary returns. Text wraps inside an upright viewport. This experiment uses bundled real bytes; network fetching, loading progress and file-picker controls belong to application integration.

## Predict and try

Predict whether adding one byte can invalidate an otherwise complete file. Press E. The appended `0x80` says that a varint continues, but no following byte exists, so the decoder rejects its header. Press R and predict whether the counts change: they return unchanged because the original fixture bytes were never modified.

## Question and answer

Why validate each object after decoding its protobuf message?

Wire decoding establishes how fields are represented, not whether they form valid geometry. For example, a list of mesh indices can be valid protobuf while referring past the vertex array. Validation protects the kernel conversion and reports the malformed document instead of letting invalid geometry reach rendering.

## Validation

The eight-subject source-retention and shader-include checks pass four tests. The independent wasm checkpoint and full production native test modules compile. The cumulative native suite passes 195 tests with 14 ignored. Browser checks pass 98 captures across eight modes at desktop and upright sizes, including the red malformed-header error and exact recovery. All 92 earlier captures are byte-identical. These results do not establish HTTP loading, hierarchy UI or physical-device acceptance.
