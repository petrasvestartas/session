const s={title:"00 · Empty project to a WASM message",html:`<h1 id="00-empty-project-to-a-wasm-message">00 · Empty project to a WASM message<a class="anchor" href="#/course/00-environment#00-empty-project-to-a-wasm-message" aria-label="Link to this section">#</a></h1>
<p>Five files and one command. At the end, the browser shows a line of text written by Rust. No GPU yet.</p>
<p><img src="/session/docs/course/docs/illustrations/toolchain.svg" alt="Four tools and four artefacts: cargo produces a .wasm a browser cannot load on its own, wasm-bindgen writes the JavaScript that can, Trunk assembles the page around it, and the browser runs start()." loading="lazy" decoding="async"></p>
<p><code>cargo</code> compiles to <code>.wasm</code>, Trunk wraps and serves it, the browser runs <code>start()</code>.</p>
<h2 id="make-the-folder">Make the folder<a class="anchor" href="#/course/00-environment#make-the-folder" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>cd <span class="s">~/session</span>            <span class="c"># the folder that holds session_rust</span>
mkdir <span class="s">-p</span> <span class="s">session_view/src</span> <span class="s">session_view/.cargo</span>
cd <span class="s">session_view</span></code></pre></div>
<p>Every command below runs here.</p>
<h2 id="step-1-cargotoml">Step 1 · <code>Cargo.toml</code><a class="anchor" href="#/course/00-environment#step-1-cargotoml" aria-label="Link to this section">#</a></h2>
<p>New file: the crate&#39;s name and every dependency the course will use, listed once.</p>
<p><code>lessons/00/Cargo.toml</code> · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>[workspace]

[package]
name = &quot;<span class="s">session_viewer</span>&quot;
version = &quot;<span class="s">0.1.0</span>&quot;
edition = &quot;<span class="s">2024</span>&quot;

[lib]
crate-type = [&quot;<span class="s">cdylib</span>&quot;, &quot;<span class="s">rlib</span>&quot;]  <span class="c"># cdylib = the .wasm the browser loads; rlib lets tests and examples link the same code</span>

[dependencies]
session_rust = { path = &quot;<span class="s">../../../../session_rust</span>&quot; }
anyhow = &quot;<span class="s">1.0</span>&quot;
serde = { version = &quot;<span class="s">1.0</span>&quot;, features = [&quot;<span class="s">derive</span>&quot;] }
serde_yaml_ng = &quot;<span class="s">0.10</span>&quot;
toml = &quot;<span class="s">=0.8.23</span>&quot;
serde_json = { version = &quot;<span class="s">1.0</span>&quot;, features = [&quot;<span class="s">float_roundtrip</span>&quot;] }
winit = &quot;<span class="s">0.30</span>&quot;
wgpu = &quot;<span class="s">29.0</span>&quot;
glyphon = &quot;<span class="s">=0.11.0</span>&quot; <span class="c"># text renderer; \`=\` pins the one release built for wgpu 29</span>
log = &quot;<span class="s">0.4</span>&quot;
console_error_panic_hook = &quot;<span class="s">0.1.6</span>&quot;
console_log = &quot;<span class="s">1.0</span>&quot;
wasm-bindgen = &quot;<span class="s">0.2</span>&quot;
wasm-bindgen-futures = &quot;<span class="s">0.4</span>&quot;
web-sys = { version = &quot;<span class="s">0.3</span>&quot;, features = [
    &quot;<span class="s">Document</span>&quot;,
    &quot;<span class="s">Window</span>&quot;,
    &quot;<span class="s">Element</span>&quot;,
    &quot;<span class="s">HtmlCanvasElement</span>&quot;,
    &quot;<span class="s">EventTarget</span>&quot;,
    &quot;<span class="s">Event</span>&quot;,
    &quot;<span class="s">Location</span>&quot;,
    &quot;<span class="s">Performance</span>&quot;,
    &quot;<span class="s">EventSource</span>&quot;,
    &quot;<span class="s">MessageEvent</span>&quot;,
    &quot;<span class="s">Request</span>&quot;,
    &quot;<span class="s">RequestCache</span>&quot;,
    &quot;<span class="s">RequestInit</span>&quot;,
    &quot;<span class="s">RequestMode</span>&quot;,
    &quot;<span class="s">Response</span>&quot;,
    &quot;<span class="s">Headers</span>&quot;,
    &quot;<span class="s">AbortController</span>&quot;,
    &quot;<span class="s">AbortSignal</span>&quot;,
    ] }
getrandom = { version = &quot;<span class="s">0.2</span>&quot;, features = [&quot;<span class="s">js</span>&quot;] }

js-sys = &quot;<span class="s">0.3</span>&quot;
prost = &quot;<span class="s">0.14</span>&quot;
bytemuck = { version = &quot;<span class="s">1</span>&quot;, features = [&quot;<span class="s">derive</span>&quot;] }

[profile.dev.package.&quot;*&quot;]
opt-level = <span class="s">3</span> <span class="c"># dependencies optimised even in debug builds, so a large scene still parses fast</span>

[profile.release]
strip = <span class="s">true</span>

[package.metadata.wasm-pack.profile.release]
wasm-opt = <span class="s">false</span>


<span class="c"># Native builds only (tests, examples). A TOML table runs to the next header, so keep this one</span>
<span class="c"># near the end: a wasm dependency written below it would silently leave the wasm build.</span>
[target.'cfg(not(target_arch = &quot;wasm32&quot;))'.dependencies]
pollster = &quot;<span class="s">0.4</span>&quot;





[dev-dependencies]
naga = { version = &quot;<span class="s">=29.0.4</span>&quot;, features = [&quot;<span class="s">wgsl-in</span>&quot;] }</code></pre></div>
<h2 id="step-2-cargoconfigtoml">Step 2 · <code>.cargo/config.toml</code><a class="anchor" href="#/course/00-environment#step-2-cargoconfigtoml" aria-label="Link to this section">#</a></h2>
<p>New file: every cargo command builds for the browser, and <code>cargo xtest</code> runs tests on this machine.</p>
<p><code>lessons/00/.cargo/config.toml</code> · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c"># wasm32 = WebAssembly, the code a browser runs; every cargo command builds it unless told otherwise.</span>
[build]
target = &quot;<span class="s">wasm32-unknown-unknown</span>&quot;
target-dir = &quot;<span class="s">../../../target/lessons</span>&quot; <span class="c"># one build folder for all lessons, so dependencies compile once</span>

[target.wasm32-unknown-unknown]
rustflags = [&quot;<span class="s">-C</span>&quot;, &quot;<span class="s">link-arg=--max-memory=4294967296</span>&quot;] <span class="c"># memory may grow to 4 GB, all a 32-bit address can reach</span>

<span class="c"># wasm32 has no test runner and no files, so \`cargo xtest\` runs tests on this machine instead.</span>
[alias]
xtest = &quot;<span class="s">test --target x86_64-unknown-linux-gnu</span>&quot;</code></pre></div>
<h2 id="step-3-trunktoml">Step 3 · <code>Trunk.toml</code><a class="anchor" href="#/course/00-environment#step-3-trunktoml" aria-label="Link to this section">#</a></h2>
<p>New file: how Trunk builds the page, what it watches, and where it serves it.</p>
<p><code>lessons/00/Trunk.toml</code> · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>[build]
dist = &quot;<span class="s">../../../target/lessons/00/dist</span>&quot;
release = <span class="s">true</span>
no_sri = <span class="s">true</span> <span class="c"># SRI = a hash on each script tag that the browser checks; not used here</span>
public_url = &quot;<span class="s">./</span>&quot; <span class="c"># links relative to the page, so the site works from any folder</span>

[watch]
watch = [&quot;<span class="s">src</span>&quot;, &quot;<span class="s">Cargo.toml</span>&quot;, &quot;<span class="s">index.html</span>&quot;, &quot;<span class="s">assets</span>&quot;, &quot;<span class="s">../../../../session_rust/src</span>&quot;] <span class="c"># trunk serve rebuilds when these change</span>

[serve]
port = <span class="s">8770</span>
addresses = [&quot;<span class="s">127.0.0.1</span>&quot;]</code></pre></div>
<h2 id="step-4-cargolock">Step 4 · <code>Cargo.lock</code><a class="anchor" href="#/course/00-environment#step-4-cargolock" aria-label="Link to this section">#</a></h2>
<p>Cargo&#39;s exact dependency versions; copy it, never edit it.</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>cargo <span class="s">generate-lockfile</span></code></pre></div>
<h2 id="step-5-indexhtml">Step 5 · <code>index.html</code><a class="anchor" href="#/course/00-environment#step-5-indexhtml" aria-label="Link to this section">#</a></h2>
<p>New file: the page Trunk fills with the compiled module, and one line for Rust to write into.</p>
<p><code>lessons/00/index.html</code> · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>&lt;!doctype html&gt;
&lt;html&gt;
  &lt;head&gt;
    &lt;meta charset=&quot;<span class="s">utf-8</span>&quot;&gt;
    &lt;title&gt;Session checkpoint 00&lt;/title&gt;
    <span class="c">&lt;!-- Trunk compiles the crate and puts the script that loads the .wasm here; wasm-opt=&quot;0&quot; skips the slow size pass. --&gt;</span>
    &lt;link data-trunk rel=&quot;<span class="s">rust</span>&quot; data-wasm-opt=&quot;<span class="s">0</span>&quot;&gt;
  &lt;/head&gt;
  &lt;body&gt;
    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Loading WASM&lt;/output&gt;
  &lt;/body&gt;
&lt;/html&gt;</code></pre></div>
<h2 id="step-6-srclibrs">Step 6 · <code>src/lib.rs</code><a class="anchor" href="#/course/00-environment#step-6-srclibrs" aria-label="Link to this section">#</a></h2>
<p>New file: the function the browser runs once the module loads; it writes one line into the page.</p>
<p><code>lessons/00/src/lib.rs</code> · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> wasm_bindgen::prelude::*;

<span class="c">// The browser calls this once, as soon as the module has loaded.</span>
#[wasm_bindgen(start)]
<span class="k">pub</span> <span class="k">fn</span> start() {
    <span class="c">// A panic would show only &quot;unreachable&quot; in the console; this prints its message instead.</span>
    console_error_panic_hook::set_once();
    <span class="k">let</span> document = web_sys::window()
        .expect(&quot;<span class="s">browser window</span>&quot;) <span class="c">// expect: take the value, or stop with this message if there is none</span>
        .document()
        .expect(&quot;<span class="s">document</span>&quot;);
    <span class="k">let</span> status = document
        .get_element_by_id(&quot;<span class="s">status</span>&quot;)
        .expect(&quot;<span class="s">status element</span>&quot;);
    status.set_text_content(Some(&quot;<span class="s">Checkpoint 00: Rust/WASM ready</span>&quot;));
    status
        .set_attribute(&quot;<span class="s">data-checkpoint</span>&quot;, &quot;<span class="s">00</span>&quot;)
        .expect(&quot;<span class="s">status attribute</span>&quot;);
}</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/00-environment#check" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>cargo <span class="s">check</span> <span class="s">--lib</span>
trunk <span class="s">serve</span> <span class="s">--port</span> <span class="s">8780</span></code></pre></div>
<p>The first check compiles every dependency and takes a few minutes. Open <a href="http://localhost:8780/" target="_blank" rel="noopener">http://localhost:8780/</a>: the page shows <strong>Checkpoint 00: Rust/WASM ready</strong>. Stop with Ctrl+C.</p>
<p><img src="/session/docs/course/docs/screenshots/00.png" alt="Checkpoint 00 in Chrome: the status line written by Rust." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li><em>failed to load manifest for dependency <code>session_rust</code></em>: the folder is not next to <code>session_rust</code>.</li>
<li>An error naming a crate or feature: compare your <code>Cargo.toml</code> with step 1.</li>
<li>The page stays on <em>Loading WASM</em>: open <code>localhost:8780</code>, not the file, and wait for the build.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/00-environment#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/00/session_viewer/
├── .cargo/
│   └── config.toml  +
├── src/
│   └── lib.rs  +
├── Cargo.toml  +
├── Trunk.toml  +
└── index.html  +</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/00/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/00-environment#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/01-first-frame">01 · First WebGPU frame</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/00-environment#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 00 in Chrome: the status line written by Rust.</p>
<p><a href="/session/docs/course/docs/screenshots/00.png"><img src="/session/docs/course/docs/screenshots/00.png" alt="Full viewer result for 00 environment" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"make-the-folder",text:"Make the folder"},{level:2,id:"step-1-cargotoml",text:"Step 1 · Cargo.toml"},{level:2,id:"step-2-cargoconfigtoml",text:"Step 2 · .cargo/config.toml"},{level:2,id:"step-3-trunktoml",text:"Step 3 · Trunk.toml"},{level:2,id:"step-4-cargolock",text:"Step 4 · Cargo.lock"},{level:2,id:"step-5-indexhtml",text:"Step 5 · index.html"},{level:2,id:"step-6-srclibrs",text:"Step 6 · src/lib.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
