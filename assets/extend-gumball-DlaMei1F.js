const e={title:"Adding a gumball",html:`<h1 id="adding-a-gumball">Adding a gumball<a class="anchor" href="#/course/extend-gumball#adding-a-gumball" aria-label="Link to this section">#</a></h1>
<p>Start here: <a href="#/course/25-gumball">Implement a solid gumball, step by step</a>. It includes every code edit, the mesh shader and a real screenshot. The designs below are historical; use the supplement’s status table for supported operations and limits.</p>
<div class="note"><p class="note-title">Built in lesson 21</p><p>This was the design; <code>src/app/gizmo.rs</code> and <code>src/state/edit.rs</code> are what it became, and
<a href="#/course/21-editing">lesson 21</a> teaches them. Two decisions went the other way in the end: the
widget draws in the two lane types the control net already uses rather than in a private
lane, and the drag is committed once on release rather than per frame. Read this for the
reasoning; read the code for what runs.</p>
</div>
<h2 id="the-widget">The widget<a class="anchor" href="#/course/extend-gumball#the-widget" aria-label="Link to this section">#</a></h2>
<ul>
<li>Transform gizmo on the selected object: three arrows translate along a world axis, three quarter-arcs rotate about one, three balls scale along one, a centre ball scales uniformly.</li>
<li>Screen-constant at any zoom, depth and projection: a control that shrinks with distance stops being grabbable exactly when the object needs it.</li>
<li>First thing here that <strong>changes a document</strong> — <code>Session::set_xform</code> inside a transaction. <code>history.rs</code> already stores <code>Op::Xform</code> absolute before/after, and <code>set_xform</code> records it whenever a transaction is open (<code>session_rust/src/session.rs</code>), so the viewer&#39;s commit is three lines.</li>
</ul>
<h2 id="where-every-piece-goes">Where every piece goes<a class="anchor" href="#/course/extend-gumball#where-every-piece-goes" aria-label="Link to this section">#</a></h2>
<table>
<thead>
<tr>
<th>file</th>
<th>what lands there</th>
</tr>
</thead>
<tbody><tr>
<td><code>src/app/gizmo.rs</code> (new)</td>
<td><code>Handle</code>, handle geometry, screen-space hit test, drag laws. Pure CPU: no wgpu, no kernel, no <code>State</code>.</td>
</tr>
<tr>
<td><code>src/engine/gpu/gizmo.rs</code> (new)</td>
<td>The lane: two <code>GrowBuf</code> tables, its one-row instance group, pipelines, <code>draw</code>.</td>
</tr>
<tr>
<td><code>src/shaders/gizmo.wgsl</code> (new)</td>
<td>Vertex/fragment pair on the scene contract.</td>
</tr>
<tr>
<td><code>src/math.rs</code></td>
<td><code>mat_translation</code>, <code>mat_rotation_about</code>, <code>mat_scale_about</code>, an <code>Aabb</code> midpoint.</td>
</tr>
<tr>
<td><code>src/camera.rs</code></td>
<td><code>world_per_px(at, vp_h)</code>, <code>ray_at(cursor, viewport, aspect)</code>.</td>
</tr>
<tr>
<td><code>src/engine/gpu/objects.rs</code></td>
<td><code>set_translation</code>, <code>set_place</code>, the retained local box per row.</td>
</tr>
<tr>
<td><code>src/engine/gpu/mod.rs</code></td>
<td><code>pub gizmo: GizmoLane</code> beside <code>controls</code>; <code>set_place</code> / <code>set_translation</code> beside <code>set_selected</code>.</td>
</tr>
<tr>
<td><code>src/engine/gpu/render.rs</code></td>
<td>Two draws at the end of <code>scene_list</code> (<code>render.rs</code>).</td>
</tr>
<tr>
<td><code>src/state.rs</code></td>
<td><code>gizmo</code> field beside <code>controls</code> (<code>state.rs</code>), the named actions, the per-frame rebuild in <code>render</code>.</td>
</tr>
<tr>
<td><code>src/app/input.rs</code></td>
<td>The gesture: press, drag, hover, release, cancel, Escape, Ctrl+Z.</td>
</tr>
<tr>
<td><code>src/app/scene.rs</code></td>
<td><code>Scene::place_object</code> — the <code>Rc::make_mut</code> split, one kernel transaction.</td>
</tr>
<tr>
<td><code>src/app/inspection.rs</code></td>
<td>A <code>&quot;gizmo&quot;</code> key beside <code>&quot;model&quot;</code>.</td>
</tr>
</tbody></table>
<ul>
<li>Nothing in <code>pick.rs</code> changes, nothing in <code>id_pass</code>; reasons under hit testing.</li>
</ul>
<h2 id="the-handle-set">The handle set<a class="anchor" href="#/course/extend-gumball#the-handle-set" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch</span>
<span class="k">pub</span> <span class="k">enum</span> Handle {
 TranslateX, TranslateY, TranslateZ,
 RotateX, RotateY, RotateZ,
 ScaleX, ScaleY, ScaleZ,
 ScaleUniform,
}

<span class="k">impl</span> Handle {
 <span class="c">/// (verb, axis, unit) — title of the numeric entry. Axis is &quot;&quot; for uniform scale.</span>
 <span class="k">pub</span> <span class="k">fn</span> labels(<span class="k">self</span>) -&gt; (&amp;'static str, &amp;'static str, &amp;'static str);
}</code></pre></div>
<ul>
<li>Ten handles, no plane quads: a plane translate is two axis drags, and quads would crowd where arcs and arrows already make a pick ambiguous.</li>
<li>No free-move on the centre ball in v1: one ball cannot be both uniform scale and screen-plane translate. Later a modifier, not an eleventh handle.</li>
<li>World-axis only: the drag maths stays independent of the object, so a delta is a plain world matrix for one row or many. <code>gpu.objects.row(row).model</code> makes an object-aligned mode cheap later.</li>
</ul>
<h2 id="screen-constant-size">Screen-constant size<a class="anchor" href="#/course/extend-gumball#screen-constant-size" aria-label="Link to this section">#</a></h2>
<ul>
<li>ONE length in CSS px, everything else a fraction of it, so retuning is one number:</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch, all CSS px</span>
<span class="k">const</span> ARC_R: f64 = <span class="s">72</span>.<span class="s">0</span>; <span class="c">// the one length: arc radius and arrow tip</span>
<span class="k">const</span> AXIS_BALL: f64 = ARC_R * <span class="s">0</span>.<span class="s">5</span>;
<span class="k">const</span> BALL_R: f64 = <span class="s">5</span>.<span class="s">0</span>;
<span class="k">const</span> SHAFT_HW: f64 = <span class="s">2</span>.<span class="s">0</span>;
<span class="k">const</span> GRAB: f64 = <span class="s">8</span>.<span class="s">0</span>;
<span class="k">const</span> ARC_CHORDS: usize = <span class="s">16</span>;</code></pre></div>
<ul>
<li><code>ARC_R = 72</code> spans 144 px on a 400 CSS px phone canvas, a third of the width: grabbable, object still visible.</li>
<li>Axis balls at half the arc radius: 36 px from the centre ball and from the arrow tip, over 4× <code>GRAB</code>, so collinear handles never conflict. They sit on the <strong>positive</strong> axis with the arrows, so pulling outward grows the object and press distance stays positive — no sign-preserving floor anywhere.</li>
<li><code>BALL_R = 5</code> beats the 3.5 px control dot (<code>state.rs:596</code>): a handle never reads as a control point.</li>
<li><code>GRAB = 8</code> beats <code>PICK_RADIUS = 6</code> (<code>pick.rs:34</code>) — grabbed, not aimed at — and <code>CLICK_SLOP = 4</code> (<code>input.rs:17</code>), so a click-length press stays on its handle.</li>
<li>16 chords per quarter arc: sagitta <code>r(1 − cos(θ/2))</code>, <code>θ = π/32</code>, is 0.087 px at <code>r = 72</code>.</li>
<li>51 <code>CylinderSegment</code> (3 shafts + 48 chords) + 4 <code>GlyphPoint</code> = 55 rows, rebuilt every frame; nothing to cache or invalidate.</li>
</ul>
<h3 id="world-units-per-pixel">World units per pixel<a class="anchor" href="#/course/extend-gumball#world-units-per-pixel" aria-label="Link to this section">#</a></h3>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch, Camera, world units (mm), vp_h in framebuffer px</span>
<span class="k">fn</span> world_per_px(&amp;<span class="k">self</span>, at: [f64; <span class="s">3</span>], vp_h: f64) -&gt; f64 {
 <span class="k">let</span> d = <span class="k">if</span> <span class="k">self</span>.perspective { depth_along_view(at) } <span class="k">else</span> { <span class="k">self</span>.distance_world };
 <span class="s">2</span>.<span class="s">0</span> * d * (FOVY_DEG * <span class="s">0</span>.<span class="s">5</span>).to_radians.tan / vp_h
}</code></pre></div>
<ul>
<li>Ortho is depth-free: <code>view_proj_anchored</code> half-height is <code>dist * tan(FOVY/2)</code> (<code>camera.rs</code>).</li>
<li>Perspective takes the gizmo origin&#39;s own depth: the frustum widens with depth, so <code>distance</code> resizes the widget as you orbit an off-centre object.</li>
<li><code>vp_h</code> is <code>gpu.config.height</code>, framebuffer px; CSS constants scale by <code>f64::from(config.width) / logical_size[0]</code> first, as <code>upload_controls</code> does (<code>state.rs</code>).</li>
<li><code>FOVY_DEG</code> lives once (<code>math.rs</code>). <code>target</code>/<code>distance</code> are metres, <code>origin</code>/<code>distance_world</code> world units (<code>camera.rs</code>); mixing them silently disables camera-relative rendering.</li>
</ul>
<h2 id="drawing-it">Drawing it<a class="anchor" href="#/course/extend-gumball#drawing-it" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch</span>
<span class="k">pub</span> <span class="k">struct</span> GizmoLane {
 bars: GrowBuf, <span class="c">// CylinderSegment rows: shafts and arc chords</span>
 dots: GrowBuf, <span class="c">// GlyphPoint rows: the four balls</span>
 bars_group: wgpu::BindGroup, <span class="c">// group 3, l.ink_rows</span>
 dots_group: wgpu::BindGroup, <span class="c">// group 3, same layout</span>
 instance: wgpu::Buffer, <span class="c">// ONE Instance row</span>
 translation: wgpu::Buffer, <span class="c">// ONE anchored translation</span>
 group: wgpu::BindGroup, <span class="c">// group 2, l.instance</span>
 gpu: GizmoPipelines,
}</code></pre></div>
<ul>
<li>No new row type: <code>CylinderSegment</code> (<code>segments.rs</code>, 40 B) for shaft and chord, <code>GlyphPoint</code> (<code>glyphs.rs</code>, 48 B) for ball, as <code>upload_controls</code> fills (<code>state.rs</code>). Group 3 reuses <code>Layouts.ink_rows</code>, no new layout.</li>
<li>Pipelines from <code>scene_module</code> (<code>pipelines/mod.rs:191</code>), <strong>not</strong> <code>ink_module</code>: ink fragments are gated on <code>ink_disc_visible</code> (<code>glyph.wgsl</code>), so a handle behind a face would vanish. Hence not a <code>GlyphLane</code>/<code>SegmentLane</code> pair like <code>controls</code>.</li>
<li>Two more reasons for a private lane: the ink lanes bind <code>objects.ink_group</code> at group 2, not a one-row group; and <code>msaa_now</code> (<code>src/engine/gpu/mod.rs</code>) flips the canvas to 4× MSAA once <code>glyphs.sphere_count &gt; 0</code>, so balls in the shared glyph lane would change every frame&#39;s antialiasing.</li>
<li><code>DepthMode::Always</code> (<code>pipelines/mod.rs:38</code>), already used for marker discs, gives always-on-top with no private pass. It also draws over itself: CPU-sort the 55 rows back-to-front before upload, rather than add a depth attachment and a pass.</li>
<li>Wiring: field beside <code>controls</code> (<code>src/engine/gpu/mod.rs</code>), into <code>build</code>, <code>retarget</code>, <code>reset</code>, <code>release</code>, <code>allocated_bytes</code>; two draws after <code>self.text.draw(pass)</code> at the end of <code>scene_list</code> (<code>render.rs</code>), over authored text too; <code>SHADERS</code> into <code>lane_shaders</code> (<code>src/engine/gpu/mod.rs</code>) for the layout mirror test.</li>
</ul>
<h3 id="two-shader-conventions-that-will-catch-you">Two shader conventions that will catch you<a class="anchor" href="#/course/extend-gumball#two-shader-conventions-that-will-catch-you" aria-label="Link to this section">#</a></h3>
<ul>
<li><code>GlyphPoint.radius &lt; 0</code> means <strong>pixels</strong> (<code>glyph.wgsl:52</code>): the balls use a negative, CSS-scaled radius.</li>
<li><code>CylinderSegment</code> has no pixel mode — <code>half_width_px</code> returns the global pen for radius ≤ 0 (<code>ribbon.wgsl</code>) — so shafts and arcs take a positive world radius, <code>SHAFT_HW × scale × world_per_px</code>, recomputed each frame.</li>
<li>The instance row needs <code>flags = 0</code>, <code>color = [1,1,1,1]</code>: both shaders compute <code>g.color * inst.color</code>, then substitute <code>SELECT_COLOR</code> under <code>FLAG_SELECTED</code> (<code>glyph.wgsl</code>, <code>ribbon.wgsl</code>). A borrowed selected row turns the widget yellow.</li>
</ul>
<h2 id="anchoring-it">Anchoring it<a class="anchor" href="#/course/extend-gumball#anchoring-it" aria-label="Link to this section">#</a></h2>
<ul>
<li>Shaders place a point as <code>place(i, p) = instances[i].model * p + translations[i]</code> (<code>scene.wgsl:57</code>), <code>translations[i] = (world − anchor) as f32</code>. A kilometre out, absolute f32 loses a millimetre (<code>objects.rs</code> test <code>the_anchor_is_what_keeps_a_small_move</code>) and drifts again on every re-anchor.</li>
<li>So the lane owns <strong>one</strong> instance row in <code>l.instance</code> (<code>layouts.rs:46</code>): identity model, <code>translation = (origin − gpu.objects.anchor) as f32</code>, flags 0, white. Handle vertices are <strong>local offsets about the origin</strong>, which the size formula already produces, and moving the widget is one 16-byte write.</li>
<li>Rewrite the origin after <code>rebase_anchor</code> in <code>State::render</code> (<code>state.rs</code>): a re-anchor changes what the offset means, and the scale is recomputed there too, from the frame&#39;s camera.</li>
</ul>
<h2 id="hit-testing-pixels-for-the-grab-a-ray-for-the-drag">Hit testing: pixels for the grab, a ray for the drag<a class="anchor" href="#/course/extend-gumball#hit-testing-pixels-for-the-grab-a-ray-for-the-drag" aria-label="Link to this section">#</a></h2>
<p><strong>Join the id pass?</strong> No:</p>
<ul>
<li>It is asynchronous: <code>Picker::request</code> (<code>pick.rs</code>) → <code>take_pending</code> next frame (<code>render.rs</code>) → <code>poll</code> a frame or more later (<code>state.rs</code>). A press must know instantly, or the object behind the handle is selected first.</li>
<li><code>State::touch</code> cancels any pick in flight (<code>state.rs:242</code>): a GPU-picking gizmo cancels its own question every frame of its own drag.</li>
<li>Hover fires on every <code>CursorMoved</code> (<code>input.rs</code>); a pass, a copy and an async map per pointer event is not a hover budget.</li>
<li>The id path earns its complexity on data the CPU lacks — half a million sheet segments behind one row, streamed cloud points (<code>pick.rs:1</code>). Ten analytic handles are the opposite case.</li>
</ul>
<p><strong>So:</strong> CPU, in <strong>screen space</strong> — project with the frame&#39;s own <code>gpu.frame.mvp_f32</code> and measure pixel distances, the space the widget is defined in.</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch</span>
<span class="k">pub</span> <span class="k">fn</span> hit(&amp;<span class="k">self</span>, cursor: [f64; <span class="s">2</span>], projected: &amp;Handles, grab_px: f64) -&gt; Option&lt;Handle&gt;;</code></pre></div>
<ul>
<li>Priority, first match wins: centre ball → axis balls → shafts → arcs. Shafts all start at the origin, so centre-first keeps the centre ball grabbable; arcs last, largest ambiguous area.</li>
<li>Arcs test as <strong>projected polylines</strong> over the 16 drawn chords: the angular bound is free, where a plane-radius test picks the three quarters that are not drawn.</li>
<li>Shafts test as projected 2D segments clamped to drawn length — no ray-parallel branch to get the sign wrong.</li>
<li><code>PickMode</code> (<code>pick.rs</code>) gains no arm and <code>id_pass</code> no draw: invisible to picks in all four modes. It is not an object.</li>
<li>Consequence: no occlusion test, so a handle behind a wall still picks. Right, because it also <em>draws</em> over the wall — drawn-always and picked-always must be one rule.</li>
</ul>
<h3 id="the-ray-the-drag-does-need">The ray the drag does need<a class="anchor" href="#/course/extend-gumball#the-ray-the-drag-does-need" aria-label="Link to this section">#</a></h3>
<ul>
<li>A drag needs a world answer — the cursor on the axis line, or crossing the drag plane. One ray per press and per drag frame, never per pointer event.</li>
<li>No matrix inverse: <code>Camera::zoom_at</code> (<code>camera.rs</code>) already builds the view-plane frame — <code>right</code>, <code>self.up</code>, <code>half_h = distance * tan(FOVY/2)</code>, <code>half_w = half_h * aspect</code>. The ortho branch of <code>view_proj_anchored</code> uses the same <code>h</code> (<code>camera.rs</code>), so one rectangle serves both projections.</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch, Camera, world units</span>
<span class="k">let</span> ndc = [<span class="s">2</span>.<span class="s">0</span> * cx / vp_w - <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span> - <span class="s">2</span>.<span class="s">0</span> * cy / vp_h];
<span class="k">let</span> on_plane = target + right * ndc[<span class="s">0</span>] * half_w + up * ndc[<span class="s">1</span>] * half_h;
<span class="c">// perspective: origin = eye, direction = normalize(on_plane - eye)</span>
<span class="c">// orthographic: origin = on_plane, direction = forward</span></code></pre></div>
<h2 id="the-drag-laws">The drag laws<a class="anchor" href="#/course/extend-gumball#the-drag-laws" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>Every frame computes an absolute delta from the press state, never an increment.</strong> A dropped frame changes nothing and no float drift accumulates.</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch</span>
<span class="k">pub</span> <span class="k">struct</span> Drag {
 handle: Handle,
 start_world: [f64; <span class="s">3</span>], <span class="c">// translate: point on the axis; uniform scale: the plane point</span>
 start_angle: f64, <span class="c">// rotate</span>
 start_dist: f64, <span class="c">// scale</span>
 plane: [f64; <span class="s">3</span>], <span class="c">// uniform scale: drag plane normal, frozen at press</span>
 pivot: [f64; <span class="s">3</span>],
 turns: f64, <span class="c">// rotate: accumulated winding, so a drag can exceed ±180°</span>
}

<span class="k">pub</span> <span class="k">fn</span> begin_drag(handle: Handle, ray: Ray, pivot: [f64; <span class="s">3</span>]) -&gt; Option&lt;Drag&gt;;
<span class="k">pub</span> <span class="k">fn</span> update_drag(d: &amp;<span class="k">mut</span> Drag, ray: Ray) -&gt; Option&lt;Mat4&gt;; <span class="c">// an absolute world delta</span></code></pre></div>
<ul>
<li><strong>Translate.</strong> Press: closest point on the axis line to the press ray, <code>t = ((w·axis) − (d·axis)(d·w)) / (1 − (d·axis)²)</code>, <code>w = ray.origin − pivot</code>. Frame: <code>mat_translation(cur − start)</code>.</li>
<li>Reject when <code>|1 − (d·axis)²| &lt; 1e-8</code>: the ray is parallel, the answer unbounded, and the handle edge-on. That frame does not move.</li>
<li><strong>Rotate.</strong> Press: ray against the plane through the pivot normal to the axis, <code>θ0 = atan2(dp·v, dp·u)</code> in right-handed <code>(u, v, axis)</code>, so the angle grows CCW about <code>+axis</code>. Frame: <code>T(pivot) · R(axis, θ − θ0) · T(−pivot)</code>.</li>
<li><code>atan2</code> returns <code>(−π, π]</code>: a naive difference caps a drag at ±180°, so 200° reads as −160°. Add ±2π to <code>turns</code> when the step exceeds π — two lines, full turns.</li>
<li>Refuse the press if the ray misses the plane (grazing view): a <code>θ0 = 0</code> fallback jumps the first frame by the whole measured angle.</li>
<li><strong>Uniform scale.</strong> Press: freeze the drag plane as the world axis most aligned with the view direction, so a mid-drag view change cannot jump the factor; <code>d0 = |p − pivot|</code>, floored at <code>1e-4</code>. Frame: <code>f = d / d0</code>, <code>T(pivot) · S(f) · T(−pivot)</code>.</li>
<li><strong>No damping exponent</strong>: the archive&#39;s 0.25 power meant sixteen times the cursor travel to double the object.</li>
<li>Floor the <strong>factor</strong> at 0.01, not a ratio before a power: a zero must never commit a singular or mirroring matrix, and a pre-power floor gives an arbitrary smallest size (<code>0.01^0.25 ≈ 0.316</code>).</li>
<li><strong>Axis scale.</strong> Press: <code>t0 = (p − pivot)·axis</code>. Frame: <code>f = t / t0</code> floored at 0.01, <code>S(1 + ax(f−1), 1 + ay(f−1), 1 + az(f−1))</code> about the pivot.</li>
<li>Past the pivot <code>t</code> goes negative and the object collapses rather than mirroring: a mirror is a change nobody asked for by dragging.</li>
<li><strong>Pivot</strong> = midpoint of <code>gpu.objects.row_bounds(row)</code> (<code>objects.rs:466</code>), the point <code>fit_selected_or_all</code> already frames with (<code>state.rs</code>); <code>Aabb</code> has no midpoint helper, so one line in <code>math.rs</code>.</li>
<li><strong>Only a translate moves the widget</strong>; rotate and scale leave the pivot put, or the handles crawl away from what they turn.</li>
<li>Orbiting mid-drag is impossible: the drag branch in <code>CursorMoved</code> returns before the camera controller sees the move, as <code>orbiting</code>/<code>panning</code> do (<code>input.rs</code>).</li>
</ul>
<h2 id="writing-the-move-16-bytes-or-112">Writing the move: 16 bytes or 112<a class="anchor" href="#/course/extend-gumball#writing-the-move-16-bytes-or-112" aria-label="Link to this section">#</a></h2>
<ul>
<li>The branch is on the <strong>write</strong>, not the geometry type: placement here is a 96 B model row plus a 16 B anchored translation (<code>layouts.rs:44</code>).</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch, InstanceTable</span>
<span class="k">pub</span> <span class="k">fn</span> set_translation(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, row: u32, world: [f64; <span class="s">3</span>]); <span class="c">// 16 B</span>
<span class="k">pub</span> <span class="k">fn</span> set_place(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, row: u32, place: &amp;Mat4); <span class="c">// 112 B</span></code></pre></div>
<ul>
<li>Pure translate: <code>translation[row]</code> in f64 and one 16 B <code>translations.write_at</code> — the body of <code>rebuild</code>&#39;s loop for one row (<code>objects.rs</code>).</li>
<li>Rotate or scale: the 96 B row with a zeroed translation column as <code>append</code> does, <strong>plus</strong> the 16 B translation = 112 B, because the pivot is the box centre, so every rotation moves the translation too.</li>
<li>Both add to the <strong>f64 base</strong>, never the f32 the GPU holds — <code>objects.rs</code> test <code>an_edit_written_past_the_base_does_not_survive_a_rebase</code>.</li>
<li>Both refresh <code>world_bounds[row]</code> (<code>objects.rs:105</code>) and the matching <code>BoundedRow.lo/hi</code> (<code>objects.rs:64</code>, walked by <code>update_inside</code>); nothing else recomputes them, so skipping it leaves <code>F</code> framing the old place and the inside test stale.</li>
<li>That needs the row&#39;s <strong>local</strong> box, dropped after upload (<code>scene.rs:219</code>), so <code>InstanceTable</code> retains a <code>Vec&lt;Aabb&gt;</code> of local boxes — <strong>the one new piece of retained state the feature forces</strong>.</li>
<li>Both bump <code>geometry_revision</code>, cache key of the finite-visibility tiles (<code>render.rs</code>) and the outline masks (<code>MaskKey.geometry</code>, <code>render.rs</code>): <strong>every drag frame re-renders the masks and re-bins the tiles.</strong> Measure that before deciding whether a live drag drops <code>view.show_outlines</code>.: <strong>every drag frame re-renders the masks and re-bins the tiles.</strong> Measure that before deciding whether a live drag drops <code>view.show_outlines</code>.</li>
<li><code>Gpu::set_place</code> / <code>set_translation</code> forward beside <code>set_selected</code> / <code>set_hidden</code> (<code>src/engine/gpu/mod.rs</code>), each calling <code>splat.invalidate</code>: splat records fold <code>mvp × model</code> per cloud.</li>
<li>Set <code>state.interacting = true</code> for the drag, as orbit and pan do (<code>input.rs:96</code>).</li>
</ul>
<h2 id="the-commit-one-transaction-in-the-kernel">The commit: one transaction in the kernel<a class="anchor" href="#/course/extend-gumball#the-commit-one-transaction-in-the-kernel" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch, Scene</span>
<span class="k">pub</span> <span class="k">fn</span> place_object(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, world_place: &amp;Mat4) -&gt; bool;</code></pre></div>
<ul>
<li>Resolve the row to <code>(document index, guid)</code> with <code>Scene::identity_of</code> (<code>scene.rs</code>).</li>
<li>Invert the walk — forward it is <code>object_place = doc.place · world_xform(guid)</code> (<code>placement</code>, <code>scene.rs</code>, used at), <code>world_xform = ancestors · local</code> (<code>session.rs:379</code>):</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>local_new = (ancestors)⁻¹ · doc.place⁻¹ · new_object_place</code></pre></div>
<ul>
<li>An identity manifest <code>place</code> and no ancestor transform collapse that to <code>local_new = D · local_old</code> for world delta <code>D</code>. Both forms belong in the code: the collapse runs, the general form survives a manifest that places a file.</li>
<li><strong><code>Rc::make_mut(&amp;mut doc.session)</code> first.</strong> <code>FileDoc.session</code> is shared — a manifest listing one file twice hands both documents the same <code>Rc</code> (<code>scene.rs</code>), <code>live.rs</code> keeps its own clone — so without the split, moving one placement moves them all. Test: <code>an_edit_must_split_a_session_two_placements_share</code> (<code>scene.rs</code>).</li>
<li>Then three lines:</li>
</ul>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// sketch</span>
session.begin(&quot;<span class="s">move</span>&quot;);
session.set_xform(&amp;guid, local_new);
session.commit;</code></pre></div>
<ul>
<li><code>set_xform</code> records <code>Op::Xform</code> absolute on both sides <strong>by itself</strong> when a transaction is open (<code>session.rs:346</code>) — and records nothing when none is.</li>
<li><strong>Do not call <code>Scene::rebuild</code></strong> (<code>scene.rs</code>): a full re-walk whose own comment says streamed clouds and sheets cannot come back. The instance row holds the answer, and <code>placement</code> reproduces it on any later rebuild.</li>
<li>Call <code>update_label</code> (<code>src/state/text.rs</code>) after the commit, or the nameplate stays at the old position — it is placed from <code>row_bounds(row)</code>.</li>
</ul>
<h2 id="undo-and-redo">Undo and redo<a class="anchor" href="#/course/extend-gumball#undo-and-redo" aria-label="Link to this section">#</a></h2>
<ul>
<li><code>State::undo</code> / <code>redo</code> call <code>Session::undo</code> / <code>redo</code> (<code>session.rs:1622</code>), which take the buffer out and put it back, so no borrow problem reaches the viewer. <code>History</code> caps at <code>CAPACITY = 64</code> and clears redo on commit.</li>
<li><strong>No viewer-side undo stack</strong>: a second source of truth for one edit. <code>XformOp</code> is absolute on both sides, so a re-baked delta cannot double a move.</li>
<li><code>Session::undo</code> answers only <code>bool</code>, so after undo or redo call <code>session.world_xforms</code> once for that document (one downward pass, <code>session.rs:398</code>) and re-place its rows through <code>placement</code> and <code>Gpu::set_place</code> — hundreds of rows, not millions. Then re-park the origin and call <code>update_label</code>.</li>
<li>Bind in <code>Input::key</code> (<code>input.rs</code>); <code>Input</code> already tracks <code>ctrl</code> (<code>input.rs</code>), and modifiers arrive through <code>ModifiersChanged</code>, which the Ctrl-click edge pick already depends on.</li>
</ul>
<h2 id="numeric-entry">Numeric entry<a class="anchor" href="#/course/extend-gumball#numeric-entry" aria-label="Link to this section">#</a></h2>
<ul>
<li>Under <code>CLICK_SLOP</code> a press on a handle is a <strong>click</strong> asking for a typed value, past it a <strong>drag</strong> — the same constant already decides click-versus-drag for selection (<code>input.rs:16</code>).</li>
<li>Title from <code>Handle::labels</code> — &quot;Move X (mm)&quot;, &quot;Rotate Z (deg)&quot;, &quot;Scale (factor)&quot; — anchored at the press point.</li>
<li>No egui: a DOM element beside <code>#viewer-status</code> (<code>index.html:54</code>), driven by an accessor shaped like <code>feedback::status</code> (<code>src/app/feedback.rs</code>) behind <code>#[cfg(target_arch = &quot;wasm32&quot;)]</code>, applied by <code>State::gizmo_value(f64)</code>.</li>
<li><code>manual_delta(handle, value, pivot)</code> is relative and undamped, with the same 0.01 floor on any scale factor. A parse failure reports through <code>status</code> rather than cancelling silently.</li>
<li>The canvas carries <code>tabindex=&quot;0&quot;</code> (<code>index.html:52</code>): return focus on apply and on cancel, or the next keystroke goes nowhere.</li>
</ul>
<h2 id="multi-selection">Multi-selection<a class="anchor" href="#/course/extend-gumball#multi-selection" aria-label="Link to this section">#</a></h2>
<ul>
<li><code>Scene.selected</code> is <code>Option&lt;u32&gt;</code> (<code>scene.rs</code>), changed only in <code>State::select</code> (<code>state.rs</code>), so there is no centroid question: the pivot is one row&#39;s box midpoint, parked or cleared in <code>select</code>, <code>clear</code> and <code>escape_selection</code>.</li>
<li>Multi-selection first needs <code>Scene.selected</code> to become a set, changing <code>set_selected</code>, the <code>selection_revision</code> outline-mask key and <code>fit_selected_or_all</code>; writing it now would be inventing.</li>
<li>Keep the boundary so it stays cheap later: <code>begin_drag</code> / <code>update_drag</code> take a pivot and return a world matrix, so N rows is a loop. Keep press-time placements in a map keyed by row.</li>
</ul>
<h2 id="a-gumball-on-a-streamed-cloud-or-a-sheet">A gumball on a streamed cloud or a sheet<a class="anchor" href="#/course/extend-gumball#a-gumball-on-a-streamed-cloud-or-a-sheet" aria-label="Link to this section">#</a></h2>
<ul>
<li><code>add_streamed_cloud</code> (<code>scene.rs</code>) and <code>add_sheet</code> (<code>scene.rs</code>) push a <code>FileDoc</code> whose session is an empty <code>Rc::new(Session::new(&amp;name))</code>, <code>display_only: true</code>: an object row, no guid, nothing to <code>set_xform</code>, nothing to save.</li>
<li>Do not refuse the move — the row is first-class everywhere else. It writes the instance row, opens <strong>no transaction</strong>, and says so in the status line: a <strong>view placement</strong>.</li>
<li>The cheapest object here to move: half a million sheet segments behind one row move for one 16-byte write, because each is placed through <code>place(i, p)</code>. Later slices arrive into the same row, so a move survives streaming.</li>
<li>Two obligations: <code>splat.invalidate</code> on every write to a cloud row; and <code>gpu.bounds</code> / <code>scene.tables.bounds</code> are unioned once per slice (<code>scene.rs:352</code>) and never recomputed, so <code>fit_all</code> frames the old place — recompute after a commit, or say so in the code.</li>
</ul>
<h2 id="input-routing-and-cancellation">Input routing and cancellation<a class="anchor" href="#/course/extend-gumball#input-routing-and-cancellation" aria-label="Link to this section">#</a></h2>
<ul>
<li>Everything reaches <code>State</code> as a <strong>named action</strong>, as <code>request_selection</code> / <code>escape_selection</code> / <code>enable_controls</code> do (<code>input.rs</code>): <code>gizmo_press</code>, <code>gizmo_hover</code>, <code>gizmo_drag</code>, <code>gizmo_release</code>, <code>gizmo_cancel</code>, <code>gizmo_value</code>, <code>toggle_gizmo</code>, <code>undo</code>, <code>redo</code>. No widget reaches into <code>Scene</code>.</li>
<li><code>gizmo_press(x, y) -&gt; bool</code> goes in the <strong>Pressed</strong> arm of <code>Input::left</code> (<code>input.rs</code>), before <code>left_down</code> is armed, returning early when it grabs, so a press on a handle never becomes an object pick.</li>
<li><strong>Left-drag is unclaimed</strong>: RMB orbits, MMB pans (<code>input.rs:88</code>), and <code>CursorMoved</code> forwards to the camera only while <code>orbiting || panning</code>. In it (<code>input.rs:115</code>): <code>gizmo_drag</code> when a handle is engaged, <code>gizmo_hover</code> otherwise, both before the camera branch.</li>
<li><code>Input::cancel</code> (<code>input.rs</code>) already runs on focus loss and <code>pointercancel</code> (<code>Msg::CancelPointer</code>, <code>lib.rs</code>); <code>gizmo_cancel</code> hangs there, so a release the viewer never sees cannot leave the widget tracking the cursor.</li>
<li>Escape (<code>input.rs:56</code>) takes a live drag <strong>first</strong> and restores the press-time placement, then falls through to <code>escape_selection</code>; otherwise Escape during a drag both commits the move and drops the selection.</li>
<li>Hover sets <code>dirty</code> only when the hovered handle <strong>changes</strong> — <code>needs_frame</code> and <code>dirty</code> are split for this (<code>state.rs</code>) — and must not call <code>State::touch</code>, which cancels the pick in flight (<code>state.rs:242</code>).</li>
<li>Touch: one finger orbits, two pan and pinch (<code>src/app/touch.rs</code>). No free gesture is left for a handle drag, so touch gets tap-to-select plus the numeric entry.</li>
</ul>
<h2 id="the-order-that-compiles">The order that compiles<a class="anchor" href="#/course/extend-gumball#the-order-that-compiles" aria-label="Link to this section">#</a></h2>
<p>Each step builds on its own.</p>
<ol>
<li><strong><code>src/math.rs</code></strong> — <code>mat_translation</code>, <code>mat_rotation_about(pivot, axis, angle)</code>, <code>mat_scale_about(pivot, factors)</code>, f64 column-major beside <code>mat_mul</code> (<code>math.rs</code>), plus the <code>Aabb</code> midpoint, with unit tests.</li>
<li><strong><code>src/camera.rs</code></strong> — <code>world_per_px</code> and <code>ray_at</code>, from the <code>zoom_at</code> view-plane frame (<code>camera.rs</code>).</li>
<li><strong><code>src/engine/gpu/objects.rs</code></strong> — retain the local <code>Aabb</code> per row; <code>set_translation</code> and <code>set_place</code>, refreshing <code>world_bounds[row]</code> and the matching <code>BoundedRow</code>, bumping <code>geometry_revision</code>. Test: &quot;a moved row reports the moved world box&quot;.</li>
<li><strong><code>src/engine/gpu/mod.rs</code></strong> — <code>Gpu::set_place</code> / <code>set_translation</code> beside <code>set_selected</code> (<code>src/engine/gpu/mod.rs</code>), each calling <code>splat.invalidate</code>.</li>
<li><strong><code>src/shaders/gizmo.wgsl</code> + <code>src/engine/gpu/gizmo.rs</code></strong> — the lane, empty: two <code>GrowBuf</code> tables on <code>l.ink_rows</code>, one-row group 2 on <code>l.instance</code>, <code>new</code> / <code>retarget</code> / <code>reset</code> / <code>release</code> / <code>allocated_bytes</code> / <code>set_origin</code> / <code>upload</code> / <code>draw</code>; pipelines from <code>scene_module</code> at <code>DepthMode::Always</code>; <code>SHADERS</code> into <code>lane_shaders</code>.</li>
<li><strong><code>src/engine/gpu/mod.rs</code> + <code>render.rs</code></strong> — the field into <code>build</code>, <code>retarget</code>, <code>reset</code>, <code>release</code>, <code>allocated_bytes</code>, and two draws at the end of <code>scene_list</code>. Tables empty, picture unchanged: the wiring lands before anything about it can be wrong.</li>
<li><strong><code>src/app/gizmo.rs</code></strong> — the widget, pure CPU: <code>Handle</code> and <code>labels</code>, geometry builders returning local-offset rows, the hit test, <code>begin_drag</code> / <code>update_drag</code>, <code>manual_delta</code>. Unit tests for hit-test priority and every drag law. <code>docs/locator.py</code> refuses to run when a taught file matches no zone, and this path matches none (<code>docs/locator.py:43-76</code>): add it to the <code>scene</code> zone&#39;s matchers first.</li>
<li><strong><code>src/state.rs</code></strong> — the <code>gizmo</code> field (<code>src/state.rs</code>); <code>upload_gizmo</code> from <code>render</code> after <code>rebase_anchor</code> (<code>state.rs</code>); origin set or cleared in <code>select</code>, <code>clear</code>, <code>escape_selection</code>. <strong>First visibly working point</strong>, no interaction yet.</li>
<li><strong><code>src/state.rs</code> + <code>src/app/input.rs</code></strong> — the named actions and the gesture. Objects move; nothing is recorded yet.</li>
<li><strong><code>src/app/scene.rs</code> + <code>src/state.rs</code></strong> — <code>Scene::place_object</code>: the <code>Rc::make_mut</code> split, the inverse composition, <code>begin</code> → <code>set_xform</code> → <code>commit</code>, then <code>update_label</code>. Streamed and sheet rows take the view-placement branch.</li>
<li><strong><code>src/state.rs</code> + <code>src/app/input.rs</code></strong> — <code>undo</code> / <code>redo</code> through the kernel, re-placing the document&#39;s rows from <code>world_xforms</code>, re-parking the origin.</li>
<li><strong>The numeric entry</strong> — the DOM element, <code>State::gizmo_value</code>, the status line on a parse failure.</li>
<li><strong>The test surfaces</strong> — <code>inspection::publish</code> gains <code>&quot;gizmo&quot;</code> (origin, hovered and engaged handle), so a browser check can assert a drag; a headless <code>src/selftest</code> case renders a frame with the widget up, and <code>selftest/lifecycle.rs</code> must survive the new retained local-box vector.</li>
</ol>
<h2 id="what-we-take-from-the-old-viewer-and-what-we-do-not">What we take from the old viewer and what we do not<a class="anchor" href="#/course/extend-gumball#what-we-take-from-the-old-viewer-and-what-we-do-not" aria-label="Link to this section">#</a></h2>
<p><strong>Ports as it stands</strong></p>
<ul>
<li><code>HandleKind</code> and its <code>labels</code>, with this viewer&#39;s world unit for &quot;mm&quot;.</li>
<li>The absolute-delta discipline — a rule, not code.</li>
<li>The two-mode gesture, on <code>CLICK_SLOP</code> (<code>input.rs</code>): the archive&#39;s <code>GUMBALL_DRAG_THRESHOLD_SQ = 16.0</code> is the same 4 px, and a second constant must not appear.</li>
<li><code>project_ray_on_axis</code> and the ray-plane intersection, for the <strong>drag</strong> only.</li>
<li>Freezing the uniform-scale drag plane at press.</li>
<li>The reason for the scale floor: no singular or mirroring matrix from a zero.</li>
</ul>
<p><strong>Adapts</strong></p>
<ul>
<li>The one-length factoring (<code>SCREEN_PX / ARC_RADIUS</code>); <code>Camera::unit</code> and <code>distance_world</code> replace the archive&#39;s two hard-coded <code>VIEWER_TO_MM = 1000.0</code>.</li>
<li>The screen-constant formula, keeping the depth term and the widget&#39;s own depth in perspective. Viewport height is the surface height (<code>state.rs:96</code>) — no panels here, so no visible-rect to chase.</li>
<li>The drag laws, with three corrections: rotation unwraps instead of capping at ±180°, the damping exponents go, the scale floor moves onto the factor.</li>
<li>The numeric popup — title from <code>labels</code>, at the press point, Enter applies, Escape cancels — as a DOM element, reporting a parse failure instead of cancelling silently.</li>
<li>The purity of <code>gumball.rs</code>: <code>src/app/gizmo.rs</code> keeps the shape, unit-testable without a GPU.</li>
<li>Anchoring: the archive&#39;s raw-millimetre identity instance becomes one <strong>anchored</strong> row — the original looks right near the origin and disintegrates far from it.</li>
</ul>
<p><strong>Replaced</strong></p>
<ul>
<li><code>undo_state.rs</code> and <code>state_undo.rs</code> in full — by <code>session_rust/src/history.rs</code>.</li>
<li><code>snapshots_after</code> and the dual-absolute snapshots — <code>XformOp</code> and <code>ReplaceOp</code> are absolute on both sides.</li>
<li><code>drag_geom_snapshots</code> and <code>drag_nurbs_snapshots</code> — the first serves only a commit that bakes coordinates; the second was never written to.</li>
<li>The baking commit branch and its remove-then-re-add of the GPU representation — a transform is a placement and stays in <code>Session::xforms</code>.</li>
<li><code>commit_object_transform</code>&#39;s five-way branch by geometry type — by one branch on the write.</li>
<li><code>GumballState</code> owning four wgpu buffers and their bind groups — by one lane in <code>Gpu</code> and one field on <code>State</code>.</li>
<li>The dedicated gumball pass and its depth clear — by <code>DepthMode::Always</code> plus the CPU sort.</li>
<li><code>ray_vs_arrow</code>, <code>ray_vs_arc</code>, <code>ray_vs_sphere_hit</code> — by the screen-space test, which drops the arrow test&#39;s ray-parallel sign error and the arc test that picked a full circle.</li>
<li>A single <code>best_dist</code> across four groups in three incompatible units — by one space: pixels.</li>
<li>The multi-selection centroid and its ~120 lines of per-type representative points — by the selected row&#39;s box midpoint.</li>
<li>Escape exiting the application, and no mid-drag cancel — by Escape cancelling the drag first and by <code>Input::cancel</code>.</li>
<li>The duplicated <code>mat4_mul</code>, the dead <code>pipelines.gumball</code>, the dead <code>gumball_dragged</code> flag.</li>
</ul>
<p><strong>Not taken, and why that is not a gap</strong></p>
<ul>
<li>Arrowhead cones: no cone lane here, and a thickened shaft end reads as an arrow using <code>CylinderSegment</code> alone.</li>
<li><code>transform_locked</code>: no such set exists, and inventing one before a row needs locking is speculative.</li>
<li>Edit-mode control dragging (~2000 archive lines): out of scope for v1, and the hook exists — <code>SelectionMode::Controls { parent, selected }</code> (<code>src/app/selection.rs</code>), committing through <code>Op::Replace</code>.</li>
<li>Snapping: the archive wired <code>snap.rs</code> into its draw tools only, never the gumball.</li>
<li>Object-aligned and CPlane-aligned modes, and a relocate gesture: the archive is world-axis-only too.</li>
<li>A <code>PickMode::Gizmo</code> arm: <code>PickMode</code> stays at four variants.</li>
</ul>
<h2 id="what-to-check-on-screen">What to check on screen<a class="anchor" href="#/course/extend-gumball#what-to-check-on-screen" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>Step 8</strong> — select an object: the widget sits at its box centre. Orbit, zoom to a metre and to a kilometre, Space to flip projection, pan far enough to re-anchor: same pixel size, no jitter. A streamed cloud and a sheet get the same widget at the same size.</li>
<li><strong>Step 9</strong> — drag all ten handles: hover colour changes only when the hovered handle changes, a translate carries the widget along, rotate and scale leave the pivot put, a rotate past half a turn keeps turning.</li>
<li><strong>Step 9</strong> — press a handle and release without moving: no object selected behind it; press empty space and selection works as before.</li>
<li><strong>Step 9</strong> — mid-drag, tab away or let the browser steal the pointer, and press Escape: the drag ends, the object returns to where it started, the selection survives.</li>
<li><strong>Step 9</strong> — drag a selected sheet: every segment moves together and the frame time does not change.</li>
<li><strong>Step 10</strong> — move, reload from the same source: the move is in the document. Move a file the manifest places twice: the other placement does not move. Move a streamed cloud: the status line names it a view placement.</li>
<li><strong>Step 11</strong> — move, undo, redo five times: the object lands in exactly the same two positions each time, and the widget follows.</li>
<li><strong>Step 12</strong> — type a number into a handle: it moves by exactly that. Type <code>abc</code>: the status line says so, nothing moves. Type <code>0</code> into a scale: no collapse.</li>
<li><strong>Throughout</strong> — with <code>?inspect=1</code>, <code>data-viewer-inspection</code> carries <code>gizmo</code> and the <code>model</code> of the moved row, and <code>pick_busy</code> is false while the cursor moves without a drag.</li>
</ul>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/extend-gumball#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Reference result from the supported <a href="#/course/22-runtime-helpers">implementation tutorials</a>. Select a line and press <strong>7</strong> for an isometric view. The whole viewer shows the selected line with solid cylindrical shafts, cone tips, rotation rings and scale spheres, while the surrounding scene stays visible. The capture uses the maintained viewer and the <a href="/session/docs/course/docs/extensions/nested.pb">nested fixture</a>.</p>
<p><a href="/session/docs/course/docs/screenshots/extensions-gumball-overview.png"><img src="/session/docs/course/docs/screenshots/extensions-gumball-overview.png" alt="Full viewer result for extend gumball" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"the-widget",text:"The widget"},{level:2,id:"where-every-piece-goes",text:"Where every piece goes"},{level:2,id:"the-handle-set",text:"The handle set"},{level:2,id:"screen-constant-size",text:"Screen-constant size"},{level:3,id:"world-units-per-pixel",text:"World units per pixel"},{level:2,id:"drawing-it",text:"Drawing it"},{level:3,id:"two-shader-conventions-that-will-catch-you",text:"Two shader conventions that will catch you"},{level:2,id:"anchoring-it",text:"Anchoring it"},{level:2,id:"hit-testing-pixels-for-the-grab-a-ray-for-the-drag",text:"Hit testing: pixels for the grab, a ray for the drag"},{level:3,id:"the-ray-the-drag-does-need",text:"The ray the drag does need"},{level:2,id:"the-drag-laws",text:"The drag laws"},{level:2,id:"writing-the-move-16-bytes-or-112",text:"Writing the move: 16 bytes or 112"},{level:2,id:"the-commit-one-transaction-in-the-kernel",text:"The commit: one transaction in the kernel"},{level:2,id:"undo-and-redo",text:"Undo and redo"},{level:2,id:"numeric-entry",text:"Numeric entry"},{level:2,id:"multi-selection",text:"Multi-selection"},{level:2,id:"a-gumball-on-a-streamed-cloud-or-a-sheet",text:"A gumball on a streamed cloud or a sheet"},{level:2,id:"input-routing-and-cancellation",text:"Input routing and cancellation"},{level:2,id:"the-order-that-compiles",text:"The order that compiles"},{level:2,id:"what-we-take-from-the-old-viewer-and-what-we-do-not",text:"What we take from the old viewer and what we do not"},{level:2,id:"what-to-check-on-screen",text:"What to check on screen"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{e as default};
