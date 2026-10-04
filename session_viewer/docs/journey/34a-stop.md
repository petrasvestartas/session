# 34a · Stop the viewer when its GPU device fails

**Typing: 15–30 minutes.** [Estimate](typing-load.md).

Connect both GPU error callbacks to the same `Fault`. When the first error arrives, mark the device failed immediately so the input handler cannot submit another frame.

## Type

Continue from [Keep the first GPU failure with its device](34-fault.md). [Save or recover your work](recovery.md).

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

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the state checks. The first GPU failure must remain unchanged when another callback reports a later failure.

**Verified checkpoint in Chrome.**

![Actual browser result: Stop the viewer when its GPU device fails.](../screenshots/journey/34a-stop-browser.png)

[Verification scope](release.md).

Run the state checks on your computer from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked --target host-tuple -j4
```

<details>
<summary>Code explanation and diagram</summary>

Schedule disposal on the next microtask: the active callback must return before its Rust closure is released. Compare the captured Fault allocation with the installed runtime before taking that runtime out of its slot. An old device’s callback must leave a replacement alone.

Drop then cancels pending requests and removes input listeners. The page shows “Cannot draw”. Reloading starts a new viewer; automatic recovery comes later.

GPU callback → first fault recorded → input guard → deferred matching-owner disposal → visible failure.

![Stop a failed device’s runtime](../illustrations/journey-34a.svg)

Why record a failure before deferring cleanup, and why check the device identity again during cleanup?

The immediate signal blocks any intervening input or GPU updates. Deferred cleanup waits for the active callback to return; its identity check prevents an old device’s callback from stopping a replacement.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Move the failure guard below panel.update and explain which GPU writes could happen before it. Then remove the identity check in stop_if and explain how an older callback could stop a new runtime.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 34a-stop
npm --prefix ../session_tests run course -- save 34a-stop
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Production records the first GPU failure and gates UI/uploads/rendering before further GPU work. This checkpoint establishes the stopped lifetime. Structured diagnostics, persisted reports and bounded recovery remain next.

Chrome intercepts requestDevice in the test only and calls the real GPUDevice.destroy(). It observes the real lost promise and then verifies visible device-loss feedback, disposed runtime and zero further queue writes, submissions, allocations or surface configuration for attempted keyboard, camera, resize and late event input. A normal same-tab reload creates a working viewer again. The test’s observer is not part of the tutorial application.

Additional verification: Run the GPU-stop browser acceptance described below. Destroying the real test device must show “Cannot draw” and stop further rendering. Reloading the same page starts a fresh viewer.

Chrome destroys a real GPUDevice, verifies visible loss feedback and disposed listeners, and observes no GPU work from subsequent input. A same-tab reload produces the final working screenshot; it does not recover unsaved edits.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 34a-stop
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
