const s={title:"37 · Attribute features in red and a one-row command dock",html:`<h1 id="37-attribute-features-in-red-and-a-one-row-command-dock">37 · Attribute features in red and a one-row command dock<a class="anchor" href="#/course/37-command-dock#37-attribute-features-in-red-and-a-one-row-command-dock" aria-label="Link to this section">#</a></h1>
<p>Attribute features draw in red at twice the default pen, centroids are no longer a feature, and the command dock starts collapsed with its options before the field.</p>
<h2 id="step-1-srcappwalkmodrs">Step 1 · src/app/walk/mod.rs<a class="anchor" href="#/course/37-command-dock#step-1-srcappwalkmodrs" aria-label="Link to this section">#</a></h2>
<p>Outline, axis and section draw red at 2 px and a one-point section is a 12 px red dot; centroid is gone.</p>
<p><code>lessons/37/src/app/walk/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>use session_rust::AABB;</code> in <code>lessons/36/src/app/walk/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> session_rust::Color;</code></pre></div>
<p>Replaces the line <code>const ATTRIBUTE_FEATURES: [&amp;str; 4] = [&quot;outline&quot;, &quot;axis&quot;,…</code> in <code>lessons/36/src/app/walk/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Feature types \`Attributes On\` draws.</span>
<span class="k">const</span> ATTRIBUTE_FEATURES: [&amp;str; <span class="s">3</span>] = [&quot;<span class="s">outline</span>&quot;, &quot;<span class="s">axis</span>&quot;, &quot;<span class="s">section</span>&quot;];
<span class="k">const</span> ATTRIBUTE_LINE_PX: f64 = <span class="s">2</span>.<span class="s">0</span>; <span class="c">// twice the 1 px pen</span>
<span class="k">const</span> ATTRIBUTE_DOT_PX: f64 = <span class="s">12</span>.<span class="s">0</span>; <span class="c">// twice the 6 px point</span>

<span class="c">/// Draw an element's features, red and thick, into its own row.</span></code></pre></div>
<p>Replaces the line <code>let r = if let (1, Some(p)) = (outline.point_count(), out…</code> in <code>lessons/36/src/app/walk/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="c">// one point is a dot, more is a polyline</span>
            <span class="k">let</span> r = <span class="k">if</span> <span class="k">let</span> (<span class="s">1</span>, Some(<span class="k">mut</span> p)) = (outline.point_count(), outline.get_point(<span class="s">0</span>)) {
                p.pointcolor = Color::red();
                p.width = ATTRIBUTE_DOT_PX;
                walk_point(w.glyph, &amp;p, cx.row)
            } <span class="k">else</span> {
                <span class="k">let</span> <span class="k">mut</span> outline = outline.clone();
                outline.linecolor = Color::red();
                outline.width = ATTRIBUTE_LINE_PX;
                walk_polyline(w.seg, &amp;outline, cx.row)</code></pre></div>
<p>Replaces the 7 lines from <code>let centroid = Polyline::new(vec![Point::new(5.0, 5.0, 5.…</code> in <code>lessons/36/src/app/walk/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> dot = Polyline::new(vec![Point::new(<span class="s">5</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>)]);
        element.add_feature(ElementFeature::new(&quot;<span class="s">section</span>&quot;, -<span class="s">1</span>, vec![dot], &quot;<span class="s">section</span>&quot;));</code></pre></div>
<h2 id="step-2-srcappuirs">Step 2 · src/app/ui.rs<a class="anchor" href="#/course/37-command-dock#step-2-srcappuirs" aria-label="Link to this section">#</a></h2>
<p>The dock starts as one row, options sit before the field, and <code>+</code> opens the history.</p>
<p><code>lessons/37/src/app/ui.rs</code> · edit · type this</p>
<p>Replaces the line <code>command_collapsed: bool,</code> in <code>lessons/36/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    command_expanded: bool,                         <span class="c">// history shown above the field</span></code></pre></div>
<p>Replaces the line <code>let panel = if model.command_collapsed {</code> in <code>lessons/36/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> panel = <span class="k">if</span> !model.command_expanded {
        egui::Panel::bottom(&quot;<span class="s">command-line-collapsed</span>&quot;).exact_size(<span class="k">if</span> polyline_options {
            <span class="s">58</span>.<span class="s">0</span>
        } <span class="k">else</span> {
            <span class="s">30</span>.<span class="s">0</span></code></pre></div>
<p>Replaces the line <code>.inner_margin(6),</code> in <code>lessons/36/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                .inner_margin(egui::Margin::symmetric(<span class="s">6</span>, <span class="s">4</span>)),</code></pre></div>
<p>Replaces the line <code>ui.max_rect().top() - 6.0,</code> in <code>lessons/36/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                ui.max_rect().top() - <span class="s">4</span>.<span class="s">0</span>,</code></pre></div>
<p>Replaces the line <code>if !model.command_collapsed {</code> in <code>lessons/36/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="c">// the history above the field</span>
            <span class="k">if</span> model.command_expanded {</code></pre></div>
<p>Added after the line <code>.sum();</code> in <code>lessons/36/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="c">// options before the field</span>
                <span class="k">for</span> name <span class="k">in</span> inline_options {
                    <span class="k">let</span> label = name.split_once('<span class="s"> </span>').map_or(*name, |(_, option)| option);
                    <span class="k">let</span> selected = model.command.trim().eq_ignore_ascii_case(name)
                        || (model.command.ends_with('<span class="s"> </span>')
                            &amp;&amp; model.command.split_whitespace().count() == <span class="s">1</span>
                            &amp;&amp; Some(name) == inline_options.first());
                    <span class="k">let</span> option = ui.selectable_label(selected, label);
                    record(controls, &amp;format!(&quot;<span class="s">command/option/</span>{<span class="s">label</span>}&quot;), label, &amp;option);
                    <span class="k">if</span> option.clicked() {
                        <span class="k">let</span> (text, run) = <span class="k">crate</span>::app::command::accept(name);
                        <span class="k">if</span> run {
                            *command = Some(text);
                            model.command.clear();
                        } <span class="k">else</span> {
                            model.command = text;
                        }
                        model.completion_visible = <span class="s">false</span>;
                        model.inline_suffix = <span class="s">false</span>;
                        model.focus_command = <span class="s">true</span>;
                    }
                }</code></pre></div>
<p>Delete the 21 lines from <code>for name in inline_options {</code> in <code>lessons/36/src/app/ui.rs</code>.</p>
<p>Replaces the line <code>.button(if model.command_collapsed { &quot;+&quot; } else { &quot;−&quot; })</code> in <code>lessons/36/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    .button(<span class="k">if</span> model.command_expanded { &quot;<span class="s">−</span>&quot; } <span class="k">else</span> { &quot;<span class="s">+</span>&quot; })</code></pre></div>
<p>Replaces the line <code>model.command_collapsed = !model.command_collapsed;</code> in <code>lessons/36/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    model.command_expanded = !model.command_expanded;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/37-command-dock#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/37/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Type <code>Attributes On</code>: features are red and thicker; the command dock is a single row until you press <code>+</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/37-command-dock#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/capstone">Capstone</a>: the viewer as it is today.</p>
`,toc:[{level:2,id:"step-1-srcappwalkmodrs",text:"Step 1 · src/app/walk/mod.rs"},{level:2,id:"step-2-srcappuirs",text:"Step 2 · src/app/ui.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"next",text:"Next"}]};export{s as default};
