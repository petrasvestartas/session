const s={title:"35 · Attributes On|Off and a phone keyboard",html:`<h1 id="35-attributes-onoff-and-a-phone-keyboard">35 · Attributes On|Off and a phone keyboard<a class="anchor" href="#/course/35-attributes#35-attributes-onoff-and-a-phone-keyboard" aria-label="Link to this section">#</a></h1>
<p><code>Attributes On</code> draws an element&#39;s outlines, axes and sections inside its own row, and a hidden input lets a phone type into the command line.</p>
<h2 id="step-1-cargotoml">Step 1 · Cargo.toml<a class="anchor" href="#/course/35-attributes#step-1-cargotoml" aria-label="Link to this section">#</a></h2>
<p>web-sys gains the three DOM types the hidden input needs.</p>
<p><code>lessons/35/Cargo.toml</code> · edit · type this</p>
<p>Added after the line <code>&quot;HtmlElement&quot;,</code> in <code>lessons/34/Cargo.toml</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &quot;HtmlInputElement&quot;,
    &quot;KeyboardEvent&quot;,
    &quot;PointerEvent&quot;,</code></pre></div>
<h2 id="step-2-indexhtml">Step 2 · index.html<a class="anchor" href="#/course/35-attributes#step-2-indexhtml" aria-label="Link to this section">#</a></h2>
<p>A 1×1 invisible <code>&lt;input&gt;</code> is the element a phone keyboard can attach to.</p>
<p><code>lessons/35/index.html</code> · edit · type this</p>
<p>Added after the line <code>&lt;canvas id=&quot;canvas&quot; tabindex=&quot;0&quot; role=&quot;application&quot; aria-…</code> in <code>lessons/34/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">&lt;!-- A phone keyboard opens only for an editable element: this one takes the command line's typing (app/agent.rs). --&gt;</span>
    &lt;input id=&quot;<span class="s">command-agent</span>&quot; type=&quot;<span class="s">text</span>&quot; autocomplete=&quot;<span class="s">off</span>&quot; autocapitalize=&quot;<span class="s">off</span>&quot; autocorrect=&quot;<span class="s">off</span>&quot; spellcheck=&quot;<span class="s">false</span>&quot; enterkeyhint=&quot;<span class="s">go</span>&quot; tabindex=&quot;<span class="s">-1</span>&quot; aria-hidden=&quot;<span class="s">true</span>&quot; style=&quot;<span class="s">position: fixed; bottom: 0; left: 0; width: 1px; height: 1px; opacity: 0; border: 0; padding: 0; pointer-events: none;</span>&quot;&gt;</code></pre></div>
<h2 id="step-3-srcappmodrs">Step 3 · src/app/mod.rs<a class="anchor" href="#/course/35-attributes#step-3-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>The agent module is declared.</p>
<p><code>lessons/35/src/app/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>#[cfg(target_arch = &quot;wasm32&quot;)]</code> in <code>lessons/34/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> agent;
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]</code></pre></div>
<h2 id="step-4-srcappagentrs">Step 4 · src/app/agent.rs<a class="anchor" href="#/course/35-attributes#step-4-srcappagentrs" aria-label="Link to this section">#</a></h2>
<p>New file: the agent focuses the hidden input and replays its value into the command field.</p>
<p><code>lessons/35/src/app/agent.rs</code> · new file · type this</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! A phone raises its keyboard only for a focused DOM input, never a canvas, so this hidden \`&lt;input&gt;\` stands in.</span>

<span class="k">use</span> super::ui::MODEL;
<span class="k">use</span> wasm_bindgen::JsCast;
<span class="k">use</span> wasm_bindgen::closure::Closure;
<span class="k">use</span> winit::event_loop::EventLoopProxy;

<span class="k">const</span> AGENT: &amp;str = &quot;<span class="s">command-agent</span>&quot;; <span class="c">// id of the hidden input</span>

<span class="c">/// One message from the hidden input.</span>
<span class="k">pub</span> <span class="k">enum</span> AgentEvent {
    Text(String),   <span class="c">// the whole typed text</span>
    Key(egui::Key), <span class="c">// Enter, Tab, Escape or an arrow</span>
}

<span class="c">/// The hidden input and its listeners.</span>
<span class="k">pub</span> <span class="k">struct</span> CommandAgent {
    input: web_sys::HtmlInputElement,                     <span class="c">// the hidden input</span>
    canvas: web_sys::HtmlCanvasElement,                   <span class="c">// the viewer canvas</span>
    on_input: Closure&lt;<span class="k">dyn</span> FnMut(web_sys::Event)&gt;,         <span class="c">// text changed</span>
    on_key: Closure&lt;<span class="k">dyn</span> FnMut(web_sys::KeyboardEvent)&gt;,   <span class="c">// editing key pressed</span>
    on_pointer: Closure&lt;<span class="k">dyn</span> FnMut(web_sys::PointerEvent)&gt;, <span class="c">// tap on the canvas</span>
}

<span class="c">/// The hidden input element, if the page has one.</span>
<span class="k">fn</span> element() -&gt; Option&lt;web_sys::HtmlInputElement&gt; {
    web_sys::window()?
        .document()?
        .get_element_by_id(AGENT)?
        .dyn_into()
        .ok()
}

<span class="k">impl</span> CommandAgent {
    <span class="c">/// Install the listeners; each one sends a message.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(
        canvas: web_sys::HtmlCanvasElement,
        proxy: EventLoopProxy&lt;<span class="k">crate</span>::Msg&gt;,
    ) -&gt; Result&lt;<span class="k">Self</span>, wasm_bindgen::JsValue&gt; {
        <span class="k">let</span> input =
            element().ok_or_else(|| wasm_bindgen::JsValue::from_str(&quot;<span class="s">no #command-agent</span>&quot;))?;
        <span class="k">let</span> on_input = {
            <span class="k">let</span> proxy = proxy.clone();
            <span class="k">let</span> input = input.clone();
            Closure::&lt;<span class="k">dyn</span> FnMut(web_sys::Event)&gt;::new(<span class="k">move</span> |_| {
                <span class="k">let</span> _ = proxy.send_event(<span class="k">crate</span>::Msg::Agent(AgentEvent::Text(input.value())));
            })
        };
        <span class="k">let</span> on_key = {
            <span class="k">let</span> proxy = proxy.clone();
            Closure::&lt;<span class="k">dyn</span> FnMut(web_sys::KeyboardEvent)&gt;::new(
                <span class="k">move</span> |event: web_sys::KeyboardEvent| {
                    <span class="k">let</span> key = <span class="k">match</span> event.key().as_str() {
                        &quot;<span class="s">Enter</span>&quot; =&gt; egui::Key::Enter,
                        &quot;<span class="s">Tab</span>&quot; =&gt; egui::Key::Tab,
                        &quot;<span class="s">Escape</span>&quot; =&gt; egui::Key::Escape,
                        &quot;<span class="s">ArrowUp</span>&quot; =&gt; egui::Key::ArrowUp,
                        &quot;<span class="s">ArrowDown</span>&quot; =&gt; egui::Key::ArrowDown,
                        _ =&gt; <span class="k">return</span>,
                    };
                    event.prevent_default();
                    <span class="k">let</span> _ = proxy.send_event(<span class="k">crate</span>::Msg::Agent(AgentEvent::Key(key)));
                },
            )
        };
        <span class="c">// a tap on the command line focuses the input, raising the keyboard</span>
        <span class="k">let</span> on_pointer = {
            <span class="k">let</span> input = input.clone();
            Closure::&lt;<span class="k">dyn</span> FnMut(web_sys::PointerEvent)&gt;::new(<span class="k">move</span> |event: web_sys::PointerEvent| {
                <span class="k">if</span> !matches!(event.pointer_type().as_str(), &quot;<span class="s">touch</span>&quot; | &quot;<span class="s">pen</span>&quot;) {
                    <span class="k">return</span>;
                }

                <span class="k">let</span> point = egui::pos2(event.offset_x() <span class="k">as</span> f32, event.offset_y() <span class="k">as</span> f32);
                <span class="k">let</span> (line, popup) = MODEL.with_borrow(|m| {
                    (
                        m.command_rect.is_some_and(|r| r.contains(point)),
                        m.completion_rect.is_some_and(|r| r.contains(point)),
                    )
                });

                <span class="k">if</span> line {
                    <span class="k">let</span> _ = input.focus();
                } <span class="k">else</span> <span class="k">if</span> !popup {
                    <span class="k">let</span> _ = input.blur();
                }
            })
        };
        input.add_event_listener_with_callback(&quot;<span class="s">input</span>&quot;, on_input.as_ref().unchecked_ref())?;
        input.add_event_listener_with_callback(&quot;<span class="s">keydown</span>&quot;, on_key.as_ref().unchecked_ref())?;
        canvas
            .add_event_listener_with_callback(&quot;<span class="s">pointerdown</span>&quot;, on_pointer.as_ref().unchecked_ref())?;
        Ok(<span class="k">Self</span> {
            input,
            canvas,
            on_input,
            on_key,
            on_pointer,
        })
    }
}

<span class="k">impl</span> Drop <span class="k">for</span> CommandAgent {
    <span class="c">/// Remove the listeners.</span>
    <span class="k">fn</span> drop(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> _ = <span class="k">self</span>
            .input
            .remove_event_listener_with_callback(&quot;<span class="s">input</span>&quot;, <span class="k">self</span>.on_input.as_ref().unchecked_ref());
        <span class="k">let</span> _ = <span class="k">self</span>
            .input
            .remove_event_listener_with_callback(&quot;<span class="s">keydown</span>&quot;, <span class="k">self</span>.on_key.as_ref().unchecked_ref());
        <span class="k">let</span> _ = <span class="k">self</span>.canvas.remove_event_listener_with_callback(
            &quot;<span class="s">pointerdown</span>&quot;,
            <span class="k">self</span>.on_pointer.as_ref().unchecked_ref(),
        );
    }
}

<span class="c">/// Copy the command line text into the hidden input.</span>
<span class="k">pub</span> <span class="k">fn</span> sync(command: &amp;str) {
    <span class="k">if</span> <span class="k">let</span> Some(input) = element()
        &amp;&amp; input.value() != command
    {
        input.set_value(command);
    }
}</code></pre></div>
<h2 id="step-5-srcappcommandrs">Step 5 · src/app/command.rs<a class="anchor" href="#/course/35-attributes#step-5-srcappcommandrs" aria-label="Link to this section">#</a></h2>
<p><code>Attributes</code>, <code>Attributes On</code>, <code>Attributes Off</code> are parsed and listed.</p>
<p><code>lessons/35/src/app/command.rs</code> · edit · type this</p>
<p>Added after the line <code>Layers(Option&lt;bool&gt;),</code> in <code>lessons/34/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Attributes(Option&lt;bool&gt;), <span class="c">// element features on, off or toggle</span></code></pre></div>
<p>Added after the line <code>&quot;layers&quot; =&gt; &quot;Layers (On Off): show or hide the layer panel&quot;,</code> in <code>lessons/34/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">attributes</span>&quot; =&gt; {
            &quot;<span class="s">Attributes (On Off): draw or remove the element features, moving with their element</span>&quot;
        }</code></pre></div>
<p>Added after the line <code>_ =&gt; Err(&quot;Layers (On Off)&quot;.into()),</code> in <code>lessons/34/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        },
        &quot;<span class="s">attributes</span>&quot; =&gt; <span class="k">match</span> rest.as_slice() {
            [] =&gt; Ok(Command::Attributes(None)),
            [value] <span class="k">if</span> value.eq_ignore_ascii_case(&quot;<span class="s">on</span>&quot;) =&gt; Ok(Command::Attributes(Some(<span class="s">true</span>))),
            [value] <span class="k">if</span> value.eq_ignore_ascii_case(&quot;<span class="s">off</span>&quot;) =&gt; Ok(Command::Attributes(Some(<span class="s">false</span>))),
            _ =&gt; Err(&quot;<span class="s">Attributes (On Off)</span>&quot;.into()),</code></pre></div>
<p>Added after the line <code>&quot;layers&quot; =&gt; &amp;[&quot;Layers On&quot;, &quot;Layers Off&quot;],</code> in <code>lessons/34/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">attributes</span>&quot; =&gt; &amp;[&quot;<span class="s">Attributes On</span>&quot;, &quot;<span class="s">Attributes Off</span>&quot;],</code></pre></div>
<p>Replaces the 3 lines from <code>&quot;Arctic&quot;, &quot;Controls&quot;, &quot;Curve&quot;, &quot;Delete&quot;, &quot;Edge&quot;, &quot;Escape&quot;…</code> in <code>lessons/34/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">Arctic</span>&quot;,
        &quot;<span class="s">Attributes</span>&quot;,
        &quot;<span class="s">Controls</span>&quot;,
        &quot;<span class="s">Curve</span>&quot;,
        &quot;<span class="s">Delete</span>&quot;,
        &quot;<span class="s">Edge</span>&quot;,
        &quot;<span class="s">Escape</span>&quot;,
        &quot;<span class="s">Explode</span>&quot;,
        &quot;<span class="s">Extend</span>&quot;,
        &quot;<span class="s">Face</span>&quot;,
        &quot;<span class="s">Fit</span>&quot;,
        &quot;<span class="s">Hide</span>&quot;,
        &quot;<span class="s">Layers</span>&quot;,
        &quot;<span class="s">Line</span>&quot;,
        &quot;<span class="s">Move</span>&quot;,
        &quot;<span class="s">Object</span>&quot;,
        &quot;<span class="s">Open</span>&quot;,
        &quot;<span class="s">Point</span>&quot;,
        &quot;<span class="s">Polyline</span>&quot;,
        &quot;<span class="s">Redo</span>&quot;,
        &quot;<span class="s">Rotate</span>&quot;,
        &quot;<span class="s">Save</span>&quot;,
        &quot;<span class="s">Scale</span>&quot;,
        &quot;<span class="s">Show</span>&quot;,
        &quot;<span class="s">Snap</span>&quot;,
        &quot;<span class="s">Split</span>&quot;,
        &quot;<span class="s">SSAO</span>&quot;,
        &quot;<span class="s">Trim</span>&quot;,
        &quot;<span class="s">Undo</span>&quot;,</code></pre></div>
<p>Added after the line <code>assert_eq!(parse(&quot;Layers OFF&quot;), Ok(Command::Layers(Some(f…</code> in <code>lessons/34/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        assert_eq!(
            parse(&quot;<span class="s">Attributes off</span>&quot;),
            Ok(Command::Attributes(Some(<span class="s">false</span>)))
        );
        assert_eq!(parse(&quot;<span class="s">Attributes</span>&quot;), Ok(Command::Attributes(None)));</code></pre></div>
<h2 id="step-6-srcappsceners">Step 6 · src/app/scene.rs<a class="anchor" href="#/course/35-attributes#step-6-srcappsceners" aria-label="Link to this section">#</a></h2>
<p>The scene keeps an <code>attributes</code> flag and skips wood&#39;s baked attribute groups, which never get a row.</p>
<p><code>lessons/35/src/app/scene.rs</code> · edit · type this</p>
<p>Added after the line <code>pub selected: Option&lt;u32&gt;,</code> in <code>lessons/34/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> attributes: bool,</code></pre></div>
<p>Added after the line <code>selected: None,</code> in <code>lessons/34/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            attributes: <span class="s">false</span>,</code></pre></div>
<p>Added after the line <code>self.guid_to_row.reserve(count);</code> in <code>lessons/34/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> baked = baked_attributes(&amp;session);</code></pre></div>
<p>Replaces the line <code>if !is_drawable(geom) {</code> in <code>lessons/34/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">if</span> !is_drawable(geom) || baked.contains(guid.as_str()) {</code></pre></div>
<p>Added after the line <code>row,</code> in <code>lessons/34/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                attributes: <span class="k">self</span>.attributes,</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/34/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Guids under an \`attributes\` group; they get no row.</span>
<span class="k">fn</span> baked_attributes(session: &amp;Session) -&gt; HashSet&lt;String&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = HashSet::new();
    <span class="k">let</span> <span class="k">mut</span> stack: Vec&lt;_&gt; = session
        .tree
        .root()
        .into_iter()
        .map(|n| (n, <span class="s">false</span>))
        .collect();

    <span class="k">while</span> <span class="k">let</span> Some((node, inside)) = stack.pop() {
        <span class="k">let</span> node = node.borrow();
        <span class="k">let</span> inside = inside || node.name == &quot;<span class="s">attributes</span>&quot;;

        <span class="k">if</span> inside &amp;&amp; session.lookup.contains_key(&amp;node.name) {
            out.insert(node.name.clone());
        }

        stack.extend(node.children().into_iter().map(|c| (c, inside)));
    }

    out
}</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/34/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Element features draw in the element's row and move with it.</span>
    #[test]
    <span class="k">fn</span> attributes_share_the_element_row_and_its_placement() {
        <span class="k">use</span> session_rust::element::ElementFeature;
        <span class="k">use</span> session_rust::{Element, Mesh, Polyline};

        <span class="k">let</span> <span class="k">mut</span> element = Element::new(&quot;<span class="s">beam</span>&quot;);
        element.set_geometry(Mesh::create_box(<span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>));
        <span class="k">let</span> axis = Polyline::new(vec![Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Point::new(<span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)]);
        element.add_feature(ElementFeature::new(&quot;<span class="s">axis</span>&quot;, -<span class="s">1</span>, vec![axis], &quot;<span class="s">axis</span>&quot;));
        <span class="k">let</span> <span class="k">mut</span> source = Session::new(&quot;<span class="s">attributes</span>&quot;);
        source.add_element(element, None);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene.add_file(file(&quot;<span class="s">beam</span>&quot;, Rc::new(source), <span class="s">false</span>));
        <span class="k">let</span> plain = scene.tables.seg.ribbons.len();
        assert_eq!(scene.object_count(), <span class="s">1</span>);

        scene.attributes = <span class="s">true</span>;
        <span class="k">let</span> doc = scene.docs.remove(<span class="s">0</span>);
        scene.reset_rows();
        scene.add_file(doc);
        assert_eq!(scene.object_count(), <span class="s">1</span>);
        assert_eq!(scene.tables.seg.ribbons.len(), plain + <span class="s">1</span>);
        assert!(scene.tables.seg.ribbons.iter().all(|r| r.instance_id == <span class="s">0</span>));

        <span class="k">let</span> moved = scene.transform_rows(&amp;[<span class="s">0</span>], &amp;Xform::translation(<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), &quot;<span class="s">Move</span>&quot;);
        assert_eq!(moved.map(|m| m.len()), Some(<span class="s">1</span>));
        assert_eq!(
            scene
                .placement_of(<span class="s">0</span>)
                .unwrap()
                .transform_point(&amp;Point::new(<span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>))[<span class="s">0</span>],
            <span class="s">105</span>.<span class="s">0</span>
        );
    }

    <span class="c">/// Baked attribute copies never get a row.</span>
    #[test]
    <span class="k">fn</span> baked_attributes_never_get_a_row() {
        <span class="k">use</span> session_rust::{Element, Mesh, Polyline};

        <span class="k">let</span> <span class="k">mut</span> element = Element::new(&quot;<span class="s">beam</span>&quot;);
        element.set_geometry(Mesh::create_box(<span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>));
        <span class="k">let</span> <span class="k">mut</span> source = Session::new(&quot;<span class="s">attributes</span>&quot;);
        source.add_element(element, None);
        <span class="k">let</span> group = source.add_group(&quot;<span class="s">attributes</span>&quot;);
        <span class="k">let</span> axis = Polyline::new(vec![Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Point::new(<span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)]);
        source.add_polyline(axis, Some(&amp;group));
        assert_eq!(source.lookup.len(), <span class="s">2</span>);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene.add_file(file(&quot;<span class="s">beam</span>&quot;, Rc::new(source), <span class="s">false</span>));
        assert_eq!(scene.object_count(), <span class="s">1</span>);

        scene.attributes = <span class="s">true</span>;
        <span class="k">let</span> doc = scene.docs.remove(<span class="s">0</span>);
        scene.reset_rows();
        scene.add_file(doc);
        assert_eq!(scene.object_count(), <span class="s">1</span>);
    }</code></pre></div>
<p>Added after the line <code>row,</code> in <code>lessons/34/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            attributes: <span class="k">self</span>.attributes,</code></pre></div>
<h2 id="step-7-srcappwalkmodrs">Step 7 · src/app/walk/mod.rs<a class="anchor" href="#/course/35-attributes#step-7-srcappwalkmodrs" aria-label="Link to this section">#</a></h2>
<p>With <code>attributes</code> on, the walker draws each element&#39;s features into the element&#39;s own row.</p>
<p><code>lessons/35/src/app/walk/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>use session_rust::AABB;</code> in <code>lessons/34/src/app/walk/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> session_rust::Element;</code></pre></div>
<p>Added after the line <code>pub row: u32,</code> in <code>lessons/34/src/app/walk/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> attributes: bool, <span class="c">// draw element features inside its row</span></code></pre></div>
<p>Added after the line <code>faces: false,</code> in <code>lessons/34/src/app/walk/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        }
    }
}

<span class="c">/// Feature types \`Attributes On\` draws on the element.</span>
<span class="k">const</span> ATTRIBUTE_FEATURES: [&amp;str; <span class="s">4</span>] = [&quot;<span class="s">outline</span>&quot;, &quot;<span class="s">axis</span>&quot;, &quot;<span class="s">section</span>&quot;, &quot;<span class="s">centroid</span>&quot;];

<span class="c">/// Draw an element's features, red and thick, into its own row.</span>
<span class="k">fn</span> walk_attributes(w: &amp;<span class="k">mut</span> Walk, cx: &amp;WalkCx, e: &amp;Element, bounds: &amp;<span class="k">mut</span> AABB) {
    <span class="k">for</span> feature <span class="k">in</span> e.features() {
        <span class="k">if</span> !ATTRIBUTE_FEATURES.contains(&amp;feature.feature_type.as_str()) {
            <span class="k">continue</span>;
        }

        <span class="k">for</span> outline <span class="k">in</span> &amp;feature.outlines {
            <span class="k">let</span> r = <span class="k">if</span> <span class="k">let</span> (<span class="s">1</span>, Some(p)) = (outline.point_count(), outline.get_point(<span class="s">0</span>)) {
                walk_point(w.glyph, &amp;p, cx.row)
            } <span class="k">else</span> {
                walk_polyline(w.seg, outline, cx.row)
            };
            bounds.union_with(&amp;r.bounds);</code></pre></div>
<p>Replaces the line <code>Geometry::Element(e) =&gt; match e.geometry() {</code> in <code>lessons/34/src/app/walk/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        Geometry::Element(e) =&gt; {
            <span class="k">let</span> <span class="k">mut</span> row = <span class="k">match</span> e.geometry() {
                ElementGeometry::Mesh(m) =&gt; {
                    <span class="k">let</span> (arena, <span class="k">mut</span> ink) = w.solid();
                    walk_mesh(
                        arena,
                        &amp;<span class="k">mut</span> ink,
                        m,
                        &amp;MeshCx {
                            cx,
                            opts: &amp;MeshOpts::ELEMENT,
                        },
                    )
                }
                ElementGeometry::BRep(b) =&gt; {
                    <span class="k">let</span> (arena, <span class="k">mut</span> ink) = w.solid();
                    walk_brep(arena, &amp;<span class="k">mut</span> ink, b, cx)
                }
                ElementGeometry::None =&gt; Row::thin(AABB::empty()),
            };

            <span class="k">if</span> cx.attributes {
                walk_attributes(w, cx, e, &amp;<span class="k">mut</span> row.bounds);
            }

            row
        }
    }
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> session_rust::Mesh;
    <span class="k">use</span> session_rust::Point;
    <span class="k">use</span> session_rust::Polyline;
    <span class="k">use</span> session_rust::element::ElementFeature;

    <span class="c">/// A box element with an axis, a section dot and a non-attribute cut.</span>
    <span class="k">fn</span> walk_element(attributes: bool) -&gt; (Upload, Row) {
        <span class="k">let</span> <span class="k">mut</span> element = Element::new(&quot;<span class="s">beam</span>&quot;);
        element.set_geometry(Mesh::create_box(<span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>));
        <span class="k">let</span> axis = Polyline::new(vec![Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Point::new(<span class="s">100</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)]);
        element.add_feature(ElementFeature::new(&quot;<span class="s">axis</span>&quot;, -<span class="s">1</span>, vec![axis], &quot;<span class="s">axis</span>&quot;));
        <span class="k">let</span> centroid = Polyline::new(vec![Point::new(<span class="s">5</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>, <span class="s">5</span>.<span class="s">0</span>)]);
        element.add_feature(ElementFeature::new(
            &quot;<span class="s">centroid</span>&quot;,
            -<span class="s">1</span>,
            vec![centroid],
            &quot;<span class="s">centroid</span>&quot;,
        ));
        <span class="k">let</span> cut = Polyline::new(vec![Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">200</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)]);
        element.add_feature(ElementFeature::new(&quot;<span class="s">cut</span>&quot;, <span class="s">0</span>, vec![cut], &quot;<span class="s">cut</span>&quot;));
        <span class="k">let</span> <span class="k">mut</span> up = Upload::default();
        <span class="k">let</span> cx = WalkCx {
            vert_base: <span class="s">0</span>,
            cloud_px: <span class="s">0</span>.<span class="s">0</span>,
            row: <span class="s">4</span>,
            attributes,
        };
        <span class="k">let</span> row = walk_geometry(
            &amp;<span class="k">mut</span> Walk::of(&amp;<span class="k">mut</span> up),
            &amp;cx,
            &amp;Geometry::Element(std::rc::Rc::new(element)),
        );
        (up, row)
    }

    <span class="c">/// Attributes add one ribbon and one dot to the element's own row.</span>
    #[test]
    <span class="k">fn</span> attributes_join_the_element_row() {
        <span class="k">let</span> (off, row_off) = walk_element(<span class="s">false</span>);
        <span class="k">let</span> (on, row_on) = walk_element(<span class="s">true</span>);
        assert_eq!(on.seg.ribbons.len(), off.seg.ribbons.len() + <span class="s">1</span>);
        assert_eq!(on.glyph.dots.len(), off.glyph.dots.len() + <span class="s">1</span>);
        assert!(on.seg.ribbons.iter().all(|r| r.instance_id == <span class="s">4</span>));
        assert_eq!(on.glyph.dots.last().map(|d| d.instance_id), Some(<span class="s">4</span>));
        assert_eq!(row_off.bounds.max_point()[<span class="s">0</span>], <span class="s">5</span>.<span class="s">0</span>);
        assert_eq!(row_on.bounds.max_point()[<span class="s">0</span>], <span class="s">100</span>.<span class="s">0</span>);
        assert_eq!(row_on.bounds.max_point()[<span class="s">1</span>], <span class="s">5</span>.<span class="s">0</span>);
    }
}</code></pre></div>
<h2 id="step-8-srcappwalkbreprs">Step 8 · src/app/walk/brep.rs<a class="anchor" href="#/course/35-attributes#step-8-srcappwalkbreprs" aria-label="Link to this section">#</a></h2>
<p>Brep walkers pass <code>attributes: false</code>.</p>
<p><code>lessons/35/src/app/walk/brep.rs</code> · edit · type this</p>
<p>Added after the line <code>row: 5,</code> in <code>lessons/34/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            attributes: <span class="s">false</span>,</code></pre></div>
<p>Added after the line <code>row: 7,</code> in <code>lessons/34/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            attributes: <span class="s">false</span>,</code></pre></div>
<h2 id="step-9-srcappwalkbrep_orientrs">Step 9 · src/app/walk/brep_orient.rs<a class="anchor" href="#/course/35-attributes#step-9-srcappwalkbrep_orientrs" aria-label="Link to this section">#</a></h2>
<p>The orient walker passes <code>attributes: false</code>.</p>
<p><code>lessons/35/src/app/walk/brep_orient.rs</code> · edit · type this</p>
<p>Added after the line <code>row: 0,</code> in <code>lessons/34/src/app/walk/brep_orient.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    attributes: <span class="s">false</span>,</code></pre></div>
<h2 id="step-10-srcappwalkmesh_inkrs">Step 10 · src/app/walk/mesh_ink.rs<a class="anchor" href="#/course/35-attributes#step-10-srcappwalkmesh_inkrs" aria-label="Link to this section">#</a></h2>
<p>Mesh ink passes <code>attributes: false</code>.</p>
<p><code>lessons/35/src/app/walk/mesh_ink.rs</code> · edit · type this</p>
<p>Added after the line <code>row: 0,</code> in <code>lessons/34/src/app/walk/mesh_ink.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            attributes: <span class="s">false</span>,</code></pre></div>
<p>Added after the line <code>row: 7,</code> in <code>lessons/34/src/app/walk/mesh_ink.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            attributes: <span class="s">false</span>,</code></pre></div>
<h2 id="step-11-srcappmesh_previewrs">Step 11 · src/app/mesh_preview.rs<a class="anchor" href="#/course/35-attributes#step-11-srcappmesh_previewrs" aria-label="Link to this section">#</a></h2>
<p>The mesh preview passes <code>attributes: false</code>.</p>
<p><code>lessons/35/src/app/mesh_preview.rs</code> · edit · type this</p>
<p>Added after the line <code>row: 0,</code> in <code>lessons/34/src/app/mesh_preview.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                attributes: <span class="s">false</span>,</code></pre></div>
<h2 id="step-12-srcappsurface_previewrs">Step 12 · src/app/surface_preview.rs<a class="anchor" href="#/course/35-attributes#step-12-srcappsurface_previewrs" aria-label="Link to this section">#</a></h2>
<p>The surface preview passes <code>attributes: false</code>.</p>
<p><code>lessons/35/src/app/surface_preview.rs</code> · edit · type this</p>
<p>Added after the line <code>row: 0,</code> in <code>lessons/34/src/app/surface_preview.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                attributes: <span class="s">false</span>,</code></pre></div>
<p>Added after the line <code>row: 0,</code> in <code>lessons/34/src/app/surface_preview.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                attributes: <span class="s">false</span>,</code></pre></div>
<p>Added after the line <code>row: 3,</code> in <code>lessons/34/src/app/surface_preview.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            attributes: <span class="s">false</span>,</code></pre></div>
<h2 id="step-13-srcappuirs">Step 13 · src/app/ui.rs<a class="anchor" href="#/course/35-attributes#step-13-srcappuirs" aria-label="Link to this section">#</a></h2>
<p>The UI remembers the command rect and replays the agent&#39;s typing into the field.</p>
<p><code>lessons/35/src/app/ui.rs</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>completion_rect: Option&lt;egui::Rect&gt;,</code> in <code>lessons/34/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span>(<span class="k">crate</span>) completion_rect: Option&lt;egui::Rect&gt;, <span class="c">// where the list is, for taps</span>
    <span class="k">pub</span>(<span class="k">crate</span>) command_rect: Option&lt;egui::Rect&gt;,    <span class="c">// where the field is, for taps</span></code></pre></div>
<p>Added after the line <code>touches: std::collections::HashSet&lt;u64&gt;,</code> in <code>lessons/34/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    agent_value: String,</code></pre></div>
<p>Added after the line <code>touches: std::collections::HashSet::new(),</code> in <code>lessons/34/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
            agent_value: String::new(),</code></pre></div>
<p>Added after the line <code>(consumed || escape, response.repaint || escape)</code> in <code>lessons/34/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    }

    <span class="c">/// Feed the hidden input's typing into the field; returns keys for the viewport.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">pub</span> <span class="k">fn</span> agent(&amp;<span class="k">mut</span> <span class="k">self</span>, event: super::agent::AgentEvent) {
        <span class="k">use</span> super::agent::AgentEvent;
        <span class="k">let</span> id = egui::Id::new(&quot;<span class="s">command-input</span>&quot;);
        <span class="k">let</span> key = |key, pressed| egui::Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: <span class="s">false</span>,
            modifiers: egui::Modifiers::NONE,
        };
        <span class="k">let</span> events = &amp;<span class="k">mut</span> <span class="k">self</span>.input.egui_input_mut().events;

        <span class="k">match</span> event {
            AgentEvent::Text(value) =&gt; {
                <span class="k">let</span> shared = <span class="k">self</span>
                    .agent_value
                    .chars()
                    .zip(value.chars())
                    .take_while(|(a, b)| a == b)
                    .count();

                <span class="k">for</span> _ <span class="k">in</span> shared..<span class="k">self</span>.agent_value.chars().count() {
                    events.push(key(egui::Key::Backspace, <span class="s">true</span>));
                    events.push(key(egui::Key::Backspace, <span class="s">false</span>));
                }

                <span class="k">let</span> added: String = value.chars().skip(shared).collect();

                <span class="k">if</span> !added.is_empty() {
                    events.push(egui::Event::Text(added));
                }

                <span class="k">self</span>.agent_value = value;
                <span class="k">self</span>.context.memory_mut(|memory| memory.request_focus(id));
                MODEL.with_borrow_mut(|model| model.command_open = <span class="s">true</span>);
            }
            AgentEvent::Key(k) =&gt; {
                events.push(key(k, <span class="s">true</span>));
                events.push(key(k, <span class="s">false</span>));
            }
        }</code></pre></div>
<p>Added after the line <code>self.publish();</code> in <code>lessons/34/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// the field changed on its own: the hidden input follows</span>
        #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
        MODEL.with_borrow(|model| {
            <span class="k">if</span> <span class="k">self</span>.agent_value != model.command {
                <span class="k">self</span>.agent_value.clone_from(&amp;model.command);
                super::agent::sync(&amp;model.command);
            }
        });</code></pre></div>
<h2 id="step-14-srcstaters">Step 14 · src/state.rs<a class="anchor" href="#/course/35-attributes#step-14-srcstaters" aria-label="Link to this section">#</a></h2>
<p><code>show_attributes</code> toggles the flag and rebuilds the rows.</p>
<p><code>lessons/35/src/state.rs</code> · edit · type this</p>
<p>Added after the line <code>self.touch();</code> in <code>lessons/34/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    }

    <span class="c">/// Attributes On|Off: draw the features inside each element; \`None\` toggles.</span>
    <span class="k">pub</span> <span class="k">fn</span> show_attributes(&amp;<span class="k">mut</span> <span class="k">self</span>, value: Option&lt;bool&gt;) -&gt; bool {
        <span class="k">let</span> show = value.unwrap_or(!<span class="k">self</span>.scene.attributes);
        <span class="k">self</span>.scene.attributes = show;
        <span class="k">self</span>.select(None);
        <span class="k">self</span>.scene.rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
        <span class="k">self</span>.place_gizmo(None);
        <span class="k">self</span>.refresh_layers();
        <span class="k">self</span>.update_label();
        <span class="k">self</span>.touch();
        show</code></pre></div>
<h2 id="step-15-srcstateeditrs">Step 15 · src/state/edit.rs<a class="anchor" href="#/course/35-attributes#step-15-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>The <code>Attributes</code> command reports On or Off.</p>
<p><code>lessons/35/src/state/edit.rs</code> · edit · type this</p>
<p>Added after the line <code>}</code> in <code>lessons/34/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Command::Attributes(value) =&gt; {
                <span class="k">let</span> shown = <span class="k">self</span>.show_attributes(value);
                Ok(format!(&quot;<span class="s">Attributes </span>{}&quot;, <span class="k">if</span> shown { &quot;<span class="s">On</span>&quot; } <span class="k">else</span> { &quot;<span class="s">Off</span>&quot; }))
            }</code></pre></div>
<h2 id="step-16-srclibrs">Step 16 · src/lib.rs<a class="anchor" href="#/course/35-attributes#step-16-srclibrs" aria-label="Link to this section">#</a></h2>
<p>Agent events reach the UI through the message loop.</p>
<p><code>lessons/35/src/lib.rs</code> · edit · type this</p>
<p>Added after the line <code>CancelPointer,</code> in <code>lessons/34/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    Agent(app::agent::AgentEvent),                <span class="c">// a phone keyboard key</span></code></pre></div>
<p>Added after the line <code>pointer_cancellation: Option&lt;app::input::PointerCancellat…</code> in <code>lessons/34/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    agent: Option&lt;app::agent::CommandAgent&gt;,                    <span class="c">// phone keyboard listener</span></code></pre></div>
<p>Added after the line <code>pointer_cancellation: None,</code> in <code>lessons/34/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            agent: None,</code></pre></div>
<p>Replaces the line <code>match app::input::PointerCancellation::new(canvas, proxy.…</code> in <code>lessons/34/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">match</span> app::input::PointerCancellation::new(canvas.clone(), proxy.clone()) {
                Ok(listener) =&gt; <span class="k">self</span>.pointer_cancellation = Some(listener),
                Err(error) =&gt; log::warn!(&quot;<span class="s">Cannot register pointer cancellation: </span>{<span class="s">error:?</span>}&quot;),
            }

            <span class="k">match</span> app::agent::CommandAgent::new(canvas, proxy.clone()) {
                Ok(agent) =&gt; <span class="k">self</span>.agent = Some(agent),
                Err(error) =&gt; log::warn!(&quot;<span class="s">Cannot register the command agent: </span>{<span class="s">error:?</span>}&quot;),</code></pre></div>
<p>Added after the line <code>self.input.cancel();</code> in <code>lessons/34/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                state.touch();
            }
            Msg::Agent(event) =&gt; {
                <span class="c">// phone keys become key presses</span>
                <span class="k">if</span> <span class="k">let</span> Some(ui) = <span class="k">self</span>.ui.as_mut() {
                    ui.agent(event);
                }</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/35-attributes#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/35/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Type <code>Attributes On</code>: outlines and axes appear inside each element and move with it; on a phone, tap the command line and the keyboard opens.</p>
<h2 id="next">Next<a class="anchor" href="#/course/35-attributes#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/36-translucent-faces">36 · Translucent faces and the Opacity command</a></p>
`,toc:[{level:2,id:"step-1-cargotoml",text:"Step 1 · Cargo.toml"},{level:2,id:"step-2-indexhtml",text:"Step 2 · index.html"},{level:2,id:"step-3-srcappmodrs",text:"Step 3 · src/app/mod.rs"},{level:2,id:"step-4-srcappagentrs",text:"Step 4 · src/app/agent.rs"},{level:2,id:"step-5-srcappcommandrs",text:"Step 5 · src/app/command.rs"},{level:2,id:"step-6-srcappsceners",text:"Step 6 · src/app/scene.rs"},{level:2,id:"step-7-srcappwalkmodrs",text:"Step 7 · src/app/walk/mod.rs"},{level:2,id:"step-8-srcappwalkbreprs",text:"Step 8 · src/app/walk/brep.rs"},{level:2,id:"step-9-srcappwalkbrep_orientrs",text:"Step 9 · src/app/walk/brep_orient.rs"},{level:2,id:"step-10-srcappwalkmesh_inkrs",text:"Step 10 · src/app/walk/mesh_ink.rs"},{level:2,id:"step-11-srcappmesh_previewrs",text:"Step 11 · src/app/mesh_preview.rs"},{level:2,id:"step-12-srcappsurface_previewrs",text:"Step 12 · src/app/surface_preview.rs"},{level:2,id:"step-13-srcappuirs",text:"Step 13 · src/app/ui.rs"},{level:2,id:"step-14-srcstaters",text:"Step 14 · src/state.rs"},{level:2,id:"step-15-srcstateeditrs",text:"Step 15 · src/state/edit.rs"},{level:2,id:"step-16-srclibrs",text:"Step 16 · src/lib.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"next",text:"Next"}]};export{s as default};
