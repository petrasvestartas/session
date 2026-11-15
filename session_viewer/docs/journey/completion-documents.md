# Documents, hierarchy and history

Show a loaded document in the layer panel, hide and restore its geometry, and undo an edit without losing object identity.

## Prerequisites and ownership

This is a chapter of the [application subject](completion-application.md), using that same executable checkpoint. Complete decoding, object rows, scene rendering and the application lifecycle first. This chapter teaches `app/scene.rs`, `scene_rows.rs`, the synchronization modules under `scene_sync/`, `hierarchy.rs`, `layers.rs`, `state/panel.rs` and the layer-panel adapter. It also explains the document boundary used by instancing, released geometry and display refinement; their detailed algorithms belong to the rendering and loading chapters.

## Build

Connect decoded documents, stable display rows and the registered layer panel through the application host.

### A file, a document and a row

The decoder produces a kernel `Session`. `FileDoc` adds application information: a visible name, file placement, point-size override and whether the file is a display-only shell.

```rust
--8<-- "session_viewer/src/app/scene.rs:document-placement"
```

`Rc<Session>` allows two file placements to refer to the same loaded kernel document. Each placement still has its own document index in `Scene::docs`. Consequently the application identity of an object is `(document index, GUID)`, not its GUID alone. Hiding one placement must not accidentally hide another file containing the same identifier.

The scene's `order`, `owners` and `guid_to_row` tables connect that identity to a display row. A row identifies an object for rendering and picking; it does not identify a triangle or polyline segment. The object's geometry may occupy several rendering lanes.

Scene text has a separate owner sentinel and stable text key. `scene_text.rs` registers document titles as text objects, and `state/text.rs` positions a title above the CAD bounds. The comparison fixture has four kernel objects plus this selectable title: Select All reports five, although the hierarchy lists four geometry children. Deleting text retires its active row; `app/edit.rs` joins that text operation to the documents' transaction so one Undo restores both text and geometry. This distinction explains the count without changing selection behavior.

```rust
--8<-- "session_viewer/src/app/scene_rows.rs:document-footprint"
```

`Footprint::One` records a contiguous span in one lane. `Many` indexes a collection of spans, as needed when a BRep contributes faces and edges. `Cloud` selects the cloud representation. `None` is meaningful for a row with no ordinary lane allocation, including a shell or a free row. These variants keep resource accounting separate from object identity.

`Scene::add_file` walks the document through the completed geometry converters. `upload_to` drains the resulting work into GPU resources. The application updates bounds, labels and hierarchy after appending the document. Merely parsing a file cannot update the visible scene: conversion and upload are distinct steps.

### Describe a change before synchronizing it

History belongs to the kernel document. Scene synchronization translates recorded operations into display invalidations. `GEOMETRY` means regenerate geometry; `PLACE` means recalculate placement; `PRESENCE` means reconsider whether it exists on screen; `SUBTREE` extends the change to descendants.

```rust
--8<-- "session_viewer/src/app/scene_sync/notes.rs:document-history-notes"
```

`commit` reads the open transaction's operations before committing it. `stepped` reads the transaction that an undo or redo just moved to the opposite history stack. Undo visits operations in reverse order. An added object being undone requires different geometry work from that same addition being replayed forward.

The remaining code in `notes.rs` maps each operation to these flags. A replacement invalidates geometry. A transform invalidates placement and descendants. A tree edit can also invalidate geometry when its inherited colour changes. This is why a universal "rebuild everything" callback is unnecessary, but a universal "upload one matrix" callback would be incorrect.

`scene_sync/nodes.rs` resolves the affected tree nodes and descendants. Weak references cache known nodes without keeping a removed subtree alive. `allocation.rs` manages the object's lane allocation and available capacity. `preview.rs` applies temporary editing geometry; committing or cancelling must reconcile that preview with the document's real representation. The parent `scene_sync.rs` combines this work into `Staged`, which records ordered GPU updates for the next upload.

### Delete, undo and reuse memory

Deleting an object can hide its resident rows while retaining them for undo. `scene_sync/tombs.rs` associates those rows with a weak reference to the kernel history tomb. The display row cannot be reused while that tomb owns it: a later pick or undo must not mistake a different object for the deleted one.

This retention is bounded. The scene's tomb budget is 256 MiB of lane data, and older retained allocations can be released. When history no longer reaches the object, the weak reference also stops justifying retention. A hidden sink row keeps discarded lane data from becoming visible while storage awaits compaction.

`scene_sync/compaction.rs` addresses accumulated dead storage. Editable-lane compaction starts only after dead bytes reach 16 MiB and dead plus retained tomb bytes outweigh live bytes. Cloud compaction has a separate threshold because its buffers and point counts differ. Repacking updates buffer bindings and invalidates dependent splat state. It is not enough to change the CPU count while leaving the GPU bound to the previous buffers.

Idle kernel purging also checks shared ownership. A shared `Rc<Session>` is left alone: making a private copy merely to purge it would change memory and history semantics. These conditions belong to resource ownership, not to the visual appearance of the Delete command.

### Flatten a hierarchy for a panel

`Hierarchy` converts the document tree into a flat list with parents before children. Each node stores depth, the index after its last descendant, a range into the hierarchy's row list, and document/path identity. That representation supports indentation, collapsed subtrees and group actions without recursively rebuilding widgets for every frame.

The cache refreshes when `Scene::row_revision` changes. Open nodes and switched-off groups retain document-scoped identities across rebuilds. A path distinguishes groups with the same name under different parents. A graph edge remains separate from the tree: `Hierarchy::edges` connects object rows for the graph table, while the tree defines grouping and inherited placement.

The panel bounds its work: pages contain 128 nodes; node, row and edge limits prevent an unbounded UI allocation. The `truncated` state records when the complete hierarchy cannot be shown. Truncation must not silently redefine which objects exist in the kernel document.

`app/layers.rs` supplies category rows and document-tree operations, including grouping, moving objects between layers and editing layer names. `state/panel.rs` translates panel keys into scene operations and GPU updates. Face and edge colour overrides are separate maps keyed by document/object identity. An edge-colour action checks whether a row has faces before changing that channel.

### Keep panel drawing separate from actions

The existing panel registry is the integration point; the completed command dock does not need a new implementation.

```rust
--8<-- "session_viewer/src/app/ui/mod.rs:application-panels"
```

Each panel fills its model from application state, draws widgets and then applies completed actions. `ui/layers.rs` retains expansion, graph-table and rename state. A completed rename is applied before the click that ended it. Deferring the mutation until egui finishes avoids rebuilding the hierarchy halfway through drawing its rows.

The number box precedes the command line so Escape can belong to its active edit. The `Panel` contract also distinguishes owning keys, owning text history and closing on Escape. Ctrl+Z in an active text field must edit that field rather than undoing scene geometry. The same ownership rules guide the phone keyboard adapter.

### Large documents retain different things

`scene_instances.rs` walks a definition once under a hidden row and gives its instances independent placements. Batch keys remain unique across clearing so a newly created definition cannot be confused with an old GPU batch.

`scene_release.rs` can drop a display-only document's kernel geometry while retaining its tree, names and display rows. The source URL lets loading restore geometry before an edit. Created documents and documents with undo history are not released through that path. A visible object is therefore not proof that its editable kernel data is resident.

`scene_display.rs` retains bounded source and refinement caches and performs camera-dependent display refinement. It separates source bytes, refinement bytes and transient work. Rendering quality can improve during idle frames without changing the document's authoritative geometry or its history.

## Run and observe

Run the cumulative `application` checkpoint with the recorded comparison fixture. Enter `Layers All`. The panel should list the loaded document and its four objects. Toggle the document's visibility: its geometry disappears while the document remains listed. Toggle it back: the same geometry returns. Enter `Select All`: five rows are selected, including the title. Enter `Delete`, then `Undo`; the four objects and their title return with their identity and placement preserved.

At 390 pixels wide, the open panel takes 240 pixels and leaves a narrow scene region. The document nameplate can be clipped by that region. Enter `Layers Off` to inspect the full-width geometry again. Hiding the document's geometry leaves its independent title visible; deleting all five selected rows removes both.

![The comparison document has four geometric objects and an independent title: five selectable objects. This is the desktop view; the open panel crowds the upright scene.](../screenshots/completion/document-selection.png)

The comparison document has four geometric objects and an independent title: five selectable objects. This is the desktop view; the open panel crowds the upright scene.

## Predict and try

Predict whether hiding a document and deleting an object should have the same undo behavior. Hide and restore the document, then delete and undo one object. Visibility changes which retained rows are drawn; deletion changes the kernel document and its recorded transaction. Their identical-looking empty pixels do not imply identical state changes.

## Question and answer

Why does a display object keep an identity separate from its lane allocation?

One object can occupy multiple lanes, and its geometry can grow, shrink or be compacted. Stable document/object identity lets selection, hierarchy and undo continue referring to that object while the renderer changes where its vertices and segments are stored.

## Validation

The cumulative application passes 68 targeted scene tests and seven hierarchy tests. Twelve browser captures verify layer expansion, document hide/show, Select All, Delete and Undo at desktop and upright sizes. The four hierarchy children regain the same keys and labels after Undo; geometry and its separate title return. The initial harness incorrectly expected four selections and is preserved as a failure; the corrected count follows the reviewed text-row implementation. The narrow-panel nameplate clipping remains visible and unchanged. Shared-document and file-import interactions are still pending. Four source files gain inclusion markers only; their remaining bytes match the tested application snapshot exactly.
