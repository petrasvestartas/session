# 34a · Stop the viewer when its GPU device fails

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 15–30 minutes.** 28 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Connect actual GPU failure callbacks, stop new UI and GPU work, and dispose only the runtime that belongs to the failed device.

**Follow:** GPU callback → first fault recorded → input guard → deferred matching-owner disposal → visible failure.

Connect both GPU error callbacks to the same `Fault`. When the first error arrives, mark the device failed immediately so the input handler cannot submit another frame.

Schedule disposal on the next microtask: the active callback must return before its Rust closure is released. Compare the captured Fault allocation with the installed runtime before taking that runtime out of its slot. An old device’s callback must leave a replacement alone.

Drop then cancels pending requests and removes input listeners. The page shows “Cannot draw”. Reloading starts a new viewer; automatic recovery comes later.

![Stop a failed device’s runtime](../illustrations/journey-34a.svg)

## Type the change

Continue from [Keep the first GPU failure with its device](34-fault.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-34a-stop` (from `session_viewer`).

### 1. `src/browser_runtime.rs`

Retain the device failure identity beside the callbacks that use that device.

<details>
<summary>Locate the existing block</summary>

```rust
    reload: Shared,
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34a-stop-01.rs"
```

### 2. `src/browser_runtime.rs`

Only the matching device may dispose the active runtime; drop its owner after the slot borrow ends.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn install(listeners: Listeners, request: Rc<RefCell<ReadGate>>, reload: Shared) {
    ACTIVE.with(|slot| slot.replace(Some(Runtime { _listeners: listeners, request, reload })));
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34a-stop-02.rs"
```

### 3. `src/browser.rs`

Mark failure immediately, then defer identity-checked disposal and visible feedback until callbacks return.

<details>
<summary>Locate the existing block</summary>

```rust
fn save_result(bytes: &[u8]) -> Result<String, String> {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34a-stop-03.rs"
```

### 4. `src/browser.rs`

Connect the real error and device-loss callbacks to this device’s shared signal.

<details>
<summary>Locate the existing block</summary>

```rust
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34a-stop-04.rs"
```

### 5. `src/browser.rs`

Refuse new input, UI updates and GPU work as soon as the callback marks a failure.

<details>
<summary>Locate the existing block</summary>

```rust
    let owned_reload = std::rc::Rc::clone(&reload);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34a-stop-05.rs"
```

### 6. `src/browser.rs`

Install the runtime with the same device identity used by its failure callbacks.

<details>
<summary>Locate the existing block</summary>

```rust
    crate::browser_runtime::install(listeners, owned_request, owned_reload);
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34a-stop-06.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the GPU-stop browser acceptance described below. Destroying the real test device must show “Cannot draw” and stop further rendering. Reloading the same page starts a fresh viewer.

**Verified checkpoint in Chrome.**

![Actual browser result: Stop the viewer when its GPU device fails.](../screenshots/journey/34a-stop-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Move the failure guard below panel.update and explain which GPU writes could happen before it. Then remove the identity check in stop_if and explain how an older callback could stop a new runtime.

</details>

## Explain the change

Why record a failure before deferring cleanup, and why check the device identity again during cleanup?

<details>
<summary>Compare your explanation</summary>

The immediate signal blocks any intervening input or GPU updates. Deferred cleanup waits for the active callback to return; its identity check prevents an old device’s callback from stopping a replacement.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34a-stop
npm --prefix ../session_tests run course -- save 34a-stop
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Production records the first GPU failure and gates UI/uploads/rendering before further GPU work. This checkpoint establishes the stopped lifetime. Structured diagnostics, persisted reports and bounded recovery remain next.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Chrome intercepts requestDevice in the test only and calls the real GPUDevice.destroy(). It observes the real lost promise and then verifies visible device-loss feedback, disposed runtime and zero further queue writes, submissions, allocations or surface configuration for attempted keyboard, camera, resize and late event input. A normal same-tab reload creates a working viewer again. The test’s observer is not part of the tutorial application.

Chrome destroys a real GPUDevice, verifies visible loss feedback and disposed listeners, and observes no GPU work from subsequent input. A same-tab reload produces the final working screenshot; it does not recover unsaved edits.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 34a-stop
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
