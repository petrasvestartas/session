const s={title:"02 · Camera",html:`<h1 id="02-camera">02 · Camera<a class="anchor" href="#/course/02-camera#02-camera" aria-label="Link to this section">#</a></h1>
<p>The triangle orbits, pans and zooms toward the cursor.</p>
<p><img src="/session/docs/course/docs/illustrations/camera-basis.svg" alt="Orbit turns the orientation about the target, pan slides the target across the camera's own plane, and the wheel scales the distance; the view-projection is rebuilt from those three every frame." loading="lazy" decoding="async"></p>
<h2 id="step-1-srccamerars">Step 1 · src/camera.rs<a class="anchor" href="#/course/02-camera#step-1-srccamerars" aria-label="Link to this section">#</a></h2>
<p>New file: the camera with orbit, pan, zoom and its view matrix.</p>
<p><code>lessons/02/src/camera.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> session_rust::{AABB, Point, Quaternion, Vector, Xform};

<span class="c">/// Vertical field of view: the camera sees 60° from the bottom edge of the screen to the top.</span>
<span class="k">pub</span> <span class="k">const</span> FOVY_DEG: f64 = <span class="s">60</span>.<span class="s">0</span>;

<span class="c">/// The unit the scene file is written in.</span>
#[derive(Clone, Copy, PartialEq)]
<span class="k">pub</span> <span class="k">enum</span> Unit {
    Millimeters,
    Meters,
}

<span class="k">impl</span> Unit {
    <span class="c">/// e.g. 2500 mm x 0.001 = 2.5 m.</span>
    <span class="k">pub</span> <span class="k">fn</span> to_meters(<span class="k">self</span>) -&gt; f64 {
        <span class="k">match</span> <span class="k">self</span> {
            Unit::Millimeters =&gt; <span class="s">0</span>.<span class="s">001</span>,
            Unit::Meters =&gt; <span class="s">1</span>.<span class="s">0</span>,
        }
    }
}

<span class="c">/// Near plane = the closest depth drawn: 1/10 000 of the target distance, so 0.3 mm at 3 m.</span>
<span class="k">pub</span> <span class="k">const</span> NEAR_FRACTION: f64 = <span class="s">1</span>.<span class="s">0</span>e-<span class="s">4</span>;

#[derive(Clone, Copy)]
<span class="k">pub</span> <span class="k">enum</span> View {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
    Iso,
}

<span class="c">/// An orbit camera is set by what you look AT, how far away, and which way it faces; the eye follows from those.</span>
<span class="k">pub</span> <span class="k">struct</span> Camera {
    <span class="k">pub</span> target: [f64; <span class="s">3</span>],        <span class="c">// always meters, whatever unit the file uses</span>
    <span class="k">pub</span> distance: f64,           <span class="c">// eye to target, meters</span>
    <span class="k">pub</span> orientation: Quaternion,
    <span class="k">pub</span> world_up: [f64; <span class="s">3</span>],      <span class="c">// yaw turns around this axis, +Z</span>
    <span class="k">pub</span> position: [f64; <span class="s">3</span>],      <span class="c">// the eye, derived by update_position()</span>
    <span class="k">pub</span> up: [f64; <span class="s">3</span>],            <span class="c">// screen up, derived too</span>
    <span class="k">pub</span> perspective: bool,       <span class="c">// false = orthographic: parallel lines stay parallel, far things do not shrink</span>
    <span class="k">pub</span> unit: Unit,</code></pre></div>
<p><code>lessons/02/src/camera.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> scene_extent: f64,       <span class="c">// meters; the far plane must reach this far past the target</span>
}

<span class="k">impl</span> Camera {
    <span class="c">/// Looks at the origin from 3 m, turned 30° and tilted 30° down.</span>
    <span class="k">pub</span> <span class="k">fn</span> new() -&gt; <span class="k">Self</span> {
        <span class="k">use</span> std::f64::consts::FRAC_PI_6;

        <span class="c">// Quaternion = a rotation stored as four numbers. Turn 30° about Z, then tilt 30° down.</span>
        <span class="k">let</span> yaw_q = Quaternion::from_axis_angle(Vector::z_axis(), -FRAC_PI_6);
        <span class="k">let</span> rv = yaw_q.rotate_vector(Vector::x_axis());
        <span class="k">let</span> pitch_q = Quaternion::from_axis_angle(rv, -FRAC_PI_6);
        <span class="k">let</span> orientation = (pitch_q * yaw_q).normalized(); <span class="c">// b * a = apply a first, then b</span>

        <span class="k">let</span> <span class="k">mut</span> cam = <span class="k">Self</span> {
            target: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>],
            distance: <span class="s">3</span>.<span class="s">0</span>,
            orientation,
            world_up: [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>],
            position: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>],
            up: [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>],
            perspective: <span class="s">true</span>,
            unit: Unit::Millimeters,
            scene_extent: <span class="s">0</span>.<span class="s">0</span>,
        };

        cam.update_position();

        cam
    }

    <span class="c">/// 0.005 radians per pixel: a 200 px drag turns about 57°.</span>
    <span class="k">pub</span> <span class="k">fn</span> orbit(&amp;<span class="k">mut</span> <span class="k">self</span>, dx: f32, dy: f32) {
        <span class="k">let</span> wu = Vector::new(<span class="k">self</span>.world_up[<span class="s">0</span>], <span class="k">self</span>.world_up[<span class="s">1</span>], <span class="k">self</span>.world_up[<span class="s">2</span>]);
        <span class="k">let</span> right = <span class="k">self</span>.orientation.rotate_vector(Vector::x_axis());
        <span class="c">// horizontal pixels turn about up, vertical pixels tilt about right</span>
        <span class="k">let</span> yaw_q = Quaternion::from_axis_angle(wu, (-dx * <span class="s">0</span>.<span class="s">005</span>) <span class="k">as</span> f64);
        <span class="k">let</span> pitch_q = Quaternion::from_axis_angle(right, (-dy * <span class="s">0</span>.<span class="s">005</span>) <span class="k">as</span> f64);

        <span class="k">self</span>.orientation = (yaw_q * (pitch_q * <span class="k">self</span>.orientation.duplicate())).normalized(); <span class="c">// normalized() removes the drift many small turns add</span>
        <span class="k">self</span>.update_position();
    }

    <span class="c">/// Slide the target across the screen plane; the view direction stays.</span>
    <span class="k">pub</span> <span class="k">fn</span> pan(&amp;<span class="k">mut</span> <span class="k">self</span>, dx: f32, dy: f32) {
        <span class="k">let</span> right = <span class="k">self</span>.orientation.rotate_vector(Vector::x_axis());
        <span class="c">// farther away, a pixel moves more</span>
        <span class="k">let</span> k = <span class="k">self</span>.distance * <span class="s">0</span>.<span class="s">0015</span>;

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            <span class="k">self</span>.target[i] += (-(dx <span class="k">as</span> f64) * right[i] + dy <span class="k">as</span> f64 * <span class="k">self</span>.up[i]) * k;
        }

        <span class="k">self</span>.update_position();
    }

    <span class="c">/// Move the eye toward or away from the target.</span>
    <span class="k">pub</span> <span class="k">fn</span> zoom(&amp;<span class="k">mut</span> <span class="k">self</span>, amount: f32) {
        <span class="k">self</span>.distance = zoom_distance(<span class="k">self</span>.distance, amount);
        <span class="k">self</span>.update_position();</code></pre></div>
<p><code>lessons/02/src/camera.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    }

    <span class="c">/// Zoom so the point under the cursor stays under the cursor.</span>
    <span class="k">pub</span> <span class="k">fn</span> zoom_at(&amp;<span class="k">mut</span> <span class="k">self</span>, amount: f32, cursor: (f64, f64), viewport: (f64, f64)) {
        <span class="c">// nothing for an empty viewport or a lost cursor</span>
        <span class="k">if</span> viewport.<span class="s">0</span> &lt;= <span class="s">0</span>.<span class="s">0</span> || viewport.<span class="s">1</span> &lt;= <span class="s">0</span>.<span class="s">0</span> || !cursor.<span class="s">0</span>.is_finite() || !cursor.<span class="s">1</span>.is_finite()
        {
            <span class="k">return</span>;
        }

        <span class="k">let</span> new_dist = zoom_distance(<span class="k">self</span>.distance, amount);
        <span class="k">let</span> k = new_dist / <span class="k">self</span>.distance; <span class="c">// 0.9 after one wheel step</span>
        <span class="c">// NDC = normalized device coordinates: -1..1 across the screen, y up</span>
        <span class="k">let</span> ndc_x = <span class="s">2</span>.<span class="s">0</span> * cursor.<span class="s">0</span> / viewport.<span class="s">0</span> - <span class="s">1</span>.<span class="s">0</span>;
        <span class="k">let</span> ndc_y = <span class="s">1</span>.<span class="s">0</span> - <span class="s">2</span>.<span class="s">0</span> * cursor.<span class="s">1</span> / viewport.<span class="s">1</span>;
        <span class="c">// half the view size at the target plane</span>
        <span class="k">let</span> half_h = <span class="k">self</span>.distance * (FOVY_DEG * <span class="s">0</span>.<span class="s">5</span>).to_radians().tan();
        <span class="k">let</span> half_w = half_h * (viewport.<span class="s">0</span> / viewport.<span class="s">1</span>);
        <span class="k">let</span> right = <span class="k">self</span>.orientation.rotate_vector(Vector::x_axis());

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            <span class="k">let</span> cursor_off = right[i] * ndc_x * half_w + <span class="k">self</span>.up[i] * ndc_y * half_h;
            <span class="k">self</span>.target[i] += cursor_off * (<span class="s">1</span>.<span class="s">0</span> - k); <span class="c">// pull the target toward the cursor</span>
        }

        <span class="k">self</span>.distance = new_dist;
        <span class="k">self</span>.update_position();
    }

    <span class="k">pub</span> <span class="k">fn</span> toggle_projection(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.perspective = !<span class="k">self</span>.perspective;
    }</code></pre></div>
<p><code>lessons/02/src/camera.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Switch projection and keep what was on screen in view.</span>
    <span class="k">pub</span> <span class="k">fn</span> toggle_projection_framed(&amp;<span class="k">mut</span> <span class="k">self</span>, bounds: &amp;AABB, aspect: f64) {
        <span class="k">self</span>.perspective = !<span class="k">self</span>.perspective;

        <span class="c">// only perspective can lose content; refit to what ortho showed</span>
        <span class="k">if</span> !<span class="k">self</span>.perspective || !bounds.is_valid() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> s = <span class="k">self</span>.unit.to_meters();
        <span class="k">let</span> t = <span class="k">self</span>.origin(); <span class="c">// target, scene units</span>
        <span class="k">let</span> right = <span class="k">self</span>.orientation.rotate_vector(Vector::x_axis());
        <span class="k">let</span> up = Vector::new(<span class="k">self</span>.up[<span class="s">0</span>], <span class="k">self</span>.up[<span class="s">1</span>], <span class="k">self</span>.up[<span class="s">2</span>]);
        <span class="k">let</span> fwd = Vector::new(
            (<span class="k">self</span>.target[<span class="s">0</span>] - <span class="k">self</span>.position[<span class="s">0</span>]) / <span class="k">self</span>.distance,
            (<span class="k">self</span>.target[<span class="s">1</span>] - <span class="k">self</span>.position[<span class="s">1</span>]) / <span class="k">self</span>.distance,
            (<span class="k">self</span>.target[<span class="s">2</span>] - <span class="k">self</span>.position[<span class="s">2</span>]) / <span class="k">self</span>.distance,
        );
        <span class="k">let</span> half_h = <span class="k">self</span>.distance * (FOVY_DEG * <span class="s">0</span>.<span class="s">5</span>).to_radians().tan() / s; <span class="c">// half view height</span>
        <span class="k">let</span> half_w = half_h * aspect;
        <span class="k">let</span> <span class="k">mut</span> lo = [f64::INFINITY; <span class="s">3</span>];
        <span class="k">let</span> <span class="k">mut</span> hi = [f64::NEG_INFINITY; <span class="s">3</span>];

        <span class="c">// clip every box corner to the visible rectangle</span>
        <span class="k">for</span> corner <span class="k">in</span> bounds.corners() {
            <span class="k">let</span> c = [corner[<span class="s">0</span>] - t[<span class="s">0</span>], corner[<span class="s">1</span>] - t[<span class="s">1</span>], corner[<span class="s">2</span>] - t[<span class="s">2</span>]];
            <span class="k">let</span> dx = (c[<span class="s">0</span>] * right[<span class="s">0</span>] + c[<span class="s">1</span>] * right[<span class="s">1</span>] + c[<span class="s">2</span>] * right[<span class="s">2</span>]).clamp(-half_w, half_w);
            <span class="k">let</span> dy = (c[<span class="s">0</span>] * up[<span class="s">0</span>] + c[<span class="s">1</span>] * up[<span class="s">1</span>] + c[<span class="s">2</span>] * up[<span class="s">2</span>]).clamp(-half_h, half_h);
            <span class="k">let</span> dz = c[<span class="s">0</span>] * fwd[<span class="s">0</span>] + c[<span class="s">1</span>] * fwd[<span class="s">1</span>] + c[<span class="s">2</span>] * fwd[<span class="s">2</span>];

            <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                <span class="k">let</span> p = t[i] + dx * right[i] + dy * up[i] + dz * fwd[i];
                lo[i] = lo[i].min(p);
                hi[i] = hi[i].max(p);
            }
        }

        <span class="k">let</span> clipped = AABB::from_points(
            &amp;[
                Point::new(lo[<span class="s">0</span>], lo[<span class="s">1</span>], lo[<span class="s">2</span>]),
                Point::new(hi[<span class="s">0</span>], hi[<span class="s">1</span>], hi[<span class="s">2</span>]),
            ],
            <span class="s">0</span>.<span class="s">0</span>,
        );
        <span class="k">self</span>.fit(&amp;clipped, aspect);
    }

    <span class="c">/// The target in scene units.</span>
    <span class="k">pub</span> <span class="k">fn</span> origin(&amp;<span class="k">self</span>) -&gt; Point {
        <span class="k">let</span> s = <span class="k">self</span>.unit.to_meters();
        Point::new(<span class="k">self</span>.target[<span class="s">0</span>] / s, <span class="k">self</span>.target[<span class="s">1</span>] / s, <span class="k">self</span>.target[<span class="s">2</span>] / s)
    }

    <span class="c">/// Eye to target distance in scene units.</span>
    <span class="k">pub</span> <span class="k">fn</span> distance_world(&amp;<span class="k">self</span>) -&gt; f64 {
        <span class="k">self</span>.distance / <span class="k">self</span>.unit.to_meters()
    }</code></pre></div>
<p><code>lessons/02/src/camera.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// One matrix, two jobs: view moves the world in front of the eye, projection flattens it onto the screen.</span>
    <span class="k">pub</span> <span class="k">fn</span> view_proj(&amp;<span class="k">self</span>, aspect: f64) -&gt; Xform {
        <span class="k">self</span>.view_proj_anchored(aspect, &amp;<span class="k">self</span>.origin())
    }

    <span class="c">/// Anchor = a point subtracted before the f32 conversion, so a model 100 km out keeps millimetres.</span>
    <span class="k">pub</span> <span class="k">fn</span> view_proj_anchored(&amp;<span class="k">self</span>, aspect: f64, anchor: &amp;Point) -&gt; Xform {
        <span class="k">let</span> dist = <span class="k">self</span>.distance;
        <span class="k">let</span> a = <span class="k">self</span>.unit.to_meters();
        <span class="k">let</span> anchor = Point::new(anchor[<span class="s">0</span>] * a, anchor[<span class="s">1</span>] * a, anchor[<span class="s">2</span>] * a); <span class="c">// to meters</span>
        <span class="c">// far plane reaches the whole scene</span>
        <span class="k">let</span> far = (dist * <span class="s">10</span>.<span class="s">0</span>).max(dist + <span class="s">2</span>.<span class="s">0</span> * <span class="k">self</span>.scene_extent);
        <span class="k">let</span> projection = <span class="k">if</span> <span class="k">self</span>.perspective {
            <span class="c">// Reverse-Z: far and near are swapped so depth 1 is near, which spends float precision near the eye.</span>
            Xform::perspective(FOVY_DEG.to_radians(), aspect, far, dist * NEAR_FRACTION)
        } <span class="k">else</span> {
            <span class="k">let</span> h = dist * (FOVY_DEG * <span class="s">0</span>.<span class="s">5</span>).to_radians().tan(); <span class="c">// half view height</span>
            <span class="c">// depth range covers the scene, not more</span>
            <span class="k">let</span> extent = <span class="k">if</span> <span class="k">self</span>.scene_extent &gt; <span class="s">0</span>.<span class="s">0</span> {
                <span class="k">self</span>.scene_extent
            } <span class="k">else</span> {
                dist
            };
            <span class="k">let</span> r = (dist + <span class="s">2</span>.<span class="s">0</span> * extent).max(<span class="s">1</span>.<span class="s">0</span>e-<span class="s">6</span>);
            Xform::orthographic(-aspect * h, aspect * h, -h, h, r, -r)
        };

        <span class="c">// eye and target relative to the anchor</span>
        <span class="k">let</span> eye = Point::new(
            <span class="k">self</span>.position[<span class="s">0</span>] - anchor[<span class="s">0</span>],
            <span class="k">self</span>.position[<span class="s">1</span>] - anchor[<span class="s">1</span>],
            <span class="k">self</span>.position[<span class="s">2</span>] - anchor[<span class="s">2</span>],
        );
        <span class="k">let</span> target = Point::new(
            <span class="k">self</span>.target[<span class="s">0</span>] - anchor[<span class="s">0</span>],
            <span class="k">self</span>.target[<span class="s">1</span>] - anchor[<span class="s">1</span>],
            <span class="k">self</span>.target[<span class="s">2</span>] - anchor[<span class="s">2</span>],
        );
        <span class="k">let</span> up = Vector::new(<span class="k">self</span>.up[<span class="s">0</span>], <span class="k">self</span>.up[<span class="s">1</span>], <span class="k">self</span>.up[<span class="s">2</span>]);
        <span class="k">let</span> view = Xform::look_at_right_handed(&amp;eye, &amp;target, &amp;up);

        <span class="c">// scene units to meters</span>
        <span class="k">let</span> s = <span class="k">self</span>.unit.to_meters();
        <span class="k">let</span> scale = Xform::scale_xyz(s, s, s);

        projection * view * scale <span class="c">// read right to left: scale first, projection last</span>
    }

    <span class="c">/// Turn to a named view, orthographic.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_view(&amp;<span class="k">mut</span> <span class="k">self</span>, view: View) {
        <span class="k">use</span> std::f64::consts::{FRAC_PI_2, FRAC_PI_6, PI};
        <span class="k">let</span> z = Vector::z_axis();
        <span class="k">let</span> x = Vector::x_axis();
        <span class="k">self</span>.orientation = <span class="k">match</span> view {
            View::Front =&gt; Quaternion::from_axis_angle(z, <span class="s">0</span>.<span class="s">0</span>),
            View::Back =&gt; Quaternion::from_axis_angle(z, PI),
            View::Right =&gt; Quaternion::from_axis_angle(z, FRAC_PI_2),
            View::Left =&gt; Quaternion::from_axis_angle(z, -FRAC_PI_2),
            View::Top =&gt; Quaternion::from_axis_angle(x, -FRAC_PI_2),
            View::Bottom =&gt; Quaternion::from_axis_angle(x, FRAC_PI_2),</code></pre></div>
<p><code>lessons/02/src/camera.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            View::Iso =&gt; {
                <span class="k">let</span> yaw_q = Quaternion::from_axis_angle(z, -FRAC_PI_6);
                <span class="k">let</span> rv = yaw_q.rotate_vector(x);
                (Quaternion::from_axis_angle(rv, -FRAC_PI_6) * yaw_q).normalized()
            }
        };

        <span class="k">self</span>.perspective = <span class="s">false</span>;
        <span class="k">self</span>.update_position();
    }

    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        *<span class="k">self</span> = Camera::new();
    }

    <span class="c">/// Frame a box: look at its center, back off until it fits.</span>
    <span class="k">pub</span> <span class="k">fn</span> fit(&amp;<span class="k">mut</span> <span class="k">self</span>, bounds: &amp;AABB, aspect: f64) {
        <span class="k">if</span> !bounds.is_valid() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> s = <span class="k">self</span>.unit.to_meters();

        <span class="k">self</span>.target = [bounds.cx * s, bounds.cy * s, bounds.cz * s];</code></pre></div>
<p><code>lessons/02/src/camera.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// half the view angle, sideways and up</span>
        <span class="k">let</span> half_fov_y = FOVY_DEG.to_radians() * <span class="s">0</span>.<span class="s">5</span>;
        <span class="k">let</span> half_fov_x = (aspect * half_fov_y.tan()).atan();
        <span class="k">let</span> (tx, ty) = (half_fov_x.tan(), half_fov_y.tan());

        <span class="k">let</span> fwd = <span class="k">self</span>.orientation.rotate_vector(Vector::y_axis());
        <span class="k">let</span> up = <span class="k">self</span>.orientation.rotate_vector(Vector::z_axis());
        <span class="k">let</span> right = <span class="k">self</span>.orientation.rotate_vector(Vector::x_axis());

        <span class="c">// the distance every corner needs to be in view</span>
        <span class="k">let</span> <span class="k">mut</span> distance: f64 = <span class="s">0</span>.<span class="s">0</span>;
        <span class="k">let</span> <span class="k">mut</span> extent: f64 = <span class="s">0</span>.<span class="s">0</span>;

        <span class="k">for</span> p <span class="k">in</span> <span class="k">self</span>.offsets(bounds) {
            <span class="k">let</span> (x, y, z) = (dot3(&amp;p, &amp;right), dot3(&amp;p, &amp;up), dot3(&amp;p, &amp;fwd));
            extent = extent.max((x * x + y * y + z * z).sqrt());
            distance = distance.max(x.abs() / tx + z);
            distance = distance.max(y.abs() / ty + z);
        }

        <span class="k">if</span> extent &lt;= <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">return</span>;
        }

        <span class="k">self</span>.distance = (distance * <span class="s">1</span>.<span class="s">05</span>).max(<span class="s">1</span>.<span class="s">0</span>e-<span class="s">6</span>); <span class="c">// 5% margin</span>
        <span class="k">self</span>.scene_extent = extent; <span class="c">// farthest corner from the target</span>
        <span class="k">self</span>.update_position();
    }

    <span class="c">/// The eight box corners in meters, relative to the target.</span>
    <span class="k">fn</span> offsets(&amp;<span class="k">self</span>, bounds: &amp;AABB) -&gt; [[f64; <span class="s">3</span>]; <span class="s">8</span>] {
        <span class="k">let</span> s = <span class="k">self</span>.unit.to_meters();
        <span class="k">let</span> <span class="k">mut</span> out = [[<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>]; <span class="s">8</span>];

        <span class="k">for</span> (offset, c) <span class="k">in</span> out.iter_mut().zip(bounds.corners()) {
            *offset = [
                c[<span class="s">0</span>] * s - <span class="k">self</span>.target[<span class="s">0</span>],
                c[<span class="s">1</span>] * s - <span class="k">self</span>.target[<span class="s">1</span>],
                c[<span class="s">2</span>] * s - <span class="k">self</span>.target[<span class="s">2</span>],
            ];
        }

        out
    }</code></pre></div>
<p><code>lessons/02/src/camera.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Widen the far plane for a box that arrived later, view untouched.</span>
    <span class="k">pub</span> <span class="k">fn</span> grow_extent(&amp;<span class="k">mut</span> <span class="k">self</span>, bounds: &amp;AABB) {
        <span class="k">if</span> !bounds.is_valid() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> extent: f64 = <span class="s">0</span>.<span class="s">0</span>;

        <span class="k">for</span> p <span class="k">in</span> <span class="k">self</span>.offsets(bounds) {
            extent = extent.max((p[<span class="s">0</span>] * p[<span class="s">0</span>] + p[<span class="s">1</span>] * p[<span class="s">1</span>] + p[<span class="s">2</span>] * p[<span class="s">2</span>]).sqrt());
        }

        <span class="k">if</span> extent.is_finite() &amp;&amp; extent &gt; <span class="k">self</span>.scene_extent {
            <span class="k">self</span>.scene_extent = extent;
        }
    }

    <span class="k">pub</span> <span class="k">fn</span> set_unit(&amp;<span class="k">mut</span> <span class="k">self</span>, unit: Unit) {
        <span class="k">self</span>.unit = unit;
    }

    <span class="c">/// Recompute the eye and up from orientation, target and distance.</span>
    <span class="k">pub</span> <span class="k">fn</span> update_position(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> fwd = <span class="k">self</span>.orientation.rotate_vector(Vector::y_axis()); <span class="c">// eye to target</span>
        <span class="k">let</span> up = <span class="k">self</span>.orientation.rotate_vector(Vector::z_axis());

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            <span class="k">self</span>.position[i] = <span class="k">self</span>.target[i] - fwd[i] * <span class="k">self</span>.distance;
            <span class="k">self</span>.up[i] = up[i];
        }
    }
}

<span class="c">/// Dot product of an array and a vector.</span>
<span class="k">fn</span> dot3(p: &amp;[f64; <span class="s">3</span>], v: &amp;Vector) -&gt; f64 {
    p[<span class="s">0</span>] * v[<span class="s">0</span>] + p[<span class="s">1</span>] * v[<span class="s">1</span>] + p[<span class="s">2</span>] * v[<span class="s">2</span>]
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// The depth value of a point \`depth\` meters in front of the eye.</span>
    <span class="k">fn</span> ndc_depth(cam: &amp;Camera, depth: f64) -&gt; f64 {
        <span class="k">let</span> fwd = cam.orientation.rotate_vector(Vector::y_axis());
        <span class="k">let</span> (s, o) = (cam.unit.to_meters(), cam.origin());
        <span class="k">let</span> p = Point::new(
            cam.position[<span class="s">0</span>] + fwd[<span class="s">0</span>] * depth,
            cam.position[<span class="s">1</span>] + fwd[<span class="s">1</span>] * depth,
            cam.position[<span class="s">2</span>] + fwd[<span class="s">2</span>] * depth,
        );
        cam.view_proj(<span class="s">1</span>.<span class="s">5</span>).transform_point(&amp;Point::new(
            p[<span class="s">0</span>] / s - o[<span class="s">0</span>],
            p[<span class="s">1</span>] / s - o[<span class="s">1</span>],
            p[<span class="s">2</span>] / s - o[<span class="s">2</span>],
        ))[<span class="s">2</span>]
    }

    <span class="c">/// The near plane cuts just ahead of the eye at any distance.</span>
    #[test]
    <span class="k">fn</span> near_plane_cuts_a_millimetre_ahead_not_a_beam() {</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/02/src/camera.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> cam = Camera::new();
        cam.scene_extent = <span class="s">118</span>.<span class="s">0</span>;

        <span class="k">for</span> dist <span class="k">in</span> [<span class="s">122</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">5</span>] {
            cam.distance = dist;
            cam.update_position();
            assert!(
                ndc_depth(&amp;cam, dist * <span class="s">2</span>.<span class="s">0</span> * NEAR_FRACTION) &lt; <span class="s">1</span>.<span class="s">0</span>,
                &quot;<span class="s">dist </span>{<span class="s">dist</span>}<span class="s">: cut too early</span>&quot;
            );
            assert!(
                ndc_depth(&amp;cam, dist * <span class="s">0</span>.<span class="s">5</span> * NEAR_FRACTION) &gt; <span class="s">1</span>.<span class="s">0</span>,
                &quot;<span class="s">dist </span>{<span class="s">dist</span>}<span class="s">: near plane missing</span>&quot;
            );
            assert!(
                ndc_depth(&amp;cam, dist + <span class="s">2</span>.<span class="s">0</span> * cam.scene_extent - <span class="s">1</span>.<span class="s">0</span>e-<span class="s">6</span>) &gt; <span class="s">0</span>.<span class="s">0</span>,
                &quot;<span class="s">dist </span>{<span class="s">dist</span>}<span class="s">: far plane short of the scene</span>&quot;
            );
        }
    }

    #[test]
    <span class="k">fn</span> distant_orthographic_depth_distinguishes_four_millimetres() {
        <span class="k">let</span> <span class="k">mut</span> cam = Camera::new();
        cam.perspective = <span class="s">false</span>;
        cam.scene_extent = <span class="s">2</span>.<span class="s">0</span>;

        <span class="k">for</span> distance <span class="k">in</span> [<span class="s">3</span>.<span class="s">3</span>, <span class="s">13</span>.<span class="s">2</span>, <span class="s">52</span>.<span class="s">8</span>] {
            cam.distance = distance;
            cam.update_position();
            <span class="k">let</span> front = ndc_depth(&amp;cam, distance) <span class="k">as</span> f32;
            <span class="k">let</span> rear = ndc_depth(&amp;cam, distance + <span class="s">0</span>.<span class="s">004</span>) <span class="k">as</span> f32;
            assert!(
                front - rear &gt; rear.abs() * <span class="s">1</span>.<span class="s">9073486</span>e-<span class="s">6</span>,
                &quot;<span class="s">distance </span>{<span class="s">distance</span>}<span class="s">: hidden ink falls within float tolerance</span>&quot;
            );
        }
    }

    #[test]
    <span class="k">fn</span> orthographic_depth_contains_the_scene_before_and_behind_the_eye() {
        <span class="k">let</span> <span class="k">mut</span> cam = Camera::new();
        cam.perspective = <span class="s">false</span>;

        <span class="k">for</span> extent <span class="k">in</span> [<span class="s">0</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">118</span>.<span class="s">0</span>] {
            cam.scene_extent = extent;

            <span class="k">for</span> distance <span class="k">in</span> [<span class="s">1</span>.<span class="s">0</span>e-<span class="s">6</span>, <span class="s">0</span>.<span class="s">5</span>, <span class="s">122</span>.<span class="s">0</span>] {
                cam.distance = distance;
                cam.update_position();

                <span class="k">for</span> depth <span class="k">in</span> [distance - extent, distance + extent] {
                    <span class="k">let</span> projected = ndc_depth(&amp;cam, depth);
                    assert!(
                        (<span class="s">0</span>.<span class="s">0</span>..=<span class="s">1</span>.<span class="s">0</span>).contains(&amp;projected),
                        &quot;<span class="s">extent </span>{<span class="s">extent</span>}<span class="s">, distance </span>{<span class="s">distance</span>}<span class="s">, depth </span>{<span class="s">depth</span>}<span class="s">: </span>{<span class="s">projected</span>}&quot;
                    );
                }
            }
        }
    }
}

<span class="c">/// The distance after \`amount\` wheel steps, 0.9 per step.</span>
<span class="k">fn</span> zoom_distance(distance: f64, amount: f32) -&gt; f64 {
    <span class="k">if</span> !amount.is_finite() {
        <span class="k">return</span> distance;
    }

    <span class="c">// at most ten steps per event, never zero</span>
    (distance * <span class="s">0</span>.<span class="s">9_f64</span>.powf(f64::from(amount).clamp(-<span class="s">10</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>))).clamp(<span class="s">1</span>.<span class="s">0</span>e-<span class="s">6</span>, <span class="s">1</span>.<span class="s">0</span>e<span class="s">15</span>)
}

#[cfg(test)]
<span class="k">mod</span> wheel_tests {
    <span class="k">use</span> super::*;

    #[test]
    <span class="k">fn</span> coalesced_wheel_events_remain_positive_and_preserve_the_cursor_anchor() {
        <span class="k">let</span> <span class="k">mut</span> camera = Camera::new();
        <span class="k">let</span> before = camera.distance;
        camera.zoom_at(<span class="s">16</span>.<span class="s">0</span>, (<span class="s">800</span>.<span class="s">0</span>, <span class="s">500</span>.<span class="s">0</span>), (<span class="s">1600</span>.<span class="s">0</span>, <span class="s">1000</span>.<span class="s">0</span>));
        assert!(camera.distance &gt; before * <span class="s">0</span>.<span class="s">3</span>);
        assert!(camera.distance &lt; before);
        <span class="k">let</span> distance = camera.distance;
        camera.zoom(f32::NAN);
        assert_eq!(camera.distance, distance);
        assert!(
            (zoom_distance(zoom_distance(before, <span class="s">1</span>.<span class="s">0</span>), <span class="s">1</span>.<span class="s">0</span>) - zoom_distance(before, <span class="s">2</span>.<span class="s">0</span>)).abs()
                &lt; <span class="s">1</span>e-<span class="s">10</span>
        );
    }
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/02/</code>.</p>
<h2 id="step-2-srclibrs">Step 2 · src/lib.rs<a class="anchor" href="#/course/02-camera#step-2-srclibrs" aria-label="Link to this section">#</a></h2>
<p>Wire the camera into the struct, the gestures and the uniform upload.</p>
<p><code>lessons/02/src/lib.rs</code> · edit · type this</p>
<p>Replaces the lines from <code>use wasm_bindgen::prelude::*;</code> to <code>use wgpu::util::DeviceExt;</code> in <code>lessons/01/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> session_rust::AABB;
<span class="k">use</span> session_rust::Point;
<span class="k">use</span> wasm_bindgen::prelude::*;
<span class="k">use</span> wgpu::util::DeviceExt; <span class="c">// a trait that adds create_buffer_init to Device; its methods work only once imported</span>
<span class="k">pub</span> <span class="k">mod</span> camera;</code></pre></div>
<p>Replaces the lines from <code>pipeline: wgpu::RenderPipeline,</code> to <code>scale: f64,</code> in <code>lessons/01/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    uniform: wgpu::Buffer, <span class="c">// kept, so every frame can rewrite the camera matrix</span>
    group: wgpu::BindGroup,
    scale: f64, <span class="c">// device pixels per CSS pixel, 2.0 on most phones</span>
    camera: camera::Camera,</code></pre></div>
<p>Replaces the lines from <code>let _ = (dx, dy, pan);</code> to <code>let _ = (delta, x, y);</code> in <code>lessons/01/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> pan {
            <span class="k">self</span>.camera.pan(dx, dy);
        } <span class="k">else</span> {
            <span class="k">self</span>.camera.orbit(dx, dy);
        }
    }

    <span class="c">/// delta in wheel steps; x, y: the cursor in CSS pixels.</span>
    <span class="k">pub</span> <span class="k">fn</span> zoom(&amp;<span class="k">mut</span> <span class="k">self</span>, delta: f32, x: f64, y: f64) {
        <span class="k">self</span>.camera.zoom_at(
            delta,
            (x * <span class="k">self</span>.scale, y * <span class="k">self</span>.scale), <span class="c">// CSS to device pixels, the unit of config.width</span>
            (<span class="k">self</span>.config.width <span class="k">as</span> f64, <span class="k">self</span>.config.height <span class="k">as</span> f64),
        );</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            cache: None,
        });</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// Back off until a 3 x 2 m box around the triangle fills the view.</span>
        <span class="k">let</span> <span class="k">mut</span> camera = camera::Camera::new();
        camera.unit = camera::Unit::Meters;
        camera.set_view(camera::View::Top);
        camera.fit(
            &amp;AABB::from_points(
                &amp;[Point::new(-<span class="s">1</span>.<span class="s">5</span>, -<span class="s">1</span>.<span class="s">0</span>, -<span class="s">0</span>.<span class="s">1</span>), Point::new(<span class="s">1</span>.<span class="s">5</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">1</span>)],
                <span class="s">0</span>.<span class="s">0</span>,
            ),
            <span class="s">1</span>.<span class="s">5</span>, <span class="c">// width / height, a guess until the first frame</span>
        );</code></pre></div>
<p>Replaces the lines from <code>group,</code> to <code>scale: 1.0,</code> in <code>lessons/01/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            uniform,
            group,
            scale: <span class="s">1</span>.<span class="s">0</span>,
            camera,</code></pre></div>
<p>Added above</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> output = <span class="k">match</span> <span class="k">self</span>.surface.get_current_texture() {</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// Overwrite the 64-byte uniform; the queue copies it before this frame's draw runs.</span>
        <span class="k">let</span> mvp = <span class="k">self</span>.camera.view_proj_anchored(
            width <span class="k">as</span> f64 / height <span class="k">as</span> f64,
            &amp;session_rust::Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        );
        <span class="k">self</span>.queue
            .write_buffer(&amp;<span class="k">self</span>.uniform, <span class="s">0</span>, bytemuck::cast_slice(&amp;mvp.to_f32()));</code></pre></div>
<p>Replaces the lines from <code>Ok(serde_json::json!({</code> to <code>.to_string())</code> in <code>lessons/01/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        Ok(serde_json::json!({&quot;<span class="s">stage</span>&quot;: <span class="s">2</span>, &quot;<span class="s">objects</span>&quot;: <span class="s">1</span>, &quot;<span class="s">width</span>&quot;:width,&quot;<span class="s">height</span>&quot;:height,&quot;<span class="s">scale</span>&quot;:scale,&quot;<span class="s">drawn</span>&quot;:<span class="s">true</span>}).to_string())</code></pre></div>
<h2 id="step-3-indexhtml">Step 3 · index.html<a class="anchor" href="#/course/02-camera#step-3-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy the page: the title and status say checkpoint 02.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/02/index.html</code> · edit · copy the file</p>
<p>Replaces the line <code>&lt;title&gt;01 - First WebGPU frame&lt;/title&gt;</code> in <code>lessons/01/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;title&gt;Session checkpoint 02&lt;/title&gt;</code></pre></div>
<p>Replaces the line <code>&lt;output id=&quot;status&quot;&gt;Starting checkpoint 01&lt;/output&gt;</code> in <code>lessons/01/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 02&lt;/output&gt;</code></pre></div>
<p>Replaces the line <code>document.getElementById(&#39;status&#39;).textContent = &#39;Checkpoi…</code> in <code>lessons/01/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        document.getElementById('status').textContent = 'Checkpoint 02 · ' + state.objects + ' objects · ' + state.width + '×' + state.height;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/02-camera#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/02/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: The triangle orbits, pans and zooms toward the cursor; status: <strong>Checkpoint 02 · 1 objects</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/02.png" alt="Checkpoint 02: the same triangle seen from the production camera; drag to orbit, Shift-drag to pan, wheel to zoom at the cursor." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>Dragging moves twice as far on a high-DPI screen: the cursor is scaled twice.</li>
<li>A distant model jitters: coordinates become f32 before the anchor is subtracted.</li>
<li>The triangle disappears: reversed depth needs a zero clear and a Greater comparison.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/02-camera#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/02/src/
├── shaders/
│   └── first.wgsl
├── camera.rs  +
└── lib.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: gesture → camera → anchored matrix → uniform → vertex. Every file at this point: <code>lessons/02/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/02-camera#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/03-identity">03 · Object rows and identity</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/02-camera#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 02: the same triangle seen from the production camera; drag to orbit, Shift-drag to pan, wheel to zoom at the cursor.</p>
<p><a href="/session/docs/course/docs/screenshots/02.png"><img src="/session/docs/course/docs/screenshots/02.png" alt="Full viewer result for 02 camera" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srccamerars",text:"Step 1 · src/camera.rs"},{level:2,id:"step-2-srclibrs",text:"Step 2 · src/lib.rs"},{level:2,id:"step-3-indexhtml",text:"Step 3 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
