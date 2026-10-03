# 28b · Give generated objects the same source owner

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 16–31 minutes.** 25 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Build demo triangles and Example Box from kernel geometry; derive all displays on insertion.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Kernel triangle or box → PreparedMesh → source-owning Object → derived display.

**Before you finish, explain:** What does None provenance mean after every object owns kernel geometry?

Close the migration. Demo triangles and the optional extra triangle are small kernel meshes; Example Box retains the kernel mesh it already creates. All enter through PreparedMesh. Every Object can now keep its source geometry, placement and optional file provenance independently of the drawing cache.

![Imported and generated source geometry converge on the same preparation and insertion boundary.](../illustrations/journey-28b.svg)

PreparedMesh::triangle creates three kernel Points and one triangular face. Color is an object attribute. The kernel stores its colour channels in bytes, so these demo colours are quantized before the display adapter reads them; a slight shade change is expected. This does not change the source/drawing ownership rule.

Change Scene::insert to consume a prepared value. The geometry goes to Object::geometry, display goes into the Rc drawing cache, and provenance goes to Object::source. Remove Option from geometry because every call now supplies it. Identity still initializes placement.

Example Box stops discarding its kernel mesh after conversion. Imported objects use the same insertion function, so import no longer patches source fields on the last inserted row. Bounds, picking, GPU matrices and Move keep the ownership conventions already established.

## Type the change

Continue [Retain the imported mesh behind each row](28a-imported.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-28b-generated`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/prepared.rs`

Create demo source geometry before deriving the drawing representation.

Find this exact block:

```rust
impl PreparedMesh {
```

Replace that block with:

```rust
--8<-- "journey/code/28b-generated-01.rs"
```

### 2. `src/scene.rs`

All insertion callers now supply prepared source geometry.

Find this exact block:

```rust
use crate::mesh::Mesh;
```

Replace that block with:

```rust
--8<-- "journey/code/28b-generated-02.rs"
```

### 3. `src/scene.rs`

Every current object has source geometry; only imported-file provenance stays optional.

Find this exact block:

```rust
    pub geometry: Option<Rc<session_rust::Mesh>>,
```

Replace that block with:

```rust
--8<-- "journey/code/28b-generated-03.rs"
```

### 4. `src/scene.rs`

Describe the same two triangles as source points and an object colour.

Find this exact block:

```rust
        let pink = Mesh::new(vec![
            [-0.7, -0.6, 0.25, 0.9, 0.25, 0.45],
            [ 0.5, -0.6, 0.25, 0.9, 0.25, 0.45],
            [-0.1,  0.6, 0.25, 0.9, 0.25, 0.45],
        ], vec![0, 1, 2]).expect("Valid pink triangle");
        let turquoise = Mesh::new(vec![
            [-0.4, -0.2, 0.75, 0.05, 0.7, 0.7],
            [ 0.8, -0.2, 0.75, 0.05, 0.7, 0.7],
            [ 0.2,  0.8, 0.75, 0.05, 0.7, 0.7],
        ], vec![0, 1, 2]).expect("Valid turquoise triangle");
```

Replace that block with:

```rust
--8<-- "journey/code/28b-generated-04.rs"
```

### 5. `src/scene.rs`

Consume one prepared value into one source-owning document record.

Find this exact block:

```rust
    pub fn insert(&mut self, mesh: Mesh) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.objects.push(Object { id, mesh: Rc::new(mesh), geometry: None, model: session_rust::Xform::identity(), source: None });
        Ok(id)
    }
```

Replace that block with:

```rust
--8<-- "journey/code/28b-generated-05.rs"
```

### 6. `src/scene.rs`

Import uses the same complete record insertion as generated geometry.

Find this exact block:

```rust
        for prepared in loaded.meshes {
            self.insert(prepared.display)?;
            let object = self.objects.last_mut().unwrap();
            object.geometry = Some(prepared.geometry);
            object.source = prepared.source;
        }
```

Replace that block with:

```rust
--8<-- "journey/code/28b-generated-06.rs"
```

### 7. `src/scene.rs`

Retain the kernel box instead of keeping only its display.

Find this exact block:

```rust
        let id = self.insert(Mesh::from_kernel(&source)?)?;
```

Replace that block with:

```rust
--8<-- "journey/code/28b-generated-07.rs"
```

### 8. `src/scene.rs`

Keep source geometry for the optional triangle too.

Find this exact block:

```rust
            let extra = Mesh::new(vec![
                [-0.9, 0.3, 0.5, 0.2, 0.8, 0.3],
                [-0.4, 0.3, 0.5, 0.2, 0.8, 0.3],
                [-0.65, 0.9, 0.5, 0.2, 0.8, 0.3],
            ], vec![0, 1, 2]).expect("Valid extra triangle");
```

Replace that block with:

```rust
--8<-- "journey/code/28b-generated-08.rs"
```

### 9. `src/imported_source_tests.rs`

The source is now required, so remove the temporary Option unwrap in the check.

Find this exact block:

```rust
    let geometry = Rc::clone(imported.geometry.as_ref().unwrap());
```

Replace that block with:

```rust
--8<-- "journey/code/28b-generated-09.rs"
```

### 10. `src/imported_source_tests.rs`

Compare the required shared owner after its display row changes.

Find this exact block:

```rust
    assert!(Rc::ptr_eq(moved_row.geometry.as_ref().unwrap(), &geometry));
```

Replace that block with:

```rust
--8<-- "journey/code/28b-generated-10.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run Example Box, Select Next three times, Move 0.4,0.2,0 and Fit Selected. The placed box still works. All five kinds of row created so far—two demos, extra triangle, generated box and imported meshes—retain kernel geometry. The next check proves history shares it.

**Actual Chrome screenshot.**

Give generated objects the same source owner. These commands run in the actual dock; kernel ownership is checked separately in Rust.

![Actual browser result: Give generated objects the same source owner.](../screenshots/journey/28b-generated-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Change PreparedMesh::triangle’s object colour, then compare its kernel colour with the uploaded display colour. Explain why the conversion may quantize channels and why changing the camera does not rewrite either geometry representation. Restore the original code.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

It means the object was made here instead of imported from a file. It no longer means geometry is missing: Object::geometry is now a required shared source mesh. The display is derived from that source on the same insertion path.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 28b-generated
npm --prefix ../session_tests run course -- save 28b-generated
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The maintained viewer creates and edits source geometry first, then synchronizes display tables. This checkpoint establishes that direction for every current mesh object. Later geometry families extend it without making GPU data the editable document.

[Validation status and course release](release.md).
