const e={title:"Adding a command line",html:`<h1 id="adding-a-command-line">Adding a command line<a class="anchor" href="#/course/extend-command-line#adding-a-command-line" aria-label="Link to this section">#</a></h1>
<p>Use the <a href="#/course/command-line-walkthrough">screenshot walkthrough</a>, <a href="#/course/23-geometry-commands">geometry-command code lesson</a>, and <a href="#/course/27-egui-interface">egui interface lesson</a>. The design below is historical; the maintained interface uses egui rather than DOM inputs.</p>
<div class="note"><p class="note-title">Built in lesson 21</p><p>This was the design; <code>src/app/command.rs</code> parses and <code>State::run_command</code> dispatches, and
<a href="#/course/21-editing">lesson 21</a> teaches them. The parser ended up in <code>app/</code>, not <code>state/</code>, so
what a line MEANS can be tested without a window or a device.</p>
</div>
<h2 id="the-shape-of-the-finished-thing">The shape of the finished thing<a class="anchor" href="#/course/extend-command-line#the-shape-of-the-finished-thing" aria-label="Link to this section">#</a></h2>
<ul>
<li>An <code>&lt;input&gt;</code> and a <code>&lt;pre&gt;</code> in <code>index.html</code>, a keydown listener in <code>src/app/input.rs</code>, two <code>Msg</code> variants, a table in a new <code>src/state/command.rs</code>, one named action per command on <code>State</code>.</li>
<li>Commands are <code>pub fn</code> on <code>State</code> beside <code>hide_selected</code> (<code>src/state.rs</code>) and <code>escape_selection</code>; the typed line picks one, never mutates a document.</li>
<li>Every editing command opens and closes a kernel transaction (<code>Session::begin</code>, <code>session_rust/src/session.rs</code>). No viewer-side undo stack: two stacks, two answers on undo.</li>
<li>Rubber band and snap marker are a <code>SegmentLane</code>/<code>GlyphLane</code> pair in <code>Gpu</code>, not a bespoke renderer.</li>
</ul>
<h2 id="the-field-is-markup-not-rust-created-dom">The field is markup, not Rust-created DOM<a class="anchor" href="#/course/extend-command-line#the-field-is-markup-not-rust-created-dom" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>&lt;div id=&quot;<span class="s">viewer-command</span>&quot;&gt;
 &lt;pre id=&quot;<span class="s">viewer-command-log</span>&quot; aria-live=&quot;<span class="s">polite</span>&quot;&gt;&lt;/pre&gt;
 &lt;input id=&quot;<span class="s">viewer-command-input</span>&quot; autocomplete=&quot;<span class="s">off</span>&quot; autocapitalize=&quot;<span class="s">off</span>&quot; spellcheck=&quot;<span class="s">false</span>&quot;&gt;
&lt;/div&gt;</code></pre></div>
<ul>
<li>Beside <code>#viewer-status</code> (<code>index.html:54</code>); it and <code>#viewer-error</code> are markup Rust only <em>finds</em> (<code>feedback.rs:15</code>). Nothing under <code>src/</code> creates an element; one would put layout in a string literal.</li>
<li>No <code>pointer-events:none</code> (the status line has it): the input must take clicks or the mouse can never focus it.</li>
<li><code>position:fixed; bottom:0</code>, z-index under <code>#viewer-docs</code>&#39;s 10 (<code>index.html:34</code>): the folded-corner link stays reachable.</li>
<li>Overlay the canvas, never shrink it (<code>100vw</code>/<code>100dvh</code>, <code>index.html:28</code>): shrinking fires <code>State::resize</code> (<code>src/state.rs</code>) per show/hide, re-uploading controls and re-targeting every lane.</li>
</ul>
<h2 id="keystrokes-the-browser-already-arbitrates-so-nothing-needs-a-mode-flag">Keystrokes: the browser already arbitrates, so nothing needs a mode flag<a class="anchor" href="#/course/extend-command-line#keystrokes-the-browser-already-arbitrates-so-nothing-needs-a-mode-flag" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>WindowEvent::KeyboardInput { event,.. } =&gt; {
 viewer_focused
 &amp;&amp; event.state == ElementState::Pressed
 &amp;&amp; !event.repeat
 &amp;&amp; <span class="k">self</span>.input.key(state, event.logical_key.as_ref)
}</code></pre></div>
<ul>
<li><code>viewer_focused</code> (<code>src/lib.rs:309-321</code>) is <code>active_element.id == &quot;canvas&quot;</code>. With the field focused it is false, <code>Input::key</code> never runs, and <code>f</code>, <code>h</code>, <code>[</code>, <code>]</code>, <code>1</code>-<code>7</code> type characters.</li>
<li>So <code>Input::key</code> (<code>src/app/input.rs</code>) needs no guard, boolean or mode enum: each is a weaker copy of the browser&#39;s decision.</li>
</ul>
<h2 id="the-listener-owns-the-field-one-message-carries-the-line">The listener owns the field; one message carries the line<a class="anchor" href="#/course/extend-command-line#the-listener-owns-the-field-one-message-carries-the-line" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">struct</span> CommandLine {
 input: web_sys::HtmlInputElement,
 keydown: wasm_bindgen::closure::Closure&lt;<span class="k">dyn</span> FnMut(web_sys::KeyboardEvent)&gt;,
 history: std::rc::Rc&lt;std::cell::RefCell&lt;History&gt;&gt;, <span class="c">// lines, cursor, stashed buffer</span>
}
<span class="k">impl</span> CommandLine {
 <span class="k">pub</span> <span class="k">fn</span> new(proxy: EventLoopProxy&lt;<span class="k">crate</span>::Msg&gt;) -&gt; Result&lt;<span class="k">Self</span>, wasm_bindgen::JsValue&gt;;
}
<span class="k">impl</span> Drop <span class="k">for</span> CommandLine { <span class="c">/* remove_event_listener_with_callback */</span> }</code></pre></div>
<ul>
<li>Shaped on <code>PointerCancellation</code> (<code>src/app/input.rs</code>): own element and closure, detach in <code>Drop</code> (<code>input.rs</code>), so JavaScript cannot hold a handle into freed wasm memory.</li>
<li>Install in <code>App::resumed</code> beside the pointer listener (<code>src/lib.rs</code>), another <code>proxy.clone</code>.</li>
<li>Two <code>Msg</code> variants (<code>src/lib.rs</code>), two <code>user_event</code> arms:</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>Command(String),
CommandCancel,
Msg::Command(line) =&gt; state.command(&amp;line),
Msg::CommandCancel =&gt; state.command_cancel,</code></pre></div>
<ul>
<li>The picture follows by itself: <code>user_event</code> ends in <code>request_if_needed</code> (<code>src/lib.rs</code>), every named action in <code>State::touch</code> (<code>src/state.rs:242</code>).</li>
<li><code>State</code> lives inside the event loop (<code>src/lib.rs</code>), so a DOM callback&#39;s one path to it is the <code>EventLoopProxy</code> (<code>loader::post</code>).</li>
<li>Not a <code>#[wasm_bindgen]</code> export: <code>reload_scene</code> is one because the embedding page calls it (<code>index.html:76</code>); the inline script stays lifetime wiring.</li>
</ul>
<h2 id="three-web-sys-features-or-step-4-does-not-compile">Three web-sys features, or step 4 does not compile<a class="anchor" href="#/course/extend-command-line#three-web-sys-features-or-step-4-does-not-compile" aria-label="Link to this section">#</a></h2>
<ul>
<li><code>Cargo.toml:24-43</code> has <code>Document</code>, <code>Window</code>, <code>Element</code>, <code>HtmlCanvasElement</code>, <code>EventTarget</code>, <code>Event</code>, and lacks:</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>&quot;HtmlInputElement&quot;, <span class="c">#.value /.set_value</span>
&quot;KeyboardEvent&quot;, <span class="c">#.key /.prevent_default</span>
&quot;HtmlElement&quot;, <span class="c"># canvas.focus, handing focus back</span></code></pre></div>
<ul>
<li>web-sys is gated per type: without the gate the type does not exist and the error lands far from the cause.</li>
</ul>
<h2 id="escape-in-the-order-a-user-expects-it">Escape, in the order a user expects it<a class="anchor" href="#/course/extend-command-line#escape-in-the-order-a-user-expects-it" aria-label="Link to this section">#</a></h2>
<ul>
<li>The listener branches only on whether the field is empty, so it never asks Rust.</li>
<li>Not empty: clear the field and send <code>Msg::CommandCancel</code>. Empty: send it and <code>canvas.focus</code>.</li>
<li>The next Escape reaches winit and <code>escape_selection</code> (<code>src/app/input.rs</code>). Chain: text, tool, focus, selection; two presses at most to get out.</li>
<li><code>command_cancel</code> is idempotent because it has two senders; a double cancel shows only under a fast second press.</li>
</ul>
<h2 id="one-table-never-three">One table, never three<a class="anchor" href="#/course/extend-command-line#one-table-never-three" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">struct</span> Cmd {
 name: &amp;'static str,
 aliases: &amp;'static [&amp;'static str],
 args: &amp;'static str, <span class="c">// &quot;[sx sy sz]&quot; - shown by \`help\` and by completion</span>
 run: fn(&amp;<span class="k">mut</span> State, &amp;Args) -&gt; Reply,
}
<span class="k">const</span> CMDS: &amp;[Cmd] = &amp;[ <span class="c">/*... */</span> ];
<span class="k">fn</span> lookup(verb: &amp;str) -&gt; Option&lt;&amp;'static Cmd&gt;; <span class="c">// pure, natively testable</span></code></pre></div>
<ul>
<li>Dispatcher, <code>help</code> and completion all read <code>CMDS</code>. The archive&#39;s three lists disagree already: <code>move</code>, <code>pt</code>, <code>ln</code>, <code>crv</code>, <code>cylinder</code>, <code>polyline</code>, <code>delete</code>, <code>rm</code> dispatch without completing; <code>help</code> omits <code>curve</code> and <code>move</code>.</li>
<li><code>lookup</code> is a free function over a const slice, so it tests in <code>cargo test</code> without a window.</li>
</ul>
<h2 id="tokenising">Tokenising<a class="anchor" href="#/course/extend-command-line#tokenising" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">let</span> line = line.trim;
<span class="k">if</span> line.is_empty { <span class="k">return</span>; }
<span class="k">self</span>.log(&amp;format!(&quot;<span class="s">&gt; </span>{<span class="s">line</span>}&quot;));
<span class="k">let</span> <span class="k">mut</span> parts = line.split_whitespace;
<span class="k">let</span> verb = parts.next.unwrap.to_ascii_lowercase;</code></pre></div>
<ul>
<li>Whitespace is the only separator; only the verb is lowercased, because guids and document names are case-sensitive everywhere else.</li>
<li>No quoting or escaping, so a name with a space cannot be typed; state it on <code>help</code>. Revisit when a command takes a file path.</li>
<li>Unknown verb: one line, nothing changes — <code>unknown: &#39;foo&#39; (type &#39;help&#39;)</code>.</li>
</ul>
<h2 id="arguments-missing-and-wrong-are-different-answers">Arguments: &quot;missing&quot; and &quot;wrong&quot; are different answers<a class="anchor" href="#/course/extend-command-line#arguments-missing-and-wrong-are-different-answers" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">enum</span> ArgError { NotANumber(String), OutOfRange(String) }
<span class="k">fn</span> f64_at(parts: &amp;[&amp;str], i: usize) -&gt; Result&lt;Option&lt;f64&gt;, ArgError&gt;;
<span class="k">fn</span> positive(name: &amp;str, v: f64) -&gt; Result&lt;f64, ArgError&gt;;</code></pre></div>
<ul>
<li><code>Ok(None)</code>: absent, default applies. <code>Err(_)</code>: not a number, so the command does nothing and says so.</li>
<li>Cascading defaults stay: one argument to <code>box</code> is a cube (<code>sy</code>, <code>sz</code> default to <code>sx</code>); two is an error, since no reading of &quot;two of three extents&quot; is obviously right.</li>
<li>Validate sign and finiteness in the arm: <code>Session::add_brep</code> refuses only zero faces <em>and</em> zero vertices (<code>session_rust/src/session.rs</code>), so a negative radius sails through.</li>
<li>Report the kernel&#39;s refusals: <code>add_polyline</code> <code>None</code> below 2 points (<code>session.rs:1379</code>), <code>add_nurbscurve</code> below 2 CVs, <code>add_mesh</code> when empty or faceless.</li>
</ul>
<h2 id="the-command-table">The command table<a class="anchor" href="#/course/extend-command-line#the-command-table" aria-label="Link to this section">#</a></h2>
<p>Selection-scoped commands act on <code>Scene::selected</code> (<code>src/app/scene.rs</code>) and report &quot;nothing selected&quot; rather than guessing.</p>
<h3 id="document-and-history">Document and history<a class="anchor" href="#/course/extend-command-line#document-and-history" aria-label="Link to this section">#</a></h3>
<table>
<thead>
<tr>
<th>Command</th>
<th>Arguments</th>
<th>Kernel call</th>
<th>Records</th>
</tr>
</thead>
<tbody><tr>
<td><code>undo</code></td>
<td>—</td>
<td><code>Session::undo</code> (<code>session.rs:1622</code>)</td>
<td>cursor back</td>
</tr>
<tr>
<td><code>redo</code></td>
<td>—</td>
<td><code>Session::redo</code> (<code>session.rs:1629</code>)</td>
<td>cursor forward</td>
</tr>
<tr>
<td><code>hist</code></td>
<td>—</td>
<td><code>History::{depth, can_undo, can_redo}</code> (<code>history.rs:214-224</code>)</td>
<td>reads only</td>
</tr>
<tr>
<td><code>save</code></td>
<td>—</td>
<td><code>file_json_dumps</code> / <code>pb_dumps</code> (<code>session.rs:581</code>, <code>605</code>)</td>
<td>clears history</td>
</tr>
</tbody></table>
<ul>
<li><code>save</code> purges undo by design: both dumps call <code>history.clear</code> first and take <code>&amp;mut self</code> (<code>session.rs:582</code>, <code>606</code>), so it cannot run through a shared <code>&amp;Session</code> and the log line must say the history is gone.</li>
</ul>
<h3 id="solids">Solids<a class="anchor" href="#/course/extend-command-line#solids" aria-label="Link to this section">#</a></h3>
<table>
<thead>
<tr>
<th>Command</th>
<th>Arguments</th>
<th>Kernel call</th>
</tr>
</thead>
<tbody><tr>
<td><code>box</code></td>
<td><code>[sx [sy sz]]</code></td>
<td><code>BRep::create_box</code> (<code>session_rust/src/brep.rs</code>) then <code>Session::add_brep</code> (<code>session.rs:1454</code>)</td>
</tr>
<tr>
<td><code>cyl</code></td>
<td><code>[r h]</code></td>
<td><code>BRep::create_cylinder</code> (<code>session_rust/src/brep.rs</code>)</td>
</tr>
<tr>
<td><code>sphere</code></td>
<td><code>[r]</code></td>
<td><code>BRep::create_sphere</code> (<code>session_rust/src/brep.rs</code>)</td>
</tr>
<tr>
<td><code>cone</code></td>
<td><code>[r h]</code></td>
<td><code>BRep::create_cone</code> (<code>session_rust/src/brep.rs</code>)</td>
</tr>
<tr>
<td><code>pyramid</code></td>
<td><code>[base h]</code></td>
<td><code>BRep::create_pyramid</code> (<code>session_rust/src/brep.rs</code>)</td>
</tr>
<tr>
<td><code>torus</code></td>
<td><code>[R r]</code></td>
<td><code>BRep::create_torus</code> (<code>session_rust/src/brep.rs</code>)</td>
</tr>
</tbody></table>
<ul>
<li><code>create_cone</code>/<code>create_torus</code>, not <code>Primitives::*_surface</code>: the archive&#39;s <code>NurbsSurface</code> versions cost four special cases (delete, undo, redo, snap). A BRep walks the same tail as every other solid.</li>
</ul>
<h3 id="curves">Curves<a class="anchor" href="#/course/extend-command-line#curves" aria-label="Link to this section">#</a></h3>
<table>
<thead>
<tr>
<th>Command</th>
<th>Arguments</th>
<th>Kernel call</th>
</tr>
</thead>
<tbody><tr>
<td><code>line</code></td>
<td><code>[x0 y0 z0 x1 y1 z1]</code>, else interactive</td>
<td><code>Line::from_points</code> (<code>line.rs:79</code>) then <code>Session::add_line</code> (<code>session.rs:1359</code>)</td>
</tr>
<tr>
<td><code>poly</code></td>
<td>interactive</td>
<td><code>Polyline::new</code> (<code>polyline.rs:44</code>) then <code>Session::add_polyline</code> (<code>session.rs:1379</code>)</td>
</tr>
<tr>
<td><code>circle</code></td>
<td><code>[r]</code></td>
<td><code>Primitives::circle</code> (<code>primitives.rs:108</code>) then <code>Session::add_nurbscurve</code> (<code>session.rs:1422</code>)</td>
</tr>
<tr>
<td><code>ellipse</code></td>
<td><code>[a b]</code></td>
<td><code>Primitives::ellipse</code> (<code>primitives.rs:132</code>)</td>
</tr>
<tr>
<td><code>arc</code></td>
<td>interactive, three points</td>
<td><code>Primitives::arc</code> (<code>primitives.rs:156</code>)</td>
</tr>
<tr>
<td><code>curve</code></td>
<td><code>[deg]</code>, interactive control points</td>
<td><code>NurbsCurve::create</code> (<code>nurbscurve.rs:259</code>)</td>
</tr>
<tr>
<td><code>interpcrv</code></td>
<td>interactive through-points</td>
<td><code>NurbsCurve::create_interpolated</code> (<code>nurbscurve.rs:303</code>)</td>
</tr>
</tbody></table>
<h3 id="surfaces-and-meshes">Surfaces and meshes<a class="anchor" href="#/course/extend-command-line#surfaces-and-meshes" aria-label="Link to this section">#</a></h3>
<table>
<thead>
<tr>
<th>Command</th>
<th>Arguments</th>
<th>Kernel call</th>
</tr>
</thead>
<tbody><tr>
<td><code>extrude</code></td>
<td><code>dx dy dz</code>, selected curve</td>
<td><code>Primitives::create_extrusion</code> (<code>primitives.rs:822</code>) then <code>Session::add_nurbssurface</code> (<code>session.rs:1438</code>)</td>
</tr>
<tr>
<td><code>loft</code></td>
<td><code>[deg_v]</code>, selected curves</td>
<td><code>Primitives::create_loft</code> (<code>primitives.rs:1033</code>)</td>
</tr>
<tr>
<td><code>revolve</code></td>
<td><code>[angle]</code>, selected curve</td>
<td><code>Primitives::create_revolve</code> (<code>primitives.rs:1271</code>)</td>
</tr>
<tr>
<td><code>mesh</code></td>
<td><code>[u v]</code>, selected surface</td>
<td><code>Primitives::quad_mesh</code> (<code>primitives.rs:1946</code>) then <code>Session::add_mesh</code> (<code>session.rs:1411</code>)</td>
</tr>
<tr>
<td><code>pipe</code></td>
<td><code>r</code>, selected line</td>
<td><code>Primitives::cylinder_mesh</code> (<code>primitives.rs:392</code>)</td>
</tr>
</tbody></table>
<h3 id="edits">Edits<a class="anchor" href="#/course/extend-command-line#edits" aria-label="Link to this section">#</a></h3>
<table>
<thead>
<tr>
<th>Command</th>
<th>Arguments</th>
<th>Kernel call</th>
<th>Records</th>
</tr>
</thead>
<tbody><tr>
<td><code>move</code></td>
<td><code>dx dy dz</code>, else interactive</td>
<td><code>Xform::translation</code> (<code>xform.rs:139</code>) then <code>Session::set_xform</code> (<code>session.rs:346</code>)</td>
<td><code>Op::Xform</code>, absolute before/after</td>
</tr>
<tr>
<td><code>rotate</code></td>
<td><code>deg [ax ay az]</code></td>
<td><code>Xform::rotation</code> (<code>xform.rs:203</code>)</td>
<td><code>Op::Xform</code></td>
</tr>
<tr>
<td><code>scale</code></td>
<td><code>factor</code></td>
<td><code>Xform::scale_uniform</code> (<code>xform.rs:913</code>)</td>
<td><code>Op::Xform</code></td>
</tr>
<tr>
<td><code>orient</code></td>
<td>interactive, plane to plane</td>
<td><code>Xform::plane_to_plane</code> (<code>xform.rs:721</code>)</td>
<td><code>Op::Xform</code></td>
</tr>
<tr>
<td><code>delete</code></td>
<td>—</td>
<td><code>Session::remove_object</code> (<code>session.rs:1575</code>)</td>
<td><code>Op::Remove(Tombstone)</code></td>
</tr>
<tr>
<td><code>name</code></td>
<td><code>&lt;text&gt;</code></td>
<td><code>Session::replace</code> (<code>session.rs:1589</code>)</td>
<td><code>Op::Replace</code>, absolute snapshots</td>
</tr>
</tbody></table>
<ul>
<li><code>remove_object</code> empties the typed vector, <code>lookup</code>, the xform table, the tree node with its subtree and the graph node with its edges at once, and records the tombstone undo restores from (<code>session.rs:1575-1586</code>, <code>history.rs:46-58</code>). The archive removed three tables by hand and left tree and graph nodes dangling.</li>
<li><code>Session::replace</code> is the only recorded reshape; mutating through <code>lookup</code> compiles and is explicitly not recorded (<code>session.rs:1589-1604</code>), so undo cannot see it.</li>
</ul>
<h3 id="queries-read-only-no-transaction">Queries — read-only, no transaction<a class="anchor" href="#/course/extend-command-line#queries-read-only-no-transaction" aria-label="Link to this section">#</a></h3>
<table>
<thead>
<tr>
<th>Command</th>
<th>Arguments</th>
<th>Kernel call</th>
</tr>
</thead>
<tbody><tr>
<td><code>dist</code></td>
<td>two picked points or objects</td>
<td><code>Closest::{curve_point, polyline_point, mesh_point}</code> (<code>closest.rs:25</code>, <code>260</code>, <code>990</code>)</td>
</tr>
<tr>
<td><code>xsect</code></td>
<td>two selected objects</td>
<td><code>intersection::{line_line, line_plane}</code> (<code>intersection.rs:94</code>, <code>198</code>)</td>
</tr>
<tr>
<td><code>bbox</code></td>
<td>—</td>
<td><code>InstanceTable::row_bounds</code> (<code>src/engine/gpu/objects.rs</code>)</td>
</tr>
</tbody></table>
<ul>
<li><code>bbox</code> reads the GPU table: <code>fit_selected_or_all</code> already reads <code>row_bounds</code> (<code>src/state.rs</code>), current after every upload.</li>
</ul>
<h3 id="commands-that-already-exist-zero-new-code">Commands that already exist — zero new code<a class="anchor" href="#/course/extend-command-line#commands-that-already-exist-zero-new-code" aria-label="Link to this section">#</a></h3>
<table>
<thead>
<tr>
<th>Command</th>
<th>Calls</th>
</tr>
</thead>
<tbody><tr>
<td><code>fit</code></td>
<td><code>State::fit_selected_or_all</code> (<code>src/state.rs</code>)</td>
</tr>
<tr>
<td><code>hide</code></td>
<td><code>State::hide_selected</code> (<code>src/state.rs</code>)</td>
</tr>
<tr>
<td><code>show</code></td>
<td><code>State::show_all</code> (<code>src/state.rs</code>)</td>
</tr>
<tr>
<td><code>clear</code></td>
<td><code>State::clear</code> (<code>src/state.rs</code>) plus <code>history.clear</code></td>
</tr>
<tr>
<td><code>xray</code></td>
<td><code>State::toggle_xray</code> (<code>src/state.rs</code>)</td>
</tr>
<tr>
<td><code>controls</code></td>
<td><code>State::enable_controls</code> (<code>src/state.rs</code>)</td>
</tr>
<tr>
<td><code>names</code></td>
<td><code>State::toggle_selected_names</code> (<code>src/state.rs</code>)</td>
</tr>
<tr>
<td><code>cloudsize</code></td>
<td><code>State::set_cloud_size</code> (<code>src/state.rs</code>)</td>
</tr>
<tr>
<td><code>view</code></td>
<td><code>Camera::set_view</code> (<code>src/camera.rs</code>)</td>
</tr>
<tr>
<td><code>esc</code></td>
<td><code>State::escape_selection</code> (<code>src/state.rs</code>)</td>
</tr>
<tr>
<td><code>help</code></td>
<td><code>CMDS</code> itself</td>
</tr>
</tbody></table>
<ul>
<li>Ten commands before a kernel call is written: one function per action, reachable from a key, a typed line or anything later.</li>
</ul>
<h2 id="the-coordinate-parser">The coordinate parser<a class="anchor" href="#/course/extend-command-line#the-coordinate-parser" aria-label="Link to this section">#</a></h2>
<ul>
<li>New <code>src/app/coords.rs</code>, <code>pub mod coords;</code> in the portable block of <code>src/app/mod.rs:5-16</code>; it names no <code>web_sys</code>, <code>wgpu</code> or <code>State</code>, so it tests natively.</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">enum</span> Coord {
 Absolute([f64; <span class="s">3</span>]),
 Relative([f64; <span class="s">3</span>]),
 Polar { distance: f64, degrees: f64 },
 Distance(f64),
}
<span class="k">pub</span> <span class="k">enum</span> ParseError { Empty, BadNumber(String), BadTuple(String) }
<span class="k">pub</span> <span class="k">fn</span> parse(s: &amp;str) -&gt; Result&lt;Coord, ParseError&gt;;</code></pre></div>
<ul>
<li>f64, not the archive&#39;s f32: every destination is f64 (<code>PickedPoint.position</code>, <code>src/app/scene.rs:108</code>; <code>InstanceTable.translation</code>) and the narrowing happens once, at the anchor.</li>
<li><code>Result</code>, not <code>Option</code>: only the parser can tell &quot;you typed nothing&quot; from &quot;you typed <code>1,2,</code>&quot;.</li>
<li>Check order is the grammar: trim, empty is <code>Empty</code>; <code>@</code> with a <code>&lt;</code> is polar <code>distance&lt;degrees</code>, else a relative tuple; a comma anywhere is an absolute tuple; a bare number is a distance.</li>
<li>Consequences for the <code>help</code> line: <code>@5</code> is an error; <code>5</code> is always a distance, never a coordinate; a trailing comma is a parse failure, not an empty third field.</li>
<li>A two-field tuple takes the third from the active frame&#39;s origin plane, not world zero: two numbers are typed on the plane being looked at.</li>
</ul>
<h3 id="one-basis-for-all-four-forms">One basis for all four forms<a class="anchor" href="#/course/extend-command-line#one-basis-for-all-four-forms" aria-label="Link to this section">#</a></h3>
<ul>
<li>All four resolve in the active construction plane&#39;s frame. <code>cplane</code> sets it; unset it is world XY, so the rules coincide until the user changes it, and the prompt names the active frame.</li>
<li>The archive uses world axes for <code>Absolute</code>/<code>Relative</code>, plane axes for <code>Polar</code>/<code>Distance</code>. In a front view <code>100,0</code> lands on world XY and nothing on screen says which basis ran, so the mixed rule is undiscoverable.</li>
</ul>
<h2 id="where-a-world-point-under-the-cursor-comes-from">Where a world point under the cursor comes from<a class="anchor" href="#/course/extend-command-line#where-a-world-point-under-the-cursor-comes-from" aria-label="Link to this section">#</a></h2>
<ul>
<li>No unprojection exists today and the pick answer carries no position: <code>Pick { row: u32, sub: u32 }</code> (<code>src/engine/gpu/pick.rs:15-18</code>). A plane hit is new camera math.</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A cursor ray in WORLD units, from the anchored view-projection's inverse.</span>
<span class="k">pub</span> <span class="k">fn</span> ray(&amp;<span class="k">self</span>, cursor: (f64, f64), viewport: (f64, f64), aspect: f64, anchor: &amp;Point)
 -&gt; Option&lt;(Point, Vector)&gt;;</code></pre></div>
<ul>
<li>In <code>src/camera.rs</code> beside <code>view_proj_anchored</code>: it inverts a matrix built there, and elsewhere duplicates the reverse-Z and unit conventions.</li>
<li>Unit trap: <code>view_proj_anchored</code> works in metres (<code>self.unit.to_meters</code>, <code>src/camera.rs</code>), world positions in millimetres. Divide back, or the ray is off by a thousand and the hit lands past the far plane.</li>
<li><code>Xform::inverse</code> returns <code>Option&lt;Xform&gt;</code> (<code>session_rust/src/xform.rs</code>), so the signature is <code>Option</code> out: a degenerate projection is real right after a resize to zero.</li>
<li>The hit itself is kernel math: a <code>Line</code> from the ray through <code>intersection::line_plane(&amp;line, plane, false)</code> (<code>session_rust/src/intersection.rs</code>), the plane&#39;s projection of the ray origin as the parallel fallback.</li>
<li>Do not port the archive&#39;s rule for <em>choosing</em> the plane (<code>ProjMode</code> plus a view-matrix column); re-derive from <code>set_view</code> (<code>src/camera.rs</code>) and <code>toggle_projection_framed</code>.</li>
</ul>
<h2 id="snap-is-cpu-work-and-picking-is-not">Snap is CPU work, and picking is not<a class="anchor" href="#/course/extend-command-line#snap-is-cpu-work-and-picking-is-not" aria-label="Link to this section">#</a></h2>
<ul>
<li>The pick path answers <em>which object</em>: async, once per click, halo, generation guard (<code>src/state.rs:498-525</code>, applied atop <code>render</code>). Tools needing identity use it untouched.</li>
<li>The snap answers <em>where, precisely</em>, every mouse-move. The id buffer carries no position, so it cannot answer that in principle, and a readback per mouse-move puts a GPU round trip in the cursor&#39;s path.</li>
<li>Candidates come from <code>Scene::geometry(row)</code> (<code>src/app/scene.rs:525</code>) per document across <code>Scene::docs</code>. Gather once per tool session, only <em>project</em> per frame: <code>point_at</code> per mouse-move stalls the cursor in a dense scene.</li>
<li><code>session.world_xforms</code> once per document, never <code>world_xform</code> per object (as <code>Scene::add_file</code> does): the per-object call rescans the tree and is quadratic.</li>
<li><code>Scene::geometry</code> is <code>None</code> for streamed clouds, sheets and text rows, so they offer no candidates; say so on screen the first time a tool runs there.</li>
<li>Aperture 8 CSS px, between the numbers around it: click slop 4 px (<code>src/app/input.rs:16</code>), snap marker 3.5 px (<code>src/state.rs:594</code>, <code>-3.5 * scale</code>, negative meaning screen-space). Below ~<code>3.5 + 4</code> the user aims at what the marker covers.</li>
<li>Do not exclude the object being moved: it stays put until commit, and excluding it makes &quot;move this corner onto that corner&quot; impossible.</li>
</ul>
<h2 id="a-command-with-no-arguments-starts-a-tool">A command with no arguments starts a tool<a class="anchor" href="#/course/extend-command-line#a-command-with-no-arguments-starts-a-tool" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">enum</span> Tool { Idle, Point, Line, Polyline, Curve { degree: usize }, Move }
<span class="k">pub</span> <span class="k">struct</span> ToolState {
 tool: Tool,
 points: Vec&lt;[f64; 3]&gt;, <span class="c">// committed, world</span>
 cursor: Option&lt;Snap&gt;,
 origins: Vec&lt;(u32, Mat4)&gt;, <span class="c">// Move: row and its placement at start</span>
 dirty: bool,
}</code></pre></div>
<ul>
<li><code>Idle</code> doubles as &quot;no tool&quot;, so nothing wraps it in an <code>Option</code>.</li>
<li>While a tool is live every typed line goes to the tool, and the prompt says so: <code>box</code> during a polyline is a coordinate that fails to parse.</li>
<li>Multi-point keywords (<code>c</code>/<code>close</code>, <code>u</code>/<code>undo</code>) match before the parser, only for many-point tools; otherwise <code>c</code> as a control point during <code>line</code> reads as &quot;close&quot; and the parser must know the tool.</li>
<li>Empty Enter finishes an open polyline or curve, Escape cancels; both reach <code>command_cancel</code>/<code>tool_finish</code> from the listener.</li>
<li>A bare distance as a tool&#39;s <em>first</em> input is an error, not a point: the archive falls back to plane origin plus x-axis, so <code>50</code> silently places (50, 0, 0).</li>
<li><code>Move</code> captures each row&#39;s placement before it starts, so cancel restores it exactly.</li>
</ul>
<h2 id="the-preview-is-its-own-lane-pair">The preview is its own lane pair<a class="anchor" href="#/course/extend-command-line#the-preview-is-its-own-lane-pair" aria-label="Link to this section">#</a></h2>
<ul>
<li>Two fields on <code>Gpu</code> (<code>src/engine/gpu/mod.rs</code>), after <code>controls</code> and <code>control_net</code>:</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> preview: SegmentLane,
<span class="k">pub</span> preview_marks: GlyphLane,</code></pre></div>
<ul>
<li>Five one-line edits beside their <code>controls</code> twins: <code>allocated_bytes</code> (<code>src/engine/gpu/mod.rs</code>), construction, <code>retarget</code>, <code>reset</code>, <code>release</code>. Missing one leaks buffers across a scene change or leaves the lane on the wrong sample count after an MSAA flip.</li>
<li>Two draw calls in <code>scene_list</code> (<code>src/engine/gpu/render.rs</code>), beside <code>control_net.draw_ribbons</code> and <code>controls.draw_dots</code>.</li>
<li>Zero lines in the id pass (<code>render.rs:203-295</code>). <code>control_net</code> proves it is free: colour only, in no id pass. The preview is unpickable by construction, not by a filter.</li>
<li>Do not reuse <code>controls</code>/<code>control_net</code>: <code>upload_controls</code> (<code>src/state.rs</code>) resets both unconditionally on every resize and selection change, so a shared rubber band vanishes mid-drag.</li>
<li>Preview geometry is world-space and every ink lane composes <code>model[row]</code> with <code>anchored_translation[row]</code>, so it needs one row with identity model and zero translation:</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>scene.push_row(usize::MAX, &quot;<span class="s">__preview__</span>&quot;, Xform::identity.m, <span class="s">0</span>);</code></pre></div>
<ul>
<li><code>usize::MAX</code> as owner is the idiom for a row with no kernel object (<code>register_text</code>, <code>src/app/scene_text.rs</code>); <code>push_row</code> is <code>pub(super)</code>.</li>
<li>Push it wherever rows are minted from empty — <code>Scene::new</code> and the tail of <code>reset_rows</code> — not at start-up alone and not in <code>Scene::rebuild</code> alone. <code>reset_rows</code> clears <code>order</code>, <code>owners</code> and <code>guid_to_row</code> on every rebuild, so a cached row goes stale; and <code>rebuild</code> runs on no ordinary load path (a document arrives through <code>State::append</code> → <code>add_file</code> → <code>upload_to</code>), so a row reserved only there does not exist until the first edit. Keep the number on <code>Scene</code> and re-read it after every rebuild.</li>
<li>Colour it distinctly from the control net&#39;s <code>0xffcc8866</code> (<code>src/state.rs:606</code>).</li>
</ul>
<h2 id="reporting-back">Reporting back<a class="anchor" href="#/course/extend-command-line#reporting-back" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Append one line to the command scrollback. textContent only.</span>
<span class="k">pub</span> <span class="k">fn</span> log(line: &amp;str);</code></pre></div>
<ul>
<li>Beside <code>status</code> in <code>src/app/feedback.rs</code>. <code>set_text_content</code>, never <code>set_inner_html</code>: the module doc states it (<code>feedback.rs</code>), both writers follow it, and a log echoing typed text is where the rule is load-bearing.</li>
<li><code>status</code> stays a one-line transient: &quot;Select one object before pressing F10&quot; (<code>src/state.rs</code>).</li>
<li>Cap the scrollback at 200 lines, in one place. The <code>&lt;pre&gt;</code>&#39;s text is replaced wholesale per write: 200 lines of ~80 characters is 16 KB rewritten, 20 000 lines 1.6 MB per logging keystroke.</li>
<li>Three shapes: <code>&gt; {line}</code> echoes input before dispatch, <code>+ box_0 (100x100x100 mm)</code> reports an add, a leading <code>? </code> marks a parse failure.</li>
<li>Prompts from a click have no return value, so the tool pushes them into the same capped function.</li>
</ul>
<h2 id="how-an-editing-command-reaches-the-document-history">How an editing command reaches the document history<a class="anchor" href="#/course/extend-command-line#how-an-editing-command-reaches-the-document-history" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">fn</span> edit&lt;R&gt;(&amp;<span class="k">mut</span> <span class="k">self</span>, label: &amp;str, f: <span class="k">impl</span> FnOnce(&amp;<span class="k">mut</span> Session) -&gt; R) -&gt; Result&lt;R, EditRefused&gt;;</code></pre></div>
<ul>
<li>In order: resolve the document, take a mutable session, <code>begin(label)</code>, run the closure, <code>commit</code>, rebuild or write the GPU row, reselect, <code>touch</code>.</li>
<li>Label the transaction with the typed line: the label lives on the <code>Transaction</code> (<code>session_rust/src/history.rs</code>), so a history listing reads back as a command log for free.</li>
<li><code>Session::begin</code> commits any open transaction first (<code>history.rs:227-231</code>), so an early return cannot leave one open.</li>
</ul>
<h3 id="taking-the-session-mutably">Taking the session mutably<a class="anchor" href="#/course/extend-command-line#taking-the-session-mutably" aria-label="Link to this section">#</a></h3>
<ul>
<li>The document holds <code>Rc&lt;Session&gt;</code> (<code>src/app/scene.rs</code>) and every mutator takes <code>&amp;mut self</code>: the split is <code>Rc::make_mut</code>.</li>
<li>Why: one manifest listing a file twice hands both documents the same <code>Rc</code>, so without the split, moving one placement moves the other and the live source&#39;s cached copy (<code>an_edit_must_split_a_session_two_placements_share</code>; <code>FileDoc</code> comment).</li>
<li>The deep copy costs only when the <code>Rc</code> is shared; a loader-made document is uniquely owned.</li>
<li>Refuse an edit on a live document rather than split it: <code>LiveSource</code> keeps its own <code>Rc&lt;Session&gt;</code> per URL (<code>src/app/live.rs</code>), so the count is at least two and the poller would overwrite the edit on its next tick. Say so on the log line.</li>
</ul>
<h3 id="which-rows-can-be-edited-at-all">Which rows can be edited at all<a class="anchor" href="#/course/extend-command-line#which-rows-can-be-edited-at-all" aria-label="Link to this section">#</a></h3>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The owning document index, or None when the row has no kernel object.</span>
<span class="k">pub</span> <span class="k">fn</span> editable_doc(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;usize&gt;;</code></pre></div>
<ul>
<li>Beside <code>hidden_rows</code> in <code>src/app/scene.rs</code>. <code>None</code> for streamed clouds, sheets and text rows, as <code>Scene::geometry</code> already is: <code>add_streamed_cloud</code> and <code>add_sheet</code> push a <code>FileDoc</code> with an empty <code>Session::new(&amp;name)</code> shell, <code>display_only: true</code> (, <code>430-436</code>).</li>
<li>Refuse a whole-scene rebuild while <code>scene.streamed</code> or <code>scene.sheets</code> is non-empty — &quot;Streamed clouds and sheets cannot come back (no kernel object)&quot; (, warnings at) — or an edit elsewhere silently discards a multi-million-point resident prefix.</li>
<li>A sheet is one row for tens of thousands of segments (<code>push_row</code> once), so deleting the selection deletes the whole sheet; the entity is metadata (<code>SheetBatch.resolved</code>), not editable geometry.</li>
</ul>
<h3 id="keeping-the-selection-across-a-rebuild">Keeping the selection across a rebuild<a class="anchor" href="#/course/extend-command-line#keeping-the-selection-across-a-rebuild" aria-label="Link to this section">#</a></h3>
<ul>
<li><code>reset_rows</code> sets <code>selected = None</code> and clears <code>guid_to_row</code> (<code>src/app/scene.rs</code>), so a rebuild destroys every row number.</li>
<li>Capture <code>Scene::identity_of(row)</code> first, re-resolve after through a new <code>Scene::row_of(&amp;identity)</code> over the private <code>guid_to_row</code>.</li>
<li>The hide set is the model: it survives because it stores <code>(document index, guid)</code>, and <code>add_file</code> re-reads it into <code>FLAG_HIDDEN</code> as rows return.</li>
<li>Anything in <code>SelectionMode</code> but <code>Object</code> is dropped: an edge index into re-walked geometry is not the same edge, and a control-point id into a replaced object is not the same point.</li>
</ul>
<h2 id="move-is-the-one-edit-that-skips-the-rebuild">Move is the one edit that skips the rebuild<a class="anchor" href="#/course/extend-command-line#move-is-the-one-edit-that-skips-the-rebuild" aria-label="Link to this section">#</a></h2>
<ul>
<li>No per-row placement writer exists: <code>append</code> alone writes <code>translation</code> and <code>rows[i].model</code>, <code>rebuild</code> rewrites the whole table, <code>set_flag</code> touches flags only (<code>src/engine/gpu/objects.rs</code>). Add one beside <code>set_flag</code>:</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Rewrite one row's placement. \`translate_only\` writes the 16 B translation row alone.</span>
<span class="k">pub</span> <span class="k">fn</span> set_place(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, row: u32, place: &amp;Mat4, translate_only: bool);</code></pre></div>
<ul>
<li>A pure translate updates <code>translation[row]</code> in f64, re-anchors that one value against <code>last_origin</code>, writes one <code>[f32; 4]</code>: <strong>16 bytes</strong>. Rotate or scale also rewrites <code>rows[row].model</code> with columns 12/13/14 zeroed and writes one <code>Instance</code>, <strong>96 bytes</strong> by the const assert at <code>src/engine/gpu/instance.rs</code> — <strong>112 bytes</strong>, seven times a translate.</li>
<li>Write the increment into the f64 base, never the f32 the shader reads: <code>rebuild</code> recomputes every row from <code>translation</code>, so a delta on the returned f32 is erased by the next re-anchor (<code>anchored</code> doc comment, <code>objects.rs</code>).</li>
<li>Both writes must refresh <code>world_bounds[row]</code> and its bounded-row entry. Stale values break <code>fit_selected_or_all</code> (<code>src/state.rs</code>), <code>update_inside</code> and the selection box, and none of those failures points back at the move.</li>
<li>Write the kernel side in the same transaction: <code>Session::set_xform(guid, xf)</code> (<code>session_rust/src/session.rs:346</code>), recorded as <code>Op::Xform</code> with absolute before and after (<code>history.rs:107-113</code>). GPU only and the next rebuild snaps it back; kernel only and nothing moves until something else rebuilds.</li>
<li>Compose the GPU matrix as the walk does — the document&#39;s <code>place</code> times the session&#39;s <em>world</em> xform for that guid (the private free function <code>placement</code>, <code>src/app/scene.rs</code>, fed the map <code>session.world_xforms</code> returns) — because <code>set_xform</code> sets the <strong>local</strong> transform relative to the tree parent (<code>session.rs:345</code>). Expose it once as <code>Scene::placement_of(row)</code>.</li>
<li>So <code>move</code> needs no rebuild: rows keep identity, the selection survives, pick tables stay valid. The cheapest edit, and the right one to build first on a large document.</li>
</ul>
<h2 id="testing-and-a-real-limit">Testing, and a real limit<a class="anchor" href="#/course/extend-command-line#testing-and-a-real-limit" aria-label="Link to this section">#</a></h2>
<ul>
<li><code>coords::parse</code>, <code>lookup</code> and the argument helpers are pure and test natively, with in-file <code>#[cfg(test)] mod tests</code> as <code>src/app/scene.rs</code> and <code>src/engine/gpu/objects.rs</code> do.</li>
<li><code>State::command</code> cannot: <code>State::new</code> takes an <code>Arc&lt;Window&gt;</code> and is built only in the wasm loader (<code>src/app/loader.rs</code>), while the native harness builds <code>Scene</code>, <code>Gpu</code>, <code>Camera</code> directly (<code>src/selftest/lifecycle.rs</code>).</li>
<li>Publish <code>command_log</code>, <code>tool</code> and <code>history_depth</code> (<code>session_rust/src/history.rs:222</code>) in the <code>?inspect=1</code> snapshot (<code>src/app/inspection.rs:33-70</code>): <code>tests/interaction.cjs</code> and <code>tests/streamed-controls.cjs</code> drive the viewer through it.</li>
</ul>
<h2 id="the-order-that-compiles">The order that compiles<a class="anchor" href="#/course/extend-command-line#the-order-that-compiles" aria-label="Link to this section">#</a></h2>
<ol>
<li><strong>The parser alone.</strong> <code>src/app/coords.rs</code>, <code>pub mod coords;</code>. Tests pass on both targets; nothing references it.</li>
<li><strong>Dispatch over actions that exist.</strong> <code>mod command;</code> (<code>src/state.rs:18-20</code>), a <code>shell</code> field initialised in <code>State::new</code>, arms calling <code>fit_selected_or_all</code>, <code>hide_selected</code>, <code>show_all</code>, <code>clear</code>, <code>toggle_xray</code>, <code>enable_controls</code>, <code>escape_selection</code>, <code>set_cloud_size</code>; results to <code>self.status</code>.</li>
<li><strong>The log sink and the markup.</strong> <code>feedback::log</code>, the three elements, <code>State::command</code> writing there.</li>
<li><strong>The field is live.</strong> Three web-sys features, <code>CommandLine</code>, <code>Msg::Command</code>/<code>CommandCancel</code>, installed in <code>App::resumed</code>. Ten commands end to end; <code>f</code> in the field no longer fits the view, free from <code>src/lib.rs</code>.</li>
<li><strong>Edit targets and selection survival, no edits.</strong> <code>Scene::row_of</code>, <code>Scene::editable_doc</code>, <code>reselect(identity)</code>.</li>
<li><strong>The first kernel edit: add and delete.</strong> The <code>edit</code> wrapper, <code>Rc::make_mut</code>, <code>begin</code>/<code>commit</code>, <code>Scene::rebuild</code>, <code>camera.grow_extent(&amp;gpu.bounds)</code>, reselect, <code>touch</code>; rebuild refused while <code>streamed</code> or <code>sheets</code> is non-empty.</li>
<li><strong>Undo and redo.</strong> <code>Session::undo</code>/<code>redo</code> plus the same rebuild-and-reselect tail, <code>history.depth</code> on the log line.</li>
<li><strong>Move, skipping the rebuild.</strong> <code>InstanceTable::set_place</code>, <code>Scene::placement_of</code>, <code>State::move_selected</code>.</li>
<li><strong>The preview lane pair, drawing nothing.</strong> Two <code>Gpu</code> fields, five twin edits, two draw calls, zero id-pass lines, the reserved row in <code>Scene::rebuild</code>.</li>
<li><strong>The cursor ray and the snap.</strong> <code>Camera::ray</code>, the hit through <code>intersection::line_plane</code>, candidates cached per tool session and projected per frame.</li>
<li><strong>The tool.</strong> <code>ToolState</code>, <code>tool_text</code>, <code>tool_click</code>, <code>tool_cancel</code>; <code>line</code>, <code>poly</code>, interactive <code>move</code>.</li>
<li><strong>Observability.</strong> The three inspection fields and <code>tests/command.cjs</code>, shaped on <code>tests/interaction.cjs</code>.</li>
</ol>
<h2 id="what-we-take-from-the-old-viewer-and-what-we-do-not">What we take from the old viewer and what we do not<a class="anchor" href="#/course/extend-command-line#what-we-take-from-the-old-viewer-and-what-we-do-not" aria-label="Link to this section">#</a></h2>
<table>
<thead>
<tr>
<th>Part</th>
<th>Verdict</th>
<th>Why</th>
</tr>
</thead>
<tbody><tr>
<td><code>coord_parser.rs</code>, 44 lines</td>
<td><strong>Ports</strong>, f32 → f64</td>
<td>Pure; destinations are f64, narrowing once at the anchor.</td>
</tr>
<tr>
<td>The parse/resolve split</td>
<td><strong>Ports</strong></td>
<td><code>parse</code> decides the form, the caller the basis; testable without a GPU.</td>
</tr>
<tr>
<td><code>ray_to_plane</code></td>
<td><strong>Ports</strong></td>
<td>Kernel math; plane projection as the parallel fallback.</td>
</tr>
<tr>
<td>Multi-point keywords before the parser</td>
<td><strong>Ports</strong></td>
<td>Keeps a control point named <code>c</code> from reading as &quot;close&quot;.</td>
</tr>
<tr>
<td>History cursor with a stashed live buffer</td>
<td><strong>Ports</strong></td>
<td>What users expect from an up-arrow.</td>
</tr>
<tr>
<td>Cancel being idempotent</td>
<td><strong>Ports</strong></td>
<td>Two senders reach it here too.</td>
</tr>
<tr>
<td>Snap gathered once, projected per frame</td>
<td><strong>Ports</strong></td>
<td>No <code>point_at</code> per mouse-move; the moved object stays a candidate.</td>
</tr>
<tr>
<td><code>execute_command</code>&#39;s dispatch shape</td>
<td><strong>Adapts</strong></td>
<td>Split, verb, match, unknown fallback kept; arms call named actions.</td>
</tr>
<tr>
<td>The command list</td>
<td><strong>Adapts</strong></td>
<td>Three disagreeing lists become one <code>CMDS</code>.</td>
</tr>
<tr>
<td><code>ToolState</code> / <code>DrawTool</code></td>
<td><strong>Adapts</strong></td>
<td>One field on <code>State</code>; the input layer never reaches a document.</td>
</tr>
<tr>
<td>The preview</td>
<td><strong>Adapts</strong></td>
<td>A reserved row in its own lane pair, not magic guids in the arena.</td>
</tr>
<tr>
<td>The snap engine</td>
<td><strong>Adapts</strong></td>
<td>Source is <code>Scene::geometry(row)</code>; non-resident rows give nothing.</td>
</tr>
<tr>
<td>Construction-plane <em>selection</em></td>
<td><strong>Adapts</strong></td>
<td>Math ports; the plane comes from <code>set_view</code>/<code>toggle_projection_framed</code>.</td>
</tr>
<tr>
<td>Return-a-string-and-log-it</td>
<td><strong>Adapts</strong></td>
<td>Same contract, sink is <code>feedback::log</code>, <code>set_text_content</code> only.</td>
</tr>
<tr>
<td><code>undo_state.rs</code> + <code>state_undo.rs</code>, 303 lines</td>
<td><strong>Replaced</strong></td>
<td>The kernel keeps transactions, tombstones, absolute pairs, a 64-deep cursor.</td>
</tr>
<tr>
<td><code>p(parts, i, default)</code></td>
<td><strong>Replaced</strong></td>
<td>Cannot fail: <code>box abc</code> builds a cube, <code>sphere -5</code> a negative radius.</td>
</tr>
<tr>
<td><code>set_guid(name)</code> on cone, torus, curve</td>
<td><strong>Replaced</strong></td>
<td>Non-opaque guids; two <code>cone_3</code> collide. <code>add_*</code> mints its own.</td>
</tr>
<tr>
<td><code>replace_or_push_nurbs</code> / <code>tree.add</code> bypass</td>
<td><strong>Replaced</strong></td>
<td>One bypass spawns four special cases.</td>
</tr>
<tr>
<td><code>del</code>&#39;s three-table removal</td>
<td><strong>Replaced</strong></td>
<td>Leaves tree and graph nodes dangling.</td>
</tr>
<tr>
<td><code>clear</code> as written</td>
<td><strong>Replaced</strong></td>
<td>Keeps the undo stack, so the next undo replays dead guids.</td>
</tr>
<tr>
<td><code>commit_object_transform</code>&#39;s five branches</td>
<td><strong>Replaced</strong></td>
<td>One home for the pose; its baked cases need a geometry snapshot to undo.</td>
</tr>
<tr>
<td>The whole egui mechanism</td>
<td><strong>Replaced</strong></td>
<td>Its borrow dance solves a problem a DOM field does not have.</td>
</tr>
<tr>
<td>Focus retention and the Enter backstop</td>
<td><strong>Replaced</strong></td>
<td>Both exist because egui shares the viewport&#39;s keyboard queue.</td>
</tr>
<tr>
<td>Escape falling to <code>event_loop.exit</code></td>
<td><strong>Replaced</strong></td>
<td>Quitting destroys the WebGPU device; Escape already clears the selection.</td>
</tr>
<tr>
<td>CPU raycasting for identity</td>
<td><strong>Replaced</strong></td>
<td>The id pass is better and built; only snap stays on the CPU.</td>
</tr>
<tr>
<td><code>cmd_counter</code>, one counter per prefix</td>
<td><strong>Replaced</strong></td>
<td><code>box_0, sphere_1, line_2</code> reads as a per-type index and is not one.</td>
</tr>
<tr>
<td>Viewer colours in kernel geometry</td>
<td><strong>Replaced</strong></td>
<td>Display concern: <code>ObjectRow::new</code> starts white, the walk tints.</td>
</tr>
<tr>
<td><code>tree_ui.rs</code>, gumball, edit points</td>
<td><strong>Replaced</strong></td>
<td>Each is its own ownership question.</td>
</tr>
<tr>
<td><code>fit</code> unioning <code>session.cached_boxes</code></td>
<td><strong>Replaced</strong></td>
<td><code>row_bounds</code> is current after every upload.</td>
</tr>
</tbody></table>
<h2 id="what-to-check-on-screen">What to check on screen<a class="anchor" href="#/course/extend-command-line#what-to-check-on-screen" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>Step 2.</strong> Nothing visible; <code>cargo check</code> passes, <code>cargo test</code> runs the lookup.</li>
<li><strong>Step 3.</strong> The log box sits above the status line, wraps at phone width, does not eat canvas clicks.</li>
<li><strong>Step 4.</strong> In the field <code>f</code>, <code>h</code>, <code>1</code>-<code>7</code> type characters; on the canvas they move the camera. <code>fit</code> re-frames, up-arrow recalls the previous line, Escape clears the box, then returns focus, then the selection.</li>
<li><strong>Step 5.</strong> Nothing visible; <code>hide</code> then <code>show</code> returns the selection to the same object.</li>
<li><strong>Step 6.</strong> <code>box 100</code> puts a cube at the origin, log <code>+ box_0</code>; <code>delete</code> removes it; <code>box abc</code> changes nothing and says so; in a streamed scene it is refused with a reason and the cloud survives.</li>
<li><strong>Step 7.</strong> <code>box 100</code>, <code>delete</code>, <code>undo</code> brings it back selected, <code>redo</code> removes it. Ten rounds without drift; <code>hist</code> returns to zero.</li>
<li><strong>Step 8.</strong> <code>move 0 0 500</code> moves immediately, no reload, still selected; <code>F</code> frames it (world bounds); <code>undo</code> restores exactly; instant on a large document, unlike step 6.</li>
<li><strong>Step 9.</strong> Nothing visible, frame time unchanged, <code>?inspect=1</code> shows two lanes at zero rows.</li>
<li><strong>Step 10.</strong> The marker follows the cursor onto endpoints and midpoints within ~8 px, never a streamed cloud or sheet, and the log said why the first time.</li>
<li><strong>Step 11.</strong> <code>line</code>, click, <code>@100,0</code>: the second point lands 100 mm along the plane&#39;s x-axis and commits. Escape mid-tool leaves nothing; clicking the rubber band picks what is behind it; during <code>move</code> the original stays snappable until the second click.</li>
<li><strong>Step 12.</strong> <code>tests/command.cjs</code> types a line, reads <code>command_log</code>, <code>tool</code>, <code>history_depth</code> from <code>?inspect=1</code>, passes headless.</li>
</ul>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/extend-command-line#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Reference result from the supported <a href="#/course/22-runtime-helpers">implementation tutorials</a>. The white egui command area shows a completed line command and its feedback, with the created geometry visible in the full viewer. The capture uses the maintained viewer and the <a href="/session/docs/course/docs/extensions/nested.pb">nested fixture</a>.</p>
<p><a href="/session/docs/course/docs/screenshots/extensions-command-create.png"><img src="/session/docs/course/docs/screenshots/extensions-command-create.png" alt="Full viewer result for extend command line" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"the-shape-of-the-finished-thing",text:"The shape of the finished thing"},{level:2,id:"the-field-is-markup-not-rust-created-dom",text:"The field is markup, not Rust-created DOM"},{level:2,id:"keystrokes-the-browser-already-arbitrates-so-nothing-needs-a-mode-flag",text:"Keystrokes: the browser already arbitrates, so nothing needs a mode flag"},{level:2,id:"the-listener-owns-the-field-one-message-carries-the-line",text:"The listener owns the field; one message carries the line"},{level:2,id:"three-web-sys-features-or-step-4-does-not-compile",text:"Three web-sys features, or step 4 does not compile"},{level:2,id:"escape-in-the-order-a-user-expects-it",text:"Escape, in the order a user expects it"},{level:2,id:"one-table-never-three",text:"One table, never three"},{level:2,id:"tokenising",text:"Tokenising"},{level:2,id:"arguments-missing-and-wrong-are-different-answers",text:'Arguments: "missing" and "wrong" are different answers'},{level:2,id:"the-command-table",text:"The command table"},{level:3,id:"document-and-history",text:"Document and history"},{level:3,id:"solids",text:"Solids"},{level:3,id:"curves",text:"Curves"},{level:3,id:"surfaces-and-meshes",text:"Surfaces and meshes"},{level:3,id:"edits",text:"Edits"},{level:3,id:"queries-read-only-no-transaction",text:"Queries — read-only, no transaction"},{level:3,id:"commands-that-already-exist-zero-new-code",text:"Commands that already exist — zero new code"},{level:2,id:"the-coordinate-parser",text:"The coordinate parser"},{level:3,id:"one-basis-for-all-four-forms",text:"One basis for all four forms"},{level:2,id:"where-a-world-point-under-the-cursor-comes-from",text:"Where a world point under the cursor comes from"},{level:2,id:"snap-is-cpu-work-and-picking-is-not",text:"Snap is CPU work, and picking is not"},{level:2,id:"a-command-with-no-arguments-starts-a-tool",text:"A command with no arguments starts a tool"},{level:2,id:"the-preview-is-its-own-lane-pair",text:"The preview is its own lane pair"},{level:2,id:"reporting-back",text:"Reporting back"},{level:2,id:"how-an-editing-command-reaches-the-document-history",text:"How an editing command reaches the document history"},{level:3,id:"taking-the-session-mutably",text:"Taking the session mutably"},{level:3,id:"which-rows-can-be-edited-at-all",text:"Which rows can be edited at all"},{level:3,id:"keeping-the-selection-across-a-rebuild",text:"Keeping the selection across a rebuild"},{level:2,id:"move-is-the-one-edit-that-skips-the-rebuild",text:"Move is the one edit that skips the rebuild"},{level:2,id:"testing-and-a-real-limit",text:"Testing, and a real limit"},{level:2,id:"the-order-that-compiles",text:"The order that compiles"},{level:2,id:"what-we-take-from-the-old-viewer-and-what-we-do-not",text:"What we take from the old viewer and what we do not"},{level:2,id:"what-to-check-on-screen",text:"What to check on screen"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{e as default};
