const s={title:"15 · Publication and streamed reads",html:`<h1 id="15-publication-and-streamed-reads">15 · Publication and streamed reads<a class="anchor" href="#/course/15-publication#15-publication-and-streamed-reads" aria-label="Link to this section">#</a></h1>
<p>The local scene stays visible while streamed cloud metadata loads in one bounded request.</p>
<p><img src="/session/docs/course/docs/illustrations/metadata-window.svg" alt="The file is small fields between huge arrays; the window fetches the small fields once and skips the arrays by length." loading="lazy" decoding="async"></p>
<h2 id="step-1-srcappstreamrs">Step 1 · src/app/stream.rs<a class="anchor" href="#/course/15-publication#step-1-srcappstreamrs" aria-label="Link to this section">#</a></h2>
<p>Streaming reads bounded chunks and keeps stable source addresses.</p>
<p><code>lessons/15/src/app/stream.rs</code> · edit · type this</p>
<p>Added after the <code>}</code> line of <code>lessons/14/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Each range request costs a network round trip, so read 64 KiB ahead and answer later small reads from memory.</span>
#[cfg(any(target_arch = &quot;<span class="s">wasm32</span>&quot;, test))]
#[derive(Default)]
<span class="k">struct</span> MetadataWindow {
    at: u64, <span class="c">// file position of \`bytes[0]\`</span>
    bytes: Vec&lt;u8&gt;,
}

#[cfg(any(target_arch = &quot;<span class="s">wasm32</span>&quot;, test))]
<span class="k">impl</span> MetadataWindow {
    <span class="c">/// The cached bytes at \`at\`, if all present.</span>
    <span class="k">fn</span> slice(&amp;<span class="k">self</span>, at: u64, length: u64) -&gt; Option&lt;&amp;[u8]&gt; {
        <span class="k">let</span> start = usize::try_from(at.checked_sub(<span class="k">self</span>.at)?).ok()?;
        <span class="k">let</span> length = usize::try_from(length).ok()?;
        <span class="k">self</span>.bytes.get(start..start.checked_add(length)?)
    }

    <span class="c">/// How much to read at once: at least \`length\`, up to 64 KiB.</span>
    <span class="k">fn</span> read_length(at: u64, length: u64, end: u64) -&gt; Option&lt;u64&gt; {
        <span class="k">if</span> !bounded_range(at, length) {
            <span class="k">return</span> None;
        }

        body_end(at, length, end)?;
        Some(length.max(<span class="s">64</span> * <span class="s">1024</span>).min(end - at))
    }
}

<span class="c">/// A packed \`int32\` (varint) array in full.</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The reads: find the arrays, then fetch slices by range.</span></code></pre></div>
<p><code>lessons/15/src/app/stream.rs</code> · edit · type this</p>
<p>Added after the <code>const MAX_TABLE_BYTES: u64 = 128 * 1024 * 1024;</code> line in <code>mod web</code> of <code>lessons/14/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">impl</span> MetadataWindow {
        <span class="c">/// The bytes at \`at\`, reading more when not cached.</span>
        <span class="k">async</span> <span class="k">fn</span> read(
            &amp;<span class="k">mut</span> <span class="k">self</span>,
            url: &amp;str,
            at: u64,
            length: u64,
            fields: &amp;CloudFields,
        ) -&gt; Option&lt;&amp;[u8]&gt; {
            body_end(at, length, fields.end)?;

            <span class="k">if</span> <span class="k">self</span>.slice(at, length).is_none() {
                <span class="k">let</span> read_length = <span class="k">Self</span>::read_length(at, length, fields.end)?;
                <span class="k">self</span>.bytes = source_range(url, at, read_length, &amp;fields.revision).<span class="k">await</span>?;
                <span class="k">self</span>.at = at;
            }

            <span class="k">self</span>.slice(at, length)
        }
    }

    <span class="c">/// Read one range of the file.</span></code></pre></div>
<p><code>lessons/15/src/app/stream.rs</code> · edit · type this</p>
<p>Replaces the 4 lines from \`\` of <code>lessons/14/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> window = MetadataWindow::default();

        <span class="k">while</span> at &lt; fields.end {
            <span class="k">let</span> header = window
                .read(url, at, <span class="s">64</span>.min(fields.end - at), fields)
                .<span class="k">await</span>?;
            <span class="k">let</span> (tag, used) = varint(header, <span class="s">0</span>)?;</code></pre></div>
<p>Replaces the 6 lines from <code>let skip = skip_scalar(&amp;header, used, wire)?;</code> in <code>fn cloud_lod</code> of <code>lessons/14/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">let</span> skip = skip_scalar(header, used, wire)?;
                at = body_end(at, (used + skip) <span class="k">as</span> u64, fields.end)?;
                <span class="k">continue</span>;
            }

            <span class="k">let</span> (length, extra) = varint(header, used)?;</code></pre></div>
<p>Replaces the 3 lines from <code>let raw = source_range(url, body, length, &amp;fi…</code> in <code>fn cloud_lod</code> of <code>lessons/14/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">let</span> raw = window.read(url, body, length, fields).<span class="k">await</span>?;

                <span class="k">if</span> !lod.set_field(field, raw) {</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/15/src/app/stream.rs</code> · edit · copy the file</p>
<p>Added after the <code>use super::*;</code> line in <code>mod tests</code> of <code>lessons/14/src/app/stream.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The window serves nearby reads from cache.</span>
    #[test]
    <span class="k">fn</span> metadata_window_checks_ranges_without_copying_or_following_large_skips() {
        <span class="k">let</span> window = MetadataWindow {
            at: <span class="s">100</span>,
            bytes: vec![<span class="s">1</span>, <span class="s">2</span>, <span class="s">3</span>, <span class="s">4</span>],
        };
        assert_eq!(window.slice(<span class="s">101</span>, <span class="s">2</span>), Some(&amp;[<span class="s">2</span>, <span class="s">3</span>][..]));
        assert_eq!(window.slice(<span class="s">104</span>, <span class="s">0</span>), Some(&amp;[][..]));
        assert!(window.slice(<span class="s">99</span>, <span class="s">1</span>).is_none());
        assert!(window.slice(<span class="s">103</span>, <span class="s">2</span>).is_none());
        assert!(window.slice(u64::MAX, <span class="s">2</span>).is_none());
        assert_eq!(
            MetadataWindow::read_length(<span class="s">100</span>, <span class="s">64</span>, <span class="s">1_000_000_000</span>),
            Some(<span class="s">65_536</span>)
        );
        assert_eq!(MetadataWindow::read_length(<span class="s">100</span>, <span class="s">64</span>, <span class="s">170</span>), Some(<span class="s">70</span>));
        assert_eq!(
            MetadataWindow::read_length(<span class="s">100</span>, <span class="s">70_000</span>, <span class="s">1_000_000</span>),
            Some(<span class="s">70_000</span>)
        );
        assert_eq!(MetadataWindow::read_length(<span class="s">100</span>, <span class="s">64</span>, <span class="s">150</span>), None);
        assert_eq!(
            MetadataWindow::read_length(<span class="s">0</span>, <span class="s">64</span> * <span class="s">1024</span> * <span class="s">1024</span> + <span class="s">1</span>, u64::MAX),
            None
        );
        assert_eq!(MetadataWindow::read_length(u64::MAX - <span class="s">1</span>, <span class="s">2</span>, u64::MAX), None);
    }

    <span class="c">/// A two-node table.</span></code></pre></div>
<p>Copy each file from the lesson folder to the path shown.</p>
<p>Copy from <code>lessons/15/</code> (tooling this checkpoint needs but the course does not teach):</p>
<ul>
<li><code>lessons/15/tests/publication.py</code></li>
<li><code>lessons/15/tests/streamed-controls.cjs</code></li>
</ul>
<h2 id="check">Check<a class="anchor" href="#/course/15-publication#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/15/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: The local scene stays visible while streamed cloud metadata loads in one bounded request; status: <strong>the status clears when loading finishes</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/15.png" alt="Checkpoint 15: the local scene is unchanged; the difference is in the network panel of a streamed cloud, where the header reads collapse into one window request." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>A source query misses points: display-prefix indices replace original source IDs.</li>
<li>Range loading downloads the whole file: the server does not honour the byte range.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/15-publication#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/15/src/app/
├── walk/
│   ├── bounds.rs
│   ├── brep.rs
│   ├── brep_edges.rs
│   ├── brep_orient.rs
│   ├── cloud.rs
│   ├── curves.rs
│   ├── encode.rs
│   ├── frames.rs
│   ├── mesh.rs
│   ├── mesh_ink.rs
│   ├── mesh_topology.rs
│   ├── mod.rs
│   └── points.rs
├── cloud_query.rs
├── decode.rs
├── feedback.rs
├── fetch.rs
├── input.rs
├── inspection.rs
├── knobs.rs
├── live.rs
├── loader.rs
├── manifest.rs
├── mod.rs
├── route.rs
├── scene.rs
├── selection.rs
├── stream.rs  ~
├── touch.rs
└── validate.rs</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/15/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/15-publication#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/16-accounting">16 · Resource accounting</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/15-publication#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>The scene looks the same; the network panel shows one window request per streamed cloud.</p>
<p><a href="/session/docs/course/docs/screenshots/15.png"><img src="/session/docs/course/docs/screenshots/15.png" alt="Full viewer result for 15 publication" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcappstreamrs",text:"Step 1 · src/app/stream.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
