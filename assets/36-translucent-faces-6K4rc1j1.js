const s={title:"36 · Translucent faces and the Opacity command",html:`<h1 id="36-translucent-faces-and-the-opacity-command">36 · Translucent faces and the Opacity command<a class="anchor" href="#/course/36-translucent-faces#36-translucent-faces-and-the-opacity-command" aria-label="Link to this section">#</a></h1>
<p>Elements open at 0.7 opacity so the features inside them show, <code>Opacity 0..1</code> sets it, and hidden ink dims through the glass instead of vanishing.</p>
<h2 id="step-1-srcappcommandrs">Step 1 · src/app/command.rs<a class="anchor" href="#/course/36-translucent-faces#step-1-srcappcommandrs" aria-label="Link to this section">#</a></h2>
<p><code>Opacity 0..1</code> is parsed, listed and offered with four presets.</p>
<p><code>lessons/36/src/app/command.rs</code> · edit · type this</p>
<p>Added after the line <code>Layers(Option&lt;bool&gt;),</code> in <code>lessons/35/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Opacity(f32),             <span class="c">// face alpha, 0 x-ray to 1 solid</span></code></pre></div>
<p>Added after the line <code>&quot;Attributes (On Off): draw or remove the element features…</code> in <code>lessons/35/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        }
        &quot;<span class="s">opacity</span>&quot; =&gt; {
            &quot;<span class="s">Opacity 0..1: how solid the faces are · 0 is x-ray, 1 solid · Example: Opacity 0.5</span>&quot;</code></pre></div>
<p>Added after the line <code>},</code> in <code>lessons/35/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">opacity</span>&quot; =&gt; {
            <span class="k">let</span> value = number(rest.first().copied(), &quot;<span class="s">Opacity 0.5</span>&quot;)?;
            (<span class="s">0</span>.<span class="s">0</span>..=<span class="s">1</span>.<span class="s">0</span>)
                .contains(&amp;value)
                .then_some(Command::Opacity(value <span class="k">as</span> f32))
                .ok_or_else(|| &quot;<span class="s">Opacity takes a value from 0 to 1</span>&quot;.to_string())
        }</code></pre></div>
<p>Added after the line <code>&quot;attributes&quot; =&gt; &amp;[&quot;Attributes On&quot;, &quot;Attributes Off&quot;],</code> in <code>lessons/35/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">opacity</span>&quot; =&gt; &amp;[&quot;<span class="s">Opacity 1</span>&quot;, &quot;<span class="s">Opacity 0.7</span>&quot;, &quot;<span class="s">Opacity 0.4</span>&quot;, &quot;<span class="s">Opacity 0</span>&quot;],</code></pre></div>
<p>Added after the line <code>&quot;Object&quot;,</code> in <code>lessons/35/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">Opacity</span>&quot;,</code></pre></div>
<p>Added after the line <code>assert_eq!(parse(&quot;Attributes&quot;), Ok(Command::Attributes(No…</code> in <code>lessons/35/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        assert_eq!(parse(&quot;<span class="s">Opacity 0.5</span>&quot;), Ok(Command::Opacity(<span class="s">0</span>.<span class="s">5</span>)));
        assert!(parse(&quot;<span class="s">Opacity 2</span>&quot;).is_err());
        assert!(parse(&quot;<span class="s">Opacity</span>&quot;).is_err());</code></pre></div>
<h2 id="step-2-srcstaters">Step 2 · src/state.rs<a class="anchor" href="#/course/36-translucent-faces#step-2-srcstaters" aria-label="Link to this section">#</a></h2>
<p>Elements arrive at 0.7 unless a knob or command already chose an opacity.</p>
<p><code>lessons/36/src/state.rs</code> · edit · type this</p>
<p>Added after the line <code>const SPIN_STEP: f32 = 0.004;</code> in <code>lessons/35/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">const</span> ELEMENT_OPACITY: f32 = <span class="s">0</span>.<span class="s">7</span>; <span class="c">// default element opacity</span></code></pre></div>
<p>Added after the line <code>show_selected_names: bool,</code> in <code>lessons/35/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    opacity_chosen: bool,                                   <span class="c">// an Opacity command was given</span></code></pre></div>
<p>Added after the line <code>show_selected_names: true,</code> in <code>lessons/35/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            opacity_chosen: <span class="s">false</span>,</code></pre></div>
<p>Added after the line <code>self.annotate_document(first_row);</code> in <code>lessons/35/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.dim_elements(first_row);
        <span class="c">// the layer panel lists the new rows</span></code></pre></div>
<p>Added after the line <code>show</code> in <code>lessons/35/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    }

    <span class="c">/// Make elements a little see-through the first time they arrive.</span>
    <span class="k">fn</span> dim_elements(&amp;<span class="k">mut</span> <span class="k">self</span>, first_row: usize) {
        <span class="c">// an opacity was already chosen</span>
        <span class="k">if</span> <span class="k">self</span>.opacity_chosen || <span class="k">self</span>.gpu.view.opacity &lt; <span class="s">1</span>.<span class="s">0</span> {
            <span class="k">return</span>;
        }

        <span class="c">// does the new document have elements?</span>
        <span class="k">let</span> elements = (first_row..<span class="k">self</span>.scene.object_count()).any(|row| {
            matches!(
                <span class="k">self</span>.scene.geometry(row <span class="k">as</span> u32),
                Some(session_rust::Geometry::Element(_))
            )
        });

        <span class="k">if</span> elements {
            <span class="k">self</span>.gpu.view.opacity = ELEMENT_OPACITY;
            <span class="k">self</span>.opacity_chosen = <span class="s">true</span>;
        }
    }

    <span class="c">/// Opacity &lt;value&gt;: 0 is x-ray, 1 is solid.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_opacity(&amp;<span class="k">mut</span> <span class="k">self</span>, value: f32) {
        <span class="k">self</span>.gpu.view.opacity = value.clamp(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>);
        <span class="k">self</span>.opacity_chosen = <span class="s">true</span>;
        <span class="k">self</span>.touch();</code></pre></div>
<h2 id="step-3-srcstateeditrs">Step 3 · src/state/edit.rs<a class="anchor" href="#/course/36-translucent-faces#step-3-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>The selection label follows a moved object, and <code>Opacity</code> is an edit command.</p>
<p><code>lessons/36/src/state/edit.rs</code> · edit · type this</p>
<p>Added after the line <code>}</code> in <code>lessons/35/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.update_label();</code></pre></div>
<p>Added after the line <code>self.place_gizmo(Some(active.row));</code> in <code>lessons/35/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.update_label();</code></pre></div>
<p>Added after the line <code>self.place_gizmo(Some(active.row));</code> in <code>lessons/35/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.update_label();</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/35/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Command::Opacity(value) =&gt; {
                <span class="k">self</span>.set_opacity(value);
                Ok(format!(&quot;<span class="s">Opacity </span>{<span class="s">value</span>}&quot;))
            }</code></pre></div>
<p>Added after the line <code>self.place_gizmo(Some(row));</code> in <code>lessons/35/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.update_label();</code></pre></div>
<h2 id="step-4-srcshaderstrianglewgsl">Step 4 · src/shaders/triangle.wgsl<a class="anchor" href="#/course/36-translucent-faces#step-4-srcshaderstrianglewgsl" aria-label="Link to this section">#</a></h2>
<p>A translucent solid drops its back faces, so it reads as one sheet of glass.</p>
<p><code>lessons/36/src/shaders/triangle.wgsl</code> · edit · type this</p>
<p>Added after the line <code>let front = raster_front != (in.mirrored != 0u);</code> in <code>lessons/35/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">    // translucent solids drop their back faces</span>
    <span class="k">if</span> (!front &amp;&amp; in.closed != <span class="s">0u</span> &amp;&amp; line.opacity &gt; <span class="s">0.0</span> &amp;&amp; line.opacity &lt; <span class="s">1.0</span>) {
        <span class="k">discard</span>;
    }
<span class="c">    // flat normal from screen derivatives, when the mesh has none</span></code></pre></div>
<h2 id="step-5-srcshadersink_visibilitywgsl">Step 5 · src/shaders/ink_visibility.wgsl<a class="anchor" href="#/course/36-translucent-faces#step-5-srcshadersink_visibilitywgsl" aria-label="Link to this section">#</a></h2>
<p><code>through_glass</code> returns how much ink behind a translucent face still shows.</p>
<p><code>lessons/36/src/shaders/ink_visibility.wgsl</code> · edit · type this</p>
<p>Added after the line <code>}</code> in <code>lessons/35/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Hidden ink behind a translucent face is dimmed to 1 - opacity, instead of being hidden outright.</span>
<span class="k">fn</span> through_glass(hidden: <span class="k">bool</span>) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> glass = line.opacity &gt; <span class="s">0.0</span> &amp;&amp; line.opacity &lt; <span class="s">1.0</span>;
    <span class="k">return</span> select(<span class="s">1.0</span>, select(<span class="s">0.0</span>, <span class="s">1.0</span> - line.opacity, glass), hidden);
}</code></pre></div>
<h2 id="step-6-srcshadersglyphwgsl">Step 6 · src/shaders/glyph.wgsl<a class="anchor" href="#/course/36-translucent-faces#step-6-srcshadersglyphwgsl" aria-label="Link to this section">#</a></h2>
<p>Glyph coverage is scaled by <code>through_glass</code>.</p>
<p><code>lessons/36/src/shaders/glyph.wgsl</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>let alpha = coverage(in);</code> in <code>lessons/35/src/shaders/glyph.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> hidden = !ink_disc_visible(in.pos.xy, in.centre, in.depth, sample);
    <span class="k">let</span> alpha = coverage(in) * through_glass(hidden);

    <span class="k">if</span> (alpha &lt;= <span class="s">0.0</span>) {</code></pre></div>
<h2 id="step-7-srcshadersribbonwgsl">Step 7 · src/shaders/ribbon.wgsl<a class="anchor" href="#/course/36-translucent-faces#step-7-srcshadersribbonwgsl" aria-label="Link to this section">#</a></h2>
<p>Ribbon coverage is scaled by <code>through_glass</code>.</p>
<p><code>lessons/36/src/shaders/ribbon.wgsl</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>let alpha = coverage(in);</code> in <code>lessons/35/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> hidden = !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags &amp; FLAG_SMOOTH) != <span class="s">0u</span>);
    <span class="k">let</span> alpha = coverage(in) * through_glass(hidden);

    <span class="k">if</span> (alpha &lt;= <span class="s">0.0</span>) {</code></pre></div>
<h2 id="step-8-srcshadersspherewgsl">Step 8 · src/shaders/sphere.wgsl<a class="anchor" href="#/course/36-translucent-faces#step-8-srcshadersspherewgsl" aria-label="Link to this section">#</a></h2>
<p>Sphere coverage is scaled by <code>through_glass</code>.</p>
<p><code>lessons/36/src/shaders/sphere.wgsl</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>let alpha = coverage(in);</code> in <code>lessons/35/src/shaders/sphere.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> hidden = !ink_disc_visible(in.pos.xy, in.centre, in.depth, sample);
    <span class="k">let</span> alpha = coverage(in) * through_glass(hidden);

    <span class="k">if</span> (alpha &lt;= <span class="s">0.0</span>) {</code></pre></div>
<h2 id="step-9-srcappuirs">Step 9 · src/app/ui.rs<a class="anchor" href="#/course/36-translucent-faces#step-9-srcappuirs" aria-label="Link to this section">#</a></h2>
<p>With the command box closed, phone keys are the viewport&#39;s own bindings.</p>
<p><code>lessons/36/src/app/ui.rs</code> · edit · type this</p>
<p>Replaces the lines from <code>pub fn agent(&amp;mut self, event: super::agent::AgentEvent) {</code> in <code>lessons/35/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Feed the hidden input's typing into the field; returns keys for the viewport.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">pub</span> <span class="k">fn</span> agent(&amp;<span class="k">mut</span> <span class="k">self</span>, event: super::agent::AgentEvent) -&gt; Vec&lt;String&gt; {
        <span class="k">use</span> super::agent::AgentEvent;
        <span class="k">let</span> id = egui::Id::new(&quot;<span class="s">command-input</span>&quot;);
        <span class="k">let</span> (open, empty) = MODEL.with_borrow(|m| (m.command_open, m.command.is_empty()));

        <span class="k">match</span> &amp;event {
            AgentEvent::Text(value) <span class="k">if</span> !open =&gt; {
                <span class="k">let</span> shared = <span class="k">self</span>
                    .agent_value
                    .chars()
                    .zip(value.chars())
                    .take_while(|(a, b)| a == b)
                    .count();
                <span class="k">let</span> keys = value.chars().skip(shared).map(String::from).collect();
                <span class="k">self</span>.agent_value.clear();
                super::agent::sync(&quot;&quot;);
                <span class="k">return</span> keys;
            }
            AgentEvent::Key(egui::Key::Enter) <span class="k">if</span> open &amp;&amp; empty =&gt; {
                super::feedback::command_line(<span class="s">false</span>);
                <span class="k">self</span>.context.memory_mut(|memory| memory.surrender_focus(id));
                super::feedback::status(&quot;<span class="s">Keys go to the viewer · type : for the command line</span>&quot;);
                <span class="k">return</span> Vec::new();
            }
            _ =&gt; {}
        }</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/35/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        Vec::new()</code></pre></div>
<p>Added after the line <code>.font(egui::FontId::proportional(14.0))</code> in <code>lessons/35/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                        .vertical_align(egui::Align::Center)</code></pre></div>
<h2 id="step-10-srclibrs">Step 10 · src/lib.rs<a class="anchor" href="#/course/36-translucent-faces#step-10-srclibrs" aria-label="Link to this section">#</a></h2>
<p>Keys the agent hands back are fed to the input handler.</p>
<p><code>lessons/36/src/lib.rs</code> · edit · type this</p>
<p>Replaces the line <code>ui.agent(event);</code> in <code>lessons/35/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    <span class="k">for</span> key <span class="k">in</span> ui.agent(event) {
                        <span class="k">self</span>.input
                            .key(state, winit::keyboard::Key::Character(key.as_str()));
                    }</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/36-translucent-faces#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/36/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Load elements: faces are see-through and the features inside show dimmed; type <code>Opacity 1</code> for solid, <code>Opacity 0</code> for x-ray.</p>
<h2 id="next">Next<a class="anchor" href="#/course/36-translucent-faces#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/37-command-dock">37 · Attribute features in red and a one-row command dock</a></p>
`,toc:[{level:2,id:"step-1-srcappcommandrs",text:"Step 1 · src/app/command.rs"},{level:2,id:"step-2-srcstaters",text:"Step 2 · src/state.rs"},{level:2,id:"step-3-srcstateeditrs",text:"Step 3 · src/state/edit.rs"},{level:2,id:"step-4-srcshaderstrianglewgsl",text:"Step 4 · src/shaders/triangle.wgsl"},{level:2,id:"step-5-srcshadersink_visibilitywgsl",text:"Step 5 · src/shaders/ink_visibility.wgsl"},{level:2,id:"step-6-srcshadersglyphwgsl",text:"Step 6 · src/shaders/glyph.wgsl"},{level:2,id:"step-7-srcshadersribbonwgsl",text:"Step 7 · src/shaders/ribbon.wgsl"},{level:2,id:"step-8-srcshadersspherewgsl",text:"Step 8 · src/shaders/sphere.wgsl"},{level:2,id:"step-9-srcappuirs",text:"Step 9 · src/app/ui.rs"},{level:2,id:"step-10-srclibrs",text:"Step 10 · src/lib.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"next",text:"Next"}]};export{s as default};
