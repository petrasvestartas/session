# 34fc · Preserve unsupported telemetry while pruning

**Typing: 26–51 minutes.** [Estimate](typing-load.md).

Separate retaining raw evidence from adopting it. The strict reader rejects newer schemas; using that accepted list to prune storage would also delete newer telemetry.

`retention_time` reads only a bounded version/date header. It returns a ranking timestamp without constructing a Report. A newer-format value can therefore occupy an older-report slot while remaining ineligible for a notice or typed download.

## Type

Continue from [Retrieve saved failure evidence through the command line](34fb-store.md). [Save or recover your work](recovery.md).

### 1. `src/report_store.rs`

Read a bounded date/version header for retention without adopting, converting or rewriting unsupported telemetry.

<details>
<summary>Locate the existing block</summary>

```rust
pub const MAX_BYTES: usize = 1024 * 1024;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34fc-retention-01.rs"
```

### 2. `src/report_storage.rs`

Retain raw supported or unsupported bounded JSON by header time; a failed read aborts before writing or pruning.

<details>
<summary>Locate the existing block</summary>

```rust
        let mut old = self.reports(); old.retain(|(key, _)| key != &self.key);
        old.retain(|(_, report)| js_sys::Date::parse(&report.last_seen).is_finite());
        old.sort_by(|a, b| js_sys::Date::parse(&b.1.last_seen).total_cmp(&js_sys::Date::parse(&a.1.last_seen)));
        let keep: Vec<_> = old.iter().take(2).map(|(key, _)| key).collect();
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34fc-retention-02.rs"
```

### 3. `src/report_store_tests.rs`

Prove unsupported fields/versions remain dateable without admission, with unchanged input and bounded header parsing.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{diagnostic::Report, diagnostic_shape_tests::report, report_store::{decode, MAX_BYTES}};

#[test]
fn stored_json_rejects_invalid_shape() {
    let good = serde_json::to_value(report()).unwrap();
    for (key, bad) in [("version", serde_json::json!(2)), ("tab", serde_json::json!("")),
        ("page", serde_json::json!("https://example.test/?private=1")), ("devicePixelRatio", serde_json::json!(0)),
        ("events", serde_json::json!([{"time":"now","kind":"fatal","message":"bad","elapsedMs":null}])),
        ("outcome", serde_json::json!("failed")), ("unknown", serde_json::json!(true))] {
        let mut value = good.clone(); value[key] = bad;
        assert!(decode(&value.to_string()).is_none(), "{key}");
    }
    assert!(decode("{").is_none());
}

#[test]
fn stored_json_bounds_bytes_events_and_text() {
    let json = serde_json::to_string(&report()).unwrap();
    let padding = MAX_BYTES - json.len(); let padded = json + &" ".repeat(padding);
    assert!(decode(&padded).is_some()); assert!(decode(&(padded + " ")).is_none());
    let mut value = serde_json::to_value(report()).unwrap();
    let event = serde_json::json!({"time":"now","kind":"phase","message":"test","elapsedMs":1.0});
    value["events"] = serde_json::json!(vec![event; 25]); assert!(decode(&value.to_string()).is_none());
    let mut bypass: Report = serde_json::from_value(value).unwrap();
    bypass.record("later".into(), 2.0, "phase", "bounded").unwrap();
    assert_eq!(serde_json::to_value(bypass).unwrap()["events"].as_array().unwrap().len(), 24);
    let mut failure = report(); failure.record("later".into(), 1.0, "fatal", "first reason").unwrap();
    let json = serde_json::to_string(&failure).unwrap(); assert_eq!(decode(&json), Some(failure));
    let mut value: serde_json::Value = serde_json::from_str(&json).unwrap();
    value["failure"]["message"] = serde_json::json!("😀".repeat(4097)); assert!(decode(&value.to_string()).is_none());
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34fc-retention-03.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the retention checks below. A retained newer-format report must keep its original JSON bytes, while the strict reader still refuses to adopt it.

**Verified checkpoint in Chrome.**

![Actual browser result: Preserve unsupported telemetry while pruning.](../screenshots/journey/34fc-retention-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

Read and rank older values before mutation. Keep the two newest raw strings byte for byte. If a read fails, abort rather than delete uninspected evidence. Invalid JSON or unusable headers can be pruned after a successful current write.

bounded raw JSON header → retention score → keep original stored bytes; supported schema → typed admission → candidate notice/download.

![Retain raw evidence separately from admission](../illustrations/journey-34fc.svg)

Why does refusing to adopt an unsupported report not justify treating it as malformed during pruning?

Its extra fields or version may belong to a newer viewer. Adoption requires our complete supported schema, but bounded retention needs only a usable version/date header. Keep its original stored bytes if it is one of the two newest older values; do not convert it through our older typed model.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Replace the retention-header loop with reports() again, then inspect the saved futureTelemetry value after pruning. Explain why refusing a previous-report download and deleting its original evidence are different decisions.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 34fc-retention
npm --prefix ../session_tests run course -- save 34fc-retention
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Report schema evolves as detailed telemetry is taught. Admission stays strict, while bounded retention preserves newest unsupported raw evidence without conversion. Heartbeat/lifecycle observations, new telemetry fields and bounded recovery follow.

Native tests separate retention from admission and cover byte/date/version bounds without mutating input. Headed Chrome stores actual unknown telemetry in version2, preserves its exact whitespace-bearing JSON through repeated writes and proves the typed reader still refuses it. A controlled getItem denial leaves all prior values unchanged and performs no current write. All inherited real GPU-loss, reload, previous-download and scene/history checks remain.

Chrome preserves exact raw JSON from a newer-format report through actual pruning, refuses typed adoption and aborts safely when reading an older value is denied. Full inherited live diagnostics and scene/history checks pass.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 34fc-retention
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
