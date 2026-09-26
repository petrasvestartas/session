const e={title:"Arctic lighting and contact shadows",html:`<h1 id="arctic-lighting-and-contact-shadows">Arctic lighting and contact shadows<a class="anchor" href="#/course/ssao#arctic-lighting-and-contact-shadows" aria-label="Link to this section">#</a></h1>
<p>Type <code>Arctic</code>, press <strong>Enter</strong>, then choose <strong>On</strong> or <strong>Off</strong>. <code>Arctic On</code> and <code>Arctic Off</code> also run directly. <strong>G</strong> toggles the same lighting when the viewport has keyboard focus. The <code>SSAO</code> command has been removed. Arctic starts off and preserves the camera when toggled.</p>
<p>Enabling Arctic also enables black surface outlines. Use <code>Outline Off</code> to hide them while keeping Arctic shading, or <code>Outline On</code> to show them independently. <code>Outline</code> offers clickable <strong>On</strong> and <strong>Off</strong> options; <strong>O</strong> remains the viewport shortcut. Enabling Arctic again restores outlines. Turning Arctic off leaves the outline setting unchanged.</p>
<p>Clipping sections have their own black cut boundaries and use solid light grey fill by default. <code>clipping_plane Fill Hatch</code> selects hatching; <code>clipping_plane Fill Solid</code> restores the solid fill. Cut boundaries remain visible with <code>Outline Off</code> and without Arctic.</p>
<p>Arctic keeps authored colors under neutral sky and ground lighting, with soft contact shadows on surfaces and a virtual floor beneath the lowest visible solid. Hidden objects, sheets and point clouds do not lower that floor. This screen-space approximation cannot include hidden or off-screen occluders.</p>
<h2 id="navigation">Navigation<a class="anchor" href="#/course/ssao#navigation" aria-label="Link to this section">#</a></h2>
<p>The same occlusion calculation runs during orbit, pan, zoom and stationary redraws. Drag tiers never disable it or lower its resolution. Arctic also keeps exact line visibility and the complete outline mask during slow drags, preventing lines from thinning while moving and thickening again on release. It suppresses the automatic canvas-scale/MSAA downgrade after sustained slow frames. Preserving line quality can cost more than the approximate drag rendering, especially in line-heavy scenes. While the camera moves, the filter reprojects the previous shading onto the same surface points to reduce sampling shimmer. Depth and surface-type checks reject newly exposed geometry, and neighborhood clamping bounds reused shading. Geometry edits, toggles and resizes discard history. Releasing a drag holds the last image without a quality transition or additional settling frames; a stationary camera reuses it.</p>
<p>A frame that can reuse history evaluates every second horizon slice and every second ground direction, the subset chosen by pixel position, so the spatial filter&#39;s neighbourhood sees every direction within one frame; a frame without history (the first after a toggle or an edit) evaluates all of them. The pattern is fixed in screen space, as the per-pixel noise already was, because a pattern that changes per frame shimmers through the history blend. On the dragon at 1080p the last moving frame differs from a fresh frame at the same camera on 0.73% of pixels by more than 2/255 (0.44% before this change), with the same maximum.</p>
<p>Surface occlusion runs at <code>clamp(1 / DPR, 0.25, 0.5)</code> times the canvas dimensions. On high-DPI phones this approaches CSS-pixel resolution. The broad ground-shadow field uses half that resolution with more horizon samples, then interpolates into the surface pass. These resolutions remain fixed throughout navigation.</p>
<h2 id="rendering">Rendering<a class="anchor" href="#/course/ssao#rendering" aria-label="Link to this section">#</a></h2>
<ul>
<li>Four cosine-weighted horizon slices estimate surface visibility from a linear-depth pyramid. Triangle normals remain faceted.</li>
<li>Ground contacts use twenty-four directions and six steps, with the original height and distance fades. They retain each object&#39;s contact radius.</li>
<li>The depth pyramid carries the original sampled pixel coordinates alongside a compact radius, so coarser levels reconstruct the actual occluder position. The same pass stores each half-resolution pixel&#39;s faceted normal as an octahedral RG8 image, which the horizon and filter passes read instead of re-deriving it from the triangle buffers; full-resolution reconstruction still derives exact per-sample normals.</li>
<li>Separable bilateral filtering, motion reprojection and depth-aware reconstruction produce a full-resolution R8 shading cache. Reconstruction follows the actual depth-sample centers to prevent subpixel drift.</li>
<li>At 4× MSAA, the cache stores the least occlusion among the pixel&#39;s samples. A cached correction buffer shades the other samples only in affected screen tiles. Both blends happen before MSAA resolve.</li>
<li>Browser pipelines compile after geometry appears, in idle callbacks or a deferred callback on browsers without that API. The 1× and 4× pipelines survive toggles and resizes. Camera movement creates no AO textures, bind groups or pipelines.</li>
</ul>
<p>The horizon integral follows <a href="https://www.activision.com/cdn/research/Practical_Real_Time_Strategies_for_Accurate_Indirect_Occlusion_NEW%20VERSION_COLOR.pdf" target="_blank" rel="noopener">Jimenez et al., Practical Real-Time Strategies for Accurate Indirect Occlusion</a>; <a href="https://github.com/GameTechDev/XeGTAO" target="_blank" rel="noopener">Intel&#39;s XeGTAO</a> provides a reference implementation. This implementation combines deterministic samples and spatial filtering with motion reprojection; it does not require a separate temporal-antialiasing pass.</p>
<h2 id="memory">Memory<a class="anchor" href="#/course/ssao#memory" aria-label="Link to this section">#</a></h2>
<p>At <strong>1920×1080, DPR 1</strong>, AO textures reserve <strong>8,942,580 bytes</strong> (8.94 MB): a six-level R32 depth/R16 metadata pyramid, a half-resolution RG8 normal image, two half-resolution R8 working images, quarter-resolution ground and history-depth images, a coarse occupancy mask and the full-resolution R8 cache. Resource estimates exclude driver alignment and internal allocations.</p>
<p>AO buffers add <strong>348 bytes at 1×</strong>, or <strong>8,360,016 bytes at 4×</strong>. The latter includes per-pixel sample corrections and tile indices for the cached MSAA blend. Turning Arctic off releases these textures and buffers; compiled pipelines remain available.</p>
<p>Faceted normals and contact radii come from the existing vertex, index, owner and instance buffers. Arctic neither allocates nor projects the large triangle table; picking and stroke visibility still use it when needed. The dragon fixture uses <strong>25.48 MB of total GPU buffers at 1× / 33.84 MB at 4×</strong>, below the 40 MB target.</p>
<h2 id="measurements">Measurements<a class="anchor" href="#/course/ssao#measurements" aria-label="Link to this section">#</a></h2>
<p>Native GPU timestamp medians on the dragon at 1920×1080, DPR 1, <strong>baseline → current</strong>, in milliseconds. These measurements and buffer totals were captured with outlines off; use <code>Outline Off</code> after enabling Arctic to reproduce that setup. The local <code>petras</code> laptop has an Intel i9-13900HX with Raptor Lake-S UHD graphics and an NVIDIA RTX 4080 Laptop. These measurements do not establish performance on the separate work laptop.</p>
<table>
<thead>
<tr>
<th>GPU</th>
<th align="right">MSAA</th>
<th align="right">Cached AO</th>
<th align="right">Moved AO</th>
<th align="right">Drag AO</th>
</tr>
</thead>
<tbody><tr>
<td>Intel Raptor Lake-S UHD</td>
<td align="right">1×</td>
<td align="right">0.345 → 0.193</td>
<td align="right">26.593 → 3.304</td>
<td align="right">11.720 → 3.304</td>
</tr>
<tr>
<td>Intel Raptor Lake-S UHD</td>
<td align="right">4×</td>
<td align="right">10.934 → 0.652</td>
<td align="right">45.055 → 5.804</td>
<td align="right">17.671 → 5.814</td>
</tr>
<tr>
<td>NVIDIA RTX 4080 Laptop</td>
<td align="right">1×</td>
<td align="right">0.020 → 0.014</td>
<td align="right">1.113 → 0.467</td>
<td align="right">0.449 → 0.456</td>
</tr>
<tr>
<td>NVIDIA RTX 4080 Laptop</td>
<td align="right">4×</td>
<td align="right">0.292 → 0.042</td>
<td align="right">1.812 → 0.607</td>
<td align="right">0.640 → 0.499</td>
</tr>
</tbody></table>
<p>Intel moving AO is 3.30 ms at 1× and 5.80 ms at 4×, within the 4 ms and 6 ms goals; the preceding pass measured 5.0 ms and 8.6 ms. Intel whole GPU frames during drag: 1× 11.44 ms; 4× 14.90 ms. Of the 4× figure, 1.3 to 1.7 ms (it varies between runs) is the first shader read of the multisampled depth and id textures in the half-resolution preparation pass, a cost any pass reading them pays once per frame. The NVIDIA rows predate the reduced sample sets; that GPU could not be measured afterwards because its kernel module and user-space driver versions differed. Timings measure GPU work, not browser input latency. The full effect includes depth preparation, ground shadows, filtering, history validation, MSAA reconstruction and compositing; it costs more than the GTAO horizon pass alone. The bunny also has an expensive existing edge-rendering pass independent of Arctic.</p>
<p>In a controlled native rotation regression, ground sampling and reprojection reduce variation at fixed ground points by about <strong>54%</strong> versus the spatial draft, retaining approximately <strong>97%</strong> of mean shadow strength. In a 32-frame browser capture of <code>view_live</code>, the denser ground sampling reduces variation by another <strong>16%</strong> over reprojection alone, with essentially unchanged shadow strength. Some view dependence remains because this is a screen-space approximation. Native validation passes 276 library tests; the AO GPU suite, browser contact/rotation checks and 42 published-scene configurations pass. Phone viewport checks use the desktop GPU.</p>
<h2 id="check-the-look">Check the look<a class="anchor" href="#/course/ssao#check-the-look" aria-label="Link to this section">#</a></h2>
<p>Open <code>/view_mixed</code>, run <code>Layers On</code>, select the floor-model group and run <strong>Fit</strong>. Clear selection with <strong>Escape</strong>, hide the panel with <code>Layers Off</code>, and compare <code>Arctic On</code> / <code>Arctic Off</code> at the column bases. Repeat with plates/contact and solids/BReps to inspect creases, the cone and the torus. Camera framing must remain identical.</p>
<p>Baseline references:</p>
<p><img src="/session/docs/course/docs/screenshots/ssao-floor.png" alt="Reference floor contacts" loading="lazy" decoding="async"></p>
<p><img src="/session/docs/course/docs/screenshots/ssao-plates.png" alt="Reference plate contacts" loading="lazy" decoding="async"></p>
<p><code>tests/ambient-details.cjs</code> checks primitive shadows and plate smoothness. <code>tests/ambient-floor.cjs</code> checks the full floor scene, late loading and unchanged camera framing. <code>tests/ambient-lighting.cjs</code> checks real WebGPU, memory release and controls; <code>AMBIENT_SPIN=1</code> exercises continuous rotation. Native tests cover both projections, 1×/4× MSAA, contact halos, unchanged images through drag tiers, immediate history rejection after geometry changes, and pipeline reuse across twenty toggles and twenty resizes. A rotation regression measures shadow variation at fixed world-space ground points and checks that smoothing retains contact strength.</p>
<p>The local default manifest cannot be checked because its five referenced <code>.pb</code> fixtures are missing. Published scenes provide the browser regression fixtures.</p>
<p>Review captures and benchmark reports for this checkout are under <code>target/review/arctic/</code>; <code>before/</code> preserves the original executable, browser bundle and captures.</p>
<p>For a horizontal cut at height 1200, use <code>clipping_plane XY 0,0,1200</code>; everything above is removed. <code>YZ</code> removes the +X side and <code>ZX</code> the +Y side. The default <code>Normal</code> mode takes two points: the first lies on the plane, and the direction from the first to the second is perpendicular to the plane, toward the side to remove. <code>clipping_plane Flip</code> reverses the selected plane.</p>
<p>Interactive clipping and drawing gather nearby snapping targets on demand, sharing the bounded object-dragging cache. Opening a command no longer gathers snaps from the entire scene. Cutting and section filling use GPU shaders; the first cut still performs a one-time CPU closedness check for meshes. <code>tests/clipping-mixed.cjs</code> checks command startup and hover in the published mixed scene.</p>
`,toc:[{level:2,id:"navigation",text:"Navigation"},{level:2,id:"rendering",text:"Rendering"},{level:2,id:"memory",text:"Memory"},{level:2,id:"measurements",text:"Measurements"},{level:2,id:"check-the-look",text:"Check the look"}]};export{e as default};
