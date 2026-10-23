# 1.1 · Rust and the first GPU frame

**Section 1: WebGPU Foundations, Shaders, and the Command Dock** · **Checkpoints:** `01-canvas` → `01a-gpu` (2 steps) · **Production source:** [`lib.rs`](../../../src/lib.rs) · [`engine/gpu/device.rs`](../../../src/engine/gpu/device.rs)

> **Goal:** Open a browser GPU and paint the whole canvas white.

## 1. Visual WebGPU Schematic

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="220" viewBox="0 0 840 220" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="216" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="20" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. &lt;canvas&gt; (px)</text>
  <rect x="45" y="56" width="180" height="110" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <circle cx="45" cy="56" r="3.5" fill="#111111"/>
  <text x="53" y="70" font-family="monospace" font-size="10" fill="#111111">(0,0)</text>
  <text x="175" y="160" font-family="monospace" font-size="10" fill="#111111">(W,H)</text>
  <rect x="65" y="140" width="140" height="16" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="135" y="151" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">&gt; Command Dock</text>
  <text x="135" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">DOM Surface + Events</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. CommandEncoder</text>
  <rect x="326" y="58" width="186" height="44" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="419" y="77" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Pass 1: LoadOp::Clear</text>
  <text x="419" y="93" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Clear + Scene Triangle</text>
  <line x1="419" y1="102" x2="419" y2="118" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <rect x="326" y="122" width="186" height="44" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="419" y="141" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Pass 2: LoadOp::Load</text>
  <text x="419" y="157" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Keep Pixels + Dock UI</text>
  <text x="419" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">queue.submit()</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. Clip Space (-1..+1)</text>
  <rect x="644" y="56" width="120" height="108" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <line x1="644" y1="110" x2="764" y2="110" stroke="#111111" stroke-width="1" stroke-dasharray="3 3"/>
  <line x1="704" y1="56" x2="704" y2="164" stroke="#111111" stroke-width="1" stroke-dasharray="3 3"/>
  <polygon points="704,68 662,148 746,148" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
  <circle cx="704" cy="68" r="3.5" fill="#111111"/>
  <circle cx="662" cy="148" r="3.5" fill="#111111"/>
  <circle cx="746" cy="148" r="3.5" fill="#111111"/>
  <text x="712" y="69" font-family="monospace" font-size="9" font-weight="700" fill="#111111">(0,+1)</text>
  <text x="606" y="152" font-family="monospace" font-size="9" fill="#111111">(-1,-1)</text>
  <text x="767" y="152" font-family="monospace" font-size="9" fill="#111111">(+1,-1)</text>
  <text x="704" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">@vertex -&gt; @fragment</text>
</svg>
</div>

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **HTML** | `<canvas id="canvas">` | Full-window pixel surface (`width/height: 100%`). |
| **Rust (`wgpu`)** | `Pass 1 (Clear)` → `Pass 2 (Load)` | Pass 1 clears and draws 3D scene; Pass 2 overlays Command Dock UI. |
| **WGSL** | `@vertex` & `@fragment` | Outputs clip coordinates `(-1..+1)` and RGBA pixel color. |

![Recorded visual output for Lesson 1.1 (01a-gpu)](../../screenshots/journey/01a-gpu-browser.png)

---

## 2. Complete Code Overview (All Steps Open)

<a id="checkpoint-01-canvas"></a>

### Step 1 of 2 · A page that Rust can reach (`01-canvas`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Cargo.toml, index.html</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">WebAssembly entry point</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">hidden startup status</text>
</svg>
</div>

Create a white canvas that fills the window and let Rust write a startup message to the hidden status element. This checks browser startup before we use the GPU.

`index.html` owns the page; `Cargo.toml` declares libraries; `src/lib.rs` supplies the browser entry point. `Result<(), JsValue>` returns either success with no value or a browser error. `?` stops at a failed window, document or element lookup.

> **Check:** *Does seeing the canvas prove that the GPU has drawn anything?* — No. HTML creates the canvas and CSS gives it a background. The status proves Rust ran; GPU commands begin in the next lesson.

From `session_viewer`, run `npm --prefix ../session_tests run course -- init`. It supplies the dependency lock. Create `workspace/journey/src`, then type the three files below. Keep using this project throughout the course.

### 1. `Cargo.toml`

Create the manifest. cdylib makes browser-loadable code; rlib will let our native checks use the same renderer.

Create the file and type:

```toml
--8<-- "journey/code/01-canvas-01.toml"
```

### 2. `index.html`

Let the canvas fill the window from the start. Keep startup information hidden; the documentation explains the lesson.

Create the file and type:

```html
--8<-- "journey/code/01-canvas-fullscreen-1.html"
```

### 3. `src/lib.rs`

Create Rust’s entry point. The browser calls start after loading WebAssembly; ? returns an error if a required element is missing.

Create the file and type:

```rust
--8<-- "journey/code/01-canvas-fullscreen-2.rs"
```

**Result after `01-canvas`:** The window is white. In the browser console, evaluate `document.getElementById("status").textContent`. It should say “Rust is running. The canvas is ready.” Rust has started; GPU drawing comes next.

---

<a id="checkpoint-01a-gpu"></a>

### Step 2 of 2 · Paint the first GPU frame (`01a-gpu`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/lib.rs, src/renderer.…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">device and queue</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">present</text>
</svg>
</div>

Give the GPU one instruction: clear the canvas to white. The picture will look like lesson 01, but its pixels now come from a real GPU frame.

An **adapter** chooses a GPU compatible with our canvas surface. A **device** creates resources; its **queue** receives recorded work. A **surface** supplies the image that can appear in the canvas. `.await` waits for adapter/device requests while leaving the browser free.

> **Check:** *Why does the render pass end before encoder.finish()?* — The pass borrows the encoder. Ending its scope releases that borrow so finish can take the encoder and return the recorded commands.

### 1. `src/lib.rs`

Start asynchronous GPU setup without blocking the browser. A setup error goes to the developer console.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/01a-gpu-01.rs"
```

### 2. `src/renderer.rs`

Own the device and queue; record and submit one white clear.

Create the file and type:

```rust
--8<-- "journey/code/01a-gpu-02.rs"
```

### 3. `src/browser.rs`

Connect the canvas to an adapter, configure its image and present the clear.

Create the file and type:

```rust
--8<-- "journey/code/01a-gpu-03.rs"
```

**Result after `01a-gpu`:** The canvas stays white. Change `wgpu::Color::WHITE` to `wgpu::Color::BLACK`, save, and check that it turns black. Restore WHITE. These pixels now come from the GPU.

---

## 3. Build, Verify & Save

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 1.1
npm --prefix ../session_tests run course -- save 1.1
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../recovery.md).

Continue with [1.2 · Browser presentation](02-clear.md).
