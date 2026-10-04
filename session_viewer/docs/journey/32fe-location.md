# 32fe · Give a reload URL an explicit owner

**Typing: 24–48 minutes.** [Estimate](typing-load.md).

A version says which bytes a row expects; it does not provide a way to read them again. Add ReloadUrl, an owner of one URL created by this viewer. Its Drop implementation revokes the browser Blob URL. Native checks observe its Rc lifetime through Weak; actual browser revocation is tested after the next adapter endpoint.

## Type

Continue from [Attach one origin to an imported document](32fd-origin.md). [Save or recover your work](recovery.md).

### 1. `src/lib.rs`

Keep browser URL lifetime explicit and independent of kernel ownership.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod origin;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32fe-location-01.rs"
```

### 2. `src/reload_url.rs`

Revoke a viewer-created Blob URL when its actual resource owner drops.

Create the file and type:

```rust
--8<-- "journey/code/32fe-location-02.rs"
```

### 3. `src/origin.rs`

Retain an optional reload location without retaining geometry.

<details>
<summary>Locate the existing block</summary>

```rust
    pub version: FileVersion,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32fe-location-03.rs"
```

### 4. `src/origin.rs`

Ordinary native loads start without an owned browser location.

<details>
<summary>Locate the existing block</summary>

```rust
        Self { id: uuid::Uuid::new_v4(), header, version: FileVersion::of(bytes) }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32fe-location-04.rs"
```

### 5. `src/document.rs`

Accept location ownership before a read/validation error can occur.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn load(bytes: &[u8]) -> Result<Loaded, &'static str> {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32fe-location-05.rs"
```

### 6. `src/document.rs`

Adopt the already owned URL only into validated source metadata.

<details>
<summary>Locate the existing block</summary>

```rust
    let origin = Rc::new(crate::origin::Origin::new(&message, bytes));
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32fe-location-06.rs"
```

### 7. `src/editor.rs`

Keep a URL owner alive until an import commits or fails.

<details>
<summary>Locate the existing block</summary>

```rust
    Replace(Vec<u8>),
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32fe-location-07.rs"
```

### 8. `src/editor.rs`

Preserve the same atomic insertion and replacement policy for located files.

<details>
<summary>Locate the existing block</summary>

```rust
            Action::AddBox => {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32fe-location-08.rs"
```

### 9. `src/origin_tests.rs`

Prove successful/history and failed-import URL lifetimes with Weak observers.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::editor::{Action, Editor};
use std::rc::Rc;

#[test]
fn origin_does_not_pin_a_closed_source_document() {
    let mut editor = Editor::default(); editor.apply(Action::Replace(crate::specimen::bytes())).unwrap();
    let row = &editor.scene.objects()[0]; let source = row.source().unwrap();
    let origin = Rc::clone(&source.origin);
    let document = Rc::downgrade(&source.document); let mesh = Rc::downgrade(row.geometry().unwrap());
    assert!(editor.scene.objects().iter().all(|row| Rc::ptr_eq(&row.source().unwrap().origin, &origin)));
    editor.apply(Action::Close).unwrap();
    assert!(document.upgrade().is_none() && mesh.upgrade().is_none());
    assert_eq!(origin.header.name, "Three-piece frame"); assert!(origin.header.objects.is_none());
}

#[test]
fn duplicate_imports_and_history_keep_their_own_origins() {
    let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
    editor.apply(Action::Replace(bytes.clone())).unwrap(); editor.apply(Action::Import(bytes)).unwrap();
    let first = Rc::clone(&editor.scene.objects()[0].source().unwrap().origin);
    let second = Rc::clone(&editor.scene.objects()[3].source().unwrap().origin);
    assert_ne!(first.id, second.id); assert_eq!(first.version, second.version);
    editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
    editor.apply(Action::Undo).unwrap();
    assert!(Rc::ptr_eq(&first, &editor.scene.objects()[0].source().unwrap().origin));
    assert!(Rc::ptr_eq(&second, &editor.scene.objects()[3].source().unwrap().origin));
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32fe-location-09.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the location checks below. The reload location stays alive while owned and is released once after its final owner drops. File adoption is connected next.

**Verified checkpoint in Chrome.**

![Actual browser result: Give a reload URL an explicit owner.](../screenshots/journey/32fe-location-browser.png)

[Verification scope](release.md).

Run the state checks on your computer from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked --target host-tuple -j4
```

<details>
<summary>Code explanation and diagram</summary>

Origin can retain an optional Rc<ReloadUrl>. Generated data and ordinary native load calls have no reload location and will be protected from unload. load_at accepts an already owned URL; it validates the file and adopts the location into the same Origin shared by all rows.

Add ImportAt and ReplaceAt to the editor while keeping the existing Import/Replace fixtures. The new actions hold the URL before loading, so failure drops their owner normally. History shares the Origin, keeping the reload location alive until the last relevant snapshot is gone. The browser still uses its old file adapter at this checkpoint.

Retain an owned Blob URL through imports and history, then release it with its last owner..

![Retain an owned Blob URL through imports and history, then release it with its last owner.](../illustrations/journey-32fe.svg)

Why must URL ownership exist before validation can fail?

Creating a Blob URL allocates a browser resource. If decoding or insertion fails, an action-local owner must still revoke it. A successful import shares that owner through its Origin; Close releases it only after active and history roots are gone.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Retain a separate Rc<ReloadUrl> in a caller scope and explain why closing the editor cannot revoke that caller’s resource yet.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 32fe-location
npm --prefix ../session_tests run course -- save 32fe-location
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This owner is for viewer-created Blob URLs. The later published HTTP locations have different ownership and caching rules; they must not be treated as owned Blob resources.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32fe-location
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
