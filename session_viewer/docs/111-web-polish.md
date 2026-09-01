# 111 Web polish — load progress and a shippable wasm

> **Big picture.** *Phase 14 closes.* Everything so far assumed localhost, where the manifest's 10
> sheets — 2.8–132 MB of `.pb` each, ~0.5 GB total — arrive instantly and nobody reads the wasm
> size. Shipped over a real network, both bite: half a gigabyte takes real time on a slow link with
> **no per-sheet feedback**, and a debug-ish wasm is several times larger than it needs to be. Two
> fixes, both measurable — this lesson's verify steps are numbers, not pixels.

## Files we touch

```
src/app/persistence.rs   # fetch_finish_with_progress — streamed read, % into the CLI log
src/lib.rs               # the loader's call site — throttled ticks, a loud fetch error
Cargo.toml               # web-sys stream features + [profile.release] size/speed knobs
index.html               # data-wasm-opt: the Trunk-side optimizer setting
```

## Step 1 — streamed fetch with progress: `src/app/persistence.rs` + `src/lib.rs`

The loader is no longer one gulp: fetches are pipelined (`fetch_start`/`fetch_finish`, a window of
2 — sheet N+1 downloads while N parses) and parsing is sliced (`session_from_bytes_chunked`, 25k
objects per `setTimeout(0)` slice), so the tab already stays live. What's still missing is
**feedback**: per-item progress ("sheet 3/10, 42%") and the network leg of a single large file —
`fetch_finish`'s `array_buffer()` await reports nothing until it reports everything. The streaming
version below fills that gap, reading the body in chunks against `Content-Length`. It **extends
`fetch_finish`** instead of issuing its own GET: it takes the `Fetch` the loader already has in
flight, so the window of 2 survives (sheet N+1 streams while N parses). A version that called
`fetch_with_str(url)` itself would look identical and quietly serialise every sheet:

<svg viewBox="0 0 460 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="one-gulp array_buffer awaits the whole file with a dead frozen tab then jumps to 100%, while the streamed reader loop yields a frame per chunk and ticks 5 10 100 percent" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="230" y="16" fill="#888" text-anchor="middle">same 132 MB sheet over a slow link</text>
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

Reading a body in chunks needs two more `web-sys` bindings. The third one this uses, `"Headers"`
(for `Content-Length`), has been on the list since 44's Range reader.

**Find** in `Cargo.toml`:

```toml
    "Headers",
```

**Add below it:**

```toml
    "ReadableStream",
    "ReadableStreamDefaultReader",
```

Then the reader itself. **Find** in `src/app/persistence.rs`:

```rust
pub async fn fetch_finish(f: Fetch) -> Result<Vec<u8>, JsValue>{
    let resp: Response = f.fut.await?.dyn_into()?;
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buf).to_vec())
}
```

**Add below it:**

```rust
/// `fetch_finish`, but chunked: calls `progress(loaded, total)` as the body streams in.
/// Takes the SAME in-flight `Fetch` the loader started - re-issuing the GET here would
/// throw away the window of 2 and serialise every sheet.
/// total == 0 when the server omits Content-Length (report bytes instead of % then).
pub async fn fetch_finish_with_progress(
    f: Fetch,
    progress: impl Fn(u64, u64),
) -> Result<Vec<u8>, JsValue> {
    let resp: Response = f.fut.await?.dyn_into()?;
    let total: u64 = resp.headers().get("Content-Length").ok().flatten()
        .and_then(|s| s.parse().ok()).unwrap_or(0);

    let body = resp.body().ok_or_else(|| JsValue::from_str("no body"))?;
    let reader: web_sys::ReadableStreamDefaultReader =
        body.get_reader().dyn_into()?;
    let mut out: Vec<u8> = Vec::with_capacity(total as usize);
    loop {
        let chunk = JsFuture::from(reader.read()).await?;
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

The callback feeds the CLI log line — throttled so the log isn't 500 lines of percentages. Two
boundary rules get honored here: `total == 0` (no `Content-Length`) must still *report* — throttle
on bytes, or the percentage sits at 0 forever and the user gets silence; and a **failed fetch is an
error, not an empty Vec** — `unwrap_or_default()` would feed zero bytes to the parser and append a
phantom empty doc (46's scene-wipe class of bug, at the network boundary). That
`unwrap_or_default()` is exactly what the loader does today — the fetch loop lives in `src/lib.rs`
`resumed()`. **Find** in `src/lib.rs`:

```rust
                    let bytes = match cur {
                        Some(Ok(f)) => persistence::fetch_finish(f).await.unwrap_or_default(),
                        _ => Vec::new(),
                    };
```

**Replace with:**

```rust
                    // one line per 5%, prefixed with the manifest position ("sheet 3/10")
                    let last = std::cell::Cell::new(0u64);
                    let tag = format!("sheet {}/{}", i + 1, count);
                    let bytes = match cur {
                        Some(Ok(f)) => match persistence::fetch_finish_with_progress(f, |loaded, total| {
                            if total > 0 {
                                let pct = loaded * 100 / total;
                                if pct >= last.get() + 5 || (pct == 100 && last.get() != 100) {
                                    last.set(pct);
                                    crate::engine::gpu::errors::push_gpu_error(format!("{tag} … {pct}%"));
                                }
                            } else {
                                // no Content-Length - pct would sit at 0 forever; step on 8 MB
                                let step = loaded / (8 * 1024 * 1024);
                                if step > last.get() {
                                    last.set(step);
                                    crate::engine::gpu::errors::push_gpu_error(
                                        format!("{tag} … {:.0} MB", loaded as f64 / 1.048576e6));
                                }
                            }
                        }).await {
                            Ok(b) => b,
                            Err(e) => {
                                crate::engine::gpu::errors::push_gpu_error(
                                    format!("fetch {} failed: {e:?}", item.file));
                                continue;
                            }
                        },
                        _ => Vec::new(),
                    };
```

(`push_gpu_error` = the same static-queue-to-CLI-log channel 88 built for GPU errors in
`engine/gpu/errors.rs` — second customer, despite the name. Note the read loop yields to the
browser between chunks — the chunked parse already kept the tab live; this makes the network leg
report progress instead of silence.)

## ⚠ The memory ceiling — doc lifecycle

The manifest's ten sheets are ~0.5 GB **resident** — and the *transient* cost of opening one is
3–4× its final size: file bytes + protobuf decode + kernel build ride together before the
intermediates drop (34d/37's accounting). Two facts make that the real budget: wasm32 linear memory
**never shrinks** (71's note — freed Rust memory stays mapped), and the address space caps at 4 GB,
less in practice (81's ledger). So the number that matters is the **peak**, and today's policy is
simply: fetch window of 2, drop bytes right after parse, don't open a second 0.5 GB scene.

When a real project outgrows that, the lever is **doc unload/eviction**, and the architecture
already has the seams for it: a doc is a `Session` + arena slots (45) + caches (44's tess, 52's
boxes) — all droppable while the manifest entry stays; reload on focus is 46's reconcile path.
Out of scope for this course; name the shape now so the day the browser kills your tab at 3.8 GB
you know which lever exists.

## Step 2 — the wasm diet: `Cargo.toml` + `index.html`

Measure first — the number you're improving lives in `dist/`:

```bash
trunk build --release && ls -l dist/*.wasm     # write this number down
```

Then the three standard knobs. **`Cargo.toml` already has a `[profile.release]` table** (`strip =
true`, from lesson 02) — the three keys go **into that existing table**; a second
`[profile.release]` header is a duplicate-key TOML error and `cargo build --release` refuses to
start.

**Find** in `Cargo.toml`:

```toml
strip = true
```

**Add below it:**

```toml
opt-level = "z"       # optimize for SIZE (CAD hot loops live in the kernel's f64 math,
lto = true            #   which 'z' barely slows; 's' if you measure a real regression)
codegen-units = 1     # slower compile, smaller + faster binary
```

One line in `index.html` carries the Trunk-side setting. **Find** in `index.html`:

```html
  <link data-trunk rel="rust" data-target-name="session_viewer" data-wasm-opt="0"/>
```

**Replace with:**

```html
  <link data-trunk rel="rust" data-target-name="session_viewer" data-wasm-opt="z"/>
```

That last one matters more than it looks: the course pinned `data-wasm-opt="0"` back in lesson 01
**for fast dev rebuilds** — correct then, but it also means every `trunk build --release` so far
shipped *unoptimized* wasm. `"z"` runs Binaryen's `wasm-opt` as the final pass; expect the combined
knobs to cut the binary roughly in half (measure — the lesson's claim is checkable in one command).

One more switch hides in `Cargo.toml`: `[package.metadata.wasm-pack.profile.release]` with
`wasm-opt = false` is a **second, separate** optimizer setting — it governs wasm-pack builds
independently of Trunk's `data-wasm-opt`, so flip the one your build path actually uses (or both).

Serving note, not a code change: static hosts (GitHub Pages, nginx) should serve `.wasm` with
gzip/brotli — another ~2–3× on the wire for free. Trunk's output is already compressible; there is
nothing to do in the app.

## Step 3 — verify (numbers, not pixels)

- DevTools → Network → throttle **Slow 3G** → load the manifest scene: the CLI ticks
  `sheet 3/10 … 42%`, the tab never freezes (it already didn't — the chunked parse saw to that),
  orbit works while later sheets stream in. The contrast with before is **feedback**: per-sheet
  percentages instead of silence. That contrast is the lesson.
- `ls -l dist/*.wasm` before vs after Step 2 — record both numbers in your notes; the pair is the
  proof. Cold-load time on the throttled profile should drop proportionally.
- `trunk serve` (dev) still uses fast unoptimized builds — the `"z"` costs you nothing during
  development; it's Release that changed.

## Recap

```
Ch 88: the workflow.
Ch 89: THE LAST MILE. Streamed fetch: fetch_finish_with_progress EXTENDS fetch_finish - same
       in-flight Fetch, ReadableStreamDefaultReader chunks + Content-Length →
       per-sheet progress into the CLI (throttled to 5% steps — BYTES when the server omits
       Content-Length, else pct pins at 0; a failed fetch is a loud CLI error, never an empty
       Vec → phantom doc; the chunked parse already kept the tab painting — this adds the network
       leg, without giving up the fetch window). The memory ceiling: ~0.5 GB resident sheets,
       3–4× transient per open, wasm linear memory never shrinks and caps at 4 GB — the budget is
       the PEAK; doc unload/eviction is the named lever when a scene outgrows it. Wasm diet: opt-level 'z' + lto + codegen-units 1 + Trunk data-wasm-opt 'z' (the
       course's dev-friendly '0' silently shipped unoptimized release wasm until now) ≈ half the
       binary — MEASURED, one ls before and after; brotli on the host for the wire. Phase 14
       complete: sectioned, file-fluent, duplicating, layered, measuring, testable, shippable.
```

Edited: `app/persistence.rs` (`fetch_finish_with_progress`), `src/lib.rs` (the throttled call site
in `resumed()`), `Cargo.toml` (2 web-sys stream features + `[profile.release]`), `index.html`
(`data-wasm-opt="z"`).

## Next

`112-textures.md` — an **optional appendix** outside the CAD default look: upload an image, bind a
texture + sampler onto the existing mesh pass, sample it in the fragment shader. The one rendering
idea the main path skips. After that the roadmap is done; what remains is the honest backlog, in two
files: `_KERNEL_GAPS.md` (the STEP port is the big one) and 81's perf levers, both waiting on real
scenes to demand them. Build something with it.
