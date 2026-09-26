const s={title:"03 · Object rows and identity",html:`<h1 id="03-object-rows-and-identity">03 · Object rows and identity<a class="anchor" href="#/course/03-identity#03-object-rows-and-identity" aria-label="Link to this section">#</a></h1>
<p>The same triangle drawn twice, each with its own placement and colour.</p>
<p><img src="/session/docs/course/docs/illustrations/vertex-layout.svg" alt="The 96-byte Instance layout and the vec3 alignment trap." loading="lazy" decoding="async"></p>
<p><code>vertex_index</code> walks the corners, <code>instance_index</code> walks the rows.</p>
<p><img src="/session/docs/course/docs/illustrations/instancing.svg" alt="A draw call carries two ranges: vertex_index walks the three corners, instance_index walks the object rows, and every invocation reads only the row its instance_index names - so a hundred objects are one call and one buffer." loading="lazy" decoding="async"></p>
<h2 id="step-1-srcenginegpuinstancers">Step 1 · <code>src/engine/gpu/instance.rs</code><a class="anchor" href="#/course/03-identity#step-1-srcenginegpuinstancers" aria-label="Link to this section">#</a></h2>
<p>New file: one 96-byte row per object, matrix, colour, flags.</p>
<p><code>lessons/03/src/engine/gpu/instance.rs</code> · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> session_rust::Xform;

<span class="c">/// One row per object, read by the shader, so one pipeline draws many objects; the GPU reads it by offset, so the layout must match WGSL exactly.</span>
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
<span class="k">pub</span> <span class="k">struct</span> Instance {
    <span class="k">pub</span> model: [f32; <span class="s">16</span>], <span class="c">// rotation and scale; translation is stored separately</span>
    <span class="k">pub</span> color: [f32; <span class="s">4</span>], <span class="c">// rgba tint</span>
    <span class="k">pub</span> flags: u32, <span class="c">// FLAG_* bits below</span>
    <span class="k">pub</span> _pad0: f32, <span class="c">// Padding exists only to satisfy GPU alignment rules; without it the shader reads the wrong fields.</span>
    <span class="k">pub</span> spacing: f32, <span class="c">// vertex spacing, world units; 0 = unknown</span>
    <span class="k">pub</span> _pad: u32, <span class="c">// padding</span>
}

<span class="k">const</span> _: () = assert!(std::mem::size_of::&lt;Instance&gt;() == <span class="s">96</span>);

<span class="k">impl</span> Instance {
    <span class="c">/// Selected: drawn tinted.</span>
    <span class="k">pub</span> <span class="k">const</span> FLAG_SELECTED: u32 = <span class="s">1</span> &lt;&lt; <span class="s">0</span>;

    <span class="c">/// Hidden: skipped by every draw.</span>
    <span class="k">pub</span> <span class="k">const</span> FLAG_HIDDEN: u32 = <span class="s">1</span> &lt;&lt; <span class="s">1</span>;

    <span class="c">/// Camera is inside the object: no back-face culling.</span>
    <span class="k">pub</span> <span class="k">const</span> FLAG_INSIDE: u32 = <span class="s">1</span> &lt;&lt; <span class="s">2</span>;

    <span class="c">/// Sheet fill: flat color, no edges.</span>
    <span class="k">pub</span> <span class="k">const</span> FLAG_PRINT: u32 = <span class="s">1</span> &lt;&lt; <span class="s">3</span>;

    <span class="c">/// Open mesh: no back-face culling.</span>
    <span class="k">pub</span> <span class="k">const</span> FLAG_OPEN: u32 = <span class="s">1</span> &lt;&lt; <span class="s">4</span>;

    <span class="c">/// Part of a drawing sheet.</span>
    <span class="k">pub</span> <span class="k">const</span> FLAG_SHEET: u32 = <span class="s">1</span> &lt;&lt; <span class="s">5</span>;

    <span class="c">/// Sampled surface: vertices are samples, not corners.</span>
    <span class="k">pub</span> <span class="k">const</span> FLAG_SMOOTH: u32 = <span class="s">1</span> &lt;&lt; <span class="s">6</span>;

    <span class="c">/// Single face: stays shaded in x-ray.</span>
    <span class="k">pub</span> <span class="k">const</span> FLAG_SINGLE: u32 = <span class="s">1</span> &lt;&lt; <span class="s">7</span>;

    <span class="c">/// The one row an empty scene binds: identity, grey, no flags.</span>
    <span class="k">pub</span> <span class="k">fn</span> placeholder() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            model: Xform::identity().to_f32(),
            color: [<span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>, <span class="s">1</span>.<span class="s">0</span>],
            flags: <span class="s">0</span>,
            _pad0: <span class="s">0</span>.<span class="s">0</span>,
            spacing: <span class="s">0</span>.<span class="s">0</span>,
            _pad: <span class="s">0</span>,
        }
    }
}</code></pre></div>
<h2 id="step-2-srcenginegpumodrs-and-srcenginemodrs">Step 2 · <code>src/engine/gpu/mod.rs</code> and <code>src/engine/mod.rs</code><a class="anchor" href="#/course/03-identity#step-2-srcenginegpumodrs-and-srcenginemodrs" aria-label="Link to this section">#</a></h2>
<p>Two one-line files that put the new folder into the build.</p>
<p><code>lessons/03/src/engine/gpu/mod.rs</code> · 1 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> instance;</code></pre></div>
<p><code>lessons/03/src/engine/mod.rs</code> · 1 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> gpu;</code></pre></div>
<h2 id="step-3-srcsceners">Step 3 · <code>src/scene.rs</code><a class="anchor" href="#/course/03-identity#step-3-srcsceners" aria-label="Link to this section">#</a></h2>
<p>New file: two objects that share one triangle, each with its own row: placement and colour.</p>
<p><code>lessons/03/src/scene.rs</code> · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::engine::gpu::instance::Instance;

<span class="c">/// guid = a unique name the object keeps for life; revision counts its edits, so the GPU knows which version it holds.</span>
<span class="k">pub</span> <span class="k">struct</span> SourceObject {
    <span class="k">pub</span> guid: &amp;'static str,
    <span class="k">pub</span> revision: u64,
    <span class="k">pub</span> row: Instance,
}

<span class="c">/// One triangle, two rows: each row has its own placement and colour.</span>
<span class="k">pub</span> <span class="k">fn</span> objects() -&gt; [SourceObject; <span class="s">2</span>] {
    <span class="k">let</span> <span class="k">mut</span> left = Instance::placeholder();
    left.model[<span class="s">12</span>] = -<span class="s">0</span>.<span class="s">8</span>; <span class="c">// column-major: entries 12, 13, 14 are the x, y, z move</span>
    left.color = [<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">35</span>, <span class="s">0</span>.<span class="s">2</span>, <span class="s">1</span>.<span class="s">0</span>];
    <span class="k">let</span> <span class="k">mut</span> right = Instance::placeholder();
    right.model[<span class="s">12</span>] = <span class="s">0</span>.<span class="s">8</span>;
    right.color = [<span class="s">0</span>.<span class="s">2</span>, <span class="s">0</span>.<span class="s">7</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>];
    [
        SourceObject {
            guid: &quot;<span class="s">triangle-left</span>&quot;,
            revision: <span class="s">1</span>,
            row: left,
        },
        SourceObject {
            guid: &quot;<span class="s">triangle-right</span>&quot;,
            revision: <span class="s">1</span>,
            row: right,
        },
    ]
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/03/</code>.</p>
<h2 id="step-4-srcshadersfirstwgsl">Step 4 · <code>src/shaders/first.wgsl</code><a class="anchor" href="#/course/03-identity#step-4-srcshadersfirstwgsl" aria-label="Link to this section">#</a></h2>
<p>Replace the shader: it reads its row from binding 1 and applies the model matrix.</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>Rust \`Instance\`            offset   WGSL \`struct Instance\`
model: [f32; 16]              0     model: mat4x4&lt;f32&gt;
color: [f32; 4]              64     color: vec4&lt;f32&gt;
flags: u32                   80     flags: u32
_pad0: f32                   84     thickness: f32
spacing: f32                 88     spacing: f32
_pad: u32                    92     pad: u32
size                         96     array stride</code></pre></div>
<p><code>lessons/03/src/shaders/first.wgsl</code> · edit · type this</p>
<p>Replaces the <code>fn vs_main</code> lines in <code>lessons/02/src/shaders/first.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Must match the Rust Instance byte for byte: 96 bytes, fields in the same order.</span>
<span class="k">struct</span> Instance {
    model:<span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;,
    color:<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
    flags:<span class="k">u32</span>,
    thickness:<span class="k">f32</span>,
    spacing:<span class="k">f32</span>,
    pad:<span class="k">u32</span>,
}

@group(<span class="s">0</span>) @binding(<span class="s">1</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; instances:<span class="k">array</span>&lt;Instance&gt;;<span class="c"> // the length comes from the buffer size</span>

<span class="c">// Vertex output, fragment input</span>
<span class="k">struct</span> VertexOut {
    @builtin(position) position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // the one output the GPU requires</span>
    @location(<span class="s">0</span>) color: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,
}

<span class="c">// Runs once per corner of each instance: index walks the corners, row walks the object rows.</span>
@vertex
<span class="k">fn</span> vs_main(@builtin(vertex_index) index: <span class="k">u32</span>, @builtin(instance_index) row:<span class="k">u32</span>) -&gt; VertexOut {
    <span class="k">let</span> points = <span class="k">array</span>&lt;<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, <span class="s">3</span>&gt;(
        <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(-<span class="s">0.7</span>, -<span class="s">0.55</span>, <span class="s">0.0</span>),
        <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.7</span>, -<span class="s">0.55</span>, <span class="s">0.0</span>),
        <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, <span class="s">0.65</span>, <span class="s">0.0</span>)
    );

    <span class="k">let</span> colors = <span class="k">array</span>&lt;<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, <span class="s">3</span>&gt;(
        <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.95</span>, <span class="s">0.25</span>, <span class="s">0.2</span>),
        <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.2</span>, <span class="s">0.8</span>, <span class="s">0.45</span>),
        <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.3</span>, <span class="s">0.5</span>, <span class="s">1.0</span>)
    );
    <span class="k">var</span> output: VertexOut;
    output.position = mvp * instances[row].model * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(points[index]*<span class="s">0.65</span>, <span class="s">1.0</span>);<span class="c"> // model places the object, mvp takes it to clip space</span>
    output.color = colors[index];
    output.color = instances[row].color.rgb;<span class="c"> // replaces the corner colours: one flat colour per object</span></code></pre></div>
<h2 id="step-5-srclibrs">Step 5 · <code>src/lib.rs</code><a class="anchor" href="#/course/03-identity#step-5-srclibrs" aria-label="Link to this section">#</a></h2>
<p>Six edits: new modules, the rows uploaded to a storage buffer, one draw per row.</p>
<p><code>lessons/03/src/lib.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> camera;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> engine;
<span class="k">pub</span> <span class="k">mod</span> scene;</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    camera: camera::Camera,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    objects: [scene::SourceObject; <span class="s">2</span>],</code></pre></div>
<p>Replaces the lines from <code>entries: &amp;[wgpu::BindGroupLayoutEntry {</code> to <code>}],</code> in <code>lessons/02/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            entries: &amp;[
                wgpu::BindGroupLayoutEntry {
                    binding: <span class="s">0</span>, <span class="c">// @binding(0) in first.wgsl</span>
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: <span class="s">false</span>, <span class="c">// true would let one buffer hold several cameras, chosen per draw</span>
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: <span class="s">1</span>,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: <span class="s">true</span> }, <span class="c">// storage = a large array the shader indexes</span>
                        has_dynamic_offset: <span class="s">false</span>,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        <span class="k">let</span> records = [scene::objects()[<span class="s">0</span>].row, scene::objects()[<span class="s">1</span>].row]; <span class="c">// 2 rows x 96 bytes</span>
        <span class="k">let</span> rows = device.create_buffer_init(&amp;wgpu::util::BufferInitDescriptor {
            label: Some(&quot;<span class="s">source instances</span>&quot;),
            contents: bytemuck::cast_slice(&amp;records),
            usage: wgpu::BufferUsages::STORAGE,
        });
        <span class="k">let</span> group = device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
            label: Some(&quot;<span class="s">camera</span>&quot;),
            layout: &amp;layout,
            entries: &amp;[
                wgpu::BindGroupEntry {
                    binding: <span class="s">0</span>, <span class="c">// fills the layout entry with the same number</span>
                    resource: uniform.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: <span class="s">1</span>,
                    resource: rows.as_entire_binding(),
                },
            ],</code></pre></div>
<p>Added after the camera setup, before <code>Ok(Self {</code>, in <code>lessons/03/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> objects = scene::objects();</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            camera,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            objects,</code></pre></div>
<p>Replaces the lines from <code>pass.draw(0..3, 0..1);</code> to <code>Ok(serde_json::json!({&quot;stage&quot;: 2, &quot;objects&quot;: 1, &quot;width&quot;:w…</code> in <code>lessons/02/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.objects.len() <span class="k">as</span> u32 {
                pass.draw(<span class="s">0</span>..<span class="s">3</span>, row..row + <span class="s">1</span>); <span class="c">// instances row..row + 1: the shader's instance_index is this row</span>
            }
        }
        <span class="c">// The GPU starts working only now.</span>
        <span class="k">self</span>.queue.submit([encoder.finish()]);
        <span class="c">// The canvas shows the new frame at the next display refresh.</span>
        output.present();
        Ok(serde_json::json!({&quot;<span class="s">stage</span>&quot;: <span class="s">3</span>, &quot;<span class="s">objects</span>&quot;: <span class="k">self</span>.objects.len(), &quot;<span class="s">width</span>&quot;:width,&quot;<span class="s">height</span>&quot;:height,&quot;<span class="s">scale</span>&quot;:scale,&quot;<span class="s">drawn</span>&quot;:<span class="s">true</span>}).to_string())</code></pre></div>
<h2 id="step-6-indexhtml">Step 6 · <code>index.html</code><a class="anchor" href="#/course/03-identity#step-6-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Three edits: the checkpoint number in the title, the starting status and the status text.</p>
<p><code>lessons/03/index.html</code> · edit · copy the file</p>
<p>Replaces the line <code>&lt;title&gt;Session checkpoint 02&lt;/title&gt;</code> in <code>lessons/02/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;title&gt;Session checkpoint 03&lt;/title&gt;</code></pre></div>
<p>Replaces the line <code>&lt;output id=&quot;status&quot;&gt;Starting checkpoint 02&lt;/output&gt;</code> in <code>lessons/02/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 03&lt;/output&gt;</code></pre></div>
<p>Replaces the line <code>document.getElementById(&#39;status&#39;).textContent = &#39;Checkpoi…</code> in <code>lessons/02/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        document.getElementById('status').textContent = 'Checkpoint 03 · ' + state.objects + ' objects · ' + state.width + '×' + state.height;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/03-identity#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/03/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: two triangles, orange left, blue right, status <strong>Checkpoint 03 · 2 objects</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/03.png" alt="Checkpoint 03: one triangle geometry drawn twice through two object rows, each with its own placement and tint." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>The first triangle is right and the second is garbage: the row stride is wrong. Compare <code>Instance</code> with the table in step 4.</li>
<li>Only one triangle: the draw loop still says <code>draw(0..3, 0..1)</code>, so <code>instance_index</code> never reaches 1.</li>
<li>A validation error naming binding 1: the layout entry, the bind group entry and the <code>@binding(1)</code> line do not agree.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/03-identity#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/03/src/
├── engine/
│   ├── gpu/
│   │   ├── instance.rs  +
│   │   └── mod.rs  +
│   └── mod.rs  +
├── shaders/
│   └── first.wgsl  ~
├── camera.rs
├── lib.rs  ~
└── scene.rs  +</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/03/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/03-identity#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/04a-meshes">04a · Meshes on the GPU</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/03-identity#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 03: one triangle geometry drawn twice through two object rows, each with its own placement and tint.</p>
<p><a href="/session/docs/course/docs/screenshots/03.png"><img src="/session/docs/course/docs/screenshots/03.png" alt="Full viewer result for 03 identity" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcenginegpuinstancers",text:"Step 1 · src/engine/gpu/instance.rs"},{level:2,id:"step-2-srcenginegpumodrs-and-srcenginemodrs",text:"Step 2 · src/engine/gpu/mod.rs and src/engine/mod.rs"},{level:2,id:"step-3-srcsceners",text:"Step 3 · src/scene.rs"},{level:2,id:"step-4-srcshadersfirstwgsl",text:"Step 4 · src/shaders/first.wgsl"},{level:2,id:"step-5-srclibrs",text:"Step 5 · src/lib.rs"},{level:2,id:"step-6-indexhtml",text:"Step 6 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
