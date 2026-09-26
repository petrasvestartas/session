const s={title:"27 · Build the egui panel and command interface",html:`<h1 id="27-build-the-egui-panel-and-command-interface">27 · Build the egui panel and command interface<a class="anchor" href="#/course/27-egui-interface#27-build-the-egui-panel-and-command-interface" aria-label="Link to this section">#</a></h1>
<p>White egui windows display the command input and nested layer panel over the scene.</p>
<h2 id="step-1-cargotoml">Step 1 · Cargo.toml<a class="anchor" href="#/course/27-egui-interface#step-1-cargotoml" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/27/Cargo.toml</code> · edit · copy the file</p>
<p>Added after the line <code>wgpu = &quot;29.0&quot;</code> in <code>lessons/26/Cargo.toml</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>egui = { version = &quot;<span class="s">=0.34.3</span>&quot;, default-features = <span class="s">false</span>, features = [&quot;<span class="s">default_fonts</span>&quot;] }
egui-wgpu = { version = &quot;<span class="s">=0.34.3</span>&quot;, default-features = <span class="s">false</span> }
egui-winit = { version = &quot;<span class="s">=0.34.3</span>&quot;, default-features = <span class="s">false</span> }</code></pre></div>
<p>Delete the 2 lines from <code>&quot;HtmlInputElement&quot;,</code> in <code>lessons/26/Cargo.toml</code>.</p>
<h2 id="step-2-indexhtml">Step 2 · index.html<a class="anchor" href="#/course/27-egui-interface#step-2-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/27/index.html</code> · edit · copy the file</p>
<p>Added after the line <code>}</code> in <code>lessons/26/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>      #viewer-status:empty {
        display: none;
      }</code></pre></div>
<p>Delete the 9 lines from <code>&lt;!-- The layers panel: one element, filled from Rust with…</code> in <code>lessons/26/index.html</code>.</p>
<h2 id="step-3-srcappfeedbackrs">Step 3 · src/app/feedback.rs<a class="anchor" href="#/course/27-egui-interface#step-3-srcappfeedbackrs" aria-label="Link to this section">#</a></h2>
<p>Carry status and layer information from the scene into the interface.</p>
<p><code>lessons/27/src/app/feedback.rs</code> · edit · type this</p>
<p>Added after the line <code>}</code> in <code>lessons/26/src/app/feedback.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    super::ui::MODEL.with_borrow_mut(|model| model.status = message.chars().take(<span class="s">256</span>).collect());</code></pre></div>
<p>Replaces the 7 lines from <code>pub fn command_line(open: bool) -&gt; Option&lt;web_sys::HtmlIn…</code> in <code>lessons/26/src/app/feedback.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">fn</span> command_line(open: bool) {
    super::ui::MODEL.with_borrow_mut(|model| {
        model.command_open = open;
        model.focus_command = open;

        <span class="k">if</span> open {
            model.command.clear();
        }
    });</code></pre></div>
<p>Added after the line <code>pub fn command_line(_open: bool) {}</code> in <code>lessons/26/src/app/feedback.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One row of the layers panel, as the panel needs it.</span>
#[derive(Clone)]</code></pre></div>
<p>Replaces the 28 lines from <code>let Some(document) = web_sys::window().and_then(|w| w.doc…</code> in <code>lessons/26/src/app/feedback.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    super::ui::MODEL.with_borrow_mut(|model| model.rows = rows.to_vec());
}

<span class="c">/// Show or hide the layers panel.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> layers_visible(open: bool) {
    super::ui::MODEL.with_borrow_mut(|model| {
        model.layers_open = open;

        <span class="k">if</span> !open {
            model.rows.clear();
        }
    });
}

<span class="c">/// Whether the layers panel is open.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> layers_open() -&gt; bool {
    super::ui::MODEL.with_borrow(|model| model.layers_open)</code></pre></div>
<h2 id="step-4-srcappinputrs">Step 4 · src/app/input.rs<a class="anchor" href="#/course/27-egui-interface#step-4-srcappinputrs" aria-label="Link to this section">#</a></h2>
<p>Send command-window visibility through the UI model and return Escape to scene input.</p>
<p><code>lessons/27/src/app/input.rs</code> · edit · type this</p>
<p>Delete the 96 lines from <code>pub struct CommandKeys {</code> in <code>lessons/26/src/app/input.rs</code>.</p>
<h2 id="step-5-srcappmodrs">Step 5 · src/app/mod.rs<a class="anchor" href="#/course/27-egui-interface#step-5-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>Declare the new application modules so their files join the crate.</p>
<p><code>lessons/27/src/app/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>pub mod touch;</code> in <code>lessons/26/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">mod</span> ui;</code></pre></div>
<h2 id="step-6-srcappuirs">Step 6 · src/app/ui.rs<a class="anchor" href="#/course/27-egui-interface#step-6-srcappuirs" aria-label="Link to this section">#</a></h2>
<p>Build the command and layer windows, collect their actions, then apply them after the UI borrow ends.</p>
<p><code>lessons/27/src/app/ui.rs</code> · 334 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! egui is immediate mode: the interface is rebuilt from this state every frame, so no widget object is kept in sync.</span>

<span class="k">use</span> <span class="k">crate</span>::State;
<span class="k">use</span> <span class="k">crate</span>::app::feedback::LayerRow;
<span class="k">use</span> std::cell::RefCell;
<span class="k">use</span> std::collections::VecDeque;
<span class="k">use</span> winit::window::Window;

<span class="c">// Every panel reads and writes this one struct, because an immediate-mode frame keeps no widget state of its own.</span>
#[derive(Default)]
<span class="k">pub</span> <span class="k">struct</span> Model {
    <span class="k">pub</span> layers_open: bool,                          <span class="c">// layers panel shown</span>
    <span class="k">pub</span> rows: Vec&lt;LayerRow&gt;,                        <span class="c">// its rows</span>
    <span class="k">pub</span> command_open: bool,                         <span class="c">// command line shown</span>
    <span class="k">pub</span> command: String,                            <span class="c">// text in the command field</span>
    <span class="k">pub</span> focus_command: bool,                        <span class="c">// give the field focus next frame</span>
    <span class="k">pub</span> status: String,                             <span class="c">// status line text</span>
    history: VecDeque&lt;String&gt;,                      <span class="c">// past commands and answers</span>
}

thread_local! { <span class="k">pub</span> <span class="k">static</span> MODEL: RefCell&lt;Model&gt; = RefCell::default(); } <span class="c">// the one model</span>

<span class="c">/// One clickable control and where it was drawn, for browser tests.</span>
#[derive(serde::Serialize)]
<span class="k">pub</span> <span class="k">struct</span> Control {
    key: String,    <span class="c">// what it does</span>
    label: String,  <span class="c">// text shown</span>
    rect: [f32; <span class="s">4</span>], <span class="c">// left, top, right, bottom</span>
}

<span class="c">/// The egui interface over the canvas.</span>
<span class="k">pub</span> <span class="k">struct</span> Ui {
    context: egui::Context,                    <span class="c">// egui state</span>
    input: egui_winit::State,                  <span class="c">// winit events into egui</span>
    controls: Option&lt;Vec&lt;Control&gt;&gt;,            <span class="c">// controls drawn this frame, when inspecting</span>
}

<span class="k">impl</span> Ui {
    <span class="c">/// Create the egui state for a window.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(window: &amp;Window) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> context = egui::Context::default();
        context.set_theme(egui::Theme::Light);
        context.set_visuals(visuals());
        <span class="k">let</span> input = egui_winit::State::new(
            context.clone(),
            egui::ViewportId::ROOT,
            window,
            Some(window.scale_factor() <span class="k">as</span> f32),
            window.theme(),
            Some(<span class="s">4096</span>),
        );
        <span class="k">Self</span> {
            context,
            input,
            controls: (super::route::query(&quot;<span class="s">inspect</span>&quot;).as_deref() == Some(&quot;<span class="s">1</span>&quot;)).then(Vec::new),
        }
    }

    <span class="c">/// Offer one event to the panels; (consumed, needs repaint).</span>
    <span class="k">pub</span> <span class="k">fn</span> event(&amp;<span class="k">mut</span> <span class="k">self</span>, window: &amp;Window, event: &amp;winit::event::WindowEvent) -&gt; (bool, bool) {
        <span class="k">let</span> response = <span class="k">self</span>.input.on_window_event(window, event);
        <span class="k">let</span> escape = matches!(event, winit::event::WindowEvent::KeyboardInput { event, .. }
            <span class="k">if</span> event.logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape))
            &amp;&amp; MODEL.with_borrow(|model| model.command_open);
        (response.consumed || escape, response.repaint || escape)
    }

    <span class="c">/// Lay out and draw the panels; true when the frame must be redrawn.</span>
    <span class="k">pub</span> <span class="k">fn</span> frame(&amp;<span class="k">mut</span> <span class="k">self</span>, state: &amp;<span class="k">mut</span> State) -&gt; bool {
        <span class="k">let</span> input = <span class="k">self</span>.input.take_egui_input(&amp;state.window);

        <span class="k">if</span> <span class="k">let</span> Some(controls) = <span class="k">self</span>.controls.as_mut() {
            controls.clear();
        }

        <span class="k">let</span> <span class="k">mut</span> action = None;
        <span class="k">let</span> <span class="k">mut</span> command = None;
        <span class="k">let</span> <span class="k">mut</span> output = <span class="k">self</span>.context.run_ui(input, |root| {
            <span class="k">let</span> context = root.ctx();
            MODEL.with_borrow_mut(|model| {
                layers(context, model, &amp;<span class="k">mut</span> <span class="k">self</span>.controls, &amp;<span class="k">mut</span> action);
                commands(context, model, &amp;<span class="k">mut</span> <span class="k">self</span>.controls, &amp;<span class="k">mut</span> command);
            });
        });
        <span class="k">self</span>.input
            .handle_platform_output(&amp;state.window, std::mem::take(&amp;<span class="k">mut</span> output.platform_output));
        <span class="k">let</span> changed = action.is_some() || command.is_some();

        <span class="k">if</span> <span class="k">let</span> Some(key) = action {
            state.panel_action(&amp;key);
        }

        <span class="k">if</span> <span class="k">let</span> Some(text) = command {
            <span class="k">let</span> message = state.run_command(&amp;text).unwrap_or_else(|error| error);
            <span class="k">crate</span>::app::feedback::status(&amp;message);
            MODEL.with_borrow_mut(|model| {
                <span class="k">if</span> model.history.len() == <span class="s">8</span> {
                    model.history.pop_front();
                }

                model.history.push_back(format!(&quot;<span class="s">&gt; </span>{<span class="s">text</span>}<span class="s">\\n</span>{<span class="s">message</span>}&quot;));
            });
            state.touch();
        }

        <span class="k">self</span>.publish();
        <span class="k">let</span> repaint = changed || <span class="k">self</span>.context.has_requested_repaint();
        output.pixels_per_point *=
            state.gpu.config.width <span class="k">as</span> f32 / state.window.inner_size().width.max(<span class="s">1</span>) <span class="k">as</span> f32;

        <span class="k">if</span> <span class="k">let</span> Some(ui) = state.gpu.ui.as_mut() {
            ui.prepare(
                &amp;state.gpu.ctx,
                &amp;<span class="k">self</span>.context,
                output,
                [state.gpu.config.width, state.gpu.config.height],
            );
        }

        repaint
    }

    <span class="c">/// Write the panel state onto the canvas for browser tests.</span>
    <span class="k">fn</span> publish(&amp;<span class="k">self</span>) {
        <span class="k">if</span> <span class="k">self</span>.controls.is_some()
            &amp;&amp; <span class="k">let</span> Some(canvas) = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.get_element_by_id(&quot;<span class="s">canvas</span>&quot;))
        {
            <span class="k">let</span> snapshot = MODEL.with_borrow(|model| serde_json::json!({&quot;<span class="s">framework</span>&quot;: &quot;<span class="s">egui 0.34.3</span>&quot;, &quot;<span class="s">controls</span>&quot;: <span class="k">self</span>.controls, &quot;<span class="s">command_open</span>&quot;: model.command_open, &quot;<span class="s">layers_open</span>&quot;: model.layers_open, &quot;<span class="s">command</span>&quot;: model.command, &quot;<span class="s">history</span>&quot;: model.history}));
            <span class="k">let</span> _ = canvas.set_attribute(&quot;<span class="s">data-viewer-ui</span>&quot;, &amp;snapshot.to_string());
        }

        <span class="k">if</span> <span class="k">let</span> Some(status) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(&quot;<span class="s">viewer-status</span>&quot;))
        {
            <span class="k">let</span> hidden = MODEL.with_borrow(|model| model.command_open);

            <span class="k">if</span> hidden {
                <span class="k">let</span> _ = status.set_attribute(&quot;<span class="s">hidden</span>&quot;, &quot;&quot;);
            } <span class="k">else</span> {
                <span class="k">let</span> _ = status.remove_attribute(&quot;<span class="s">hidden</span>&quot;);
            }
        }
    }
}

<span class="c">/// The white theme.</span>
<span class="k">fn</span> visuals() -&gt; egui::Visuals {
    <span class="k">let</span> <span class="k">mut</span> visuals = egui::Visuals::light();
    visuals.override_text_color = Some(egui::Color32::BLACK);
    visuals.window_fill = egui::Color32::WHITE;
    visuals.panel_fill = egui::Color32::WHITE;
    visuals.extreme_bg_color = egui::Color32::WHITE;
    visuals.selection.bg_fill = egui::Color32::BLACK;
    visuals.selection.stroke = egui::Stroke::new(<span class="s">1</span>.<span class="s">0_f32</span>, egui::Color32::WHITE);
    visuals.indent_has_left_vline = <span class="s">false</span>;

    <span class="k">for</span> widget <span class="k">in</span> [
        &amp;<span class="k">mut</span> visuals.widgets.noninteractive,
        &amp;<span class="k">mut</span> visuals.widgets.inactive,
        &amp;<span class="k">mut</span> visuals.widgets.hovered,
        &amp;<span class="k">mut</span> visuals.widgets.active,
        &amp;<span class="k">mut</span> visuals.widgets.open,
    ] {
        widget.bg_fill = egui::Color32::WHITE;
        widget.weak_bg_fill = egui::Color32::WHITE;
        widget.fg_stroke.color = egui::Color32::BLACK;
    }

    visuals
}

<span class="c">/// Remember one control's rectangle, when inspecting.</span>
<span class="k">fn</span> record(controls: &amp;<span class="k">mut</span> Option&lt;Vec&lt;Control&gt;&gt;, key: &amp;str, label: &amp;str, response: &amp;egui::Response) {
    <span class="k">let</span> Some(controls) = controls.as_mut() <span class="k">else</span> {
        <span class="k">return</span>;
    };
    <span class="k">let</span> r = response.rect;
    controls.push(Control {
        key: key.to_string(),
        label: label.to_string(),
        rect: [r.min.x, r.min.y, r.max.x, r.max.y],
    });
}

<span class="c">/// The layers panel; a click sets \`action\`.</span>
<span class="k">fn</span> layers(
    context: &amp;egui::Context, <span class="c">// the egui frame</span>
    model: &amp;<span class="k">mut</span> Model,
    controls: &amp;<span class="k">mut</span> Option&lt;Vec&lt;Control&gt;&gt;,
    action: &amp;<span class="k">mut</span> Option&lt;String&gt;,
) {
    <span class="k">if</span> !model.layers_open {
        <span class="k">return</span>;
    }

    egui::Window::new(&quot;<span class="s">Session layers</span>&quot;)
        .default_pos([<span class="s">12</span>.<span class="s">0</span>, <span class="s">12</span>.<span class="s">0</span>])
        .default_width(<span class="s">310</span>.<span class="s">0</span>)
        .resizable(<span class="s">false</span>)
        .collapsible(<span class="s">false</span>)
        .open(&amp;<span class="k">mut</span> model.layers_open)
        .show(context, |ui| {
            <span class="c">// one line per row</span>
            egui::ScrollArea::vertical()
                .max_height(context.content_rect().height() * <span class="s">0</span>.<span class="s">65</span>)
                .show(ui, |ui| {
                    <span class="k">let</span> <span class="k">mut</span> at = <span class="s">0</span>;

                    <span class="k">while</span> at &lt; model.rows.len() {
                        <span class="k">let</span> start = at;
                        <span class="k">let</span> id = model.rows[at].key.split_once('<span class="s">/</span>').map(|(_, id)| id);
                        at += <span class="s">1</span>;

                        <span class="k">if</span> id.is_some() {
                            <span class="k">while</span> at &lt; model.rows.len()
                                &amp;&amp; model.rows[at].key.split_once('<span class="s">/</span>').map(|(_, id)| id) == id
                            {
                                at += <span class="s">1</span>;
                            }
                        }

                        ui.horizontal(|ui| {
                            <span class="k">let</span> first = &amp;model.rows[start];
                            <span class="k">let</span> indent = first.label.len() - first.label.trim_start().len();
                            ui.add_space(indent <span class="k">as</span> f32 * <span class="s">4</span>.<span class="s">0</span>);

                            <span class="k">for</span> row <span class="k">in</span> &amp;model.rows[start..at] {
                                <span class="k">let</span> response = layer_button(ui, row);
                                record(controls, &amp;row.key, &amp;row.label, &amp;response);

                                <span class="k">if</span> response.clicked() {
                                    *action = Some(row.key.clone());
                                }
                            }
                        });
                    }
                });
        });
}

<span class="c">/// One layer row as a button.</span>
<span class="k">fn</span> layer_button(ui: &amp;<span class="k">mut</span> egui::Ui, row: &amp;LayerRow) -&gt; egui::Response {
    <span class="k">let</span> label = row.label.trim_start();

    <span class="k">if</span> row.key.starts_with(&quot;<span class="s">open/</span>&quot;) {
        <span class="k">let</span> (rect, response) = ui.allocate_exact_size(egui::vec2(<span class="s">12</span>.<span class="s">0</span>, <span class="s">18</span>.<span class="s">0</span>), egui::Sense::click());
        <span class="k">let</span> c = rect.center();
        <span class="k">let</span> points = <span class="k">if</span> label.starts_with('<span class="s">▾</span>') {
            vec![
                c + egui::vec2(-<span class="s">4</span>.<span class="s">0</span>, -<span class="s">2</span>.<span class="s">0</span>),
                c + egui::vec2(<span class="s">4</span>.<span class="s">0</span>, -<span class="s">2</span>.<span class="s">0</span>),
                c + egui::vec2(<span class="s">0</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>),
            ]
        } <span class="k">else</span> {
            vec![
                c + egui::vec2(-<span class="s">2</span>.<span class="s">0</span>, -<span class="s">4</span>.<span class="s">0</span>),
                c + egui::vec2(-<span class="s">2</span>.<span class="s">0</span>, <span class="s">4</span>.<span class="s">0</span>),
                c + egui::vec2(<span class="s">3</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            ]
        };
        ui.painter().add(egui::Shape::convex_polygon(
            points,
            egui::Color32::BLACK,
            egui::Stroke::NONE,
        ));
        <span class="k">return</span> response;
    }

    <span class="k">let</span> label: String = label.chars().take(<span class="s">160</span>).collect();
    <span class="k">let</span> text = <span class="k">if</span> row.key.starts_with(&quot;<span class="s">hide/</span>&quot;) {
        <span class="k">if</span> row.hidden {
            &quot;<span class="s">Show</span>&quot;.to_string()
        } <span class="k">else</span> {
            &quot;<span class="s">Hide</span>&quot;.to_string()
        }
    } <span class="k">else</span> {
        format!(&quot;{}<span class="s"> (</span>{}<span class="s">)</span>&quot;, label.trim_start_matches(&quot;<span class="s">Select </span>&quot;), row.count)
    };
    ui.button(text).on_hover_text(label)
}

<span class="c">/// The command dock; an executed line goes to \`command\`.</span>
<span class="k">fn</span> commands(
    context: &amp;egui::Context, <span class="c">// the egui frame</span>
    model: &amp;<span class="k">mut</span> Model,
    controls: &amp;<span class="k">mut</span> Option&lt;Vec&lt;Control&gt;&gt;,
    command: &amp;<span class="k">mut</span> Option&lt;String&gt;,
) {
    <span class="k">if</span> !model.command_open {
        <span class="k">return</span>;
    }

    <span class="k">let</span> <span class="k">mut</span> open = model.command_open;
    egui::Window::new(&quot;<span class="s">Command line</span>&quot;)
        .anchor(egui::Align2::LEFT_BOTTOM, [<span class="s">12</span>.<span class="s">0</span>, -<span class="s">12</span>.<span class="s">0</span>])
        .default_width(<span class="s">480</span>.<span class="s">0</span>)
        .resizable(<span class="s">false</span>)
        .collapsible(<span class="s">false</span>)
        .open(&amp;<span class="k">mut</span> open)
        .show(context, |ui| {
            <span class="k">for</span> text <span class="k">in</span> &amp;model.history {
                ui.label(text);
            }

            ui.label(&quot;<span class="s">World coordinates: x,y,z. Select a curve before trim, extend or explode.</span>&quot;);
            <span class="k">let</span> response = ui.add(
                egui::TextEdit::singleline(&amp;<span class="k">mut</span> model.command)
                    .char_limit(<span class="s">2048</span>)
                    .hint_text(
                        egui::RichText::new(&quot;<span class="s">line 0,0,0 100,0,0</span>&quot;).color(egui::Color32::BLACK),
                    )
                    .desired_width(f32::INFINITY),
            );
            record(controls, &quot;<span class="s">command/input</span>&quot;, &quot;<span class="s">Command</span>&quot;, &amp;response);

            <span class="k">if</span> model.focus_command {
                response.request_focus();
                model.focus_command = <span class="s">false</span>;
            }

            <span class="k">let</span> enter =
                response.lost_focus() &amp;&amp; ui.input(|input| input.key_pressed(egui::Key::Enter));
            ui.horizontal(|ui| {
                <span class="k">let</span> run = ui.button(&quot;<span class="s">Run</span>&quot;);
                record(controls, &quot;<span class="s">command/run</span>&quot;, &quot;<span class="s">Run</span>&quot;, &amp;run);

                <span class="k">if</span> (enter || run.clicked()) &amp;&amp; !model.command.trim().is_empty() {
                    *command = Some(std::mem::take(&amp;<span class="k">mut</span> model.command));
                }

                <span class="k">let</span> close = ui.button(&quot;<span class="s">Close (Esc)</span>&quot;);
                record(controls, &quot;<span class="s">command/close</span>&quot;, &quot;<span class="s">Close</span>&quot;, &amp;close);

                <span class="k">if</span> close.clicked() || ui.input(|input| input.key_pressed(egui::Key::Escape)) {
                    model.command_open = <span class="s">false</span>;
                }

                ui.label(&quot;<span class="s">point · line · polyline · trim · extend · explode · undo</span>&quot;);
            });

            <span class="k">if</span> !model.status.is_empty() {
                ui.label(&amp;model.status);
            }
        });
    model.command_open &amp;= open;
}</code></pre></div>
<h2 id="step-7-srcenginegpumodrs">Step 7 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/27-egui-interface#step-7-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>Add the new GPU resources, initialize them and include their allocations in the counters.</p>
<p><code>lessons/27/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>mod triangle_tiles;</code> in <code>lessons/26/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> ui;</code></pre></div>
<p>Added after the line <code>pub widget: widget::Widget,</code> in <code>lessons/26/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> ui: Option&lt;ui::Ui&gt;,</code></pre></div>
<p>Added after the line <code>widget,</code> in <code>lessons/26/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            ui: None,</code></pre></div>
<h2 id="step-8-srcenginegpurenderrs">Step 8 · src/engine/gpu/render.rs<a class="anchor" href="#/course/27-egui-interface#step-8-srcenginegpurenderrs" aria-label="Link to this section">#</a></h2>
<p>Place the new drawing work into the frame sequence.</p>
<p><code>lessons/27/src/engine/gpu/render.rs</code> · edit · type this</p>
<p>Added after the line <code>draws += self.widget.draw(encoder, view, &amp;self.targets);</code> in <code>lessons/26/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> <span class="k">let</span> Some(ui) = <span class="k">self</span>.ui.as_ref() {
            ui.draw(encoder, view);
        }</code></pre></div>
<h2 id="step-9-srcenginegpuuirs">Step 9 · src/engine/gpu/ui.rs<a class="anchor" href="#/course/27-egui-interface#step-9-srcenginegpuuirs" aria-label="Link to this section">#</a></h2>
<p>Upload egui textures and triangles, render their clipped ranges, and release requested textures.</p>
<p><code>lessons/27/src/engine/gpu/ui.rs</code> · 103 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::buffers::GpuCtx;

<span class="c">/// Draws the egui interface over the frame.</span>
<span class="k">pub</span> <span class="k">struct</span> Ui {
    renderer: egui_wgpu::Renderer, <span class="c">// egui's wgpu renderer</span>
    jobs: Vec&lt;egui::ClippedPrimitive&gt;, <span class="c">// triangles to draw this frame</span>
    screen: egui_wgpu::ScreenDescriptor, <span class="c">// canvas size and scale</span>
    free: Vec&lt;egui::TextureId&gt;, <span class="c">// textures to free next frame</span>
    textures: std::collections::HashSet&lt;egui::TextureId&gt;, <span class="c">// textures alive now</span>
}

<span class="k">impl</span> Ui {
    <span class="c">/// Create the renderer for the canvas format.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, format: wgpu::TextureFormat) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            renderer: egui_wgpu::Renderer::new(
                &amp;ctx.device,
                format,
                egui_wgpu::RendererOptions::default(),
            ),
            jobs: Vec::new(),
            screen: egui_wgpu::ScreenDescriptor {
                size_in_pixels: [<span class="s">1</span>, <span class="s">1</span>],
                pixels_per_point: <span class="s">1</span>.<span class="s">0</span>,
            },
            free: Vec::new(),
            textures: Default::default(),
        }
    }

    <span class="c">/// Upload this frame's egui output: textures, then triangles.</span>
    <span class="k">pub</span> <span class="k">fn</span> prepare(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        context: &amp;egui::Context, <span class="c">// the egui frame</span>
        output: egui::FullOutput,
        size: [u32; <span class="s">2</span>],
    ) {
        <span class="c">// free last frame's textures</span>
        <span class="k">for</span> id <span class="k">in</span> <span class="k">self</span>.free.drain(..) {
            <span class="k">self</span>.renderer.free_texture(&amp;id);
            <span class="k">self</span>.textures.remove(&amp;id);
        }

        <span class="c">// upload new or changed textures</span>
        <span class="k">for</span> (id, delta) <span class="k">in</span> &amp;output.textures_delta.set {
            <span class="k">self</span>.textures.insert(*id);
            <span class="k">self</span>.renderer
                .update_texture(&amp;ctx.device, &amp;ctx.queue, *id, delta);
        }

        <span class="k">self</span>.free = output.textures_delta.free;
        <span class="c">// shapes to triangles</span>
        <span class="k">self</span>.jobs = context.tessellate(output.shapes, output.pixels_per_point);

        <span class="k">if</span> <span class="k">self</span>.jobs.is_empty() {
            <span class="k">return</span>;
        }

        <span class="k">self</span>.screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: output.pixels_per_point,
        };
        <span class="c">// upload the vertex buffers</span>
        <span class="k">let</span> <span class="k">mut</span> encoder = ctx
            .device
            .create_command_encoder(&amp;wgpu::CommandEncoderDescriptor {
                label: Some(&quot;<span class="s">egui upload</span>&quot;),
            });
        <span class="k">let</span> <span class="k">mut</span> buffers = <span class="k">self</span>.renderer.update_buffers(
            &amp;ctx.device,
            &amp;ctx.queue,
            &amp;<span class="k">mut</span> encoder,
            &amp;<span class="k">self</span>.jobs,
            &amp;<span class="k">self</span>.screen,
        );
        buffers.push(encoder.finish());
        ctx.queue.submit(buffers);
    }

    <span class="c">/// Draw the interface over the frame.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw(&amp;<span class="k">self</span>, encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder, view: &amp;wgpu::TextureView) {
        <span class="k">if</span> <span class="k">self</span>.jobs.is_empty() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> pass = encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
            label: Some(&quot;<span class="s">egui</span>&quot;),
            color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        <span class="k">self</span>.renderer
            .render(&amp;<span class="k">mut</span> pass.forget_lifetime(), &amp;<span class="k">self</span>.jobs, &amp;<span class="k">self</span>.screen);
    }
}

<span class="c">/// Free every texture.</span>
<span class="k">impl</span> Drop <span class="k">for</span> Ui {
    <span class="c">/// Remove the DOM listener.</span>
    <span class="k">fn</span> drop(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">for</span> id <span class="k">in</span> &amp;<span class="k">self</span>.textures {
            <span class="k">self</span>.renderer.free_texture(id);
        }
    }
}</code></pre></div>
<h2 id="step-10-srclibrs">Step 10 · src/lib.rs<a class="anchor" href="#/course/27-egui-interface#step-10-srclibrs" aria-label="Link to this section">#</a></h2>
<p>Connect browser events, scene changes and drawing through the application state.</p>
<p><code>lessons/27/src/lib.rs</code> · edit · type this</p>
<p>Delete the 2 lines from <code>Command(String),</code> in <code>lessons/26/src/lib.rs</code>.</p>
<p>Replaces the 2 lines from <code>command_keys: Option&lt;app::input::CommandKeys&gt;,</code> in <code>lessons/26/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    ui: Option&lt;app::ui::Ui&gt;,</code></pre></div>
<p>Replaces the 2 lines from <code>command_keys: None,</code> in <code>lessons/26/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            ui: None,</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/26/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.ui = Some(app::ui::Ui::new(&amp;state.window));
        state.gpu.ui = Some(engine::gpu::ui::Ui::new(
            &amp;state.gpu.ctx,
            state.gpu.config.format,
        ));</code></pre></div>
<p>Delete the 12 lines from <code>if let Some(input) = app::feedback::command_line(false) {</code> in <code>lessons/26/src/lib.rs</code>.</p>
<p>Delete the 10 lines from <code>Msg::Command(line) =&gt; {</code> in <code>lessons/26/src/lib.rs</code>.</p>
<p>Added after the line <code>let Some(state) = &amp;mut self.state else { return };</code> in <code>lessons/26/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// the panels get the event first</span>
        <span class="k">if</span> <span class="k">let</span> Some(ui) = <span class="k">self</span>.ui.as_mut() {
            <span class="k">let</span> (<span class="k">mut</span> consumed, repaint) = ui.event(&amp;state.window, &amp;event);

            <span class="c">// keys reach the viewer unless the command line is open</span>
            <span class="k">if</span> matches!(event, WindowEvent::KeyboardInput { .. })
                &amp;&amp; !app::ui::MODEL.with_borrow(|model| model.command_open)
            {
                consumed = <span class="s">false</span>;
            }

            <span class="k">if</span> repaint {
                state.request_frame();
            }

            <span class="k">if</span> consumed {
                <span class="c">// a release inside a panel ends any viewer drag</span>
                <span class="k">if</span> matches!(
                    event,
                    WindowEvent::MouseInput {
                        state: ElementState::Released,
                        ..
                    }
                ) {
                    <span class="k">self</span>.input.cancel();
                    state.cancel_gesture();
                }

                <span class="k">self</span>.request_if_needed();
                <span class="k">return</span>;
            }
        }

        <span class="c">// true when the scene must be drawn again</span></code></pre></div>
<p>Added after the line <code>} else {</code> in <code>lessons/26/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    <span class="c">// panels lay out, then the scene draws</span>
                    <span class="k">let</span> repaint = <span class="k">self</span>.ui.as_mut().is_some_and(|ui| ui.frame(state));
                    state.render();

                    <span class="k">if</span> repaint {
                        state.request_frame();
                    }</code></pre></div>
<h2 id="step-11-srcstaters">Step 11 · src/state.rs<a class="anchor" href="#/course/27-egui-interface#step-11-srcstaters" aria-label="Link to this section">#</a></h2>
<p>Expose a frame request for interface changes.</p>
<p><code>lessons/27/src/state.rs</code> · edit · type this</p>
<p>Added after the line <code>self.request_selection(x, y, false, false);</code> in <code>lessons/26/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The picture changed: draw on the next redraw.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> request_frame(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.dirty = <span class="s">true</span>;
        <span class="k">self</span>.needs_frame = <span class="s">true</span>;
    }

    <span class="c">/// Something changed: drop pending picks, draw again.</span></code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/27-egui-interface#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/27/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: a command runs in a white egui window and reports <strong>geometry updated</strong> above the input.</p>
<p><img src="/session/docs/course/docs/screenshots/extensions-command-create.png" alt="Full viewer result for lesson 27" loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>Typing triggers scene shortcuts: egui input is also forwarded to the scene.</li>
<li>A UI update causes a borrow panic: its action runs inside the model borrow.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/27-egui-interface#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/27/src/
├── app/
│   ├── inspection/
│   │   └── source_memory.rs
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs
│   │   ├── brep_edges.rs
│   │   ├── brep_orient.rs
│   │   ├── cloud.rs
│   │   ├── curves.rs
│   │   ├── encode.rs
│   │   ├── frames.rs
│   │   ├── mesh.rs
│   │   ├── mesh_ink.rs
│   │   ├── mesh_topology.rs
│   │   ├── mod.rs
│   │   ├── points.rs
│   │   └── sheet.rs
│   ├── cloud_query.rs
│   ├── command.rs
│   ├── coords.rs
│   ├── cplane.rs
│   ├── decode.rs
│   ├── edit.rs
│   ├── feedback.rs  ~
│   ├── fetch.rs
│   ├── gizmo.rs
│   ├── hierarchy.rs
│   ├── input.rs  ~
│   ├── inspection.rs
│   ├── knobs.rs
│   ├── layers.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mod.rs  ~
│   ├── modeling.rs
│   ├── route.rs
│   ├── scene.rs
│   ├── scene_text.rs
│   ├── selection.rs
│   ├── sheet_query.rs
│   ├── snap.rs
│   ├── stream.rs
│   ├── touch.rs
│   ├── ui.rs  +
│   └── validate.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs
│   │   ├── backdrop.rs
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── device.rs
│   │   ├── faces.rs
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs
│   │   ├── pick.rs
│   │   ├── present.rs
│   │   ├── render.rs  ~
│   │   ├── segments.rs
│   │   ├── splat.rs
│   │   ├── surface_outline.rs
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
│   │   ├── triangle_tiles.rs
│   │   ├── ui.rs  +
│   │   ├── upload.rs
│   │   ├── view.rs
│   │   ├── widget.rs
│   │   └── widget_mesh.rs
│   ├── pipelines/
│   │   ├── layouts.rs
│   │   └── mod.rs
│   ├── mod.rs
│   ├── performance.rs
│   └── text.rs
├── shaders/
│   ├── background.wgsl
│   ├── glyph.wgsl
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl
│   ├── normals.wgsl
│   ├── physical.wgsl
│   ├── project_triangles.wgsl
│   ├── projected_triangle.wgsl
│   ├── ribbon.wgsl
│   ├── scan_triangle_tiles.wgsl
│   ├── scene.wgsl
│   ├── sphere.wgsl
│   ├── splat.wgsl
│   ├── splat_resolve.wgsl
│   ├── surface_outline.wgsl
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl
│   ├── text_plate.wgsl
│   ├── triangle.wgsl
│   ├── triangle_tiles.wgsl
│   └── widget.wgsl
├── state/
│   ├── cloud_query.rs
│   ├── edit.rs
│   ├── panel.rs
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs  ~
└── state.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: browser event → egui model → deferred action → GPU UI overlay.
Every file at this point: <code>lessons/27/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/27-egui-interface#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/28-editing-wiring">28 · Finish the shared editing wiring</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/27-egui-interface#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>The white egui command area shows a completed line command and its feedback, with the created geometry visible in the full viewer. The capture uses the maintained viewer and the <a href="/session/docs/course/docs/extensions/nested.pb">nested fixture</a>. The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in <a href="#/course/29-docked-workspace">checkpoint 8</a>.</p>
<p><a href="/session/docs/course/docs/screenshots/extensions-command-create.png"><img src="/session/docs/course/docs/screenshots/extensions-command-create.png" alt="Full viewer result for lesson 27" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-cargotoml",text:"Step 1 · Cargo.toml"},{level:2,id:"step-2-indexhtml",text:"Step 2 · index.html"},{level:2,id:"step-3-srcappfeedbackrs",text:"Step 3 · src/app/feedback.rs"},{level:2,id:"step-4-srcappinputrs",text:"Step 4 · src/app/input.rs"},{level:2,id:"step-5-srcappmodrs",text:"Step 5 · src/app/mod.rs"},{level:2,id:"step-6-srcappuirs",text:"Step 6 · src/app/ui.rs"},{level:2,id:"step-7-srcenginegpumodrs",text:"Step 7 · src/engine/gpu/mod.rs"},{level:2,id:"step-8-srcenginegpurenderrs",text:"Step 8 · src/engine/gpu/render.rs"},{level:2,id:"step-9-srcenginegpuuirs",text:"Step 9 · src/engine/gpu/ui.rs"},{level:2,id:"step-10-srclibrs",text:"Step 10 · src/lib.rs"},{level:2,id:"step-11-srcstaters",text:"Step 11 · src/state.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
