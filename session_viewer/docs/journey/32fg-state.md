# 32fg · Separate loaded and released editable ownership

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 18–35 minutes.** 32 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Represent source residency without changing retained display, identity or placement.

**Follow:** Represent source residency without changing retained display, identity or placement..

Replace Object’s private geometry/source fields with EditSource. Loaded owns the required kernel geometry and optional imported Source. Released owns an Origin and epoch. Generated Loaded rows have geometry but no imported Origin.

Keep geometry() and source() total: they return None for Released rather than panicking or substituting a display mesh. origin() remains available in either imported state. release_epoch() distinguishes a current release from a later release cycle.

Scene insertion still requires PreparedMesh geometry. It captures metadata, creates the display Rc, and adopts Loaded ownership. The renderer still sees exactly the same mesh/model/id fields. At this endpoint no code unloads a row yet; existing precision, history and close tests must pass unchanged through the new representation.

![Represent source residency without changing retained display, identity or placement.](../illustrations/journey-32fg.svg)

## Type the change

Continue from [Adopt the selected file as a reloadable source](32ff-bridge.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32fg-state` (from `session_viewer`).

### 1. `src/lib.rs`

Confine source residency representation to the viewer implementation.

Find this exact block:

```rust
pub mod reload_url;
```

Replace that block with:

```rust
--8<-- "journey/code/32fg-state-01.rs"
```

### 2. `src/edit_source.rs`

Keep Released ownership free of kernel mesh and Session values.

Create the file and type:

```rust
--8<-- "journey/code/32fg-state-02.rs"
```

### 3. `src/scene.rs`

Retain public display fields while replacing only editable ownership.

Find this exact block:

```rust
    geometry: Rc<session_rust::Mesh>,
    pub model: session_rust::Xform,
    source: Option<crate::document::Source>,
```

Replace that block with:

```rust
--8<-- "journey/code/32fg-state-03.rs"
```

### 4. `src/scene.rs`

Borrow loaded owners safely while retaining origin and epoch in the released state.

Find this exact block:

```rust
    pub fn geometry(&self) -> Option<&Rc<session_rust::Mesh>> { Some(&self.geometry) }

    pub fn source(&self) -> Option<&crate::document::Source> { self.source.as_ref() }
```

Replace that block with:

```rust
--8<-- "journey/code/32fg-state-04.rs"
```

### 5. `src/scene.rs`

Preserve the validated insertion contract and display owner.

Find this exact block:

```rust
        self.objects.push(Object { id, guid, metadata, mesh: Rc::new(prepared.display), geometry: prepared.geometry,
            model: prepared.model, source: prepared.source });
```

Replace that block with:

```rust
--8<-- "journey/code/32fg-state-05.rs"
```

### 6. `src/scene.rs`

Keep partial-replacement ownership assertions through the existing accessor boundary.

Find this exact block:

```rust
            assert!(Rc::ptr_eq(&object.geometry, &old.geometry));
```

Replace that block with:

```rust
--8<-- "journey/code/32fg-state-06.rs"
```

### 7. `src/browser.rs`

Retain origin inspection even when editable kernel ownership is absent.

Find this exact block:

```rust
    let origins: Vec<_> = editor.scene.objects().iter().filter_map(|row| row.source().map(|source|
        (&row.guid, source.origin.id.to_string(), source.origin.version.hex(),
            source.origin.location.as_ref().map(|location| location.value())))).collect();
```

Replace that block with:

```rust
--8<-- "journey/code/32fg-state-07.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the residency checks below. A cold row has no editable source but retains display, metadata, identity and placement. Browser unloading comes later.

**Verified checkpoint in Chrome.**

![Actual browser result: Separate loaded and released editable ownership.](../screenshots/journey/32fg-state-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Follow the Rc edges in Loaded and Released. Explain why adding a kernel Rc to Released would defeat unloading even if source() returned None.

</details>

## Explain the change

Which owners remain in a Released row?

<details>
<summary>Compare your explanation</summary>

Its retained row data still owns the display mesh, metadata and placement. Editable storage owns only a geometry-free Origin and release epoch. It cannot retain the old kernel Mesh or Session.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32fg-state
npm --prefix ../session_tests run course -- save 32fg-state
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

The private representation now expresses the production distinction between retained display rows and resident editable source. The next policy prevents releasing generated or modified sources.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32fg-state
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
