const s={title:"10 · Text shaping",html:`<h1 id="10-text-shaping">10 · Text shaping<a class="anchor" href="#/course/10-text-layout#10-text-shaping" aria-label="Link to this section">#</a></h1>
<p>The text specimen page compares five shaped text sizes against browser text.</p>
<p><img src="/session/docs/course/docs/illustrations/text-pipeline.svg" alt="Shape once, place per frame, raster per device scale, then a plate pass and a glyph pass." loading="lazy" decoding="async"></p>
<p>Copy each file from the lesson folder to the path shown.</p>
<p>Copy from <code>lessons/10/</code> (tooling this checkpoint needs but the course does not teach):</p>
<ul>
<li><code>lessons/10/assets/text/NotoSans-Regular.ttf</code> (binary)</li>
<li><code>lessons/10/assets/text/NotoSansSymbols-Regular.ttf</code> (binary)</li>
<li><code>lessons/10/assets/text/NotoSansSymbols2-Regular.ttf</code> (binary)</li>
</ul>
<h2 id="step-1-assetstextofltxt">Step 1 · assets/text/OFL.txt<a class="anchor" href="#/course/10-text-layout#step-1-assetstextofltxt" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/10/assets/text/OFL.txt</code> · 94 lines · copy the file, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>Copyright 2018 The Noto Project Authors (github.com/googlei18n/noto-fonts)

This Font Software is licensed under the SIL Open Font License,
Version 1.1.

This license is copied below, and is also available with a FAQ at:
http://scripts.sil.org/OFL

-----------------------------------------------------------
SIL OPEN FONT LICENSE Version 1.1 - 26 February 2007
-----------------------------------------------------------

PREAMBLE
The goals of the Open Font License (OFL) are to stimulate worldwide
development of collaborative font projects, to support the font
creation efforts of academic and linguistic communities, and to
provide a free and open framework in which fonts may be shared and
improved in partnership with others.

The OFL allows the licensed fonts to be used, studied, modified and
redistributed freely as long as they are not sold by themselves. The
fonts, including any derivative works, can be bundled, embedded,
redistributed and/or sold with any software provided that any reserved
names are not used by derivative works. The fonts and derivatives,
however, cannot be released under any other type of license. The
requirement for fonts to remain under this license does not apply to
any document created using the fonts or their derivatives.

DEFINITIONS
&quot;Font Software&quot; refers to the set of files released by the Copyright
Holder(s) under this license and clearly marked as such. This may
include source files, build scripts and documentation.

&quot;Reserved Font Name&quot; refers to any names specified as such after the
copyright statement(s).

&quot;Original Version&quot; refers to the collection of Font Software
components as distributed by the Copyright Holder(s).

&quot;Modified Version&quot; refers to any derivative made by adding to,
deleting, or substituting -- in part or in whole -- any of the
components of the Original Version, by changing formats or by porting
the Font Software to a new environment.

&quot;Author&quot; refers to any designer, engineer, programmer, technical
writer or other person who contributed to the Font Software.

PERMISSION &amp; CONDITIONS
Permission is hereby granted, free of charge, to any person obtaining
a copy of the Font Software, to use, study, copy, merge, embed,
modify, redistribute, and sell modified and unmodified copies of the
Font Software, subject to the following conditions:

1) Neither the Font Software nor any of its individual components, in
Original or Modified Versions, may be sold by itself.

2) Original or Modified Versions of the Font Software may be bundled,
redistributed and/or sold with any software, provided that each copy
contains the above copyright notice and this license. These can be
included either as stand-alone text files, human-readable headers or
in the appropriate machine-readable metadata fields within text or
binary files as long as those fields can be easily viewed by the user.

3) No Modified Version of the Font Software may use the Reserved Font
Name(s) unless explicit written permission is granted by the
corresponding Copyright Holder. This restriction only applies to the
primary font name as presented to the users.

4) The name(s) of the Copyright Holder(s) or the Author(s) of the Font
Software shall not be used to promote, endorse or advertise any
Modified Version, except to acknowledge the contribution(s) of the
Copyright Holder(s) and the Author(s) or with their explicit written
permission.

5) The Font Software, modified or unmodified, in part or in whole,
must be distributed entirely under this license, and must not be
distributed under any other license. The requirement for fonts to
remain under this license does not apply to any document created using
the Font Software.

TERMINATION
This license becomes null and void if any of the above conditions are
not met.

DISCLAIMER
THE FONT SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO ANY WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT
OF COPYRIGHT, PATENT, TRADEMARK, OR OTHER RIGHT. IN NO EVENT SHALL THE
COPYRIGHT HOLDER BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
INCLUDING ANY GENERAL, SPECIAL, INDIRECT, INCIDENTAL, OR CONSEQUENTIAL
DAMAGES, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF THE USE OR INABILITY TO USE THE FONT SOFTWARE OR FROM
OTHER DEALINGS IN THE FONT SOFTWARE.</code></pre></div>
<h2 id="step-2-assetstextreadmemd">Step 2 · assets/text/README.md<a class="anchor" href="#/course/10-text-layout#step-2-assetstextreadmemd" aria-label="Link to this section">#</a></h2>
<p>Copy the font notes: license, sources, and why glyphon is pinned to 0.11.</p>
<p><code>lessons/10/assets/text/README.md</code> · 32 lines · copy the file, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code># Bundled text fonts

Noto Sans Regular, Noto Sans Symbols Regular and Noto Sans Symbols 2 Regular, under the SIL Open Font License 1.1
(\`<span class="s">OFL.txt</span>\`, next to this file). All three are compiled into the wasm, and Trunk copies the same bytes
so \`<span class="s">text-quality.html</span>\` can compare against the browser. Fonts installed on the machine are never used.

Upstream sources:
- https://github.com/notofonts/noto-fonts/blob/main/hinted/ttf/NotoSans/NotoSans-Regular.ttf
- https://github.com/notofonts/noto-fonts/blob/main/hinted/ttf/NotoSansSymbols2/NotoSansSymbols2-Regular.ttf
- https://github.com/notofonts/noto-fonts/blob/main/hinted/ttf/NotoSansSymbols/NotoSansSymbols-Regular.ttf
- https://github.com/notofonts/noto-fonts/blob/main/LICENSE

\`<span class="s">src/engine/text.rs</span>\` shapes the text and picks the fallback font.
\`<span class="s">src/engine/gpu/text.rs</span>\` draws it with Glyphon **0.11.0**, the release that matches wgpu **29.0.4**;
Glyphon **0.12.0** needs wgpu 30. Its shader is Glyphon's own \`<span class="s">src/shader.wgsl</span>\`:
https://docs.rs/crate/glyphon/0.11.0/source/src/shader.wgsl

Noto Sans covers the Latin, Lithuanian, German and CAD characters in the samples; the two symbol fonts
fill the gaps. Another script needs another licensed font, loaded with \`<span class="s">TextDocument::replace_fonts</span>\`,
which reshapes every label and resets the GPU text. A glyph no font has shows as a .notdef box and is
counted in \`<span class="s">missing_glyphs</span>\`.

Letters in imported PDFs are not text: \`<span class="s">session_rust::pdf</span>\` already turned them into meshes, with no
string or font left. \`<span class="s">text_outline.wgsl</span>\` draws those meshes unlit, in their exact positions, with
coverage as alpha. Old PDFs can mix letters into page fills, so both sheet index runs use that pipeline
instead of guessing where a letter ends. Sheet vectors get 4x MSAA like solids, within the adapter
memory budget; at a forced 1x, or on a canvas past that budget, thin outlines look rougher. The Glyphon
lane is only for real text labels.

\`<span class="s">TextStats</span>\` counts Swash's glyph images on the CPU and their byte capacity, apart from raster keys and
the bytes of drawn glyph instances. Glyphon 0.11 does not report GPU atlas size, upload bytes or raster
time, so those are left out; \`<span class="s">preparation_ms</span>\` is the whole rebuild, not raster or upload alone.</code></pre></div>
<h2 id="step-3-srcengineperformancers">Step 3 · src/engine/performance.rs<a class="anchor" href="#/course/10-text-layout#step-3-srcengineperformancers" aria-label="Link to this section">#</a></h2>
<p>New file: frame time and memory counters, and a flag raised when a drag runs below 25 frames per second.</p>
<p><code>lessons/10/src/engine/performance.rs</code> · 206 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Frame time and memory counters, so a slow frame is blamed on a number, not a guess.</span>

<span class="c">/// Frame timing, plus a detector for drags the GPU cannot keep up with.</span>
<span class="k">pub</span> <span class="k">struct</span> Performance {
    prev_frame: f64, <span class="c">// ms</span>
    last_log: f64, <span class="c">// ms</span>
    frame_ms: f64, <span class="c">// smoothed, see \`frame\`</span>
    <span class="k">pub</span> frames: u64,
    <span class="k">pub</span> draws: u32, <span class="c">// draw calls in the last frame</span>
    <span class="k">pub</span> interacting: bool, <span class="c">// a drag or pinch is in progress</span>
    slow_run: u32, <span class="c">// slow drag frames in a row</span>
    slow: bool,
}

<span class="c">/// 40 ms is 25 frames per second; slower than that, a drag feels sticky.</span>
<span class="k">const</span> SLOW_FRAME_MS: f64 = <span class="s">40</span>.<span class="s">0</span>;

<span class="c">/// 30 slow frames in a row, over a second, means the GPU cannot keep up; one hiccup does not.</span>
<span class="k">const</span> SLOW_FRAMES: u32 = <span class="s">30</span>;

<span class="k">impl</span> Performance {
    <span class="c">/// The first frame is timed from this moment.</span>
    <span class="k">pub</span> <span class="k">fn</span> new() -&gt; <span class="k">Self</span> {
        <span class="k">let</span> t = now_ms();
        <span class="k">Self</span> {
            prev_frame: t,
            last_log: t,
            frame_ms: <span class="s">0</span>.<span class="s">0</span>,
            frames: <span class="s">0</span>,
            draws: <span class="s">0</span>,
            interacting: <span class="s">false</span>,
            slow_run: <span class="s">0</span>,
            slow: <span class="s">false</span>,
        }
    }

    <span class="c">/// True once per slow run: \`take\` hands back the flag and leaves \`false\` in its place.</span>
    <span class="k">pub</span> <span class="k">fn</span> take_slow_interaction(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; bool {
        std::mem::take(&amp;<span class="k">mut</span> <span class="k">self</span>.slow)
    }

    <span class="c">/// Record one frame; with \`perf\` on, log a summary once a second.</span>
    <span class="k">pub</span> <span class="k">fn</span> frame(&amp;<span class="k">mut</span> <span class="k">self</span>, draws: u32, objects: u32, now: f64, perf: bool) {
        <span class="k">let</span> dt = now - <span class="k">self</span>.prev_frame;
        <span class="k">self</span>.prev_frame = now;
        <span class="k">self</span>.frames += <span class="s">1</span>;
        <span class="k">self</span>.draws = draws;
        <span class="c">// a slow drag frame extends the run; any other frame ends it</span>
        <span class="k">self</span>.slow_run = <span class="k">if</span> <span class="k">self</span>.interacting &amp;&amp; dt &gt; SLOW_FRAME_MS {
            <span class="k">self</span>.slow_run + <span class="s">1</span>
        } <span class="k">else</span> {
            <span class="s">0</span>
        };

        <span class="c">// \`==\`, not \`&gt;=\`: fires once per run, not on every later frame</span>
        <span class="k">if</span> <span class="k">self</span>.slow_run == SLOW_FRAMES {
            <span class="k">self</span>.slow = <span class="s">true</span>;
        }

        <span class="c">// each new frame counts 10 %, so one 100 ms spike moves a 16 ms average only to about 24 ms</span>
        <span class="k">self</span>.frame_ms = <span class="k">if</span> <span class="k">self</span>.frame_ms == <span class="s">0</span>.<span class="s">0</span> {
            dt
        } <span class="k">else</span> {
            <span class="k">self</span>.frame_ms * <span class="s">0</span>.<span class="s">9</span> + dt * <span class="s">0</span>.<span class="s">1</span>
        };

        <span class="k">if</span> perf &amp;&amp; now - <span class="k">self</span>.last_log &gt;= <span class="s">1000</span>.<span class="s">0</span> {
            <span class="k">let</span> fps = <span class="k">if</span> <span class="k">self</span>.frame_ms &gt; <span class="s">0</span>.<span class="s">0</span> {
                <span class="s">1000</span>.<span class="s">0</span> / <span class="k">self</span>.frame_ms
            } <span class="k">else</span> {
                <span class="s">0</span>.<span class="s">0</span>
            };
            log::info!(
                &quot;<span class="s">perf: </span>{<span class="s">:.1</span>}<span class="s"> fps | </span>{<span class="s">:.2</span>}<span class="s"> ms | </span>{}<span class="s"> draws | </span>{}<span class="s"> objects | wasm capacity </span>{<span class="s">:.0</span>}<span class="s"> MiB</span>&quot;,
                fps,
                <span class="k">self</span>.frame_ms,
                draws,
                objects,
                heap_mb()
            );
            <span class="k">self</span>.last_log = now;
        }
    }
}

<span class="c">/// Milliseconds since the page loaded.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> now_ms() -&gt; f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

<span class="c">/// Milliseconds since 1970; only differences between two calls matter.</span>
#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">pub</span> <span class="k">fn</span> now_ms() -&gt; f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
        * <span class="s">1000</span>.<span class="s">0</span>
}

<span class="c">/// The wasm heap in MiB: one JavaScript ArrayBuffer that only grows, so this is capacity, not use.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> heap_mb() -&gt; f64 {
    <span class="k">use</span> wasm_bindgen::JsCast;
    <span class="c">// \`dyn_into\` is a checked cast of a JavaScript value; a wrong type gives Err, not a crash</span>
    <span class="k">let</span> Ok(memory) = wasm_bindgen::memory().dyn_into::&lt;js_sys::WebAssembly::Memory&gt;() <span class="k">else</span> {
        <span class="k">return</span> <span class="s">0</span>.<span class="s">0</span>;
    };
    memory
        .buffer()
        .unchecked_into::&lt;js_sys::ArrayBuffer&gt;() <span class="c">// no check needed: wasm memory is always an ArrayBuffer</span>
        .byte_length() <span class="k">as</span> f64
        / <span class="s">1</span>.<span class="s">048576</span>e<span class="s">6</span> <span class="c">// bytes per MiB</span>
}

<span class="c">/// Process resident memory in MiB, Linux.</span>
#[cfg(all(not(target_arch = &quot;<span class="s">wasm32</span>&quot;), target_os = &quot;<span class="s">linux</span>&quot;))]
<span class="k">pub</span> <span class="k">fn</span> heap_mb() -&gt; f64 {
    <span class="k">let</span> Ok(stats) = std::fs::read_to_string(&quot;<span class="s">/proc/self/statm</span>&quot;) <span class="k">else</span> {
        <span class="k">return</span> <span class="s">0</span>.<span class="s">0</span>;
    };
    <span class="c">// statm lists sizes in 4096-byte pages; the second number is the pages in RAM</span>
    <span class="k">let</span> Some(resident) = stats.split_whitespace().nth(<span class="s">1</span>) <span class="k">else</span> {
        <span class="k">return</span> <span class="s">0</span>.<span class="s">0</span>;
    };

    <span class="k">match</span> resident.parse::&lt;f64&gt;() {
        Ok(pages) =&gt; pages * <span class="s">4096</span>.<span class="s">0</span> / <span class="s">1</span>.<span class="s">048576</span>e<span class="s">6</span>,
        Err(_) =&gt; <span class="s">0</span>.<span class="s">0</span>,
    }
}

<span class="c">/// Native, non-Linux: no cheap measure.</span>
#[cfg(all(not(target_arch = &quot;<span class="s">wasm32</span>&quot;), not(target_os = &quot;<span class="s">linux</span>&quot;)))]
<span class="k">pub</span> <span class="k">fn</span> heap_mb() -&gt; f64 {
    <span class="s">0</span>.<span class="s">0</span>
}

<span class="c">/// Show one line of text in the page's top-left corner; the element is made on first use.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> perf_line(text: &amp;str) {
    <span class="k">let</span> Some(window) = web_sys::window() <span class="k">else</span> {
        <span class="k">return</span>;
    };
    <span class="k">let</span> Some(doc) = window.document() <span class="k">else</span> { <span class="k">return</span> };
    <span class="k">let</span> el = <span class="k">match</span> doc.get_element_by_id(&quot;<span class="s">perf</span>&quot;) {
        Some(e) =&gt; e,
        None =&gt; {
            <span class="k">let</span> Ok(e) = doc.create_element(&quot;<span class="s">pre</span>&quot;) <span class="k">else</span> {
                <span class="k">return</span>;
            };
            e.set_id(&quot;<span class="s">perf</span>&quot;);
            <span class="c">// \`let _ =\` drops a Result on purpose: an unstyled debug line is harmless</span>
            <span class="k">let</span> _ = e.set_attribute(&quot;<span class="s">style</span>&quot;, &quot;<span class="s">position:fixed;left:0;top:0;margin:0;padding:2px 6px;font:12px monospace;color:#000;background:rgba(255,255,255,.7);z-index:9;pointer-events:none</span>&quot;);

            <span class="k">if</span> <span class="k">let</span> Some(b) = doc.body() {
                <span class="k">let</span> _ = b.append_child(&amp;e);
            }

            e
        }
    };
    el.set_text_content(Some(text));
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// Feed \`count\` frames \`step_ms\` apart; true if a slow run fired.</span>
    <span class="k">fn</span> frames(perf: &amp;<span class="k">mut</span> Performance, count: u32, step_ms: f64, interacting: bool) -&gt; bool {
        perf.interacting = interacting;
        <span class="k">let</span> <span class="k">mut</span> now = perf.prev_frame;
        <span class="k">let</span> <span class="k">mut</span> slow = <span class="s">false</span>;

        <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..count {
            now += step_ms;
            perf.frame(<span class="s">1</span>, <span class="s">1</span>, now, <span class="s">false</span>);
            slow |= perf.take_slow_interaction();
        }

        slow
    }

    #[test]
    <span class="c">/// Only a run of slow drag frames fires, and only once.</span>
    <span class="k">fn</span> slow_interaction_needs_a_run_of_slow_drag_frames() {
        <span class="k">let</span> <span class="k">mut</span> perf = Performance::new();
        assert!(
            !frames(&amp;<span class="k">mut</span> perf, <span class="s">100</span>, <span class="s">60</span>.<span class="s">0</span>, <span class="s">false</span>),
            &quot;<span class="s">idle gaps are not slow frames</span>&quot;
        );
        assert!(!frames(&amp;<span class="k">mut</span> perf, <span class="s">100</span>, <span class="s">16</span>.<span class="s">7</span>, <span class="s">true</span>), &quot;<span class="s">a smooth drag is fine</span>&quot;);
        assert!(
            !frames(&amp;<span class="k">mut</span> perf, SLOW_FRAMES - <span class="s">1</span>, <span class="s">60</span>.<span class="s">0</span>, <span class="s">true</span>),
            &quot;<span class="s">one frame short of the run</span>&quot;
        );
        assert!(
            !frames(&amp;<span class="k">mut</span> perf, <span class="s">1</span>, <span class="s">16</span>.<span class="s">7</span>, <span class="s">true</span>),
            &quot;<span class="s">a fast frame resets the run</span>&quot;
        );
        assert!(frames(&amp;<span class="k">mut</span> perf, SLOW_FRAMES, <span class="s">60</span>.<span class="s">0</span>, <span class="s">true</span>), &quot;<span class="s">the run fires</span>&quot;);
        assert!(!frames(&amp;<span class="k">mut</span> perf, <span class="s">10</span>, <span class="s">60</span>.<span class="s">0</span>, <span class="s">true</span>), &quot;<span class="s">and fires once</span>&quot;);
    }
}</code></pre></div>
<h2 id="step-4-srcenginetextrs">Step 4 · src/engine/text.rs<a class="anchor" href="#/course/10-text-layout#step-4-srcenginetextrs" aria-label="Link to this section">#</a></h2>
<p>Text layout retains shaped glyph positions for rendering.</p>
<p><code>lessons/10/src/engine/text.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Shaping turns a string into placed glyphs: which glyph to draw from the font, and where the pen lands after each one.</span>
<span class="k">use</span> glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Wrap, fontdb};
<span class="k">use</span> serde::Serialize;

<span class="c">/// The font every label uses.</span>
<span class="k">pub</span> <span class="k">const</span> FONT_FAMILY: &amp;str = &quot;<span class="s">Noto Sans</span>&quot;;

<span class="c">/// The main font, bundled into the binary.</span>
<span class="k">pub</span> <span class="k">const</span> FONT_BYTES: &amp;[u8] = include_bytes!(&quot;<span class="s">../../assets/text/NotoSans-Regular.ttf</span>&quot;);

<span class="c">/// Symbol fallback font.</span>
<span class="k">pub</span> <span class="k">const</span> SYMBOL_BYTES: &amp;[u8] = include_bytes!(&quot;<span class="s">../../assets/text/NotoSansSymbols-Regular.ttf</span>&quot;);

<span class="c">/// Second symbol fallback font.</span>
<span class="k">pub</span> <span class="k">const</span> FALLBACK_BYTES: &amp;[u8] = include_bytes!(&quot;<span class="s">../../assets/text/NotoSansSymbols2-Regular.ttf</span>&quot;);

<span class="c">/// Most text bytes in one label set.</span>
<span class="k">const</span> MAX_TEXT_BYTES: usize = <span class="s">256</span> * <span class="s">1024</span>;

<span class="c">/// Text origin, orientation and size policy; physical labels use scene depth.</span>
#[derive(Clone, Debug, PartialEq)]
<span class="k">pub</span> <span class="k">enum</span> TextPlacement {
    Screen { <span class="c">// fixed on screen, CSS px</span>
        left: f32,
        top: f32,
    },
    Anchor { <span class="c">// at a world point, screen-sized, hidden behind geometry</span>
        world: [f64; <span class="s">3</span>],
        offset: [f32; <span class="s">2</span>], <span class="c">// shift from it, CSS px</span>
    },
    Nameplate { <span class="c">// centered on a world point with a plate, always on top</span>
        <span class="c">// world point</span>
        world: [f64; <span class="s">3</span>],
        padding: [f32; <span class="s">2</span>], <span class="c">// space around the text, CSS px</span>
        rounded: bool, <span class="c">// rounded plate corners</span>
    },
    WorldPlane {
        <span class="c">// top-left corner</span>
        world: [f64; <span class="s">3</span>],
        right: [f64; <span class="s">3</span>], <span class="c">// unit axis along the text</span>
        up: [f64; <span class="s">3</span>], <span class="c">// unit axis up the text</span>
        world_height: f64,
    },
    WorldBillboard { <span class="c">// facing the camera, world-sized</span>
        <span class="c">// world point</span>
        world: [f64; <span class="s">3</span>],
        world_height: f64,
    },
}</code></pre></div>
<p><code>lessons/10/src/engine/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One source label, sizes in CSS pixels.</span>
#[derive(Clone, Debug, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> TextLabel {
    <span class="k">pub</span> id: u32, <span class="c">// unique per label</span>
    <span class="k">pub</span> text: String,
    <span class="k">pub</span> font_size: f32,
    <span class="k">pub</span> line_height: f32, <span class="c">// distance between lines</span>
    <span class="k">pub</span> color: [u8; <span class="s">4</span>], <span class="c">// rgba</span>
    <span class="k">pub</span> placement: TextPlacement, <span class="c">// where it sits</span>
    <span class="k">pub</span> clip: Option&lt;[f32; 4]&gt;, <span class="c">// screen box to cut it to: left, top, right, bottom</span>
}

<span class="c">/// A label with its shaped glyphs.</span>
<span class="k">pub</span> <span class="k">struct</span> TextRun {
    <span class="k">pub</span> label: TextLabel,
    <span class="k">pub</span> buffer: Buffer, <span class="c">// its glyphs, laid out by glyphon</span>
}

<span class="c">/// Every label, shaped, with the fonts.</span>
<span class="k">pub</span> <span class="k">struct</span> TextDocument {
    <span class="k">pub</span> fonts: FontSystem,
    <span class="k">pub</span> runs: Vec&lt;TextRun&gt;, <span class="c">// shaped labels</span>
    <span class="k">pub</span> revision: u64, <span class="c">// bumps on every label change</span>
    <span class="k">pub</span> font_revision: u64, <span class="c">// bumps on every font change</span>
    <span class="k">pub</span> shape_count: u64, <span class="c">// labels shaped so far</span>
    <span class="k">pub</span> shaping_ms: f64, <span class="c">// time spent shaping</span>
}</code></pre></div>
<p><code>lessons/10/src/engine/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> TextDocument {
    <span class="c">/// An empty document with the bundled fonts.</span>
    <span class="k">pub</span> <span class="k">fn</span> new() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            fonts: bundled_fonts(),
            runs: Vec::new(),
            revision: <span class="s">0</span>,
            font_revision: <span class="s">1</span>,
            shape_count: <span class="s">0</span>,
            shaping_ms: <span class="s">0</span>.<span class="s">0</span>,
        }
    }

    <span class="c">/// Replace every label; unchanged text keeps its shaping.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_labels(&amp;<span class="k">mut</span> <span class="k">self</span>, labels: Vec&lt;TextLabel&gt;) -&gt; anyhow::Result&lt;()&gt; {
        <span class="k">let</span> <span class="k">mut</span> bytes = <span class="s">0usize</span>;
        <span class="k">let</span> <span class="k">mut</span> ids = std::collections::HashSet::new();

        <span class="c">// check every label before touching anything</span>
        <span class="k">for</span> label <span class="k">in</span> &amp;labels {
            bytes = bytes.saturating_add(label.text.len());
            anyhow::ensure!(bytes &lt;= MAX_TEXT_BYTES, &quot;<span class="s">text document exceeds 256 KiB</span>&quot;);
            anyhow::ensure!(ids.insert(label.id), &quot;<span class="s">duplicate text label ID </span>{}&quot;, label.id);
            validate_label(label)?;
        }

        <span class="c">// same labels: nothing to do</span>
        <span class="k">if</span> <span class="k">self</span>.runs.len() == labels.len() {
            <span class="k">let</span> <span class="k">mut</span> unchanged = <span class="s">true</span>;

            <span class="k">for</span> (run, label) <span class="k">in</span> <span class="k">self</span>.runs.iter().zip(&amp;labels) {
                unchanged &amp;= run.label == *label;
            }

            <span class="k">if</span> unchanged {
                <span class="k">return</span> Ok(());
            }
        }

        <span class="k">let</span> previous = std::mem::take(&amp;<span class="k">mut</span> <span class="k">self</span>.runs);
        <span class="k">let</span> <span class="k">mut</span> previous_by_id = std::collections::HashMap::new();

        <span class="k">for</span> run <span class="k">in</span> previous {
            previous_by_id.insert(run.label.id, run);
        }

        <span class="c">// reshape only labels whose text or size changed</span>
        <span class="k">for</span> label <span class="k">in</span> labels {
            <span class="k">let</span> old = previous_by_id.remove(&amp;label.id);
            <span class="k">let</span> buffer = <span class="k">match</span> old {
                Some(run) <span class="k">if</span> same_layout(&amp;run.label, &amp;label) =&gt; run.buffer,
                _ =&gt; {
                    <span class="k">self</span>.shape_count += <span class="s">1</span>;
                    <span class="k">let</span> started = <span class="k">crate</span>::engine::performance::now_ms();
                    <span class="k">let</span> buffer = shape(&amp;<span class="k">mut</span> <span class="k">self</span>.fonts, &amp;label);
                    <span class="k">self</span>.shaping_ms += <span class="k">crate</span>::engine::performance::now_ms() - started;
                    buffer
                }
            };
            <span class="k">self</span>.runs.push(TextRun { label, buffer });
        }

        <span class="k">self</span>.revision = <span class="k">self</span>.revision.wrapping_add(<span class="s">1</span>);
        Ok(())
    }</code></pre></div>
<p><code>lessons/10/src/engine/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Replace the fonts and reshape everything; bad data changes nothing.</span>
    <span class="k">pub</span> <span class="k">fn</span> replace_fonts(&amp;<span class="k">mut</span> <span class="k">self</span>, sources: Vec&lt;Vec&lt;u8&gt;&gt;) -&gt; anyhow::Result&lt;()&gt; {
        <span class="k">let</span> <span class="k">mut</span> db = fontdb::Database::new();

        <span class="k">for</span> bytes <span class="k">in</span> sources {
            <span class="k">let</span> before = db.faces().count();
            db.load_font_data(bytes);
            anyhow::ensure!(
                db.faces().count() &gt; before,
                &quot;<span class="s">font data contains no usable face</span>&quot;
            );
        }

        anyhow::ensure!(db.faces().count() &gt; <span class="s">0</span>, &quot;<span class="s">font set is empty</span>&quot;);
        db.set_sans_serif_family(FONT_FAMILY);
        <span class="k">self</span>.fonts = FontSystem::new_with_locale_and_db(&quot;<span class="s">en-US</span>&quot;.into(), db);

        <span class="k">for</span> run <span class="k">in</span> &amp;<span class="k">mut</span> <span class="k">self</span>.runs {
            <span class="k">let</span> started = <span class="k">crate</span>::engine::performance::now_ms();
            run.buffer = shape(&amp;<span class="k">mut</span> <span class="k">self</span>.fonts, &amp;run.label);
            <span class="k">self</span>.shaping_ms += <span class="k">crate</span>::engine::performance::now_ms() - started;
            <span class="k">self</span>.shape_count += <span class="s">1</span>;
        }

        <span class="k">self</span>.font_revision = <span class="k">self</span>.font_revision.wrapping_add(<span class="s">1</span>);
        <span class="k">self</span>.revision = <span class="k">self</span>.revision.wrapping_add(<span class="s">1</span>);
        Ok(())
    }

    <span class="c">/// Forget every label.</span>
    <span class="k">pub</span> <span class="k">fn</span> clear(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.runs.clear();
        <span class="k">self</span>.runs.shrink_to_fit();
        <span class="k">self</span>.revision = <span class="k">self</span>.revision.wrapping_add(<span class="s">1</span>);
    }

    <span class="c">/// Every shaped glyph with its metrics, for tests.</span>
    <span class="k">pub</span> <span class="k">fn</span> diagnostics(&amp;<span class="k">self</span>) -&gt; Vec&lt;GlyphDiagnostic&gt; {
        <span class="k">let</span> <span class="k">mut</span> out = Vec::new();

        <span class="k">for</span> run <span class="k">in</span> &amp;<span class="k">self</span>.runs {
            <span class="k">for</span> line <span class="k">in</span> run.buffer.layout_runs() {
                <span class="k">for</span> glyph <span class="k">in</span> line.glyphs {
                    out.push(GlyphDiagnostic {
                        label: run.label.id,
                        line: line.line_i,
                        cluster: [glyph.start, glyph.end],
                        glyph: glyph.glyph_id,
                        font: format!(&quot;{<span class="s">:?</span>}&quot;, glyph.font_id),
                        origin: [glyph.x, glyph.y],
                        advance: glyph.w,
                        offset: [
                            glyph.x_offset * glyph.font_size,
                            glyph.y_offset * glyph.font_size,
                        ],
                        baseline: line.line_y,
                        line_width: line.line_w,
                    });
                }
            }
        }

        out
    }
}</code></pre></div>
<p><code>lessons/10/src/engine/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Default <span class="k">for</span> TextDocument {
    <span class="c">/// Same as \`new\`.</span>
    <span class="k">fn</span> default() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::new()
    }
}

<span class="c">/// One shaped glyph, for tests.</span>
#[derive(Clone, Debug, Serialize)]
<span class="k">pub</span> <span class="k">struct</span> GlyphDiagnostic {
    <span class="k">pub</span> label: u32,
    <span class="k">pub</span> line: usize,
    <span class="k">pub</span> cluster: [usize; <span class="s">2</span>], <span class="c">// byte range in the text</span>
    <span class="k">pub</span> glyph: u16, <span class="c">// glyph id; 0 = missing</span>
    <span class="k">pub</span> font: String,
    <span class="k">pub</span> origin: [f32; <span class="s">2</span>], <span class="c">// position on the line</span>
    <span class="k">pub</span> advance: f32, <span class="c">// width</span>
    <span class="k">pub</span> offset: [f32; <span class="s">2</span>],
    <span class="k">pub</span> baseline: f32,
    <span class="k">pub</span> line_width: f32,
}

<span class="c">/// The bundled fonts as a font system.</span>
<span class="k">fn</span> bundled_fonts() -&gt; FontSystem {
    <span class="k">let</span> <span class="k">mut</span> db = fontdb::Database::new();
    db.load_font_data(FONT_BYTES.to_vec());
    db.load_font_data(FALLBACK_BYTES.to_vec());
    db.load_font_data(SYMBOL_BYTES.to_vec());
    db.set_sans_serif_family(FONT_FAMILY);
    FontSystem::new_with_locale_and_db(&quot;<span class="s">en-US</span>&quot;.into(), db)
}</code></pre></div>
<p><code>lessons/10/src/engine/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Reject sizes, positions and clips that are not finite or out of range.</span>
<span class="k">fn</span> validate_label(label: &amp;TextLabel) -&gt; anyhow::Result&lt;()&gt; {
    anyhow::ensure!(
        label.font_size.is_finite() &amp;&amp; (<span class="s">1</span>.<span class="s">0</span>..=<span class="s">256</span>.<span class="s">0</span>).contains(&amp;label.font_size),
        &quot;<span class="s">font size must be 1..256 CSS px</span>&quot;
    );
    anyhow::ensure!(
        label.line_height.is_finite()
            &amp;&amp; label.line_height &gt;= label.font_size
            &amp;&amp; label.line_height &lt;= <span class="s">1024</span>.<span class="s">0</span>,
        &quot;<span class="s">invalid text line height</span>&quot;
    );
    <span class="k">let</span> valid_placement = <span class="k">match</span> label.placement {
        TextPlacement::Screen { left, top } =&gt; left.is_finite() &amp;&amp; top.is_finite(),
        TextPlacement::Anchor { world, offset } =&gt; {
            world.iter().all(finite_f64) &amp;&amp; offset.iter().all(finite_f32)
        }
        TextPlacement::Nameplate { world, padding, .. } =&gt; {
            world.iter().all(finite_f64) &amp;&amp; padding.iter().all(valid_padding)
        }
        TextPlacement::WorldPlane {
            world,
            right,
            up,
            world_height,
        } =&gt; {
            world.iter().all(finite_f64)
                &amp;&amp; valid_plane_axes(right, up)
                &amp;&amp; world_height.is_finite()
                &amp;&amp; world_height &gt; <span class="s">0</span>.<span class="s">0</span>
        }
        TextPlacement::WorldBillboard {
            world,
            world_height,
        } =&gt; world.iter().all(finite_f64) &amp;&amp; world_height.is_finite() &amp;&amp; world_height &gt; <span class="s">0</span>.<span class="s">0</span>,
    };
    anyhow::ensure!(valid_placement, &quot;<span class="s">invalid text placement</span>&quot;);

    <span class="k">if</span> <span class="k">let</span> Some(c) = label.clip {
        anyhow::ensure!(
            c.iter().all(finite_f32) &amp;&amp; c[<span class="s">2</span>] &gt;= c[<span class="s">0</span>] &amp;&amp; c[<span class="s">3</span>] &gt;= c[<span class="s">1</span>],
            &quot;<span class="s">invalid text clip rectangle</span>&quot;
        );
    }

    Ok(())
}

<span class="c">/// True when both axes are unit length and perpendicular.</span>
<span class="k">fn</span> valid_plane_axes(right: [f64; <span class="s">3</span>], up: [f64; <span class="s">3</span>]) -&gt; bool {
    <span class="k">let</span> <span class="k">mut</span> right_length = <span class="s">0</span>.<span class="s">0</span>;
    <span class="k">let</span> <span class="k">mut</span> up_length = <span class="s">0</span>.<span class="s">0</span>;
    <span class="k">let</span> <span class="k">mut</span> dot = <span class="s">0</span>.<span class="s">0</span>;

    <span class="k">for</span> axis <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
        right_length += right[axis] * right[axis];
        up_length += up[axis] * up[axis];
        dot += right[axis] * up[axis];
    }

    (right_length.sqrt() - <span class="s">1</span>.<span class="s">0</span>).abs() &lt;= <span class="s">1</span>e-<span class="s">6</span>
        &amp;&amp; (up_length.sqrt() - <span class="s">1</span>.<span class="s">0</span>).abs() &lt;= <span class="s">1</span>e-<span class="s">6</span>
        &amp;&amp; dot.abs() &lt;= <span class="s">1</span>e-<span class="s">6</span>
}</code></pre></div>
<p><code>lessons/10/src/engine/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// True for a finite f32.</span>
<span class="k">fn</span> finite_f32(value: &amp;f32) -&gt; bool {
    value.is_finite()
}

<span class="c">/// True for a finite f64.</span>
<span class="k">fn</span> finite_f64(value: &amp;f64) -&gt; bool {
    value.is_finite()
}

<span class="c">/// True for padding in 0..256.</span>
<span class="k">fn</span> valid_padding(value: &amp;f32) -&gt; bool {
    value.is_finite() &amp;&amp; (<span class="s">0</span>.<span class="s">0</span>..=<span class="s">256</span>.<span class="s">0</span>).contains(value)
}

<span class="c">/// True when two labels shape the same: same text, size, line height.</span>
<span class="k">fn</span> same_layout(a: &amp;TextLabel, b: &amp;TextLabel) -&gt; bool {
    a.text == b.text &amp;&amp; a.font_size == b.font_size &amp;&amp; a.line_height == b.line_height
}

<span class="c">/// Shape one label into glyphs.</span>
<span class="k">fn</span> shape(fonts: &amp;<span class="k">mut</span> FontSystem, label: &amp;TextLabel) -&gt; Buffer {
    <span class="k">let</span> <span class="k">mut</span> buffer = Buffer::new(fonts, Metrics::new(label.font_size, label.line_height));
    buffer.set_size(fonts, None, None);
    buffer.set_wrap(fonts, Wrap::None);
    buffer.set_text(
        fonts,
        &amp;label.text,
        &amp;Attrs::new()
            .family(Family::Name(FONT_FAMILY))
            .metadata(label.id <span class="k">as</span> usize),
        Shaping::Advanced,
        None,
    );
    buffer.shape_until_scroll(fonts, <span class="s">false</span>);
    buffer
}</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/10/src/engine/text.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// A screen label with fixed metrics.</span>
    <span class="k">fn</span> label(text: &amp;str) -&gt; TextLabel {
        TextLabel {
            id: <span class="s">1</span>,
            text: text.into(),
            font_size: <span class="s">16</span>.<span class="s">0</span>,
            line_height: <span class="s">24</span>.<span class="s">0</span>,
            color: [<span class="s">255</span>; <span class="s">4</span>],
            placement: TextPlacement::Screen {
                left: <span class="s">0</span>.<span class="s">0</span>,
                top: <span class="s">0</span>.<span class="s">0</span>,
            },
            clip: None,
        }
    }

    #[test]
    <span class="c">/// Ligatures, accents, kerning and symbols all shape to real glyphs.</span>
    <span class="k">fn</span> ligatures_accents_spaces_and_symbols_are_shaped() {
        <span class="k">let</span> <span class="k">mut</span> doc = TextDocument::new();
        doc.set_labels(vec![label(
            &quot;<span class="s">office ffi fl ÄÖÜ ĄČĘĖĮŠŲŪŽ Ø ± 90° m² e\\u</span>{<span class="s">301</span>}&quot;,
        )])
        .unwrap();
        <span class="k">let</span> glyphs = doc.diagnostics();
        assert!(glyphs.iter().all(has_glyph));
        assert!(glyphs.iter().any(multi_byte_cluster));
        doc.set_labels(vec![label(&quot;<span class="s">AV</span>&quot;)]).unwrap();
        <span class="k">let</span> pair_width = doc.diagnostics()[<span class="s">0</span>].line_width;
        doc.set_labels(vec![label(&quot;<span class="s">A V</span>&quot;)]).unwrap();
        assert!(doc.diagnostics()[<span class="s">0</span>].line_width &gt; pair_width);
        doc.set_labels(vec![label(&quot;<span class="s">ffi</span>&quot;)]).unwrap();
        assert!(
            doc.diagnostics().len() &lt; <span class="s">3</span>,
            &quot;<span class="s">font's ffi ligature must preserve a multi-character cluster</span>&quot;
        );
    }

    <span class="c">/// True when the glyph exists.</span>
    <span class="k">fn</span> has_glyph(glyph: &amp;GlyphDiagnostic) -&gt; bool {
        glyph.glyph != <span class="s">0</span>
    }

    <span class="c">/// True when the cluster spans several bytes.</span>
    <span class="k">fn</span> multi_byte_cluster(glyph: &amp;GlyphDiagnostic) -&gt; bool {
        glyph.cluster[<span class="s">1</span>] - glyph.cluster[<span class="s">0</span>] &gt; <span class="s">1</span>
    }

    #[test]
    <span class="c">/// Moving or recoloring keeps the shaping; new fonts reshape.</span>
    <span class="k">fn</span> placement_and_color_do_not_reshape_but_font_reload_does() {
        <span class="k">let</span> <span class="k">mut</span> doc = TextDocument::new();
        <span class="k">let</span> <span class="k">mut</span> item = label(&quot;<span class="s">AVATAR</span>&quot;);
        doc.set_labels(vec![item.clone()]).unwrap();
        <span class="k">let</span> before = doc.diagnostics()[<span class="s">0</span>].line_width;
        item.color = [<span class="s">255</span>, <span class="s">255</span>, <span class="s">0</span>, <span class="s">255</span>];
        item.placement = TextPlacement::Screen {
            left: <span class="s">0</span>.<span class="s">375</span>,
            top: <span class="s">1</span>.<span class="s">25</span>,
        };
        doc.set_labels(vec![item]).unwrap();
        assert_eq!(doc.shape_count, <span class="s">1</span>);
        assert_eq!(doc.diagnostics()[<span class="s">0</span>].line_width, before);
        assert!(doc.replace_fonts(vec![vec![<span class="s">0</span>; <span class="s">8</span>]]).is_err());
        assert_eq!(doc.shape_count, <span class="s">1</span>);
        doc.replace_fonts(vec![
            FONT_BYTES.to_vec(),
            FALLBACK_BYTES.to_vec(),
            SYMBOL_BYTES.to_vec(),
        ])
        .unwrap();
        assert_eq!(doc.shape_count, <span class="s">2</span>);
        assert_eq!(doc.diagnostics()[<span class="s">0</span>].line_width, before);
    }

    #[test]
    <span class="c">/// é and e + accent shape alike; lines sit one line height apart.</span>
    <span class="k">fn</span> composed_decomposed_accents_and_multiline_baselines_match() {
        <span class="k">let</span> <span class="k">mut</span> doc = TextDocument::new();
        doc.set_labels(vec![label(&quot;<span class="s">é\\ne\\u</span>{<span class="s">301</span>}&quot;)]).unwrap();
        <span class="k">let</span> glyphs = doc.diagnostics();
        assert_eq!(glyphs.len(), <span class="s">2</span>);
        assert_eq!(glyphs[<span class="s">0</span>].glyph, glyphs[<span class="s">1</span>].glyph);
        assert_eq!(glyphs[<span class="s">0</span>].advance, glyphs[<span class="s">1</span>].advance);
        assert!((glyphs[<span class="s">1</span>].baseline - glyphs[<span class="s">0</span>].baseline - <span class="s">24</span>.<span class="s">0</span>).abs() &lt; <span class="s">0</span>.<span class="s">001</span>);
    }

    #[test]
    <span class="c">/// Symbols come from the bundled fallback fonts.</span>
    <span class="k">fn</span> bundled_fallback_covers_symbols_without_system_fonts() {
        <span class="k">let</span> <span class="k">mut</span> doc = TextDocument::new();
        doc.set_labels(vec![label(&quot;<span class="s">CAD ⚙ ⏳ ⌘</span>&quot;)]).unwrap();
        <span class="k">let</span> glyphs = doc.diagnostics();
        assert!(glyphs.iter().all(has_glyph));
        <span class="k">let</span> <span class="k">mut</span> faces = std::collections::HashSet::new();

        <span class="k">for</span> glyph <span class="k">in</span> glyphs {
            faces.insert(glyph.font);
        }

        assert!(
            faces.len() &gt;= <span class="s">2</span>,
            &quot;<span class="s">sample must exercise explicit font fallback</span>&quot;
        );
    }

    #[test]
    <span class="c">/// A bad label set leaves the old one in place.</span>
    <span class="k">fn</span> invalid_replacement_preserves_current_document() {
        <span class="k">let</span> <span class="k">mut</span> doc = TextDocument::new();
        doc.set_labels(vec![label(&quot;<span class="s">Keep</span>&quot;)]).unwrap();
        <span class="k">let</span> <span class="k">mut</span> invalid = label(&quot;<span class="s">Reject</span>&quot;);
        invalid.font_size = f32::NAN;
        assert!(doc.set_labels(vec![invalid]).is_err());
        assert_eq!(doc.runs[<span class="s">0</span>].label.text, &quot;<span class="s">Keep</span>&quot;);
    }
}</code></pre></div>
<h2 id="step-5-srcenginemodrs">Step 5 · src/engine/mod.rs<a class="anchor" href="#/course/10-text-layout#step-5-srcenginemodrs" aria-label="Link to this section">#</a></h2>
<p>The engine module exposes the rendering implementation.</p>
<p><code>lessons/10/src/engine/mod.rs</code> · edit · type this</p>
<p>Replaces <code>mod pipelines</code> in <code>lessons/09/src/engine/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> performance;
<span class="k">pub</span> <span class="k">mod</span> pipelines;
<span class="k">pub</span> <span class="k">mod</span> text;</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/10/</code>.</p>
<h2 id="step-6-srctext_layoutrs">Step 6 · src/text_layout.rs<a class="anchor" href="#/course/10-text-layout#step-6-srctext_layoutrs" aria-label="Link to this section">#</a></h2>
<p>Copy the wasm export that shapes one string at five sizes and returns every glyph as JSON.</p>
<p><code>lessons/10/src/text_layout.rs</code> · 67 lines · copy the file, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! A check on shaping alone, before any GPU text: one string at five sizes, sent to text-layout.html as JSON.</span>
<span class="k">use</span> <span class="k">crate</span>::engine::text::{FONT_FAMILY, TextDocument, TextLabel, TextPlacement};
<span class="k">use</span> wasm_bindgen::prelude::*;

<span class="c">/// JavaScript calls this as \`module.text_layout()\`; an Err arrives there as a thrown error.</span>
#[wasm_bindgen]
<span class="k">pub</span> <span class="k">fn</span> text_layout() -&gt; Result&lt;String, JsValue&gt; {
    layout_report().map_err(js_error)
}

<span class="c">/// Shape once, then move and recolor every label: the shape count must stay the same.</span>
<span class="k">fn</span> layout_report() -&gt; anyhow::Result&lt;String&gt; {
    <span class="k">let</span> <span class="k">mut</span> document = TextDocument::new();
    <span class="k">let</span> <span class="k">mut</span> labels = Vec::new();

    <span class="k">for</span> (index, size) <span class="k">in</span> [<span class="s">12</span>.<span class="s">0</span>, <span class="s">14</span>.<span class="s">0</span>, <span class="s">16</span>.<span class="s">0</span>, <span class="s">18</span>.<span class="s">0</span>, <span class="s">24</span>.<span class="s">0</span>].into_iter().enumerate() {
        labels.push(TextLabel {
            id: index <span class="k">as</span> u32,
            <span class="c">// kerning (AV, To), a ligature (ffi), é typed two ways, and ⚙ from a fallback font</span>
            text: &quot;<span class="s">AVATAR To office ffi • é e\\u</span>{<span class="s">301</span>}<span class="s"> • Ø 25 ± 0.1 mm • ⚙</span>&quot;.into(),
            font_size: size,
            line_height: size * <span class="s">1</span>.<span class="s">5</span>,
            color: [<span class="s">255</span>; <span class="s">4</span>],
            placement: TextPlacement::Screen {
                left: <span class="s">0</span>.<span class="s">0</span>,
                top: <span class="s">0</span>.<span class="s">0</span>,
            },
            clip: None,
        });
    }

    document.set_labels(labels.clone())?;
    <span class="k">let</span> shaped = document.shape_count;

    <span class="c">// color and position are not part of the shape, so this must reuse it</span>
    <span class="k">for</span> label <span class="k">in</span> &amp;<span class="k">mut</span> labels {
        label.color = [<span class="s">255</span>, <span class="s">255</span>, <span class="s">0</span>, <span class="s">255</span>];
        label.placement = TextPlacement::Screen {
            left: <span class="s">10</span>.<span class="s">0</span>,
            top: <span class="s">20</span>.<span class="s">0</span>,
        };
    }

    document.set_labels(labels)?;
    anyhow::ensure!(
        document.shape_count == shaped,
        &quot;<span class="s">placement/color unexpectedly reshaped</span>&quot;
    );
    <span class="k">let</span> <span class="k">mut</span> rows = Vec::new();

    <span class="k">for</span> run <span class="k">in</span> &amp;document.runs {
        rows.push(serde_json::json!({&quot;<span class="s">id</span>&quot;:run.label.id,&quot;<span class="s">text</span>&quot;:run.label.text,
            &quot;<span class="s">size</span>&quot;:run.label.font_size,&quot;<span class="s">lineHeight</span>&quot;:run.label.line_height}));
    }

    Ok(
        serde_json::json!({&quot;<span class="s">family</span>&quot;:FONT_FAMILY,&quot;<span class="s">rows</span>&quot;:rows,&quot;<span class="s">glyphs</span>&quot;:document.diagnostics(),
        &quot;<span class="s">shapeCount</span>&quot;:document.shape_count,&quot;<span class="s">shapedBeforePlacementChange</span>&quot;:shaped,
        &quot;<span class="s">revision</span>&quot;:document.revision,&quot;<span class="s">fontRevision</span>&quot;:document.font_revision})
        .to_string(),
    )
}

<span class="c">/// JavaScript receives the error message as a plain string.</span>
<span class="k">fn</span> js_error(error: <span class="k">impl</span> std::fmt::Display) -&gt; JsValue {
    JsValue::from_str(&amp;error.to_string())
}</code></pre></div>
<h2 id="step-7-srclibrs">Step 7 · src/lib.rs<a class="anchor" href="#/course/10-text-layout#step-7-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The crate entry point connects the camera, scene and GPU owners.</p>
<p><code>lessons/10/src/lib.rs</code> · edit · type this</p>
<p>Added after the <code>pub mod fixture;</code> line of <code>lessons/09/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> text_layout;</code></pre></div>
<p>Replaces the <code>Ok(serde_json::json!({&quot;stage&quot;:9,&quot;objects&quot;:sel…</code> line in <code>fn render</code> of <code>lessons/09/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        Ok(serde_json::json!({&quot;<span class="s">stage</span>&quot;:<span class="s">10</span>,&quot;<span class="s">objects</span>&quot;:<span class="k">self</span>.gpu.objects.len(),&quot;<span class="s">width</span>&quot;:w,&quot;<span class="s">height</span>&quot;:h,&quot;<span class="s">scale</span>&quot;:scale,&quot;<span class="s">drawn</span>&quot;:<span class="s">true</span>,&quot;<span class="s">sourceObjects</span>&quot;:<span class="k">self</span>.fixture.identities,&quot;<span class="s">sourceEdgeIds</span>&quot;:<span class="k">self</span>.fixture.pipe_source_edges,&quot;<span class="s">samples</span>&quot;:<span class="k">self</span>.gpu.targets.samples,</code></pre></div>
<h2 id="step-8-assetstext-layouthtml">Step 8 · assets/text-layout.html<a class="anchor" href="#/course/10-text-layout#step-8-assetstext-layouthtml" aria-label="Link to this section">#</a></h2>
<p>Copy the check page: it sets the same string in browser text and fails when a width differs.</p>
<p><code>lessons/10/assets/text-layout.html</code> · 46 lines · copy the file, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>&lt;!doctype html&gt;&lt;html lang=&quot;<span class="s">en</span>&quot;&gt;&lt;meta charset=&quot;<span class="s">utf-8</span>&quot;&gt;&lt;meta name=&quot;<span class="s">viewport</span>&quot; content=&quot;<span class="s">width=device-width,initial-scale=1</span>&quot;&gt;
&lt;title&gt;Checkpoint 10 — source text and shaped glyphs&lt;/title&gt;
&lt;style&gt;
@<span class="k">font-face</span>{font-family:'<span class="s">Noto Sans</span>';src:url('<span class="s">./text/NotoSans-Regular.ttf</span>')}@<span class="k">font-face</span>{font-family:'<span class="s">Noto Sans Symbols</span>';src:url('<span class="s">./text/NotoSansSymbols-Regular.ttf</span>')}@<span class="k">font-face</span>{font-family:'<span class="s">Noto Sans Symbols 2</span>';src:url('<span class="s">./text/NotoSansSymbols2-Regular.ttf</span>')}
body{background:#171717;color:white;font:<span class="s">14</span><span class="k">px</span> sans-serif;margin:<span class="s">20</span><span class="k">px</span>}.line{display:table;white-space:pre;background:black;color:white;font-family:'<span class="s">Noto Sans</span>','<span class="s">Noto Sans Symbols</span>','<span class="s">Noto Sans Symbols 2</span>';font-kerning:normal;font-variant-ligatures:common-ligatures;letter-spacing:normal;margin:<span class="s">12</span><span class="k">px</span> <span class="s">0</span>}pre{white-space:pre-wrap;overflow-wrap:anywhere}.failed{color:#ff8f8f}
&lt;/style&gt;
&lt;h1&gt;Source text → shaped runs&lt;/h1&gt;&lt;p&gt;Browser reference in the same explicit fonts; chapter 11 adds the GPU coverage renderer.&lt;/p&gt;
&lt;div id=&quot;<span class="s">specimens</span>&quot;&gt;&lt;/div&gt;&lt;output id=&quot;<span class="s">layout-status</span>&quot;&gt;Loading exact fonts and WASM…&lt;/output&gt;&lt;details&gt;&lt;summary&gt;Glyph IDs, byte clusters, advances, offsets and baselines&lt;/summary&gt;&lt;pre id=&quot;<span class="s">diagnostics</span>&quot;&gt;&lt;/pre&gt;&lt;/details&gt;
&lt;script type=&quot;<span class="s">module</span>&quot;&gt;
<span class="c">/** A failed fetch throws instead of handing an error page to the parser. */</span>
<span class="k">function</span> readText(response){<span class="k">if</span>(!response.ok)<span class="k">throw</span> Error(\`<span class="s">HTTP </span>\${<span class="s">response</span>.<span class="s">status</span>}\`);<span class="k">return</span> response.text();}
<span class="c">/** Shape in wasm, lay out the same text in the browser, and fail when a width differs by over 0.2 px. */</span>
<span class="k">async</span> <span class="k">function</span> start(){
 <span class="k">await</span> Promise.all([document.fonts.load('<span class="s">16px &quot;Noto Sans&quot;</span>'),document.fonts.load('<span class="s">16px &quot;Noto Sans Symbols&quot;</span>'),document.fonts.load('<span class="s">16px &quot;Noto Sans Symbols 2&quot;</span>')]);
 <span class="k">await</span> document.fonts.ready;
 <span class="c">// Trunk adds a hash to the .js and .wasm names; read the real names from index.html</span>
 <span class="k">const</span> html=<span class="k">await</span> fetch('<span class="s">./index.html</span>',{cache:'<span class="s">no-store</span>'}).then(readText);
 <span class="k">const</span> js=html.match(/(?:<span class="s">import</span>[^<span class="s">;</span>]*?<span class="s">from\\s</span>*|<span class="s">import\\s</span>*)[<span class="s">&quot;'</span>]([^<span class="s">&quot;'</span>]*<span class="s">session_viewer</span>[^<span class="s">&quot;'</span>]*<span class="s">\\.js</span>)[<span class="s">&quot;'</span>]/);
 <span class="k">const</span> wasm=html.match(/<span class="s">module_or_path:\\s</span>*[<span class="s">&quot;'</span>]([^<span class="s">&quot;'</span>]+<span class="s">\\.wasm</span>)[<span class="s">&quot;'</span>]/);
 <span class="k">if</span>(!js||!wasm)<span class="k">throw</span> Error('<span class="s">Build this checkpoint with Trunk before opening the fixture.</span>');
 <span class="k">const</span> module=<span class="k">await</span> import(new URL(js[<span class="s">1</span>],location.href));
 <span class="k">await</span> module.default({module_or_path:new URL(wasm[<span class="s">1</span>],location.href).href});
 <span class="k">const</span> report=JSON.parse(module.text_layout());
 <span class="k">const</span> metrics=[];
 <span class="k">for</span>(<span class="k">const</span> row of report.rows){
  <span class="k">const</span> line=document.createElement('<span class="s">div</span>');line.className='<span class="s">line</span>';line.style.fontSize=\`\${<span class="s">row</span>.<span class="s">size</span>}<span class="s">px</span>\`;line.style.lineHeight=\`\${<span class="s">row</span>.<span class="s">lineHeight</span>}<span class="s">px</span>\`;line.textContent=row.text;
  document.getElementById('<span class="s">specimens</span>').append(line);
  <span class="k">let</span> width=<span class="s">0</span>;
  <span class="k">for</span>(<span class="k">const</span> glyph of report.glyphs){<span class="k">if</span>(glyph.label===row.id)width=Math.max(width,glyph.line_width);<span class="k">if</span>(glyph.glyph===<span class="s">0</span>)<span class="k">throw</span> Error('<span class="s">Missing font glyph</span>');}
  <span class="k">const</span> browser=line.getBoundingClientRect().width;
  <span class="k">const</span> difference=Math.abs(browser-width);
  <span class="k">if</span>(difference&gt;<span class="s">0.2</span>)<span class="k">throw</span> Error(\`<span class="s">Same-font width differs at </span>\${<span class="s">row</span>.<span class="s">size</span>}<span class="s">px: </span>\${<span class="s">difference</span>}\`);
  metrics.push({size:row.size,shaper:width,browser,difference});
 }
 <span class="k">if</span>(report.shapeCount!==<span class="s">5</span>||report.shapedBeforePlacementChange!==<span class="s">5</span>)<span class="k">throw</span> Error('<span class="s">Placement/color invalidated shaping</span>');
 <span class="k">let</span> combined=<span class="s">false</span>;
 <span class="k">for</span>(<span class="k">const</span> glyph of report.glyphs){<span class="k">if</span>(glyph.cluster[<span class="s">1</span>]-glyph.cluster[<span class="s">0</span>]&gt;<span class="s">1</span>)combined=<span class="s">true</span>;}
 <span class="k">if</span>(!combined)<span class="k">throw</span> Error('<span class="s">No multi-byte or combined source cluster recorded</span>');
 window.textLayout={...report,metrics,passed:<span class="s">true</span>};
 document.getElementById('<span class="s">diagnostics</span>').textContent=JSON.stringify(window.textLayout,<span class="s">null</span>,<span class="s">2</span>);
 document.getElementById('<span class="s">layout-status</span>').textContent='<span class="s">PASS: five sizes, matched fonts/widths, clusters and unchanged shaping after placement/color edits.</span>';
}
<span class="c">/** Show a failure on the page and in the console. */</span>
<span class="k">function</span> failed(error){document.getElementById('<span class="s">layout-status</span>').textContent=String(error);document.getElementById('<span class="s">layout-status</span>').className='<span class="s">failed</span>';console.error(error);}
start().catch(failed);
&lt;/script&gt;&lt;/html&gt;</code></pre></div>
<h2 id="step-9-indexhtml">Step 9 · index.html<a class="anchor" href="#/course/10-text-layout#step-9-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/10/index.html</code> · edit · copy the file</p>
<p>Replaces the <code>&lt;title&gt;Session checkpoint 09&lt;/title&gt;</code> line of <code>lessons/09/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;title&gt;Session checkpoint 10&lt;/title&gt;</code></pre></div>
<p>Replaces the 4 lines from <code>&lt;/head&gt;</code> of <code>lessons/09/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;link data-trunk rel=&quot;<span class="s">copy-dir</span>&quot; href=&quot;<span class="s">assets/text</span>&quot; data-target-path=&quot;<span class="s">text</span>&quot;&gt;
    &lt;link data-trunk rel=&quot;<span class="s">copy-file</span>&quot; href=&quot;<span class="s">assets/text-layout.html</span>&quot;&gt;
  &lt;/head&gt;
  &lt;body&gt;
    <span class="c">&lt;!-- wgpu wraps this canvas as the Surface; tabindex=&quot;0&quot; lets it take keyboard focus. --&gt;</span>
    &lt;canvas id=&quot;<span class="s">canvas</span>&quot; tabindex=&quot;<span class="s">0</span>&quot;&gt;&lt;/canvas&gt;
    &lt;a href=&quot;<span class="s">./text-layout.html</span>&quot; style=&quot;<span class="s">position: fixed; right: 12px; top: 12px; color: white;</span>&quot;&gt;Inspect shaped text&lt;/a&gt;
    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 10&lt;/output&gt;</code></pre></div>
<p>Replaces the <code>document.getElementById(&#39;status&#39;).textContent…</code> line of <code>lessons/09/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        document.getElementById('status').textContent = 'Checkpoint 10 · ' + state.objects + ' objects · ' + state.width + '×' + state.height;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/10-text-layout#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/10/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: The text specimen page compares five shaped text sizes against browser text; status: <strong>PASS</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/10-text-layout.png" alt="Checkpoint 10: the reference page shapes one string at five sizes; the browser row behind each specimen has the same width, and the report lists every glyph with its cluster, advance and baseline." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>The specimen widths differ: font bytes, size or kerning settings differ.</li>
<li>Glyph order is wrong: character order replaces the shaped glyph sequence.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/10-text-layout#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/10/src/engine/
├── gpu/
│   ├── arena.rs
│   ├── backdrop.rs
│   ├── buffers.rs
│   ├── cloud.rs
│   ├── frame.rs
│   ├── glyphs.rs
│   ├── instance.rs
│   ├── lod.rs
│   ├── mod.rs
│   ├── objects.rs
│   ├── segments.rs
│   ├── splat.rs
│   ├── targets.rs
│   ├── text_outline.rs
│   ├── upload.rs
│   └── view.rs
├── pipelines/
│   ├── layouts.rs
│   └── mod.rs
├── mod.rs  ~
├── performance.rs  +
└── text.rs  +</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/10/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/10-text-layout#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/11-text-rendering">11 · Text rendering</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/10-text-layout#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 10: the canvas itself is unchanged.</p>
<p><a href="/session/docs/course/docs/screenshots/10.png"><img src="/session/docs/course/docs/screenshots/10.png" alt="Full viewer result for 10 text layout" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-assetstextofltxt",text:"Step 1 · assets/text/OFL.txt"},{level:2,id:"step-2-assetstextreadmemd",text:"Step 2 · assets/text/README.md"},{level:2,id:"step-3-srcengineperformancers",text:"Step 3 · src/engine/performance.rs"},{level:2,id:"step-4-srcenginetextrs",text:"Step 4 · src/engine/text.rs"},{level:2,id:"step-5-srcenginemodrs",text:"Step 5 · src/engine/mod.rs"},{level:2,id:"step-6-srctext_layoutrs",text:"Step 6 · src/text_layout.rs"},{level:2,id:"step-7-srclibrs",text:"Step 7 · src/lib.rs"},{level:2,id:"step-8-assetstext-layouthtml",text:"Step 8 · assets/text-layout.html"},{level:2,id:"step-9-indexhtml",text:"Step 9 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
