# 84 Web polish — load progress and a shippable wasm

> **Big picture.** *Phase 14 closes.* Everything so far assumed localhost, where a 17.5 MB `.pb`
> arrives instantly and nobody reads the wasm size. Shipped over a real network, both bite: the
> stress file takes seconds on a slow link with **zero feedback** (a frozen-looking tab), and a
> debug-ish wasm is several times larger than it needs to be. Two fixes, both measurable — this
> lesson's verify steps are numbers, not pixels.

## Files we touch

```
src/app/persistence.rs   # fetch_bytes_with_progress — streamed read, % into the CLI log
Cargo.toml               # [profile.release] — the size/speed knobs
index.html               # data-wasm-opt: the Trunk-side optimizer setting
```

## Step 1 — streamed fetch with progress: `src/app/persistence.rs`

34a's `fetch_bytes` awaits `array_buffer()` — one gulp, no feedback. The streaming version reads the
body in chunks and reports against `Content-Length`:

<svg viewBox="0 0 460 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="one-gulp array_buffer awaits the whole file with a dead frozen tab then jumps to 100%, while the streamed reader loop yields a frame per chunk and ticks 5 10 100 percent" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="230" y="16" fill="#888" text-anchor="middle">same 17.5 MB file over a slow link</text>
  <text x="8" y="46" fill="#e06c6c">array_buffer()</text>
  <rect x="120" y="34" width="240" height="18" fill="none" stroke="#e06c6c" stroke-width="1.2"/>
  <text x="240" y="47" fill="#e06c6c" text-anchor="middle">dead tab — no paint, no feedback</text>
  <rect x="360" y="34" width="24" height="18" fill="#e06c6c" opacity="0.35"/>
  <text x="372" y="47" fill="#d7dae0" text-anchor="middle" font-size="9">100%</text>
  <text x="8" y="96" fill="#5bbf87">reader.read()</text>
  <line x1="120" y1="88" x2="384" y2="88" stroke="#555"/>
  <rect x="120" y="82" width="30" height="12" fill="#5bbf87" opacity="0.35"/>
  <rect x="168" y="82" width="30" height="12" fill="#5bbf87" opacity="0.35"/>
  <rect x="216" y="82" width="30" height="12" fill="#5bbf87" opacity="0.35"/>
  <rect x="300" y="82" width="30" height="12" fill="#5bbf87" opacity="0.35"/>
  <rect x="354" y="82" width="30" height="12" fill="#5bbf87" opacity="0.35"/>
  <text x="135" y="112" fill="#6fb3ff" text-anchor="middle" font-size="9">5%</text>
  <text x="183" y="112" fill="#6fb3ff" text-anchor="middle" font-size="9">10%</text>
  <text x="231" y="112" fill="#6fb3ff" text-anchor="middle" font-size="9">…</text>
  <text x="369" y="112" fill="#6fb3ff" text-anchor="middle" font-size="9">100%</text>
  <text x="120" y="132" fill="#666" font-size="10">each chunk yields to the browser → a frame paints, the CLI ticks</text>
</svg>

**Add three stream types to the web-sys `features` list in `Cargo.toml`** (beside the ones 34a
already enabled for `fetch`):

```toml
    "ReadableStream",
    "ReadableStreamDefaultReader",
    "Headers",
```

```rust
/// fetch_bytes, but chunked: calls `progress(loaded, total)` as the body streams in.
/// total == 0 when the server omits Content-Length (report bytes instead of % then).
pub async fn fetch_bytes_with_progress(
    url: &str,
    progress: impl Fn(u64, u64),
) -> Result<Vec<u8>, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let resp_value =
        wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(url)).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;
    let total: u64 = resp.headers().get("Content-Length").ok().flatten()
        .and_then(|s| s.parse().ok()).unwrap_or(0);

    let body = resp.body().ok_or_else(|| JsValue::from_str("no body"))?;
    let reader: web_sys::ReadableStreamDefaultReader =
        body.get_reader().dyn_into()?;
    let mut out: Vec<u8> = Vec::with_capacity(total as usize);
    loop {
        let chunk = wasm_bindgen_futures::JsFuture::from(reader.read()).await?;
        let done = js_sys::Reflect::get(&chunk, &"done".into())?
            .as_bool().unwrap_or(true);
        if done {
            break;
        }
        let value = js_sys::Reflect::get(&chunk, &"value".into())?;
        let arr = js_sys::Uint8Array::new(&value);
        let start = out.len();
        out.resize(start + arr.length() as usize, 0);
        arr.copy_to(&mut out[start..]);
        progress(out.len() as u64, total);
    }
    Ok(out)
}
```

The callback feeds the CLI log line — throttled so the log isn't 500 lines of percentages:

```rust
    // at the call site (34a's State::new / 79's open) — update at most every 5%:
    let last = std::cell::Cell::new(0u64);
    let bytes = fetch_bytes_with_progress(url, move |loaded, total| {
        let pct = if total > 0 { loaded * 100 / total } else { 0 };
        if pct >= last.get() + 5 || pct == 100 {
            last.set(pct);
            push_gpu_error(format!("loading… {pct}%  ({loaded} bytes)"));
        }
    }).await.unwrap_or_default();
```

(`push_gpu_error` = the same static-queue-to-CLI-log channel 83 built for GPU errors — second
customer, despite the name. Note the read loop yields to the browser between chunks — the tab stays live, which *is*
the feature; the old one-gulp await gave the browser nothing to paint.)

## Step 2 — the wasm diet: `Cargo.toml` + `index.html`

Measure first — the number you're improving lives in `dist/`:

```bash
trunk build --release && ls -l dist/*.wasm     # write this number down
```

Then the three standard knobs. **`Cargo.toml` already has a `[profile.release]` table** (`strip =
true`) — add the three keys **into that existing table**; a second `[profile.release]` header is a
duplicate-key TOML error and `cargo build --release` refuses to start:

```toml
[profile.release]
strip = true          # (already here from an earlier lesson — keep it)
opt-level = "z"       # optimize for SIZE (CAD hot loops live in the kernel's f64 math,
lto = true            #   which 'z' barely slows; 's' if you measure a real regression)
codegen-units = 1     # slower compile, smaller + faster binary
```

In `index.html`, **find the `data-trunk rel="rust"` link and change `data-wasm-opt="0"` → `"z"`**
(that `"0"` was pinned in lesson 01 for fast dev rebuilds):

```html
  <link data-trunk rel="rust" data-target-name="session_viewer" data-wasm-opt="z"/>
```

That last one matters more than it looks: the course pinned `data-wasm-opt="0"` back in lesson 01
**for fast dev rebuilds** — correct then, but it also means every `trunk build --release` so far
shipped *unoptimized* wasm. `"z"` runs Binaryen's `wasm-opt` as the final pass; expect the combined
knobs to cut the binary roughly in half (measure — the lesson's claim is checkable in one command).

Serving note, not a code change: static hosts (GitHub Pages, nginx) should serve `.wasm` with
gzip/brotli — another ~2–3× on the wire for free. Trunk's output is already compressible; there is
nothing to do in the app.

## Step 3 — verify (numbers, not pixels)

- DevTools → Network → throttle **Slow 3G** → load the stress file: the CLI ticks
  `loading… 5% … 100%`, the tab never freezes, orbit works the moment parsing finishes. Compare the
  old `fetch_bytes` (one gulp): seconds of dead tab. That contrast is the lesson.
- `ls -l dist/*.wasm` before vs after Step 2 — record both numbers in your notes; the pair is the
  proof. Cold-load time on the throttled profile should drop proportionally.
- `trunk serve` (dev) still uses fast unoptimized builds — the `"z"` costs you nothing during
  development; it's Release that changed.

## Recap

```
Ch 83: the workflow.
Ch 84: THE LAST MILE. Streamed fetch: ReadableStreamDefaultReader chunks + Content-Length →
       progress into the CLI (throttled to 5% steps; the chunk loop's awaits are what keep the tab
       painting). Wasm diet: opt-level 'z' + lto + codegen-units 1 + Trunk data-wasm-opt 'z' (the
       course's dev-friendly '0' silently shipped unoptimized release wasm until now) ≈ half the
       binary — MEASURED, one ls before and after; brotli on the host for the wire. Phase 14
       complete: sectioned, file-fluent, duplicating, layered, measuring, testable, shippable.
```

Edited: `app/persistence.rs` (`fetch_bytes_with_progress` + throttled call sites), `Cargo.toml`
(`[profile.release]`), `index.html` (`data-wasm-opt="z"`), `Cargo.toml` web-sys (3 stream features).

## Next

`85-textures.md` — an **optional appendix** outside the CAD default look: upload an image, bind a
texture + sampler onto the existing mesh pass, sample it in the fragment shader. The one rendering
idea the main path skips. After that the roadmap is done; what remains is the honest backlog, in two
files: `_KERNEL_GAPS.md` (the STEP port is the big one) and 76's perf levers, both waiting on real
scenes to demand them. Build something with it.
