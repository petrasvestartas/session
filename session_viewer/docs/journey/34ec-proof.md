# 34ec · Prove saved-run exclusions before adopting storage

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 13–26 minutes.** 19 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Complete timestamp-policy acceptance before a stored candidate can create a notice or previous-report download.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** candidate → invalid clock/healthy state/incorrect chronology → excluded without mutation.

**Before you finish, explain:** Why test healthy and invalid-clock cases before wiring the policy to localStorage?

Eligibility needs both accepted and rejected examples. The preceding checkpoint covers recent failure, old failure with a fresh heartbeat, active/interrupted other tabs and temporal cutoffs. This checkpoint completes the exclusion proof before the browser adopts stored values.

Native tests reject invalid current clocks, failed timestamp parsing, start after heartbeat and a first failure before start. Ready and closed runs remain quiet. The same report compares equal before and after policy evaluation: selection reads metadata without altering its evidence. Browser Date.parse and storage are still connected next.

Chrome retains actual report-download, GPU-loss and startup-restart acceptance. This proof adds no timer, listener, visible control or storage adoption.

![Exclude false interruption evidence](../illustrations/journey-34ec.svg)

## Type the change

Continue [Choose a recent failure without blaming active tabs](34eb-recency.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-34ec-proof`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/report_recency_tests.rs`

Prove invalid clocks, healthy/closed runs, invalid failure ordering and unchanged metadata cannot become interruption evidence.

Find this exact block:

```rust
use crate::{diagnostic::Outcome, diagnostic_shape_tests::report, report_recency::eligible};

#[test]
fn previous_run_selection_respects_actual_failure_time() {
    let now = 1_000_000_000.0;
    for (name, failed, same, seen_age, failure_age, expected) in [
        ("old failure, fresh heartbeat", true, false, 5000.0, 259_200_000.0, false),
        ("recent failure", true, false, 5000.0, 10000.0, true),
        ("failure at cutoff", true, false, 5000.0, 7_200_000.0, false),
        ("failure after heartbeat", true, false, 5000.0, 1000.0, false),
        ("future heartbeat", true, false, -1.0, 1000.0, false),
        ("future failure", true, false, 0.0, -1.0, false),
        ("active other tab", false, false, 120_000.0, 0.0, false),
        ("interrupted other tab", false, false, 120_001.0, 0.0, true),
        ("stale other tab", false, false, 7_200_000.0, 0.0, false),
        ("interrupted same tab", false, true, 0.0, 0.0, true),
    ] {
        let mut value = report(); value.started = "start".into();
        if failed { value.record("failed".into(), 1.0, "fatal", "original").unwrap(); }
        value.last_seen = "seen".into(); value.tab = if same { "current" } else { "other" }.into();
        let parser = |time: &str| match time { "start" => now - 259_200_000.0, "seen" => now - seen_age,
            "failed" => now - failure_age, _ => f64::NAN };
        let selected = eligible(&value, now, "current", parser);
        assert_eq!(selected.is_some(), expected, "{name}");
        if expected { assert_eq!(selected, Some(now - if failed { failure_age } else { seen_age })); }
    }
}
```

Replace that block with:

```rust
--8<-- "journey/code/34ec-proof-01.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Import sample.pb, move its selected beam, then type Diagnostic Report to download viewer-diagnostic.json. Save still downloads viewer.session. Run the native tests for this lesson’s report policy. Finish the healthy proof view with Move 0,-0.05,0 and Fit. Chrome also verifies an actual GPU-loss report and startup-failure download, followed by a healthy reload. Saved previous reports and unsaved-edit recovery are not connected yet.

**Actual Chrome screenshot.**

Native tests prove invalid/healthy/closed/bad-order candidates are excluded without mutation. Chrome retains existing real diagnostic/loss/download acceptance; browser storage follows next.

![Actual browser result: Prove saved-run exclusions before adopting storage.](../screenshots/journey/34ec-proof-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Make the default outcome branch return a score for Ready, then run the tests and describe the false notice. Make the parser return NaN for the failure and explain why comparison/ranking must reject it.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The browser will see values from other tabs and old or edited storage. A false interruption notice is visible user behavior, so exclusions need explicit acceptance before adoption.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34ec-proof
npm --prefix ../session_tests run course -- save 34ec-proof
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Previous-report notices must avoid blaming a live or healthy tab and must reject invalid chronology. These native exclusions complete the policy acceptance before browser storage is connected.

[Validation status and course release](release.md).
