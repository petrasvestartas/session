# 29 · Give each saved object a stable identity

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 19–38 minutes.** 38 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Distinguish a scene object from the original geometry it shares.

**Follow:** Source GUID → inserted object GUID → cloned history → stable file identity.

Save needs an identifier that can connect a stored mesh, its tree row and its placement. ObjectId already names objects within this running editor, but it is a small counter belonging to this scene. The source GUID belongs to the imported geometry. Neither distinction should disappear when we write a file.

Add a GUID to Object. Prefer the original geometry GUID when it is unused. When the same file is imported again, assign a fresh UUID to the new object. Keep the original kernel mesh and Source provenance untouched. The new identity belongs to this inserted object.

![The original source GUID can be shared, but each inserted object keeps one distinct GUID through history.](../illustrations/journey-29.svg)

The while loop checks the candidate against live objects. Even a generated UUID is checked, so insertion’s uniqueness rule is explicit. Assignment happens once during insertion. Cloning a scene for history clones the stored String; Move and saving must never regenerate it.

The check imports the same bytes twice. Corresponding sources have the same GUID, while all inserted objects have distinct GUIDs. It removes an earlier object and travels through Undo/Redo to prove identity is independent of display row and history position. Save is not connected yet; this prepares its identifiers.

## Type the change

Continue from [Prove source ownership survives editing](28c-ownership.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-29-identity` (from `session_viewer`).

### 1. `Cargo.toml`

Declare the UUID generator explicitly; browser randomness uses its js feature.

Find this exact block:

```toml
prost = "=0.14.4"
```

Replace that block with:

```toml
--8<-- "journey/code/29-identity-01.toml"
```

### 2. `src/scene.rs`

Store the inserted object identity separately from kernel source identity and the local counter.

Find this exact block:

```rust
    pub id: ObjectId,
```

Replace that block with:

```rust
--8<-- "journey/code/29-identity-02.rs"
```

### 3. `src/scene.rs`

Keep the source GUID when available; assign and check a fresh stored GUID for a duplicate import.

Find this exact block:

```rust
        self.objects.push(Object { id, mesh: Rc::new(prepared.display), geometry: prepared.geometry,
```

Replace that block with:

```rust
--8<-- "journey/code/29-identity-03.rs"
```

### 4. `src/file_identity_tests.rs`

Exercise duplicate imports, row removal and history without mutating original source GUIDs.

Create the file and type:

```rust
--8<-- "journey/code/29-identity-04.rs"
```

### 5. `src/lib.rs`

Compile the file-identity regression check.

Find this exact block:

```rust
#[cfg(test)]
mod source_tests;
```

Replace that block with:

```rust
--8<-- "journey/code/29-identity-05.rs"
```

## Run and look

After typing the manifest, run this from `session_viewer` to select the fixed dependency versions. It updates Cargo.lock, preserves the previous lock, and installs any supplied binary font assets. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 29-identity
```

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Import `sample.pb` twice. Run the identity checks: copies may share a source GUID, but each scene object must have its own saved identity.

**Verified checkpoint in Chrome.**

![Actual browser result: Give each saved object a stable identity.](../screenshots/journey/29-identity-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Inspect both imported copies’ source GUIDs and object GUIDs in the check. Predict which pair matches. Add another Undo/Redo cycle, then restore the check.

</details>

## Explain the change

Why can two imports share a source GUID but need different saved GUIDs?

<details>
<summary>Compare your explanation</summary>

The source GUID identifies geometry in an original file. Two imports create two scene objects with separate placements and histories. Each needs a unique stored identity. We preserve the source GUID when available, assign a fresh UUID on collision, and store that decision in the object so saving or row changes cannot rename it.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 29-identity
npm --prefix ../session_tests run course -- save 29-identity
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

The production viewer keeps unique object identity in editable sessions and distinguishes it from shared definitions or source references. Our flat mesh editor establishes that boundary before serializing placements.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 29-identity
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
