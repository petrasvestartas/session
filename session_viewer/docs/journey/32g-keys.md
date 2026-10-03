# 32g · Identify the source release a reload belongs to

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 19–37 minutes.** 47 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Borrow the current imported release identity and epoch before asking for source data.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Active cold rows → ReloadKey::of → unique imported Origin + release epoch.

**Before you finish, explain:** Why is a file GUID insufficient to accept a reload result?

Define `ReloadKey` as an Origin owner and one release epoch. matches compares the actual imported Origin Rc and the row’s current epoch. That is the residency identity; local/saved object identity and source GUID remain separate.

Collect distinct keys from active cold rows. A later explicit reload can use them, while source adoption will also visit matching history roots. Do not fetch a history-only source without a current active request. If history later restores that source, a new request can obtain its current key.

The native test unloads, records a key, closes and imports the same bytes again. The old key cannot match the new import. A key with a changed epoch cannot match the current cold row either. No fetch or Reload Sources command exists yet; the browser retains the preceding unload behavior.

`Rc::clone` creates another owner of the same value; it does not copy the Origin. `Rc::ptr_eq` asks whether two owners refer to that same allocation. `Option<Self>` means a row might have no reload key: the question marks return None immediately for a loaded or unlocated row. `is_some_and` runs the comparison only when an origin exists.

![Borrow the current imported release identity and epoch before asking for source data.](../illustrations/journey-32g.svg)

## Type the change

Continue [Protect history and future reload tickets](32fl-guards.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32g-keys`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/lib.rs`

Give reload identity and preparation their own Rust boundary.

Find this exact block:

```rust
pub mod reload_url;
```

Replace that block with:

```rust
--8<-- "journey/code/32g-keys-01.rs"
```

### 2. `src/rehydrate.rs`

Match an imported allocation and checked release epoch instead of only a file GUID.

Create the file and type:

```rust
--8<-- "journey/code/32g-keys-02.rs"
```

### 3. `src/editor.rs`

Request each active imported release once without duplicating its geometry.

Find this exact block:

```rust
    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
```

Replace that block with:

```rust
--8<-- "journey/code/32g-keys-03.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the release-identity checks, then Unload Sources in the unchanged browser. Compare the current origin/epoch with the saved row GUID.

**Actual Chrome screenshot.**

The browser checks the existing command-only unload behavior and retained drawing. Source hydration at this endpoint is verified by the native state/GPU checks; browser fetch and automatic command replay are still pending.

![Actual browser result: Identify the source release a reload belongs to.](../screenshots/journey/32g-keys-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Compare two imports of the same file. Explain why sharing the byte version must not make a result for one import eligible for the other.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

Duplicate imports can share the same original GUID and byte version. A result must belong to this imported Origin allocation and its current release epoch. Loaded rows, closed rows and another release cycle must not match it.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32g-keys
npm --prefix ../session_tests run course -- save 32g-keys
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Production source fetches carry a release token. This explicit key prepares the same rejection of old completions across replacement, Close and another release cycle.

[Validation status and course release](release.md).
