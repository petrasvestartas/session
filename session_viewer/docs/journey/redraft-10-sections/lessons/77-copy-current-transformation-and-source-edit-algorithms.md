# 15.1 · Original transforms, part edits and solid recipes

**Section 8: Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing** · **Lesson 15.1** (Redraft `8.7`) · **49 checkpoints (`77-copy-current-transformation-and-source-edit-algorithms` → `77zzo-revolve-original-profiles-around-an-axis`)** · **Study time: 29.00000000000001–49 h**

> **What You Will Learn & Build:** Build directional scale transforms. By the final checkpoint (`77zzo-revolve-original-profiles-around-an-axis`), you will turn original curves into exact kernel surfaces around a chosen axis.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 15.1 Execution Flow: Original transforms, part edits and solid recipes</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (49 Checkpoints: 77-copy-current-transformation-and-source-edit-algorithms → 77zzo-revolve-original-profiles-around-an-axis)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="151" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">original f64 CAD</text>
    <text x="89" y="124" text-anchor="middle" font-size="11" fill="#111111">source</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="92" x2="316" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">pure transform or</text>
    <text x="254" y="124" text-anchor="middle" font-size="11" fill="#111111">original control</text>
    <text x="254" y="140" text-anchor="middle" font-size="11" fill="#111111">addresses</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="92" x2="481" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">later edit preview</text>
    <text x="419" y="124" text-anchor="middle" font-size="11" fill="#111111">and GPU display</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="92" x2="646" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">original profile</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">axis and angle</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 15.1:</text>
    <text x="215" y="181" font-size="11" fill="#111111">src/transform_scale.rs  ·  src/lib.rs  ·  src/transform_rotate.rs  ·  src/source_controls.rs  ·  src/source_deform.rs  ·  src/source_topology_colour.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">49 Checkpoints: 77-copy-current-transformation-and-source-edit-algorithms → 77zzo-revolve-original-profiles-around-an-axis — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Section 8 Precision CAD Frames, Solid Primitives, Surfacing & Sub-Object Deformation (Chapters 14–16)</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How double-precision coordinate frames (Plane), parametric solids, Loft/Sweep/Extrude surfaces, and Gumball control edits work.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="245" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. Orthonormal Frames (Ch 14)</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• Plane { origin: Vec3d, x, y, z }</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#111111">• Rejects non-finite / collinear axes</text>
    <text x="28" y="144" font-size="10.5" fill="#111111">• Basis change: World &lt;-&gt; Local Plane</text>
    <text x="28" y="162" font-size="10.5" fill="#111111">• Exact Mirror, Rotate3D, Orient,</text>
    <text x="28" y="178" font-size="10.5" fill="#111111">  and Align frame transformations</text>

    <line x1="263" y1="138" x2="293" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="295" y="68" width="265" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="427" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. Solids &amp; Surfacing (Ch 15–16)</text>
    <text x="307" y="108" font-size="10.5" fill="#111111">• Parametric Box, Cylinder, Sphere, Cone</text>
    <text x="307" y="126" font-size="10.5" font-weight="600" fill="#111111">• Extrude, Revolve, Loft, Sweep, Planar</text>
    <text x="307" y="144" font-size="10.5" fill="#111111">• Curve Split, Trim, Extend, Join, Fillet</text>
    <text x="307" y="162" font-size="10.5" fill="#111111">• Every operation commits an atomic</text>
    <text x="307" y="178" font-size="10.5" fill="#111112">  session_rust kernel transaction</text>

    <line x1="560" y1="138" x2="590" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="592" y="68" width="228" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="706" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. Rational Control-Point Deform</text>
    <text x="604" y="108" font-size="10.5" fill="#111111">• Pick NURBS control point or vertex</text>
    <text x="604" y="126" font-size="10.5" font-weight="600" fill="#111111">• Preserve homogeneous weight w</text>
    <text x="604" y="142" font-size="10.5" font-weight="600" fill="#111111">  on rational curve control drags!</text>
    <text x="604" y="162" font-size="10.5" fill="#111111">• Rebuild derived GPU tessellation</text>
    <text x="604" y="178" font-size="10.5" fill="#111111">  in-place while keeping Object ID</text>
    
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **CAD Coords** | `w10,20,0` vs `@5,0,0` vs `10<30` | `w` forces absolute World coordinates; `@` measures relative to the last accepted point; `<` uses polar distance and angle in the active Construction Plane. |
| **Rational NURBS** | Homogeneous control point `(w*x, w*y, w*z, w)` | Represents circles, ellipses, and conic arcs with zero approximation error using non-uniform weights `w`. |
| **B-Rep Split/Trim** | Plane/Cutter intersection -> Face loop classification | Splits or trims B-Rep and mesh faces along cutting planes/curves while keeping retained topology tables valid. |

**Authoritative production source (`session_viewer/src`):** [app/deform.rs](../../../../src/app/deform.rs), [state/edit.rs](../../../../src/state/edit.rs), [app/command/tool/gather.rs](../../../../src/app/command/tool/gather.rs), [app/command/tool/surfacing.rs](../../../../src/app/command/tool/surfacing.rs), [app/walk/mesh.rs](../../../../src/app/walk/mesh.rs), [app/walk/brep.rs](../../../../src/app/walk/brep.rs), [build.rs](../../../../build.rs).

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`77-copy-current-transformation-and-source-edit-algorithms`](#checkpoint-77-copy-current-transformation-and-source-edit-algorithms) | Build directional scale transforms. | `src/transform_scale.rs`, `src/lib.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **2** | [`77sa-compute-signed-construction-plane-rotation`](#checkpoint-77sa-compute-signed-construction-plane-rotation) | Compute signed construction-plane rotation. | `src/transform_rotate.rs`, `src/lib.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **3** | [`77sb-address-original-controls-and-exclusive-part-selections`](#checkpoint-77sb-address-original-controls-and-exclusive-part-selections) | Address original controls and exclusive part selections. | `src/source_controls.rs`, `src/lib.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **4** | [`77sc-collect-finite-original-controls-and-mesh-vertices`](#checkpoint-77sc-collect-finite-original-controls-and-mesh-vertices) | Collect finite original controls and mesh vertices. | `src/source_controls.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **5** | [`77sd-connect-curve-polygons-and-surface-control-nets`](#checkpoint-77sd-connect-curve-polygons-and-surface-control-nets) | Connect curve polygons and surface control nets. | `src/source_controls.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **6** | [`77se-reuse-control-collectors-across-original-geometry-families`](#checkpoint-77se-reuse-control-collectors-across-original-geometry-families) | Reuse control collectors across original geometry families. | `src/source_controls.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **7** | [`77sf-represent-original-geometry-edit-targets`](#checkpoint-77sf-represent-original-geometry-edit-targets) | Represent original geometry edit targets. | `src/source_deform.rs`, `src/lib.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **8** | [`77sg-resolve-original-mesh-vertices-for-part-edits`](#checkpoint-77sg-resolve-original-mesh-vertices-for-part-edits) | Resolve original mesh vertices for part edits. | `src/source_deform.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **9** | [`77sh-resolve-surface-grids-and-individual-source-controls`](#checkpoint-77sh-resolve-surface-grids-and-individual-source-controls) | Resolve surface grids and individual source controls. | `src/source_deform.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **10** | [`77si-collect-affected-points-across-cad-geometry-families`](#checkpoint-77si-collect-affected-points-across-cad-geometry-families) | Collect affected points across CAD geometry families. | `src/source_deform.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **11** | [`77sj-preserve-rational-weights-and-validate-brep-boundaries`](#checkpoint-77sj-preserve-rational-weights-and-validate-brep-boundaries) | Preserve rational weights and validate BRep boundaries. | `src/source_deform.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **12** | [`77sk-transform-original-mesh-and-surface-controls`](#checkpoint-77sk-transform-original-mesh-and-surface-controls) | Transform original mesh and surface controls. | `src/source_deform.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **13** | [`77sl-transform-original-line-polyline-point-and-curve-controls`](#checkpoint-77sl-transform-original-line-polyline-point-and-curve-controls) | Transform original line polyline point and curve controls. | `src/source_deform.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **14** | [`77sm-transform-shared-brep-boundary-positions-coherently`](#checkpoint-77sm-transform-shared-brep-boundary-positions-coherently) | Transform shared BRep boundary positions coherently. | `src/source_deform.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **15** | [`77sn-dispatch-original-part-edits-and-retain-element-ownership`](#checkpoint-77sn-dispatch-original-part-edits-and-retain-element-ownership) | Dispatch original part edits and retain Element ownership. | `src/source_deform.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **16** | [`77so-map-sparse-original-vertex-keys-to-display-slots`](#checkpoint-77so-map-sparse-original-vertex-keys-to-display-slots) | Map sparse original vertex keys to display slots. | `src/source_topology_colour.rs`, `src/source_mesh_topology.rs`, `src/lib.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **17** | [`77sp-compute-polygon-normals-from-original-boundaries`](#checkpoint-77sp-compute-polygon-normals-from-original-boundaries) | Compute polygon normals from original boundaries. | `src/source_mesh_topology.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **18** | [`77sq-build-one-original-edge-table-and-record-incident-faces`](#checkpoint-77sq-build-one-original-edge-table-and-record-incident-faces) | Build one original edge table and record incident faces. | `src/source_mesh_topology.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **19** | [`77sr-preserve-hole-winding-and-complete-original-mesh-topology`](#checkpoint-77sr-preserve-hole-winding-and-complete-original-mesh-topology) | Preserve hole winding and complete original mesh topology. | `src/source_mesh_topology.rs`, `src/source_deform.rs` | `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display` |
| **20** | [`77a-own-transformation-questions-and-the-selected-original-targets`](#checkpoint-77a-own-transformation-questions-and-the-selected-original-targets) | Own transformation questions and the selected original targets. | `src/transform_draft.rs`, `src/lib.rs` | `current selection and plane → frozen targets → pending transform questions` |
| **21** | [`77b-validate-point-and-number-transformation-answers`](#checkpoint-77b-validate-point-and-number-transformation-answers) | Validate point and number transformation answers. | `src/transform_draft.rs` | `point or number answer → checked references → current original transform` |
| **22** | [`77c-resolve-typed-batches-and-temporary-transformation-previews`](#checkpoint-77c-resolve-typed-batches-and-temporary-transformation-previews) | Resolve typed batches and temporary transformation previews. | `src/transform_draft.rs` | `typed batch → candidate owner → checked transforms or temporary preview` |
| **23** | [`77d-resolve-original-cad-parts-and-complete-selected-bounds`](#checkpoint-77d-resolve-original-cad-parts-and-complete-selected-bounds) | Resolve original CAD parts and complete selected bounds. | `src/source_part.rs`, `src/lib.rs`, `src/scene.rs` | `selected original part or owner → source geometry and world bounds → checked target` |
| **24** | [`77e-rebuild-an-edited-original-cad-source-before-adoption`](#checkpoint-77e-rebuild-an-edited-original-cad-source-before-adoption) | Rebuild an edited original CAD source before adoption. | `src/scene.rs`, `src/part_preview.rs` | `world edit → local original deformation → validated source → derived display` |
| **25** | [`77f-copy-original-rows-beside-their-existing-parents`](#checkpoint-77f-copy-original-rows-beside-their-existing-parents) | Copy original rows beside their existing parents. | `src/layer_edit.rs` | `selected original rows → checked copy plan → new identities and world placement` |
| **26** | [`77g-carry-transformation-state-through-the-existing-drawing-owner`](#checkpoint-77g-carry-transformation-state-through-the-existing-drawing-owner) | Carry transformation state through the existing drawing owner. | `src/tool.rs`, `src/editor.rs` | `current Draft → frozen transform owner → temporary scene and resource accounting` |
| **27** | [`77h-validate-selected-originals-before-beginning-or-committing-a-transform`](#checkpoint-77h-validate-selected-originals-before-beginning-or-committing-a-transform) | Validate selected originals before beginning or committing a transform. | `src/editor.rs` | `selected original targets → checked preview → current tool or one commit` |
| **28** | [`77i-adopt-validated-outcomes-and-preserve-repeat-copy-transactions`](#checkpoint-77i-adopt-validated-outcomes-and-preserve-repeat-copy-transactions) | Adopt validated outcomes and preserve repeat copy transactions. | `src/editor.rs` | `complete candidate batch → checked temporary scene → individual Copy transactions` |
| **29** | [`77j-route-transform-input-cancellation-and-current-plane-changes`](#checkpoint-77j-route-transform-input-cancellation-and-current-plane-changes) | Route transform input cancellation and current plane changes. | `src/editor.rs` | `owned event or typed input → current transform question → preview or cancellation` |
| **30** | [`77k-connect-transformation-lifecycle-and-part-move-dispatch`](#checkpoint-77k-connect-transformation-lifecycle-and-part-move-dispatch) | Connect transformation lifecycle and part Move dispatch. | `src/editor.rs`, `src/selection_tests.rs`, `src/edit_preview_tests.rs` | `transform lifecycle action → current plane and selected part → checked original preview` |
| **31** | [`77l-register-copy-scale-rotate-and-their-current-aliases`](#checkpoint-77l-register-copy-scale-rotate-and-their-current-aliases) | Register Copy Scale Rotate and their current aliases. | `src/transform_commands.rs`, `src/lib.rs`, `src/commands.rs`, `src/editor.rs` | `current verb or alias → checked parameter parser → Editor action` |
| **32** | [`77m-apply-direct-scale-and-rotate-about-the-current-selection-center`](#checkpoint-77m-apply-direct-scale-and-rotate-about-the-current-selection-center) | Apply direct Scale and Rotate about the current selection center. | `src/editor.rs` | `typed current factor or world axis → selection center → original transform` |
| **33** | [`77n-retain-earlier-selection-checks-with-source-part-support`](#checkpoint-77n-retain-earlier-selection-checks-with-source-part-support) | Retain earlier selection checks with source part support. | `src/edit_preview_tests.rs` | `current selection → validated source-part preview → unchanged history on cancel` |
| **34** | [`77o-exercise-original-copy-transforms-history-and-exact-persistence`](#checkpoint-77o-exercise-original-copy-transforms-history-and-exact-persistence) | Exercise original copy transforms history and exact persistence. | `src/transform_tests.rs`, `src/lib.rs` | `original selection → current Copy Scale Rotate → history → original Save/Open` |
| **35** | [`77zza-describe-curve-gathering-answers`](#checkpoint-77zza-describe-curve-gathering-answers) | Describe ordered curve questions and typed answers. | `src/gather_input.rs`, `src/lib.rs` | `original curve picks → ordered answers → Session kernel recipe` |
| **36** | [`77zzb-read-curve-gathering-values`](#checkpoint-77zzb-read-curve-gathering-values) | Read finite numbers, absolute points and distance vectors through the existing coordinate reader. | `src/gather_input.rs` | `original curve picks → ordered answers → Session kernel recipe` |
| **37** | [`77zzc-connect-gathering-recipes`](#checkpoint-77zzc-connect-gathering-recipes) | Connect typed input to a recipe, its questions and kernel builder. | `src/gather_input.rs` | `original curve picks → ordered answers → Session kernel recipe` |
| **38** | [`77zzd-parse-gathering-command-words`](#checkpoint-77zzd-parse-gathering-command-words) | Read options anywhere in a command and consume other answers in order. | `src/gather_input.rs` | `original curve picks → ordered answers → Session kernel recipe` |
| **39** | [`77zze-track-ordered-curve-picks`](#checkpoint-77zze-track-ordered-curve-picks) | Track preselected curve rows and skip answers already typed. | `src/gather_input.rs` | `original curve picks → ordered answers → Session kernel recipe` |
| **40** | [`77zzf-advance-and-cancel-curve-picks`](#checkpoint-77zzf-advance-and-cancel-curve-picks) | Toggle a picked curve, reject duplicates across groups, and advance on a full group or Enter. | `src/gather_input.rs` | `original curve picks → ordered answers → Session kernel recipe` |
| **41** | [`77zzg-prepare-placed-kernel-curves`](#checkpoint-77zzg-prepare-placed-kernel-curves) | Convert lines, polylines and NURBS curves to placed kernel curves. | `src/gather_input.rs` | `original curve picks → ordered answers → Session kernel recipe` |
| **42** | [`77zzh-check-original-loops-and-surface-controls`](#checkpoint-77zzh-check-original-loops-and-surface-controls) | Find a loop’s unit normal and scale-aware planarity tolerance. | `src/surfacing.rs`, `src/lib.rs` | `original curves → consistent section frames → validated kernel surface` |
| **43** | [`77zzi-align-original-sections-and-loft-through-them`](#checkpoint-77zzi-align-original-sections-and-loft-through-them) | Align open curve directions and closed curve seams before asking the Session kernel for a loft. | `src/surfacing.rs` | `original curves → consistent section frames → validated kernel surface` |
| **44** | [`77zzj-find-extrusion-axes-and-cap-eligibility`](#checkpoint-77zzj-find-extrusion-axes-and-cap-eligibility) | Reuse the original loop normal and planarity helpers to choose an extrusion axis. | `src/extrude_geometry.rs`, `src/lib.rs` | `original curves → offset and cap rules → shared BRep topology` |
| **45** | [`77zzk-map-cap-curves-to-surface-parameters`](#checkpoint-77zzk-map-cap-curves-to-surface-parameters) | Build a planar cap patch around the original control points. | `src/extrude_geometry.rs` | `original curves → offset and cap rules → shared BRep topology` |
| **46** | [`77zzl-build-closed-curved-extrusion-topology`](#checkpoint-77zzl-build-closed-curved-extrusion-topology) | Build the side surface from an oriented original curve. | `src/extrude_geometry.rs` | `original curves → offset and cap rules → shared BRep topology` |
| **47** | [`77zzm-extrude-original-corners-and-curves`](#checkpoint-77zzm-extrude-original-corners-and-curves) | Dispatch original corner chains and curved sections through the shared extrusion helpers. | `src/extrude_geometry.rs` | `original curves → offset and cap rules → shared BRep topology` |
| **48** | [`77zzn-connect-the-original-extrusion-recipe`](#checkpoint-77zzn-connect-the-original-extrusion-recipe) | Complete the original recipe with curve selection, signed distance and cap choices. | `src/extrude_geometry.rs` | `original curves → offset and cap rules → shared BRep topology` |
| **49** | [`77zzo-revolve-original-profiles-around-an-axis`](#checkpoint-77zzo-revolve-original-profiles-around-an-axis) | Turn original curves into exact kernel surfaces around a chosen axis. | `src/revolve_geometry.rs`, `src/lib.rs` | `original profile → axis and angle → kernel surface → one result` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-77-copy-current-transformation-and-source-edit-algorithms"></a>

### Step 1 of 49 · Build directional scale transforms (`77-copy-current-transformation-and-source-edit-algorithms`)

- **Goal:** Build directional scale transforms.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/transform_scale.rs` (*Build directional scale transforms.*) · `src/lib.rs` (*Register the source transformation component.*)

Measure scale reach in space, in the construction plane or along one chosen axis.

factor rejects a degenerate reference or nonpositive result. stretch writes a column-major transform as I+(k−1)Σaaᵀ, then chooses translation so the fixed origin maps to itself. Reuse the Session Xform owner; later Scale tools connect these pure calculations to previews and history.

> **Self-Check Question:** *How does directional scale preserve the other directions?*  
> **Answer:** Add (k−1) times each chosen axis outer product to the identity matrix.

- **What You Should See:** This preparation compiles in the cumulative browser application and preserves the current drawing. Its transforms and control views connect to the later editing tools.
- **Hands-On Lab Experiment:** For k=2 along the x axis, predict the images of [1,0,0] and [0,1,0]. Repeat about origin [5,0,0]; explain the translation term.

<details>
<summary>Implementation 77-copy-current-transformation-and-source-edit-algorithms: Build directional scale transforms</summary>

Build directional scale transforms.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/transform_scale.rs`

Build directional scale transforms.

Create the file and type:

```rust
--8<-- "journey/code/77-copy-current-transformation-and-source-edit-algorithms-typed-01.rs"
```

### 2. `src/lib.rs`

Register the source transformation component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod freeform_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77-copy-current-transformation-and-source-edit-algorithms-02.rs" skip=33
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/transform_scale.rs`

Supply independent original-source and fixed-origin regressions.

<details>
<summary>Locate the existing block</summary>

```text
ion.
fn stretch(origin: &Point, axes: &[Vector], k: f64) -> Xform {
    let mut xform = Xform::identity();

    for a in axes {
        for col in 0..3 {
            for row in 0..3 {
                xform.m[col * 4 + row] += (k - 1.0) * a[row] * a[col];
            }
        }
    }

    // the origin stays where it is
    for row in 0..3 {
        let moved: f64 = (0..3).map(|col| xform.m[col * 4 + row] * origin[col]).sum();
        xform.m[12 + row] = origin[row] - moved;
    }

    xform
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77-copy-current-transformation-and-source-edit-algorithms-typed-03.rs" skip=500
```

</details>

</details>

<a id="checkpoint-77sa-compute-signed-construction-plane-rotation"></a>

### Step 2 of 49 · Compute signed construction-plane rotation (`77sa-compute-signed-construction-plane-rotation`)

- **Goal:** Compute signed construction-plane rotation.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/transform_rotate.rs` (*Compute signed construction-plane rotation.*) · `src/lib.rs` (*Register the source transformation component.*)

Project the two reference vectors onto the construction plane axes.

The 2D cross and dot products give a signed angle; a point on the centre has no defined angle. Compose translation to the centre, rotation about the plane normal and translation back. The rightmost transform acts first. This remains f64 CAD math; the renderer converts final display placements for the GPU.

> **Self-Check Question:** *Why use atan2 of the planar cross and dot products?*  
> **Answer:** It preserves the rotation sign and quadrant without dividing by a potentially zero dot product.

- **Hands-On Lab Experiment:** Predict the signed angle from the plane x axis to its y axis and in the reverse direction. Verify that rotating about [5,5,0] keeps that centre fixed.

<details>
<summary>Implementation 77sa-compute-signed-construction-plane-rotation: Compute signed construction-plane rotation</summary>

Compute signed construction-plane rotation.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/transform_rotate.rs`

Compute signed construction-plane rotation.

Create the file and type:

```rust
--8<-- "journey/code/77sa-compute-signed-construction-plane-rotation-typed-01.rs"
```

### 2. `src/lib.rs`

Register the source transformation component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod transform_scale;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77-copy-current-transformation-and-source-edit-algorithms-04.rs" skip=26
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/transform_rotate.rs`

Supply independent original-source and fixed-origin regressions.

<details>
<summary>Locate the existing block</summary>

```text
ot(&y));

    if ax.hypot(ay) <= 1e-12 || bx.hypot(by) <= 1e-12 {
        return None;
    }

    Some((ax * by - ay * bx).atan2(ax * bx + ay * by))
}

/// A turn of `radians` about the plane normal through `center`.
pub fn rotation(center: &Point, radians: f64, plane: &Plane) -> Xform {
    let to = Xform::translation(center[0], center[1], center[2]);
    let back = Xform::translation(-center[0], -center[1], -center[2]);
    &(&to * &Xform::rotation(&plane.z_axis(), radians, false)) * &back
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77sa-compute-signed-construction-plane-rotation-typed-03.rs" skip=500
```

</details>

</details>

<a id="checkpoint-77sb-address-original-controls-and-exclusive-part-selections"></a>

### Step 3 of 49 · Address original controls and exclusive part selections (`77sb-address-original-controls-and-exclusive-part-selections`)

- **Goal:** Address original controls and exclusive part selections.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_controls.rs` (*Address original controls and exclusive part selections.*) · `src/lib.rs` (*Register the source transformation component.*)

Define stable source addresses for vertices, curve controls, surface controls and cloud points.

SelectionMode owns one original parent and one exclusive part mode; changing parents replaces the specialized state. Escape returns the parent while leaving part mode. SelectionTool describes what the next click should pick, separately from what is already selected.

> **Self-Check Question:** *Why does a control ID contain both a surface and two grid indices?*  
> **Answer:** It identifies an original control within one source surface rather than a displayed tessellation vertex.

- **Hands-On Lab Experiment:** Trace Object → Edge on parent4 → Controls on parent4 → Edge on parent9 → Escape. Predict the retained parent at each transition.

<details>
<summary>Implementation 77sb-address-original-controls-and-exclusive-part-selections: Address original controls and exclusive part selections</summary>

Address original controls and exclusive part selections.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_controls.rs`

Address original controls and exclusive part selections.

Create the file and type:

```rust
--8<-- "journey/code/77sb-address-original-controls-and-exclusive-part-selections-typed-01.rs"
```

### 2. `src/lib.rs`

Register the source transformation component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod transform_rotate;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77-copy-current-transformation-and-source-edit-algorithms-06.rs" skip=27
```

</details>

<a id="checkpoint-77sc-collect-finite-original-controls-and-mesh-vertices"></a>

### Step 4 of 49 · Collect finite original controls and mesh vertices (`77sc-collect-finite-original-controls-and-mesh-vertices`)

- **Goal:** Collect finite original controls and mesh vertices.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_controls.rs` (*Collect finite original controls and mesh vertices.*)

A control keeps its original address and f64 position.

push rejects non-finite positions before assigning a contiguous display index. Mesh controls come from original vertex keys, without inventing addresses from tessellation triangles. Cloud control display stays on the existing GPU cloud owner rather than copying every source point into this CPU vector.

> **Self-Check Question:** *Why return the inserted display index separately from the original vertex key?*  
> **Answer:** A sparse original key identifies the source; contiguous display indices connect the control net.

- **Hands-On Lab Experiment:** Predict the display indices for valid source keys2 and100 with an invalid position between them. Explain why neither display index may replace the original key.

<details>
<summary>Implementation 77sc-collect-finite-original-controls-and-mesh-vertices: Collect finite original controls and mesh vertices</summary>

Collect finite original controls and mesh vertices.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_controls.rs`

Collect finite original controls and mesh vertices.

<details>
<summary>Locate the existing block</summary>

```rust
        *self = Self::Controls {
            parent,
            selected: None,
            cloud,
        };
        true
    }

    /// Back to object selection; returns the parent.
    pub fn escape(&mut self) -> Option<u32> {
        let parent = self.parent();
        *self = Self::Object;
        parent
    }
}

/// What a click picks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectionTool {
    #[default]
    Object, // whole objects
    Edge, // edges
    Face, // faces
}
```

</details>

After that block, type:

```rust

/// One control point and where it is.
#[derive(Clone, Debug)]
pub struct Control {
    pub id: ControlId,      // which control
    pub position: [f64; 3], // world position
}

/// Every control of one object and the lines between them.
#[derive(Default)]
pub struct Controls {
    pub points: Vec<Control>,   // the controls
    pub links: Vec<[usize; 2]>, // control net lines, by index
    pub cloud: bool,            // a point cloud, controls stay on the GPU
}

impl Controls {
    /// The controls of a point cloud: its points stay on the GPU.
    pub fn cloud() -> Self {
        Self {
            cloud: true,
            ..Self::default()
        }
    }

    /// Add one control; None when its position is not finite.
    fn push(&mut self, id: ControlId, point: &Point) -> Option<usize> {
        let position = [point[0], point[1], point[2]];

        if !position.into_iter().all(f64::is_finite) {
            return None;
        }

        let index = self.points.len();
        self.points.push(Control { id, position });
        Some(index)
    }

    /// Every mesh vertex.
    fn mesh(&mut self, mesh: &session_rust::Mesh) {
        for key in mesh.vertices() {
            if let Some(vertex) = mesh.vertex.get(&key) {
                self.push(
                    ControlId::Vertex(key),
                    &Point::new(vertex.x, vertex.y, vertex.z),
                );
            }
        }
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sc-collect-finite-original-controls-and-mesh-vertices-typed-01.rs"
```

</details>

</details>

<a id="checkpoint-77sd-connect-curve-polygons-and-surface-control-nets"></a>

### Step 5 of 49 · Connect curve polygons and surface control nets (`77sd-connect-curve-polygons-and-surface-control-nets`)

- **Goal:** Connect curve polygons and surface control nets.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_controls.rs` (*Connect curve polygons and surface control nets.*)

Reuse push for finite positions and original addresses.

A curve links successive accepted controls. A surface keeps one previous index per v column plus the previous point in the current u row, producing both grid directions with bounded temporary storage. Missing controls clear the corresponding continuation; links use contiguous display indices while edits retain source u/v addresses.

> **Self-Check Question:** *Should a control net connect across a missing or invalid control?*  
> **Answer:** No. Clearing the previous index at that point breaks the link rather than bridging a gap.

- **Hands-On Lab Experiment:** Draw the links of a2×3 control grid. Remove its middle control and predict which horizontal and vertical links disappear.

<details>
<summary>Implementation 77sd-connect-curve-polygons-and-surface-control-nets: Connect curve polygons and surface control nets</summary>

Connect curve polygons and surface control nets.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_controls.rs`

Connect curve polygons and surface control nets.

<details>
<summary>Locate the existing block</summary>

```rust
            return None;
        }

        let index = self.points.len();
        self.points.push(Control { id, position });
        Some(index)
    }

    /// Every mesh vertex.
    fn mesh(&mut self, mesh: &session_rust::Mesh) {
        for key in mesh.vertices() {
            if let Some(vertex) = mesh.vertex.get(&key) {
                self.push(
                    ControlId::Vertex(key),
                    &Point::new(vertex.x, vertex.y, vertex.z),
                );
            }
        }
    }
}
```

</details>

After that block, type:

```rust

impl Controls {
    /// A curve's control polygon.
    fn curve(&mut self, curve: &NurbsCurve, index: usize) {
        let mut previous = None;

        for point in 0..curve.cv_count() {
            let current = match curve.get_cv(point) {
                Some(position) => self.push(
                    ControlId::Curve {
                        curve: index,
                        point,
                    },
                    &position,
                ),
                None => None,
            };

            if let (Some(start), Some(end)) = (previous, current) {
                self.links.push([start, end]);
            }

            previous = current;
        }
    }

    /// A surface's control net.
    fn surface(&mut self, surface: &NurbsSurface, index: usize) {
        let [width, height] = surface.m_cv_count;
        let mut previous = vec![None; height];

        for u in 0..width {
            let mut last = None;

            for (v, above) in previous.iter_mut().enumerate() {
                let current = match surface.get_cv(u, v) {
                    Some(position) => self.push(
                        ControlId::Surface {
                            surface: index,
                            u,
                            v,
                        },
                        &position,
                    ),
                    None => None,
                };

                if let (Some(start), Some(end)) = (*above, current) {
                    self.links.push([start, end]);
                }

                if let (Some(start), Some(end)) = (last, current) {
                    self.links.push([start, end]);
                }

                *above = current;
                last = current;
            }
        }
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sd-connect-curve-polygons-and-surface-control-nets-typed-01.rs"
```

</details>

</details>

<a id="checkpoint-77se-reuse-control-collectors-across-original-geometry-families"></a>

### Step 6 of 49 · Reuse control collectors across original geometry families (`77se-reuse-control-collectors-across-original-geometry-families`)

- **Goal:** Reuse control collectors across original geometry families.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_controls.rs` (*Reuse control collectors across original geometry families.*)

BRep controls retain vertex, curve and surface addresses.

Geometry dispatch reuses those collectors for Element source payloads and handles original line, polyline and point controls directly. A point cloud enables its GPU control mode; Plane and OBB have no controls here. from_geometry builds one fresh control view without mutating the original Session geometry.

> **Self-Check Question:** *Does an Element need a second mesh control algorithm?*  
> **Answer:** No. Dispatch its original Mesh or BRep to the collectors already written.

- **Hands-On Lab Experiment:** Compare a Mesh with an Element containing that same Mesh. Their control positions and vertex addresses should agree; a line should supply exactly two controls and one link.

<details>
<summary>Implementation 77se-reuse-control-collectors-across-original-geometry-families: Reuse control collectors across original geometry families</summary>

Reuse control collectors across original geometry families.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_controls.rs`

Reuse control collectors across original geometry families.

<details>
<summary>Locate the existing block</summary>

```rust
                        },
                        &position,
                    ),
                    None => None,
                };

                if let (Some(start), Some(end)) = (*above, current) {
                    self.links.push([start, end]);
                }

                if let (Some(start), Some(end)) = (last, current) {
                    self.links.push([start, end]);
                }

                *above = current;
                last = current;
            }
        }
    }
}
```

</details>

After that block, type:

```rust

impl Controls {
    /// BRep vertices, then curve and surface nets.
    fn brep(&mut self, brep: &session_rust::BRep) {
        for (index, vertex) in brep.m_vertices.iter().enumerate() {
            self.push(ControlId::Vertex(index), &vertex.point);
        }

        for (index, curve) in brep.m_curves_3d.iter().enumerate() {
            self.curve(curve, index);
        }

        for (index, surface) in brep.m_surfaces.iter().enumerate() {
            self.surface(surface, index);
        }
    }

    /// The controls of one geometry.
    pub fn from_geometry(geometry: &Geometry) -> Self {
        let mut controls = Self::default();
        controls.append_geometry(geometry);
        controls
    }

    /// Add the controls of one geometry.
    fn append_geometry(&mut self, geometry: &Geometry) {
        match geometry {
            Geometry::Mesh(mesh) => self.mesh(mesh),
            Geometry::Line(line) => {
                self.push(ControlId::Vertex(0), &line.start());
                self.push(ControlId::Vertex(1), &line.end());

                if self.points.len() == 2 {
                    self.links.push([0, 1]);
                }
            }
            Geometry::Polyline(polyline) => {
                let mut previous = None;

                for (index, coords) in polyline.coords.chunks_exact(3).enumerate() {
                    let current = self.push(
                        ControlId::Vertex(index),
                        &Point::new(coords[0], coords[1], coords[2]),
                    );

                    if let (Some(start), Some(end)) = (previous, current) {
                        self.links.push([start, end]);
                    }

                    previous = current;
                }
            }
            Geometry::NurbsCurve(curve) => self.curve(curve, 0),
            Geometry::NurbsSurface(surface) => self.surface(surface, 0),
            Geometry::BRep(brep) => self.brep(brep),
            Geometry::PointCloud(_) => self.cloud = true,
            Geometry::Point(point) => {
                self.push(ControlId::Vertex(0), point);
            }
            Geometry::Element(element) => match element.geometry() {
                ElementGeometry::Mesh(mesh) => self.mesh(mesh),
                ElementGeometry::BRep(brep) => self.brep(brep),
                ElementGeometry::None => {}
            },
            Geometry::Plane(_) | Geometry::OBB(_) => {}
        }
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77se-reuse-control-collectors-across-original-geometry-families-typed-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/source_controls.rs`

Supply independent original-source and fixed-origin regressions.

<details>
<summary>Locate the existing block</summary>

```text
,
            Geometry::PointCloud(_) => self.cloud = true,
            Geometry::Point(point) => {
                self.push(ControlId::Vertex(0), point);
            }
            Geometry::Element(element) => match element.geometry() {
                ElementGeometry::Mesh(mesh) => self.mesh(mesh),
                ElementGeometry::BRep(brep) => self.brep(brep),
                ElementGeometry::None => {}
            },
            Geometry::Plane(_) | Geometry::OBB(_) => {}
        }
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77se-reuse-control-collectors-across-original-geometry-families-typed-02.rs" skip=500
```

</details>

</details>

<a id="checkpoint-77sf-represent-original-geometry-edit-targets"></a>

### Step 7 of 49 · Represent original geometry edit targets (`77sf-represent-original-geometry-edit-targets`)

- **Goal:** Represent original geometry edit targets.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_deform.rs` (*Represent original geometry edit targets.*) · `src/lib.rs` (*Register the source transformation component.*)

Target translates the exclusive selection mode into a source edit request.

Keep the original control IDs from the preceding collector lessons. A whole-object selection has no part target; its placement is edited through the existing scene transform path. The following helpers resolve the source points affected by each part request.

> **Self-Check Question:** *Does a displayed triangle identify an editable source face?*  
> **Answer:** No. Edits use original vertex, edge, face or control addresses.

- **Hands-On Lab Experiment:** Trace a surface-control selection and a whole-object selection into Target::selected.

<details>
<summary>Implementation 77sf-represent-original-geometry-edit-targets: Represent original geometry edit targets</summary>

Represent original geometry edit targets.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_deform.rs`

Represent original geometry edit targets.

Create the file and type:

```rust
--8<-- "journey/code/77sf-represent-original-geometry-edit-targets-typed-01.rs"
```

### 2. `src/lib.rs`

Register the source transformation component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod source_controls;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77-copy-current-transformation-and-source-edit-algorithms-08.rs" skip=26
```

</details>

<a id="checkpoint-77sg-resolve-original-mesh-vertices-for-part-edits"></a>

### Step 8 of 49 · Resolve original mesh vertices for part edits (`77sg-resolve-original-mesh-vertices-for-part-edits`)

- **Goal:** Resolve original mesh vertices for part edits.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_deform.rs` (*Resolve original mesh vertices for part edits.*)

Resolve a selected vertex directly by key. A face collects its outer and hole vertices, then sorts and deduplicates them. Mesh edge numbering follows first appearance of an unordered vertex pair across original faces. This reproduces the source edge convention instead of numbering tessellation edges.

> **Self-Check Question:** *Should a face edit leave its hole rim behind?*  
> **Answer:** No. Include the original hole vertices with the outer face boundary.

- **Hands-On Lab Experiment:** Draw two faces sharing one edge. Predict the first-appearance edge numbers and the vertex keys affected by moving either face.

<details>
<summary>Implementation 77sg-resolve-original-mesh-vertices-for-part-edits: Resolve original mesh vertices for part edits</summary>

Resolve original mesh vertices for part edits.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_deform.rs`

Resolve original mesh vertices for part edits.

<details>
<summary>Locate the existing block</summary>

```rust
    Face(usize),        // one face by key
}

impl Target {
    /// The target the current selection names, if any.
    pub fn selected(mode: &SelectionMode) -> Option<Self> {
        match *mode {
            SelectionMode::Controls {
                selected: Some(id), ..
            } => Some(Self::Control(id)),
            SelectionMode::Edge { edge, .. } => Some(Self::Edge(edge)),
            SelectionMode::Face { face, .. } => Some(Self::Face(face)),
            _ => None,
        }
    }
}
```

</details>

After that block, type:

```rust

/// The mesh vertex keys a target covers.
pub(crate) fn mesh_keys(mesh: &Mesh, target: Target) -> Result<Vec<usize>, String> {
    match target {
        Target::Control(ControlId::Vertex(key)) if mesh.vertex.contains_key(&key) => Ok(vec![key]),
        Target::Face(key) => {
            let mut keys = mesh.face.get(&key).ok_or("Unknown mesh face")?.clone();

            if let Some(holes) = mesh.face_holes.get(&key) {
                keys.extend(holes.iter().flatten()); // hole rims move with the face
            }

            keys.sort_unstable();
            keys.dedup();
            Ok(keys)
        }
        Target::Edge(index) => {
            // edges are numbered in order of first appearance
            let mut seen = HashSet::new();
            let mut at = 0;

            for face in mesh.faces() {
                let keys = &mesh.face[&face];

                for i in 0..keys.len() {
                    let (a, b) = (keys[i], keys[(i + 1) % keys.len()]);
                    let pair = (a.min(b), a.max(b));

                    if seen.insert(pair) {
                        if at == index {
                            return Ok(vec![pair.0, pair.1]);
                        }

                        at += 1;
                    }
                }
            }

            Err("Unknown mesh edge".into())
        }
        _ => Err("Select a mesh vertex, edge or face".into()),
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sg-resolve-original-mesh-vertices-for-part-edits-typed-01.rs"
```

</details>

</details>

<a id="checkpoint-77sh-resolve-surface-grids-and-individual-source-controls"></a>

### Step 9 of 49 · Resolve surface grids and individual source controls (`77sh-resolve-surface-grids-and-individual-source-controls`)

- **Goal:** Resolve surface grids and individual source controls.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_deform.rs` (*Resolve surface grids and individual source controls.*)

A surface control uses its original u/v address. Face0 selects the complete control grid; edges0–3 choose its four sides. For other geometry families, control_point reuses Controls::from_geometry and its stable IDs. There is one control collector, so picking and source editing agree about the addressed point.

> **Self-Check Question:** *What does a surface boundary edit select?*  
> **Answer:** One side of the original u/v control grid, not a chain of sampled display vertices.

- **Hands-On Lab Experiment:** For a3×2 control grid, list the u/v pairs affected by each boundary and by its complete face.

<details>
<summary>Implementation 77sh-resolve-surface-grids-and-individual-source-controls: Resolve surface grids and individual source controls</summary>

Resolve surface grids and individual source controls.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_deform.rs`

Resolve surface grids and individual source controls.

<details>
<summary>Locate the existing block</summary>

```rust
                    let (a, b) = (keys[i], keys[(i + 1) % keys.len()]);
                    let pair = (a.min(b), a.max(b));

                    if seen.insert(pair) {
                        if at == index {
                            return Ok(vec![pair.0, pair.1]);
                        }

                        at += 1;
                    }
                }
            }

            Err("Unknown mesh edge".into())
        }
        _ => Err("Select a mesh vertex, edge or face".into()),
    }
}
```

</details>

After that block, type:

```rust

/// The surface control (u, v) pairs a target covers.
fn surface_keys(surface: &NurbsSurface, target: Target) -> Result<Vec<(usize, usize)>, String> {
    let [nu, nv] = surface.m_cv_count; // control grid size
    let all = || (0..nu).flat_map(|u| (0..nv).map(move |v| (u, v))).collect();

    match target {
        Target::Control(ControlId::Surface { surface: 0, u, v }) if u < nu && v < nv => {
            Ok(vec![(u, v)])
        }
        Target::Face(0) => Ok(all()),
        // edge 0..3: the four sides of the control grid
        Target::Edge(edge) if edge < 4 => Ok((0..nu)
            .flat_map(|u| (0..nv).map(move |v| (u, v)))
            .filter(|&(u, v)| match edge {
                0 => u == 0,
                1 => u + 1 == nu,
                2 => v == 0,
                _ => v + 1 == nv,
            })
            .collect()),
        _ => Err("Unknown surface control, boundary or face".into()),
    }
}

/// One control point of any geometry.
fn control_point(geometry: &Geometry, id: ControlId) -> Result<Point, String> {
    Controls::from_geometry(geometry)
        .points
        .into_iter()
        .find(|p| p.id == id)
        .map(|p| Point::new(p.position[0], p.position[1], p.position[2]))
        .ok_or("Unknown source control".into())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sh-resolve-surface-grids-and-individual-source-controls-typed-01.rs"
```

</details>

</details>

<a id="checkpoint-77si-collect-affected-points-across-cad-geometry-families"></a>

### Step 10 of 49 · Collect affected points across CAD geometry families (`77si-collect-affected-points-across-cad-geometry-families`)

- **Goal:** Collect affected points across CAD geometry families.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_deform.rs` (*Collect affected points across CAD geometry families.*)

Dispatch the selected target to the existing source key and control helpers.

A BRep face uses its incident surface controls and an edge uses its original3D curve controls. Element payloads reuse the same Mesh/BRep resolvers. This collects edit anchors without changing source geometry or display placement.

> **Self-Check Question:** *Does editing an Element require flattening its source payload?*  
> **Answer:** No. Reuse the Mesh or BRep point resolver while retaining the original Element wrapper.

- **Hands-On Lab Experiment:** Compare points returned for a Mesh and an Element containing that Mesh. Predict the rejection for a face request on an original Line.

<details>
<summary>Implementation 77si-collect-affected-points-across-cad-geometry-families: Collect affected points across CAD geometry families</summary>

Collect affected points across CAD geometry families.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_deform.rs`

Collect affected points across CAD geometry families.

<details>
<summary>Locate the existing block</summary>

```rust
                _ => v + 1 == nv,
            })
            .collect()),
        _ => Err("Unknown surface control, boundary or face".into()),
    }
}

/// One control point of any geometry.
fn control_point(geometry: &Geometry, id: ControlId) -> Result<Point, String> {
    Controls::from_geometry(geometry)
        .points
        .into_iter()
        .find(|p| p.id == id)
        .map(|p| Point::new(p.position[0], p.position[1], p.position[2]))
        .ok_or("Unknown source control".into())
}
```

</details>

After that block, type:

```rust

/// The world points a target covers.
pub fn points(geometry: &Geometry, target: Target) -> Result<Vec<Point>, String> {
    match geometry {
        Geometry::Mesh(mesh) => mesh_keys(mesh, target)?
            .into_iter()
            .map(|k| mesh.vertex_point(k).ok_or("Missing mesh vertex".into()))
            .collect(),
        Geometry::NurbsSurface(surface) => surface_keys(surface, target)?
            .into_iter()
            .map(|(u, v)| surface.get_cv(u, v).ok_or("Missing surface control".into()))
            .collect(),
        Geometry::BRep(brep) => match target {
            Target::Face(face) => {
                let face = brep.m_faces.get(face).ok_or("Unknown BRep face")?;
                let surface = brep
                    .m_surfaces
                    .get(face.surface_index as usize)
                    .ok_or("Missing face surface")?;
                Ok((0..surface.m_cv_count[0])
                    .flat_map(|u| {
                        (0..surface.m_cv_count[1]).filter_map(move |v| surface.get_cv(u, v))
                    })
                    .collect())
            }
            Target::Edge(edge) => {
                let edge = brep.m_edges.get(edge as usize).ok_or("Unknown BRep edge")?;
                let curve = brep
                    .m_curves_3d
                    .get(edge.curve_3d_index as usize)
                    .ok_or("Degenerate edge has no movable curve")?;
                Ok((0..curve.cv_count())
                    .filter_map(|i| curve.get_cv(i))
                    .collect())
            }
            Target::Control(id) => control_point(geometry, id).map(|p| vec![p]),
        },
        Geometry::Element(element) => match element.geometry() {
            session_rust::element::ElementGeometry::Mesh(mesh) => {
                points(&Geometry::Mesh(Rc::new(mesh.clone())), target)
            }
            session_rust::element::ElementGeometry::BRep(brep) => {
                points(&Geometry::BRep(Rc::new(brep.clone())), target)
            }
            _ => Err("Element has no source geometry".into()),
        },
        _ => match target {
            Target::Control(id) => control_point(geometry, id).map(|p| vec![p]),
            _ => Err("This source has no editable faces or edges".into()),
        },
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77si-collect-affected-points-across-cad-geometry-families-typed-01.rs"
```

</details>

</details>

<a id="checkpoint-77sj-preserve-rational-weights-and-validate-brep-boundaries"></a>

### Step 11 of 49 · Preserve rational weights and validate BRep boundaries (`77sj-preserve-rational-weights-and-validate-brep-boundaries`)

- **Goal:** Preserve rational weights and validate BRep boundaries.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_deform.rs` (*Preserve rational weights and validate BRep boundaries.*)

For rational curves, store the moved Euclidean point multiplied by its unchanged weight.

Before committing a BRep edit, validate topology and sample changed incident3D edges against their2D trim curves on the adjacent surfaces. Reject an edit that requires trim rebuilding; the caller can then retain the original solid atomically. This bounded check is not a general trim reconstruction algorithm.

> **Self-Check Question:** *Why preserve a rational control weight when moving its point?*  
> **Answer:** The Euclidean point changes, while its original rational weighting remains part of the curve definition.

- **Hands-On Lab Experiment:** Explain why translating one boundary curve without its adjacent surface may be rejected. Predict the homogeneous coordinates for point[2,3,4] with weight2.

<details>
<summary>Implementation 77sj-preserve-rational-weights-and-validate-brep-boundaries: Preserve rational weights and validate BRep boundaries</summary>

Preserve rational weights and validate BRep boundaries.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_deform.rs`

Preserve rational weights and validate BRep boundaries.

<details>
<summary>Locate the existing block</summary>

```rust
                points(&Geometry::Mesh(Rc::new(mesh.clone())), target)
            }
            session_rust::element::ElementGeometry::BRep(brep) => {
                points(&Geometry::BRep(Rc::new(brep.clone())), target)
            }
            _ => Err("Element has no source geometry".into()),
        },
        _ => match target {
            Target::Control(id) => control_point(geometry, id).map(|p| vec![p]),
            _ => Err("This source has no editable faces or edges".into()),
        },
    }
}
```

</details>

After that block, type:

```rust

fn set_curve_control(curve: &mut session_rust::NurbsCurve, index: usize, point: &Point) -> bool {
    if !curve.m_is_rat {
        return curve.set_cv_point(index, point);
    }
    let Some((_, _, _, weight)) = curve.get_cv_4d(index) else {
        return false;
    };
    curve.set_cv_4d(index, point[0] * weight, point[1] * weight, point[2] * weight, weight)
}

/// Refuse an edit whose edges no longer lie on their surfaces.
fn validate_boundaries(
    brep: &session_rust::BRep,
    changed_curves: &HashSet<usize>,
    changed_surfaces: &HashSet<usize>,
) -> Result<(), String> {
    if !brep.is_valid() {
        return Err("Edit would invalidate BRep topology".into());
    }

    for edge in &brep.m_edges {
        if edge.degenerated
            || (!changed_curves.contains(&(edge.curve_3d_index as usize))
                && !edge
                    .pcurves
                    .iter()
                    .any(|pc| changed_surfaces.contains(&(pc.surface_index as usize))))
        {
            continue;
        }

        let curve = &brep.m_curves_3d[edge.curve_3d_index as usize];
        let (a, b) = curve.domain();

        // sample the 3D edge against each surface's 2D curve
        for pc in &edge.pcurves {
            let surface = &brep.m_surfaces[pc.surface_index as usize];

            for ci in [pc.curve_2d_index, pc.curve_2d_index_2] {
                if ci < 0 {
                    continue;
                }

                let uv = &brep.m_curves_2d[ci as usize];
                let (u0, u1) = uv.domain();

                for sample in 0..=16 {
                    let t = sample as f64 / 16.0;
                    let p = curve.point_at(a + (b - a) * t);
                    let q = uv.point_at(u0 + (u1 - u0) * t);
                    let actual = surface
                        .point_at(q[0], q[1])
                        .ok_or("Cannot evaluate incident surface")?;
                    let tolerance = edge.tolerance.max(1e-6) * 10.0;

                    if (0..3).any(|i| (p[i] - actual[i]).abs() > tolerance) {
                        return Err("This BRep edit requires rebuilding adjacent trims; the original solid was preserved".into());
                    }
                }
            }
        }
    }

    Ok(())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sj-preserve-rational-weights-and-validate-brep-boundaries-typed-01.rs"
```

</details>

</details>

<a id="checkpoint-77sk-transform-original-mesh-and-surface-controls"></a>

### Step 12 of 49 · Transform original mesh and surface controls (`77sk-transform-original-mesh-and-surface-controls`)

- **Goal:** Transform original mesh and surface controls.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_deform.rs` (*Transform original mesh and surface controls.*)

Clone the original source before changing addressed points.

Mesh edits update only resolved vertex keys and clear the triangle BVH. Surface edits update original controls and drop the cached mesh. The original source object remains available for rollback and Undo; later display preparation regenerates geometry from the edited source.

> **Self-Check Question:** *Why clear derived acceleration and tessellation caches after source controls move?*  
> **Answer:** Those caches describe the previous source positions and must be regenerated.

- **Hands-On Lab Experiment:** Move one face of two adjacent mesh faces. List the shared vertices that change and the untouched vertices that remain identical.

<details>
<summary>Implementation 77sk-transform-original-mesh-and-surface-controls: Transform original mesh and surface controls</summary>

Transform original mesh and surface controls.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_deform.rs`

Transform original mesh and surface controls.

<details>
<summary>Locate the existing block</summary>

```rust
                    let actual = surface
                        .point_at(q[0], q[1])
                        .ok_or("Cannot evaluate incident surface")?;
                    let tolerance = edge.tolerance.max(1e-6) * 10.0;

                    if (0..3).any(|i| (p[i] - actual[i]).abs() > tolerance) {
                        return Err("This BRep edit requires rebuilding adjacent trims; the original solid was preserved".into());
                    }
                }
            }
        }
    }

    Ok(())
}
```

</details>

After that block, type:

```rust

fn transform_mesh(source: &Rc<session_rust::Mesh>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let mut mesh = (**source).clone();

    for key in mesh_keys(&mesh, target)? {
        let point = mesh
            .vertex_point(key)
            .ok_or("Missing vertex")?
            .transformed(delta);
        mesh.vertex
            .get_mut(&key)
            .ok_or("Missing vertex")?
            .set_position(point);
    }

    mesh.clear_triangle_bvh(); // stale after moving vertices
    Ok(Geometry::Mesh(Rc::new(mesh)))
}

fn transform_nurbssurface(source: &Rc<session_rust::NurbsSurface>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let mut surface = (**source).clone();

    for (u, v) in surface_keys(&surface, target)? {
        let p = surface
            .get_cv(u, v)
            .ok_or("Missing surface control")?
            .transformed(delta);

        if !surface.set_cv(u, v, &p) {
            return Err("Cannot set surface control".into());
        }
    }

    surface.m_mesh = None; // drop the cached mesh
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sk-transform-original-mesh-and-surface-controls-typed-01.rs"
```

</details>

</details>

<a id="checkpoint-77sl-transform-original-line-polyline-point-and-curve-controls"></a>

### Step 13 of 49 · Transform original line polyline point and curve controls (`77sl-transform-original-line-polyline-point-and-curve-controls`)

- **Goal:** Transform original line polyline point and curve controls.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_deform.rs` (*Transform original line polyline point and curve controls.*)

Use each original control address to move one point in a cloned source.

Reuse the rational curve setter from the preceding lesson. Rebuilding a Line from its two endpoints must restore its original identity and display metadata; changing its shape does not create a new object. Point and polyline edits retain their corresponding source owners.

> **Self-Check Question:** *Which Line metadata must survive an endpoint edit?*  
> **Answer:** Its original GUID, name, width, dash, colour and arrowhead remain unchanged.

- **Hands-On Lab Experiment:** Predict the changed endpoint and unchanged metadata when only Line vertex1 moves. Compare a rational curve edit with a polyline vertex edit.

<details>
<summary>Implementation 77sl-transform-original-line-polyline-point-and-curve-controls: Transform original line polyline point and curve controls</summary>

Transform original line polyline point and curve controls.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_deform.rs`

Transform original line polyline point and curve controls.

<details>
<summary>Locate the existing block</summary>

```rust
fn transform_nurbssurface(source: &Rc<session_rust::NurbsSurface>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let mut surface = (**source).clone();

    for (u, v) in surface_keys(&surface, target)? {
        let p = surface
            .get_cv(u, v)
            .ok_or("Missing surface control")?
            .transformed(delta);

        if !surface.set_cv(u, v, &p) {
            return Err("Cannot set surface control".into());
        }
    }

    surface.m_mesh = None; // drop the cached mesh
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}
```

</details>

After that block, type:

```rust

fn transform_polyline(source: &Rc<session_rust::Polyline>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let Target::Control(ControlId::Vertex(i)) = target else {
        return Err("Select a polyline vertex".into());
    };
    let p = source
        .get_point(i)
        .ok_or("Unknown polyline vertex")?
        .transformed(delta);
    let mut next = (**source).clone();
    next.set_point(i, &p);
    Ok(Geometry::Polyline(Rc::new(next)))
}

fn transform_nurbscurve(source: &Rc<session_rust::NurbsCurve>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let Target::Control(ControlId::Curve { curve: 0, point }) = target else {
        return Err("Select a curve control point".into());
    };
    let p = source
        .get_cv(point)
        .ok_or("Unknown curve control")?
        .transformed(delta);
    let mut next = (**source).clone();

    if !set_curve_control(&mut next, point, &p) {
        return Err("Cannot set curve control".into());
    }

    Ok(Geometry::NurbsCurve(Rc::new(next)))
}

fn transform_line(source: &Rc<session_rust::Line>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let Target::Control(ControlId::Vertex(i @ 0..=1)) = target else {
        return Err("Select a line endpoint".into());
    };
    let mut ends = [source.start(), source.end()];
    ends[i] = ends[i].transformed(delta);
    let mut next = session_rust::Line::from_points(&ends[0], &ends[1]);
    next.set_guid(source.guid().to_string());
    next.name = source.name.clone();
    next.width = source.width;
    next.dash = source.dash.clone();
    next.linecolor = source.linecolor.clone();
    next.arrowhead = source.arrowhead;
    Ok(Geometry::Line(Rc::new(next)))
}

fn transform_point(source: &Rc<session_rust::Point>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let Target::Control(ControlId::Vertex(0)) = target else {
        return Err("Unknown point".into());
    };
    Ok(Geometry::Point(Rc::new(source.transformed(delta))))
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sl-transform-original-line-polyline-point-and-curve-controls-typed-01.rs"
```

</details>

</details>

<a id="checkpoint-77sm-transform-shared-brep-boundary-positions-coherently"></a>

### Step 14 of 49 · Transform shared BRep boundary positions coherently (`77sm-transform-shared-brep-boundary-positions-coherently`)

- **Goal:** Transform shared BRep boundary positions coherently.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_deform.rs` (*Transform shared BRep boundary positions coherently.*)

Collect the selected source positions and clone the BRep.

Move coincident vertex, curve and surface controls within the source tolerance; record touched curves and surfaces, and invalidate changed surface meshes. Validate affected boundaries before returning the new solid. A failed validation returns an error and leaves the original geometry available to the transaction.

> **Self-Check Question:** *Why move coincident BRep vertices curve controls and surface controls together?*  
> **Answer:** They describe connected parts of the same original boundary, so moving only one representation can break consistency.

- **Hands-On Lab Experiment:** Trace a boundary point present in a vertex, curve and surface. Predict which caches and incident-boundary checks must run when it moves.

<details>
<summary>Implementation 77sm-transform-shared-brep-boundary-positions-coherently: Transform shared BRep boundary positions coherently</summary>

Transform shared BRep boundary positions coherently.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_deform.rs`

Transform shared BRep boundary positions coherently.

<details>
<summary>Locate the existing block</summary>

```rust
    next.name = source.name.clone();
    next.width = source.width;
    next.dash = source.dash.clone();
    next.linecolor = source.linecolor.clone();
    next.arrowhead = source.arrowhead;
    Ok(Geometry::Line(Rc::new(next)))
}

fn transform_point(source: &Rc<session_rust::Point>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let Target::Control(ControlId::Vertex(0)) = target else {
        return Err("Unknown point".into());
    };
    Ok(Geometry::Point(Rc::new(source.transformed(delta))))
}
```

</details>

After that block, type:

```rust

fn transform_brep(geometry: &Geometry, source: &Rc<session_rust::BRep>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let selected = points(geometry, target)?;
    let mut next = (**source).clone();
    // every vertex, curve or surface point at a selected position moves
    let matches = |p: &Point| {
        selected
            .iter()
            .any(|s| (0..3).all(|i| (s[i] - p[i]).abs() <= 1e-8))
    };

    for vertex in &mut next.m_vertices {
        if matches(&vertex.point) {
            vertex.point = vertex.point.transformed(delta);
        }
    }

    let mut changed_curves = HashSet::new(); // curves touched
    let mut changed_surfaces = HashSet::new(); // surfaces touched

    for (curve_index, curve) in next.m_curves_3d.iter_mut().enumerate() {
        for i in 0..curve.cv_count() {
            if let Some(p) = curve.get_cv(i)
                && matches(&p)
            {
                if !set_curve_control(curve, i, &p.transformed(delta)) {
                    return Err("Cannot set curve control".into());
                }
                changed_curves.insert(curve_index);
            }
        }
    }

    for (surface_index, surface) in next.m_surfaces.iter_mut().enumerate() {
        for u in 0..surface.m_cv_count[0] {
            for v in 0..surface.m_cv_count[1] {
                if let Some(p) = surface.get_cv(u, v)
                    && matches(&p)
                {
                    surface.set_cv(u, v, &p.transformed(delta));
                    changed_surfaces.insert(surface_index);
                }
            }
        }

        if changed_surfaces.contains(&surface_index) {
            surface.m_mesh = None; // drop the cached mesh
        }
    }

    validate_boundaries(&next, &changed_curves, &changed_surfaces)?;
    Ok(Geometry::BRep(Rc::new(next)))
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sm-transform-shared-brep-boundary-positions-coherently-typed-01.rs"
```

</details>

</details>

<a id="checkpoint-77sn-dispatch-original-part-edits-and-retain-element-ownership"></a>

### Step 15 of 49 · Dispatch original part edits and retain Element ownership (`77sn-dispatch-original-part-edits-and-retain-element-ownership`)

- **Goal:** Dispatch original part edits and retain Element ownership.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_deform.rs` (*Dispatch original part edits and retain Element ownership.*)

Validate the requested transform once, then dispatch to the source helpers written in the preceding lessons.

Element editing recursively reuses Mesh/BRep transformations and retains the Element wrapper and original feature payload. The independent regressions are registered after the original mesh-topology helpers are complete, because their hole/winding checks depend on that module. These helpers connect to the existing preview and history tools afterward.

> **Self-Check Question:** *Does moving an Element control create a separate editable display mesh?*  
> **Answer:** No. Update its original Mesh/BRep payload inside the cloned Element.

- **Hands-On Lab Experiment:** Predict the before/after owner identity when one Element mesh control moves. Trace an invalid transform through the dispatcher without mutating the source.

<details>
<summary>Implementation 77sn-dispatch-original-part-edits-and-retain-element-ownership: Dispatch original part edits and retain Element ownership</summary>

Dispatch original part edits and retain Element ownership.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_deform.rs`

Dispatch original part edits and retain Element ownership.

<details>
<summary>Locate the existing block</summary>

```rust
                if let Some(p) = surface.get_cv(u, v)
                    && matches(&p)
                {
                    surface.set_cv(u, v, &p.transformed(delta));
                    changed_surfaces.insert(surface_index);
                }
            }
        }

        if changed_surfaces.contains(&surface_index) {
            surface.m_mesh = None; // drop the cached mesh
        }
    }

    validate_boundaries(&next, &changed_curves, &changed_surfaces)?;
    Ok(Geometry::BRep(Rc::new(next)))
}
```

</details>

After that block, type:

```rust

fn transform_element(source: &Rc<session_rust::Element>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let mut next = (**source).clone();

    match source.geometry() {
        session_rust::element::ElementGeometry::Mesh(mesh) => {
            let Geometry::Mesh(mesh) =
                transform(&Geometry::Mesh(Rc::new(mesh.clone())), target, delta)?
            else {
                unreachable!()
            };
            next.set_geometry((*mesh).clone());
        }
        session_rust::element::ElementGeometry::BRep(brep) => {
            let Geometry::BRep(brep) =
                transform(&Geometry::BRep(Rc::new(brep.clone())), target, delta)?
            else {
                unreachable!()
            };
            next.set_brep_geometry((*brep).clone());
        }
        _ => return Err("Element has no source geometry".into()),
    }

    Ok(Geometry::Element(Rc::new(next)))
}

/// A copy of the geometry with the target moved by `delta`.
pub fn transform(geometry: &Geometry, target: Target, delta: &Xform) -> Result<Geometry, String> {
    if !delta.m.iter().all(|v| v.is_finite()) {
        return Err("Transform must be finite".into());
    }
    match geometry {
        Geometry::Mesh(source) => transform_mesh(source, target, delta),
        Geometry::NurbsSurface(source) => transform_nurbssurface(source, target, delta),
        Geometry::Polyline(source) => transform_polyline(source, target, delta),
        Geometry::NurbsCurve(source) => transform_nurbscurve(source, target, delta),
        Geometry::Line(source) => transform_line(source, target, delta),
        Geometry::Point(source) => transform_point(source, target, delta),
        Geometry::BRep(source) => transform_brep(geometry, source, target, delta),
        Geometry::Element(source) => transform_element(source, target, delta),
        _ => Err("This source control cannot be transformed".into()),
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sn-dispatch-original-part-edits-and-retain-element-ownership-typed-01.rs"
```

</details>

</details>

<a id="checkpoint-77so-map-sparse-original-vertex-keys-to-display-slots"></a>

### Step 16 of 49 · Map sparse original vertex keys to display slots (`77so-map-sparse-original-vertex-keys-to-display-slots`)

- **Goal:** Map sparse original vertex keys to display slots.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_topology_colour.rs` (*Pack original edge pen colours into the existing RGBA word convention.*) · `src/source_mesh_topology.rs` (*Map sparse original vertex keys to display slots.*) · `src/lib.rs` (*Register the shared source colour helper and original edge topology owner.*)

Choose a dense slot lookup only when sorted original keys are close enough together; otherwise use a sparse map.

The lookup maps original identity to contiguous display storage without changing the source keys. Pack source pen colours once with the small shared RGBA helper; edge topology will reuse that packed format.

> **Self-Check Question:** *Why not allocate a GPU slot array up to the largest original vertex key?*  
> **Answer:** Sparse original keys can be enormous even when the mesh has few vertices.

- **Hands-On Lab Experiment:** Compare original keys[0,1,2] with[2,1000000]. Predict which lookup stores each mesh and which GPU slot belongs to its last original vertex.

<details>
<summary>Implementation 77so-map-sparse-original-vertex-keys-to-display-slots: Map sparse original vertex keys to display slots</summary>

Map sparse original vertex keys to display slots.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_topology_colour.rs`

Pack original edge pen colours into the existing RGBA word convention.

Create the file and type:

```rust
--8<-- "journey/code/77so-map-sparse-original-vertex-keys-to-display-slots-typed-01.rs"
```

### 2. `src/source_mesh_topology.rs`

Map sparse original vertex keys to display slots.

Create the file and type:

```rust
--8<-- "journey/code/77so-map-sparse-original-vertex-keys-to-display-slots-typed-02.rs"
```

### 3. `src/lib.rs`

Register the shared source colour helper and original edge topology owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod source_deform;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77so-map-sparse-original-vertex-keys-to-display-slots-typed-03.rs" skip=24
```

</details>

<a id="checkpoint-77sp-compute-polygon-normals-from-original-boundaries"></a>

### Step 17 of 49 · Compute polygon normals from original boundaries (`77sp-compute-polygon-normals-from-original-boundaries`)

- **Goal:** Compute polygon normals from original boundaries.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_mesh_topology.rs` (*Compute polygon normals from original boundaries.*)

MeshTopo separates original edges, their incident face slots, winding agreement, face normals and closure.

Newell normals use all polygon boundary positions through the original-key slot lookup. Reject a degenerate normal instead of dividing by zero. Sorting face keys gives stable face slots for edge incidence and later GPU ownership.

> **Self-Check Question:** *Why sum every polygon edge instead of using its first triangle?*  
> **Answer:** The polygon may have collinear initial vertices; the complete Newell sum uses the original boundary.

- **Hands-On Lab Experiment:** Draw a planar polygon whose first three vertices are collinear. Predict why the full boundary still gives a usable normal.

<details>
<summary>Implementation 77sp-compute-polygon-normals-from-original-boundaries: Compute polygon normals from original boundaries</summary>

Compute polygon normals from original boundaries.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_mesh_topology.rs`

Compute polygon normals from original boundaries.

<details>
<summary>Locate the existing block</summary>

```rust
            };
        }

        let mut sparse = std::collections::HashMap::with_capacity(keys.len());

        for (s, &k) in keys.iter().enumerate() {
            sparse.insert(k, s as u32);
        }

        Self {
            dense: Vec::new(),
            sparse,
        }
    }

    /// The slot of key `k`.
    pub fn slot(&self, k: usize) -> usize {
        if self.dense.is_empty() {
            self.sparse[&k] as usize
        } else {
            self.dense[k] as usize
        }
    }
}
```

</details>

After that block, type:

```rust

/// Edges, faces and normals of one mesh.
pub struct MeshTopo {
    pub edges: Vec<(usize, usize, u32)>, // (low key, high key, pen colour)
    pub edge_faces: Vec<[u32; 2]>,       // two faces per edge, u32::MAX = none
    pub opposed: Vec<bool>,              // per edge: faces wind opposite ways
    pub normals: Vec<Option<[f64; 3]>>,  // per face, None when degenerate
    pub closed: bool,                    // every edge has two faces
}

/// Unit normal of one face, None when degenerate.
fn face_normal(vs: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> Option<[f64; 3]> {
    if vs.len() < 3 {
        return None;
    }

    // Newell normal: sums over every edge
    let mut n = [0.0f64; 3];

    for i in 0..vs.len() {
        let a = vpos[slots.slot(vs[i])];
        let b = vpos[slots.slot(vs[(i + 1) % vs.len()])];
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }

    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();

    if len > Tolerance::ZERO_TOLERANCE {
        Some([n[0] / len, n[1] / len, n[2] / len])
    } else {
        None
    }
}

/// Sort key of a face.
fn face_key(face: &(usize, &Vec<usize>)) -> usize {
    face.0
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sp-compute-polygon-normals-from-original-boundaries-typed-01.rs"
```

</details>

</details>

<a id="checkpoint-77sq-build-one-original-edge-table-and-record-incident-faces"></a>

### Step 18 of 49 · Build one original edge table and record incident faces (`77sq-build-one-original-edge-table-and-record-incident-faces`)

- **Goal:** Build one original edge table and record incident faces.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_mesh_topology.rs` (*Build one original edge table and record incident faces.*)

Each low-vertex slot owns a linked list of edge indices.

Insert a new unordered edge once, using its first-appearance colour index. Record at most two distinct incident face slots and whether their traversal directions oppose. The sentinel u32::MAX marks an absent list link or incident face. This preserves original edge ordering for display and picking.

> **Self-Check Question:** *How do adjacent faces reuse the same original edge?*  
> **Answer:** Search the linked edge list at the lower vertex slot for the same upper key.

- **Hands-On Lab Experiment:** Walk two adjacent faces across the same edge in opposite directions. Predict one stored edge, two incident faces and its opposed flag.

<details>
<summary>Implementation 77sq-build-one-original-edge-table-and-record-incident-faces: Build one original edge table and record incident faces</summary>

Build one original edge table and record incident faces.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_mesh_topology.rs`

Build one original edge table and record incident faces.

<details>
<summary>Locate the existing block</summary>

```rust
        let a = vpos[slots.slot(vs[i])];
        let b = vpos[slots.slot(vs[(i + 1) % vs.len()])];
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }

    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();

    if len > Tolerance::ZERO_TOLERANCE {
        Some([n[0] / len, n[1] / len, n[2] / len])
    } else {
        None
    }
}

/// Sort key of a face.
fn face_key(face: &(usize, &Vec<usize>)) -> usize {
    face.0
}
```

</details>

After that block, type:

```rust

struct EdgeTable {
    edges: Vec<(usize, usize, u32)>,
    edge_faces: Vec<[u32; 2]>,
    dir0: Vec<u8>,
    opposed: Vec<bool>,
    head: Vec<u32>,
    next: Vec<u32>,
}

impl EdgeTable {
    fn new(slots: usize) -> Self {
        Self { edges: Vec::new(), edge_faces: Vec::new(), dir0: Vec::new(),
            opposed: Vec::new(), head: vec![u32::MAX; slots], next: Vec::new() }
    }

    fn add(&mut self, low_slot: usize, lo: usize, hi: usize, dir: u8, face: usize, cols: &[session_rust::Color]) {
        let ls = low_slot;
        let mut ei = self.head[ls];

        // look for the edge among those at `lo`
        while ei != u32::MAX && self.edges[ei as usize].1 != hi {
            ei = self.next[ei as usize];
        }

        // new edge
        if ei == u32::MAX {
            ei = self.edges.len() as u32;
            let pen = match cols.get(self.edges.len()) {
                Some(color) => pack_rgba(color.to_f32()),
                None => BLACK,
            };
            self.edges.push((lo, hi, pen));
            self.edge_faces.push([u32::MAX; 2]);
            self.dir0.push(dir);
            self.opposed.push(true);
            self.next.push(self.head[ls]);
            self.head[ls] = ei;
        }

        // record this face on the edge, first free slot
        let ef = &mut self.edge_faces[ei as usize];

        if ef[0] == u32::MAX {
            ef[0] = face as u32;
            self.dir0[ei as usize] = dir;
        } else if ef[1] == u32::MAX && ef[0] != face as u32 {
            ef[1] = face as u32;
            self.opposed[ei as usize] = dir != self.dir0[ei as usize];
        }
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sq-build-one-original-edge-table-and-record-incident-faces-typed-01.rs"
```

</details>

</details>

<a id="checkpoint-77sr-preserve-hole-winding-and-complete-original-mesh-topology"></a>

### Step 19 of 49 · Preserve hole winding and complete original mesh topology (`77sr-preserve-hole-winding-and-complete-original-mesh-topology`)

- **Goal:** Preserve hole winding and complete original mesh topology.
- **Data Flow Trace:** `original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display`
- **Files You Touch:** `src/source_mesh_topology.rs` (*Preserve hole winding and complete original mesh topology.*) · `src/source_deform.rs` (*Supply independent original-source and fixed-origin regressions.*)

Collect sorted original faces and their normals.

Visit outer rings first, then hole rings; normalize hole traversal against the face normal before recording edge direction. Reuse EdgeTable::add for both passes. Closure requires a nonempty source mesh and a second incident face on every edge. The supplied regressions cover holes, winding, sparse keys and source pen ordering before part picking is connected.

> **Self-Check Question:** *Why visit outer borders before hole rings?*  
> **Answer:** It keeps the kernel’s existing outer-edge order for source colours widths and picking IDs.

- **Hands-On Lab Experiment:** Compare a closed box with a face removed. Then add a hole rim and predict its direction relative to the outer border.

<details>
<summary>Implementation 77sr-preserve-hole-winding-and-complete-original-mesh-topology: Preserve hole winding and complete original mesh topology</summary>

Preserve hole winding and complete original mesh topology.

Data flow: original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.

### 1. `src/source_mesh_topology.rs`

Preserve hole winding and complete original mesh topology.

<details>
<summary>Locate the existing block</summary>

```rust
            self.opposed.push(true);
            self.next.push(self.head[ls]);
            self.head[ls] = ei;
        }

        // record this face on the edge, first free slot
        let ef = &mut self.edge_faces[ei as usize];

        if ef[0] == u32::MAX {
            ef[0] = face as u32;
            self.dir0[ei as usize] = dir;
        } else if ef[1] == u32::MAX && ef[0] != face as u32 {
            ef[1] = face as u32;
            self.opposed[ei as usize] = dir != self.dir0[ei as usize];
        }
    }
}
```

</details>

After that block, type:

```rust

/// Collect unique edges, their faces and the face normals.
pub fn mesh_topology(m: &Mesh, keys: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> MeshTopo {
    let mut faces: Vec<(usize, &Vec<usize>)> = Vec::with_capacity(m.face.len());

    for (k, v) in m.face.iter() {
        faces.push((*k, v));
    }

    faces.sort_unstable_by_key(face_key);
    let cols = m.get_linecolors();

    let normals: Vec<_> = faces
        .iter()
        .map(|(_, vs)| face_normal(vs, vpos, slots))
        .collect();
    let mut table = EdgeTable::new(keys.len());

    // Keep the kernel's outer-edge ordering for colors, widths and picking ids.
    for holes in [false, true] {
        for (fs, (key, vs)) in faces.iter().enumerate() {
            let rings: &[Vec<usize>] = if holes {
                m.face_holes.get(key).map_or(&[], Vec::as_slice)
            } else {
                std::slice::from_ref(*vs)
            };
            for ring in rings {
                // Hole rings may be stored in either direction; a face walks them opposite its border.
                let reverse = holes
                    && normals[fs]
                        .zip(face_normal(ring, vpos, slots))
                        .is_some_and(|(a, b)| {
                            a.iter().zip(b).map(|(x, y)| x * y).sum::<f64>() > 0.0
                        });
                let n = ring.len();

                for i in 0..n {
                    let (u, v) = (ring[i], ring[(i + 1) % n]);
                    let (lo, hi, dir) = if u < v { (u, v, 0) } else { (v, u, 1) }; // edge key
                    let dir = dir ^ u8::from(reverse);
                    table.add(slots.slot(lo), lo, hi, dir, fs, cols);
                }
            }
        }
    }

    // closed when no edge is missing its second face
    let mut closed = !m.vertex.is_empty();

    for f in table.edge_faces.iter() {
        if f[1] == u32::MAX {
            closed = false;
        }
    }

    MeshTopo {
        edges: table.edges,
        edge_faces: table.edge_faces,
        opposed: table.opposed,
        normals,
        closed,
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sr-preserve-hole-winding-and-complete-original-mesh-topology-typed-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/source_mesh_topology.rs`

Supply independent original-source and fixed-origin regressions.

<details>
<summary>Locate the existing block</summary>

```text
rom(reverse);
                    table.add(slots.slot(lo), lo, hi, dir, fs, cols);
                }
            }
        }
    }

    // closed when no edge is missing its second face
    let mut closed = !m.vertex.is_empty();

    for f in table.edge_faces.iter() {
        if f[1] == u32::MAX {
            closed = false;
        }
    }

    MeshTopo {
        edges: table.edges,
        edge_faces: table.edge_faces,
        opposed: table.opposed,
        normals,
        closed,
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77sr-preserve-hole-winding-and-complete-original-mesh-topology-typed-02.rs" skip=500
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/source_deform.rs`

Supply independent original-source and fixed-origin regressions.

<details>
<summary>Locate the existing block</summary>

```text
ne(source, target, delta),
        Geometry::NurbsCurve(source) => transform_nurbscurve(source, target, delta),
        Geometry::Line(source) => transform_line(source, target, delta),
        Geometry::Point(source) => transform_point(source, target, delta),
        Geometry::BRep(source) => transform_brep(geometry, source, target, delta),
        Geometry::Element(source) => transform_element(source, target, delta),
        _ => Err("This source control cannot be transformed".into()),
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77sn-dispatch-original-part-edits-and-retain-element-ownership-typed-02.rs" skip=500
```

</details>

</details>

<a id="checkpoint-77a-own-transformation-questions-and-the-selected-original-targets"></a>

### Step 20 of 49 · Own transformation questions and the selected original targets (`77a-own-transformation-questions-and-the-selected-original-targets`)

- **Goal:** Own transformation questions and the selected original targets.
- **Data Flow Trace:** `current selection and plane → frozen targets → pending transform questions`
- **Files You Touch:** `src/transform_draft.rs` (*Own current transformation questions, modes, accepted points and original targets.*) · `src/lib.rs` (*Register the source transformation component.*)

Keep the tool kind, scale mode, accepted points, current plane and frozen original targets in one owner.

> **Self-Check Question:** *Can hover become saved placement?*  
> **Answer:** No. The question owner and temporary preview remain separate from scene placement and its history.

- **What You Should See:** The five original curve families remain visible while source transformations connect. The final exercise copies and independently scales and rotates original boxes.
- **Hands-On Lab Experiment:** Compare Copy target prompts, Scale1D reference prompts and Rotate center/angle prompts.

<details>
<summary>Implementation 77a-own-transformation-questions-and-the-selected-original-targets: Own transformation questions and the selected original targets</summary>

Own transformation questions and the selected original targets.

Data flow: current selection and plane → frozen targets → pending transform questions.

### 1. `src/transform_draft.rs`

Own current transformation questions, modes, accepted points and original targets.

Create the file and type:

```rust
--8<-- "journey/code/77a-own-transformation-questions-and-the-selected-original-targets-01.rs"
```

### 2. `src/lib.rs`

Register the source transformation component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod source_topology_colour;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77a-own-transformation-questions-and-the-selected-original-targets-02.rs" skip=33
```

</details>

<a id="checkpoint-77b-validate-point-and-number-transformation-answers"></a>

### Step 21 of 49 · Validate point and number transformation answers (`77b-validate-point-and-number-transformation-answers`)

- **Goal:** Validate point and number transformation answers.
- **Data Flow Trace:** `point or number answer → checked references → current original transform`
- **Files You Touch:** `src/transform_draft.rs` (*Validate current transform points and numbers without adopting an invalid reference.*)

Resolve a valid factor, reference reach or signed plane angle before adopting the candidate points.

> **Self-Check Question:** *Can Scale1D take a factor before its direction?*  
> **Answer:** No. Its first reference point determines the direction. Scale 2D and3D can take a factor after the origin.

- **Hands-On Lab Experiment:** Try a repeated reference point, a nonpositive factor and valid1D reference/new-length input.

<details>
<summary>Implementation 77b-validate-point-and-number-transformation-answers: Validate point and number transformation answers</summary>

Validate point and number transformation answers.

Data flow: point or number answer → checked references → current original transform.

### 1. `src/transform_draft.rs`

Validate current transform points and numbers without adopting an invalid reference.

<details>
<summary>Locate the existing block</summary>

```rust
        };format!("{} · {question} · Esc cancels{}",self.kind.name(),if self.kind==Kind::Scale{format!(" · {}D",self.mode)}else{String::new()})
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77b-validate-point-and-number-transformation-answers-01.rs"
```

</details>

<a id="checkpoint-77c-resolve-typed-batches-and-temporary-transformation-previews"></a>

### Step 22 of 49 · Resolve typed batches and temporary transformation previews (`77c-resolve-typed-batches-and-temporary-transformation-previews`)

- **Goal:** Resolve typed batches and temporary transformation previews.
- **Data Flow Trace:** `typed batch → candidate owner → checked transforms or temporary preview`
- **Files You Touch:** `src/transform_draft.rs` (*Resolve current options and atomic batches, temporary preview matrices and tool buttons.*)

Validate all typed words before replacing the owner. Copy repeats after a target; other transforms finish after their result.

> **Self-Check Question:** *Does extra input after Scale completion create a partial edit?*  
> **Answer:** No. Reject the entire typed batch before adopting any transform or answer.

- **Hands-On Lab Experiment:** Try Scale with origin, factor and an extra word. Compare Copy with two targets and Enter with the same tool cancelled by Escape.

<details>
<summary>Implementation 77c-resolve-typed-batches-and-temporary-transformation-previews: Resolve typed batches and temporary transformation previews</summary>

Resolve typed batches and temporary transformation previews.

Data flow: typed batch → candidate owner → checked transforms or temporary preview.

### 1. `src/transform_draft.rs`

Resolve current options and atomic batches, temporary preview matrices and tool buttons.

<details>
<summary>Locate the existing block</summary>

```rust
        }
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77c-resolve-typed-batches-and-temporary-transformation-previews-01.rs"
```

</details>

<a id="checkpoint-77d-resolve-original-cad-parts-and-complete-selected-bounds"></a>

### Step 23 of 49 · Resolve original CAD parts and complete selected bounds (`77d-resolve-original-cad-parts-and-complete-selected-bounds`)

- **Goal:** Resolve original CAD parts and complete selected bounds.
- **Data Flow Trace:** `selected original part or owner → source geometry and world bounds → checked target`
- **Files You Touch:** `src/source_part.rs` (*Resolve existing CAD face and boundary targets against original kernel geometry.*) · `src/lib.rs` (*Register the source transformation component.*) · `src/scene.rs` (*Connect current transformation behavior to the existing input and history owner.*)

Map displayed CAD face/boundary identities back to original geometry. Include Line, Point and Curve in precise selected bounds.

> **Self-Check Question:** *Why does direct rotation need selected line bounds?*  
> **Answer:** Its gizmo center comes from the selected bounds. Without them a line incorrectly rotates about the world origin.

- **Hands-On Lab Experiment:** Select a line from 1,0,0 to2,0,0; its center is1.5,0,0. Compare that with a selected surface boundary’s original points.

<details>
<summary>Implementation 77d-resolve-original-cad-parts-and-complete-selected-bounds: Resolve original CAD parts and complete selected bounds</summary>

Resolve original CAD parts and complete selected bounds.

Data flow: selected original part or owner → source geometry and world bounds → checked target.

### 1. `src/source_part.rs`

Resolve existing CAD face and boundary targets against original kernel geometry.

Create the file and type:

```rust
--8<-- "journey/code/77d-resolve-original-cad-parts-and-complete-selected-bounds-01.rs"
```

### 2. `src/lib.rs`

Register the source transformation component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod transform_draft;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77d-resolve-original-cad-parts-and-complete-selected-bounds-02.rs" skip=26
```

### 3. `src/scene.rs`

Connect current transformation behavior to the existing input and history owner.

<details>
<summary>Locate the existing block</summary>

```rust
        if let Some(row)=self.texts.iter().find(|row|row.id==id) {return row.bounds();}
        if let Some(row)=self.sheets.iter().find(|row|row.id==id) {return row.bounds();}
        let object = self.objects.iter().find(|object| object.id == id)?;
        let mut vertices = object.mesh.vertices().iter();
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77d-resolve-original-cad-parts-and-complete-selected-bounds-03.rs"
```

</details>

<a id="checkpoint-77e-rebuild-an-edited-original-cad-source-before-adoption"></a>

### Step 24 of 49 · Rebuild an edited original CAD source before adoption (`77e-rebuild-an-edited-original-cad-source-before-adoption`)

- **Goal:** Rebuild an edited original CAD source before adoption.
- **Data Flow Trace:** `world edit → local original deformation → validated source → derived display`
- **Files You Touch:** `src/scene.rs` (*Connect current transformation behavior to the existing input and history owner.*) · `src/part_preview.rs` (*Connect current transformation behavior to the existing input and history owner.*)

Apply the world delta in the original local frame, validate adjacent trims, and prepare the edited source before changing the row.

> **Self-Check Question:** *What happens if an edit breaks incident trims?*  
> **Answer:** Refuse the result and keep the original source, row placement and history unchanged.

- **Hands-On Lab Experiment:** Move a box face, Undo, then inspect original source identity and face boundaries after Save/Open.

<details>
<summary>Implementation 77e-rebuild-an-edited-original-cad-source-before-adoption: Rebuild an edited original CAD source before adoption</summary>

Rebuild an edited original CAD source before adoption.

Data flow: world edit → local original deformation → validated source → derived display.

### 1. `src/scene.rs`

Connect current transformation behavior to the existing input and history owner.

<details>
<summary>Locate the existing block</summary>

```rust
        let model=self.placement(target.owner).ok_or("Part owner is missing")?;
        let local=&(&model.inverse().ok_or("Part placement cannot be inverted")?*delta)*&model;
        if let Some(row)=self.objects.iter_mut().find(|r|r.id==target.owner) {
            if row.surface.is_some()||row.cad.is_some() {return Err("CAD surface part editing connects after source parameter mapping");}
            let mut source=(**row.geometry().ok_or("Reload editable sources before editing a part")?).clone();
            for key in crate::part_preview::keys(&source,target.part)? {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77e-rebuild-an-edited-original-cad-source-before-adoption-01.rs"
```

### 2. `src/part_preview.rs`

Connect current transformation behavior to the existing input and history owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust

fn same_source(a:&Scene,b:&Scene,id:ObjectId) -> bool {
    if let Some(row)=a.objects().iter().find(|r|r.id==id) {
        return b.objects().iter().find(|r|r.id==id).and_then(|r|r.geometry()).zip(row.geometry()).is_some_and(|(a,b)|Rc::ptr_eq(a,b));
    }
    a.paths().iter().find(|r|r.id==id).zip(b.paths().iter().find(|r|r.id==id)).is_some_and(|(a,b)|a.prepared.source.owner()==b.prepared.source.owner())
}
```

</details>

Replace it with:

```rust

fn same_source(a:&Scene,b:&Scene,id:ObjectId) -> bool {
    if let Some(row)=a.objects().iter().find(|r|r.id==id) {
        return b.objects().iter().find(|r|r.id==id).is_some_and(|other|match(&row.cad,&other.cad,&row.surface,&other.surface){(Some(a),Some(b),_,_)=>Rc::ptr_eq(&a.source,&b.source),(_,_,Some(a),Some(b))=>Rc::ptr_eq(&a.source,&b.source),(None,None,None,None)=>other.geometry().zip(row.geometry()).is_some_and(|(a,b)|Rc::ptr_eq(a,b)),_=>false});
    }
    a.paths().iter().find(|r|r.id==id).zip(b.paths().iter().find(|r|r.id==id)).is_some_and(|(a,b)|a.prepared.source.owner()==b.prepared.source.owner())
}
```

<details>
<summary>Locate this block</summary>

```rust
    if matches!(target.part,Part::CloudPoint(_)) {return Err("Cloud point editing connects with source transforms");}
    let model=scene.placement(target.owner).ok_or("Part owner is missing")?;
    let points=if let Some(row)=scene.objects().iter().find(|r|r.id==target.owner) {
        if row.surface.is_some()||row.cad.is_some() {return Err("CAD surface part editing connects after source parameter mapping");}
        let mesh=row.geometry().ok_or("Reload editable sources before editing a part")?;
        keys(mesh,target.part)?.into_iter().map(|key|mesh.vertex_point(key).ok_or("Missing source vertex")).collect::<Result<Vec<_>,_>>()?
    } else {
        let row=scene.paths().iter().find(|r|r.id==target.owner).ok_or("Select a mesh face, edge or curve control")?;
        let (crate::chain::Source::Curve(curve,_),Part::Control(index))=(&row.prepared.source,target.part) else {return Err("Select a curve control point");};
```

</details>

Replace it with:

```rust
    if matches!(target.part,Part::CloudPoint(_)) {return Err("Cloud point editing connects with source transforms");}
    let model=scene.placement(target.owner).ok_or("Part owner is missing")?;
    let points=if let Some(row)=scene.objects().iter().find(|r|r.id==target.owner) {
        if row.surface.is_some()||row.cad.is_some() {crate::source_part::points(scene,target).map_err(|_|"Cannot resolve original CAD part")?}else{
        let mesh=row.geometry().ok_or("Reload editable sources before editing a part")?;
        keys(mesh,target.part)?.into_iter().map(|key|mesh.vertex_point(key).ok_or("Missing source vertex")).collect::<Result<Vec<_>,_>>()?}
    } else {
        let row=scene.paths().iter().find(|r|r.id==target.owner).ok_or("Select a mesh face, edge or curve control")?;
        let (crate::chain::Source::Curve(curve,_),Part::Control(index))=(&row.prepared.source,target.part) else {return Err("Select a curve control point");};
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{scene::{Scene,ObjectId},pick_sources::{Part,Target}};
use session_rust::{Point,Xform};
use std::rc::Rc;

pub fn points(scene:&Scene,target:Target) -> Result<Vec<Point>, &'static str> {
    if scene.instances.contains_key(&target.owner) {return Err("Detach the shared definition before editing its source geometry");}
    if !target.valid(scene) {return Err("Selected part is no longer editable");}
    if matches!(target.part,Part::SheetEntity(_)) {return Err("Sheet entities are display references; edit the original drawing");}
    if matches!(target.part,Part::CloudPoint(_)) {return Err("Cloud point editing connects with source transforms");}
    let model=scene.placement(target.owner).ok_or("Part owner is missing")?;
    let points=if let Some(row)=scene.objects().iter().find(|r|r.id==target.owner) {
        if row.surface.is_some()||row.cad.is_some() {crate::source_part::points(scene,target).map_err(|_|"Cannot resolve original CAD part")?}else{
        let mesh=row.geometry().ok_or("Reload editable sources before editing a part")?;
        keys(mesh,target.part)?.into_iter().map(|key|mesh.vertex_point(key).ok_or("Missing source vertex")).collect::<Result<Vec<_>,_>>()?}
    } else {
        let row=scene.paths().iter().find(|r|r.id==target.owner).ok_or("Select a mesh face, edge or curve control")?;
        let (crate::chain::Source::Curve(curve,_),Part::Control(index))=(&row.prepared.source,target.part) else {return Err("Select a curve control point");};
        vec![curve.get_cv(index).ok_or("Missing curve control")?]
    };
    Ok(points.into_iter().map(|p|model.transform_point(&p)).collect())
}

pub fn origin(scene:&Scene,target:Target) -> Option<[f64;3]> {
    if let Part::CloudPoint(id)=target.part {let p=crate::cloud_pick::position(scene,target.owner,id)?;return Some([p[0],p[1],p[2]]);}
    let points=points(scene,target).ok()?;
    if points.is_empty() {return None;}
    Some(std::array::from_fn(|i|points.iter().map(|p|p[i]/points.len() as f64).sum()))
}

pub fn keys(mesh:&session_rust::Mesh,part:Part) -> Result<Vec<usize>, &'static str> {
    let mut keys=match part {
        Part::Face(key)=> {let mut keys=mesh.face.get(&key).ok_or("Unknown mesh face")?.clone();
            if let Some(holes)=mesh.face_holes.get(&key) {keys.extend(holes.iter().flatten());} keys},
        Part::MeshEdge(a,b) if mesh.edges().contains(&(a,b))=>vec![a,b],
        _=>return Err("Select a mesh face or edge"),
    };
    keys.sort_unstable();keys.dedup();
    if keys.iter().any(|key|mesh.vertex_point(*key).is_none()) {return Err("Missing source vertex");}
    Ok(keys)
}

// Part previews deform a cloned original source; the saved source and all other owners remain shared until one commit.
pub struct Preview {base:Scene,pub scene:Scene,target:Target,transform:Xform}

impl Preview {
    pub fn new(scene:&Scene,target:Target) -> Result<Self,&'static str> {
        points(scene,target)?;
        scene.placement(target.owner).and_then(|m|m.inverse()).ok_or("Part placement cannot be inverted")?;
        Ok(Self {base:scene.clone(),scene:scene.clone(),target,transform:Xform::identity()})
    }
    pub fn scenes(&self)->[&Scene;2] {[&self.base,&self.scene]}
    pub fn matrix(&self)->&[f64;16] {&self.transform.m}
    pub fn set(&mut self,transform:Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&transform.m) {return Err("Invalid part transform");}
        let mut scene=self.base.clone();
        if transform.m!=Xform::identity().m {scene.deform_part(self.target,&transform)?;}
        self.scene=scene;self.transform=transform;Ok(())
    }
    pub fn commit(&self,scene:&mut Scene,history:&mut crate::history::History) -> Result<bool,&'static str> {
        if !self.target.valid(scene) || scene.placement(self.target.owner).map(|m|m.m)!=self.base.placement(self.target.owner).map(|m|m.m)
            || !same_source(scene,&self.base,self.target.owner) {return Err("Part source changed; start editing again");}
        if self.transform.m==Xform::identity().m {return Ok(false);}
        history.try_edit(scene,|scene|scene.deform_part(self.target,&self.transform))?;Ok(true)
    }
}

fn same_source(a:&Scene,b:&Scene,id:ObjectId) -> bool {
    if let Some(row)=a.objects().iter().find(|r|r.id==id) {
        return b.objects().iter().find(|r|r.id==id).is_some_and(|other|match(&row.cad,&other.cad,&row.surface,&other.surface){(Some(a),Some(b),_,_)=>Rc::ptr_eq(&a.source,&b.source),(_,_,Some(a),Some(b))=>Rc::ptr_eq(&a.source,&b.source),(None,None,None,None)=>other.geometry().zip(row.geometry()).is_some_and(|(a,b)|Rc::ptr_eq(a,b)),_=>false});
    }
    a.paths().iter().find(|r|r.id==id).zip(b.paths().iter().find(|r|r.id==id)).is_some_and(|(a,b)|a.prepared.source.owner()==b.prepared.source.owner())
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77e-rebuild-an-edited-original-cad-source-before-adoption-02.rs"
```

```text
--8<-- "journey/code/77e-rebuild-an-edited-original-cad-source-before-adoption-03.rs"
```

</details>

</details>

<a id="checkpoint-77f-copy-original-rows-beside-their-existing-parents"></a>

### Step 25 of 49 · Copy original rows beside their existing parents (`77f-copy-original-rows-beside-their-existing-parents`)

- **Goal:** Copy original rows beside their existing parents.
- **Data Flow Trace:** `selected original rows → checked copy plan → new identities and world placement`
- **Files You Touch:** `src/layer_edit.rs` (*Copy checked original rows under their existing parent with current attributes, new identities and placed source sharing.*)

Stage checked copies with new row and tree identities, retain names/colours and copy attached attributes under the same parent.

> **Self-Check Question:** *Does copying a selected parent also copy every ordinary child?*  
> **Answer:** Only the explicit selected objects are copied; attached attributes follow their owner. Ordinary child objects keep their existing original parent.

- **Hands-On Lab Experiment:** Compare Copy under duplicate named groups, copied row GUIDs, retained source allocation and inverse-derived saved local placement.

<details>
<summary>Implementation 77f-copy-original-rows-beside-their-existing-parents: Copy original rows beside their existing parents</summary>

Copy original rows beside their existing parents.

Data flow: selected original rows → checked copy plan → new identities and world placement.

### 1. `src/layer_edit.rs`

Copy checked original rows under their existing parent with current attributes, new identities and placed source sharing.

<details>
<summary>Locate the existing block</summary>

```rust
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77f-copy-original-rows-beside-their-existing-parents-01.rs" skip=8
```

</details>

<a id="checkpoint-77g-carry-transformation-state-through-the-existing-drawing-owner"></a>

### Step 26 of 49 · Carry transformation state through the existing drawing owner (`77g-carry-transformation-state-through-the-existing-drawing-owner`)

- **Goal:** Carry transformation state through the existing drawing owner.
- **Data Flow Trace:** `current Draft → frozen transform owner → temporary scene and resource accounting`
- **Files You Touch:** `src/tool.rs` (*Carry current transform question owners and buttons through the existing Draft.*) · `src/editor.rs` (*Own temporary original-part and whole-placement transformation previews.*)

Keep transformation questions beside the current Draft and include their temporary scene owners in display and memory reporting.

> **Self-Check Question:** *Can a temporary transformed scene outlive cancellation?*  
> **Answer:** Cancellation drops its owner. The saved scene and history still retain only their original placements and sources.

- **Hands-On Lab Experiment:** Compare display_scene with scene during a rotation preview, and inspect presentation_scenes before and after Escape.

<details>
<summary>Implementation 77g-carry-transformation-state-through-the-existing-drawing-owner: Carry transformation state through the existing drawing owner</summary>

Carry transformation state through the existing drawing owner.

Data flow: current Draft → frozen transform owner → temporary scene and resource accounting.

### 1. `src/tool.rs`

Carry current transform question owners and buttons through the existing Draft.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
pub const MAX_POINTS: usize = 4096;

// A draft owns unfinished points and a cursor, so preview motion cannot become saved geometry or an Undo transaction.
pub struct Draft { pub tool: Box<dyn Tool>, pub points: Vec<[f64; 3]>, pub cursor: Option<[f64; 3]>, pub snapped: Option<crate::snap::Kind>, pub frame: crate::cplane::CPlane, pub plane_points: Option<Vec<[f64;3]>>, pub primitive:Option<crate::primitive_draft::Draft> }

impl Draft {
    pub fn new(shape: Shape) -> Self { Self { tool: Box::new(shape), points: Vec::new(), cursor: None, snapped: None, frame: crate::cplane::CPlane::Xy, plane_points: None, primitive:if let Shape::Primitive(shape)=shape {Some(crate::primitive_draft::Draft::new(shape,crate::cplane::CPlane::Xy,0).expect("Registered primitive option"))}else{None} } }
    pub fn prompt(&self) -> String {
        if self.plane_points.is_none() {if let Some(primitive)=&self.primitive {return primitive.prompt();}}
        let mut prompt=if let Some(points)=&self.plane_points {format!("CPlane 3 Point · point {} of 3 · Esc cancels",points.len()+1)}
            else if self.tool.count()==0 {format!("{} point {} · Enter finishes · Esc cancels",self.tool.name(),self.points.len()+1)}
```

</details>

Replace it with:

```rust
pub const MAX_POINTS: usize = 4096;

// A draft owns unfinished points and a cursor, so preview motion cannot become saved geometry or an Undo transaction.
pub struct Draft { pub tool: Box<dyn Tool>, pub points: Vec<[f64; 3]>, pub cursor: Option<[f64; 3]>, pub snapped: Option<crate::snap::Kind>, pub frame: crate::cplane::CPlane, pub plane_points: Option<Vec<[f64;3]>>, pub primitive:Option<crate::primitive_draft::Draft>, pub transform:Option<crate::transform_draft::Draft> }

impl Draft {
    pub fn new(shape: Shape) -> Self { Self { tool: Box::new(shape), points: Vec::new(), cursor: None, snapped: None, frame: crate::cplane::CPlane::Xy, plane_points: None, primitive:if let Shape::Primitive(shape)=shape {Some(crate::primitive_draft::Draft::new(shape,crate::cplane::CPlane::Xy,0).expect("Registered primitive option"))}else{None},transform:if let Shape::Transform(kind)=shape {Some(crate::transform_draft::Draft::new(kind,crate::cplane::CPlane::Xy))}else{None} } }
    pub fn prompt(&self) -> String {
        if self.plane_points.is_none() {if let Some(transform)=&self.transform {return transform.prompt();}}
        if self.plane_points.is_none() {if let Some(primitive)=&self.primitive {return primitive.prompt();}}
        let mut prompt=if let Some(points)=&self.plane_points {format!("CPlane 3 Point · point {} of 3 · Esc cancels",points.len()+1)}
            else if self.tool.count()==0 {format!("{} point {} · Enter finishes · Esc cancels",self.tool.name(),self.points.len()+1)}
```

<details>
<summary>Locate this block</summary>

```rust
            Self::Polygon {sides} if points.len()==2=>crate::planar::points("polygon",plane,*sides,points).unwrap_or_default(),_=>points.to_vec()}
    }
    fn buttons(&self)->Vec<(String,String)> {
        if let Self::Primitive(shape)=self {return shape.options.iter().map(|(label,line)|((*label).into(),(*line).into())).collect();}
        let mut buttons=vec![("Cancel".into(),"Escape".into())];
        if matches!(self,Self::Polyline|Self::Rectangle|Self::Polygon {..}) {
```

</details>

Replace it with:

```rust
            Self::Polygon {sides} if points.len()==2=>crate::planar::points("polygon",plane,*sides,points).unwrap_or_default(),_=>points.to_vec()}
    }
    fn buttons(&self)->Vec<(String,String)> {
        if let Self::Transform(kind)=self {return crate::transform_draft::Draft::new(*kind,crate::cplane::CPlane::Xy).buttons();}
        if let Self::Primitive(shape)=self {return shape.options.iter().map(|(label,line)|((*label).into(),(*line).into())).collect();}
        let mut buttons=vec![("Cancel".into(),"Escape".into())];
        if matches!(self,Self::Polyline|Self::Rectangle|Self::Polygon {..}) {
```

<details>
<summary>Locate this block</summary>

```rust
}

#[derive(Clone, Copy, Debug)]
pub enum Shape { Point, Line, Move, Polyline, Curve, Rectangle, Polygon {sides:usize}, Primitive(&'static crate::primitive_shape::Shape) }

impl Tool for Shape {
    fn name(&self) -> &'static str { match self { Self::Point => "Point", Self::Line => "Line", Self::Move => "Move", Self::Polyline => "Polyline",Self::Curve=>"Curve", Self::Rectangle=>"Polyline Rectangle",Self::Polygon {..}=>"Polyline Polygon",Self::Primitive(shape)=>shape.name } }
    fn count(&self) -> usize { match self { Self::Point => 1, Self::Line | Self::Move => 2, Self::Polyline|Self::Curve => 0,Self::Rectangle|Self::Polygon {..}=>2,Self::Primitive(_)=>0 } }
    fn sides(&self)->Option<usize> {if let Self::Polygon {sides}=self {Some(*sides)}else{None}}
    fn set_sides(&mut self,value:usize)->Result<(),&'static str> {
        if !(3..MAX_POINTS).contains(&value) {return Err("Polygon sides must be between 3 and 4095");}
```

</details>

Replace it with:

```rust
}

#[derive(Clone, Copy, Debug)]
pub enum Shape { Point, Line, Move, Polyline, Curve, Rectangle, Polygon {sides:usize}, Primitive(&'static crate::primitive_shape::Shape), Transform(crate::transform_draft::Kind) }

impl Tool for Shape {
    fn name(&self) -> &'static str { match self { Self::Point => "Point", Self::Line => "Line", Self::Move => "Move", Self::Polyline => "Polyline",Self::Curve=>"Curve", Self::Rectangle=>"Polyline Rectangle",Self::Polygon {..}=>"Polyline Polygon",Self::Primitive(shape)=>shape.name,Self::Transform(kind)=>kind.name() } }
    fn count(&self) -> usize { match self { Self::Point => 1, Self::Line | Self::Move => 2, Self::Polyline|Self::Curve => 0,Self::Rectangle|Self::Polygon {..}=>2,Self::Primitive(_)|Self::Transform(_)=>0 } }
    fn sides(&self)->Option<usize> {if let Self::Polygon {sides}=self {Some(*sides)}else{None}}
    fn set_sides(&mut self,value:usize)->Result<(),&'static str> {
        if !(3..MAX_POINTS).contains(&value) {return Err("Polygon sides must be between 3 and 4095");}
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{marker::PreparedPoint, stroke::PreparedLine};
use std::rc::Rc;

pub enum Built { Point(PreparedPoint), Line(PreparedLine), Move([f64; 3]), Polyline(crate::chain::PreparedChain), Geometry(session_rust::Geometry) }

pub trait Tool: std::fmt::Debug {
    fn name(&self) -> &'static str;
    fn count(&self) -> usize;
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str>;
    fn build_on_plane(&self,points:&[[f64;3]],_plane:crate::cplane::CPlane)->Result<Built,&'static str> {self.build(points)}
    fn outline(&self,points:&[[f64;3]],_plane:crate::cplane::CPlane)->Vec<[f64;3]> {points.to_vec()}
    fn sides(&self)->Option<usize> {None}
    fn set_sides(&mut self,_sides:usize)->Result<(),&'static str> {Err("Sides works while drawing a polygon")}
    fn buttons(&self) -> Vec<(String, String)> { vec![("Cancel".into(), "Escape".into())] }
}

#[derive(Clone, Copy, Debug)]
pub enum Shape { Point, Line, Move, Polyline, Curve, Rectangle, Polygon {sides:usize}, Primitive(&'static crate::primitive_shape::Shape), Transform(crate::transform_draft::Kind) }

impl Tool for Shape {
    fn name(&self) -> &'static str { match self { Self::Point => "Point", Self::Line => "Line", Self::Move => "Move", Self::Polyline => "Polyline",Self::Curve=>"Curve", Self::Rectangle=>"Polyline Rectangle",Self::Polygon {..}=>"Polyline Polygon",Self::Primitive(shape)=>shape.name,Self::Transform(kind)=>kind.name() } }
    fn count(&self) -> usize { match self { Self::Point => 1, Self::Line | Self::Move => 2, Self::Polyline|Self::Curve => 0,Self::Rectangle|Self::Polygon {..}=>2,Self::Primitive(_)|Self::Transform(_)=>0 } }
    fn sides(&self)->Option<usize> {if let Self::Polygon {sides}=self {Some(*sides)}else{None}}
    fn set_sides(&mut self,value:usize)->Result<(),&'static str> {
        if !(3..MAX_POINTS).contains(&value) {return Err("Polygon sides must be between 3 and 4095");}
        let Self::Polygon {sides}=self else {return Err("Sides works while drawing a polygon")};*sides=value;Ok(())
    }
    fn build_on_plane(&self,points:&[[f64;3]],plane:crate::cplane::CPlane)->Result<Built,&'static str> {
        let (kind,sides)=match self {Self::Rectangle=>("rectangle",4),Self::Polygon {sides}=>("polygon",*sides),_=>return self.build(points)};
        let points=crate::planar::points(kind,plane,sides,points)?;Self::Polyline.build(&points)
    }
    fn outline(&self,points:&[[f64;3]],plane:crate::cplane::CPlane)->Vec<[f64;3]> {
        match self {Self::Rectangle if points.len()==2=>crate::planar::points("rectangle",plane,4,points).unwrap_or_default(),
            Self::Polygon {sides} if points.len()==2=>crate::planar::points("polygon",plane,*sides,points).unwrap_or_default(),_=>points.to_vec()}
    }
    fn buttons(&self)->Vec<(String,String)> {
        if let Self::Transform(kind)=self {return crate::transform_draft::Draft::new(*kind,crate::cplane::CPlane::Xy).buttons();}
        if let Self::Primitive(shape)=self {return shape.options.iter().map(|(label,line)|((*label).into(),(*line).into())).collect();}
        let mut buttons=vec![("Cancel".into(),"Escape".into())];
        if matches!(self,Self::Polyline|Self::Rectangle|Self::Polygon {..}) {
            for name in ["Points","Rectangle","Polygon"] {buttons.push((name.into(),format!("Polyline {name}")));}
        }
        if matches!(self,Self::Polyline|Self::Curve) {buttons.insert(0,("Close".into(),"Close".into()));}buttons
    }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
            (Self::Point, [p]) => Ok(Built::Point(PreparedPoint::new(Rc::new(session_rust::Point::new(p[0], p[1], p[2])))?)),
            (Self::Line, [a, b]) => {
                if a.iter().zip(b).all(|(a, b)| (a-b).abs() < 1e-12) { return Err("Line endpoints must differ"); }
                Ok(Built::Line(PreparedLine::new(Rc::new(session_rust::Line::new(a[0], a[1], a[2], b[0], b[1], b[2])))?))
            }
            (Self::Move, [a,b]) => Ok(Built::Move(std::array::from_fn(|i| b[i]-a[i]))),
            (Self::Polyline, points) if (2..=MAX_POINTS).contains(&points.len()) => {
                if !points.windows(2).any(|p| p[0] != p[1]) { return Err("Polyline points must differ"); }
                let points=points.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect();
                Ok(Built::Polyline(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(points)))?))
            }
            (Self::Curve,points) if (2..=MAX_POINTS).contains(&points.len())=>{
                if !points.windows(2).any(|p|p[0]!=p[1]) {return Err("Curve controls must differ");}
                let controls=points.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect::<Vec<_>>();
                let source=crate::freeform_curve::curve(&controls);
                Ok(Built::Geometry(session_rust::Geometry::NurbsCurve(Rc::new(source))))
            },
            (Self::Rectangle|Self::Polygon {..},_)=>self.build_on_plane(points,crate::cplane::CPlane::Xy),
            _ => Err("The tool needs more points"),
        }
    }
}

pub const MAX_POINTS: usize = 4096;

// A draft owns unfinished points and a cursor, so preview motion cannot become saved geometry or an Undo transaction.
pub struct Draft { pub tool: Box<dyn Tool>, pub points: Vec<[f64; 3]>, pub cursor: Option<[f64; 3]>, pub snapped: Option<crate::snap::Kind>, pub frame: crate::cplane::CPlane, pub plane_points: Option<Vec<[f64;3]>>, pub primitive:Option<crate::primitive_draft::Draft>, pub transform:Option<crate::transform_draft::Draft> }

impl Draft {
    pub fn new(shape: Shape) -> Self { Self { tool: Box::new(shape), points: Vec::new(), cursor: None, snapped: None, frame: crate::cplane::CPlane::Xy, plane_points: None, primitive:if let Shape::Primitive(shape)=shape {Some(crate::primitive_draft::Draft::new(shape,crate::cplane::CPlane::Xy,0).expect("Registered primitive option"))}else{None},transform:if let Shape::Transform(kind)=shape {Some(crate::transform_draft::Draft::new(kind,crate::cplane::CPlane::Xy))}else{None} } }
    pub fn prompt(&self) -> String {
        if self.plane_points.is_none() {if let Some(transform)=&self.transform {return transform.prompt();}}
        if self.plane_points.is_none() {if let Some(primitive)=&self.primitive {return primitive.prompt();}}
        let mut prompt=if let Some(points)=&self.plane_points {format!("CPlane 3 Point · point {} of 3 · Esc cancels",points.len()+1)}
            else if self.tool.count()==0 {format!("{} point {} · Enter finishes · Esc cancels",self.tool.name(),self.points.len()+1)}
            else {format!("{} point {} of {} · Esc cancels", self.tool.name(), self.points.len()+1, self.tool.count())};
        if let Some(sides)=self.tool.sides() {prompt.push_str(&format!(" · Sides {sides}"));}
        if let Some(kind)=self.snapped { prompt.push_str(&format!(" · Snap {kind:?}")); }
        prompt
    }
    pub fn submit(&mut self, point: [f64; 3]) -> Result<Option<Built>, &'static str> { self.submit_many(&[point]) }
    pub fn submit_many(&mut self, added: &[[f64;3]]) -> Result<Option<Built>, &'static str> {
        let mut points=self.points.clone();
        for &point in added { points.push(crate::tool_coords::checked(point)?); }
        let count=self.tool.count(); let limit=if count==0 {MAX_POINTS} else {count};
        if points.len()>limit {return Err("The tool has enough points");}
        let built=if count>0 && points.len()==count {Some(self.build_world(&points)?)} else {None};
        self.points=points;self.cursor=None;self.snapped=None;Ok(built)
    }
    pub fn finish(&self) -> Result<Built,&'static str> { self.build_world(&self.points) }
    // Finished coordinates already belong to the world; their prefix prevents applying a fixed plane a second time during replay.
    pub fn command(&self, points:&[[f64;3]]) -> String {
        let mut line=self.tool.name().to_owned();
        for p in points {line.push_str(&format!(" w{},{},{}",p[0],p[1],p[2]));}line
    }
    pub fn build_world(&self, points:&[[f64;3]]) -> Result<Built,&'static str> {
        let command=self.command(points);let text=&command[self.tool.name().len()..];
        let world=crate::drawing_input::points(text,crate::drawing_coords::plane(),self.frame,&[],None)?;
        self.tool.build_on_plane(&world,self.frame)
    }
    pub fn outlines(&self)->Vec<Vec<[f64;3]>> {
        if let Some(primitive)=&self.primitive {primitive.outline(self.cursor)}else{vec![self.outline()]}
    }
    pub fn outline(&self)->Vec<[f64;3]> {
        let mut points=self.points.clone();if let Some(cursor)=self.cursor {points.push(cursor);}
        self.tool.outline(&points,self.frame)
    }
    pub fn hover(&mut self, point: Option<[f64; 3]>) -> Result<(), &'static str> {
        self.cursor = point.map(crate::tool_coords::checked).transpose()?; Ok(())
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77g-carry-transformation-state-through-the-existing-drawing-owner-01.rs"
```

```text
--8<-- "journey/code/77g-carry-transformation-state-through-the-existing-drawing-owner-02.rs"
```

```text
--8<-- "journey/code/77g-carry-transformation-state-through-the-existing-drawing-owner-03.rs"
```

</details>

### 2. `src/editor.rs`

Own temporary original-part and whole-placement transformation previews. Connect current transformation behavior to the existing input and history owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    }

    pub fn presentation_scenes(&self)->impl Iterator<Item=&Scene> {
        self.scenes().chain(self.canvas_drag.iter().filter_map(|d|d.preview.as_ref()).flat_map(|p|p.scenes())).chain(self.edit_preview.iter().flat_map(|p|p.scenes())).chain(self.gizmo_edit.iter().flat_map(|g|g.preview.scenes()))
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
```

</details>

Replace it with:

```rust
    }

    pub fn presentation_scenes(&self)->impl Iterator<Item=&Scene> {
        self.scenes().chain(self.transform_preview.iter().flat_map(|p|p.scenes())).chain(self.canvas_drag.iter().filter_map(|d|d.preview.as_ref()).flat_map(|p|p.scenes())).chain(self.edit_preview.iter().flat_map(|p|p.scenes())).chain(self.gizmo_edit.iter().flat_map(|g|g.preview.scenes()))
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
```

<details>
<summary>Locate this block</summary>

```rust
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview)=self.edit_preview.as_mut() else { return Ok(Change::View); };
        let offset=self.draft.as_ref().and_then(|d| d.points.first().zip(d.cursor)).map(|(a,b)| std::array::from_fn::<_,3,_>(|i| b[i]-a[i])).unwrap_or([0.;3]);
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?; Ok(Change::Preview)
```

</details>

Replace it with:

```rust
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        if let Some(transform)=self.draft.as_ref().and_then(|draft|draft.transform.as_ref()) {
            let matrix=transform.preview(self.draft.as_ref().and_then(|draft|draft.cursor)).unwrap_or_else(session_rust::Xform::identity);
            if let Some(preview)=self.transform_preview.as_mut(){preview.set(matrix)?;return Ok(Change::Preview);}return Ok(Change::View);
        }
        let Some(preview)=self.edit_preview.as_mut() else { return Ok(Change::View); };
        let offset=self.draft.as_ref().and_then(|d| d.points.first().zip(d.cursor)).map(|(a,b)| std::array::from_fn::<_,3,_>(|i| b[i]-a[i])).unwrap_or([0.;3]);
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?; Ok(Change::Preview)
```

<details>
<summary>Locate this block</summary>

```rust

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|g|g.preview.scene()),|p|&p.scene)
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
```

</details>

Replace it with:

```rust

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.transform_preview.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|p|p.scene()),|g|g.preview.scene()),|p|&p.scene)
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
```

<details>
<summary>Locate this block</summary>

```rust
    fn default() -> Self {
        Self {
            canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
            camera: Camera::default(),
```

</details>

Replace it with:

```rust
    fn default() -> Self {
        Self {
            canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
            camera: Camera::default(),
```

<details>
<summary>Locate this block</summary>

```rust
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
    gizmo_hover:Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
```

</details>

Replace it with:

```rust
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
    gizmo_hover:Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub transform_preview:Option<crate::gizmo_edit::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    StreamRefresh,
    PrimitiveBegin {shape:&'static crate::primitive_shape::Shape,option:usize,text:String},
    ToolSides(usize),
    ConstructionPlane(Option<crate::cplane::CPlane>), PlaneMenu, Plane3,
    Draw {shape:crate::tool::Shape,text:String}, ToolText(String), FinishTool, CloseTool,
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
    CanvasRelease {screen:[f32;2],size:[f64;2],add:bool,remove:bool},
    CanvasHit(Option<ObjectId>),CanvasCancel,
    SelectRows {ids:Vec<ObjectId>,add:bool,remove:bool},
    PanScreen {delta:[f64;2],size:[f64;2]},ZoomAt {factor:f32,screen:[f32;2]},
    Touch(Vec<Action>),
    LayerEdit {key:String,edit:crate::layer_edit::Edit},
    LayerProperty {key:String,property:crate::layer_properties::Property},
    TreePick {ids:Vec<ObjectId>,additive:bool},
    GizmoPress {pointer:i32,screen:[f32;2],size:[f64;2],modified:bool},
    GizmoTouch {pointer:i32,screen:[f32;2],size:[f64;2]},
    GizmoMove {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoRelease {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen { screen: [f32; 2], size: [f64; 2], commit: bool },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany { target: Option<crate::pick_sources::Target>, additive: bool },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Live(Box<Scene>),
    Adopt {scene:Box<Scene>,replace:bool},
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    ReplaceAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    Close,
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddCloudLod,
    AddCloud,
    AddCad,
    AddInstances,
    AddShapeReuse,
    AddNestedInstances,
    AddSheet,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
    AddPoint,
    AddCurve,
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
    Preview,
}

pub struct Editor {
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
    gizmo_hover:Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub transform_preview:Option<crate::gizmo_edit::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub construction_plane:Option<crate::cplane::CPlane>,
    pub plane_menu:bool,
    pub plane_only:bool,
    pub tool_status: String,
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
    pub pick_mode: crate::pick_sources::Mode,
    objects: crate::object_selection::Objects,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.transform_preview.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|p|p.scene()),|g|g.preview.scene()),|p|&p.scene)
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if self.scene.clouds().iter().any(|r|r.controls_visible)||self.scene.streams().iter().any(|r|r.controls_visible) {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
    }

    pub fn gizmo_hit(&self,screen:[f32;2],size:[f64;2])->Option<crate::gizmo::Handle> {self.gizmo_hit_radius(screen,size,8.)}

    pub fn gizmo_hit_radius(&self,screen:[f32;2],size:[f64;2],radius:f64)->Option<crate::gizmo::Handle> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
        let origin=self.gizmo_origin()?;
        let scale=crate::widget_layout::world_per_pixel(&self.camera.view_projection(),origin,size[1])?;
        let ray=self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2])).hit_with_radius(&ray.origin,&ray.direction,scale,radius)
    }

    pub fn number_prompt(&self,size:[f64;2])->Option<crate::number_box::NumberPrompt> {
        let edit=self.gizmo_edit.as_ref()?;let handle=edit.gizmo.typing?;
        let origin=edit.origin();let matrix=self.camera.view_projection();let scale=crate::widget_layout::world_per_pixel(&matrix,origin,size[1])?;
        let point=edit.gizmo.handle_point(handle,scale);
        let at=crate::snap_math::Screen {matrix:matrix.m,size}.point([point[0],point[1],point[2]])?;
        let unit=handle.labels().2;
        Some(crate::number_box::NumberPrompt {handle,title:handle.title(),unit,hint:if unit=="factor" {"1"} else {"0"},at:at.map(|v|v as f32)})
    }

    pub fn gizmo_active(&self)->f32 {crate::gizmo_edit::code(self.gizmo_edit.as_ref().and_then(|e|e.active()).or(self.gizmo_hover))}

    pub fn hover_gizmo(&mut self,screen:Option<([f32;2],[f64;2])>) {
        self.gizmo_hover=if self.gizmo_edit.is_some() {None} else {screen.and_then(|(point,size)|self.gizmo_hit(point,size))};
    }

    fn gizmo_press(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool)->Result<Change,&'static str> {
        self.gizmo_press_radius(pointer,screen,size,modified,8.)
    }

    fn gizmo_press_radius(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool,radius:f64)->Result<Change,&'static str> {
        if modified || self.draft.is_some() || self.gizmo_edit.as_ref().is_some_and(|g|g.pointer.is_some()) {return Ok(Change::View);}
        let Some(handle)=self.gizmo_hit_radius(screen,size,radius) else {return Ok(Change::View);};
        let origin=self.gizmo_origin().ok_or("No editable selection")?;
        let preview=crate::gizmo_edit::Preview::new(&self.scene,self.selected_objects(),self.selection().filter(|_|self.selected_part.is_some()))?;
        let ray=self.camera.ray(screen).ok_or("Invalid handle ray")?;
        let gizmo=crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2]));
        self.gizmo_edit=crate::gizmo_edit::Edit::begin(gizmo,handle,&ray,preview,pointer,screen);Ok(Change::View)
    }

    fn gizmo_move(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return Err("Invalid handle viewport");}
        let Some(ray)=self.camera.ray(screen) else {return Err("Invalid handle ray");};
        let Some(edit)=self.gizmo_edit.as_mut() else {return Ok(Change::View);};
        Ok(if edit.update(pointer,screen,size,&ray,shift)? {Change::Preview} else {Change::View})
    }

    fn gizmo_release(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if self.gizmo_edit.as_ref().is_none_or(|g|g.pointer!=Some(pointer)) {return Ok(Change::View);}
        self.gizmo_move(pointer,screen,size,shift)?;
        let edit=self.gizmo_edit.as_mut().unwrap();
        if edit.click() {self.tool_status=format!("{}: type a value in {}, Enter applies, Esc closes",edit.active().unwrap().title(),edit.active().unwrap().labels().2);return Ok(Change::Preview);}
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?; self.gizmo_edit=None;
        self.tool_status="Handle edit completed.".into();Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn gizmo_number(&mut self,text:&str)->Result<Change,&'static str> {
        let edit=self.gizmo_edit.as_mut().ok_or("No handle number is being requested")?;
        let handle=edit.gizmo.typing.ok_or("No handle number is being requested")?;
        let value=crate::gizmo::typed_value(handle,text).map_err(|_|"Enter a finite number; scale needs a positive factor")?;
        let Some(value)=value else {self.gizmo_edit=None;return Ok(Change::Preview);};
        edit.preview.set(edit.gizmo.typed(handle,value))?;
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?;self.gizmo_edit=None;
        self.tool_status=format!("{} {value} {}",handle.title(),handle.labels().2);Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        if let Some(transform)=self.draft.as_ref().and_then(|draft|draft.transform.as_ref()) {
            let matrix=transform.preview(self.draft.as_ref().and_then(|draft|draft.cursor)).unwrap_or_else(session_rust::Xform::identity);
            if let Some(preview)=self.transform_preview.as_mut(){preview.set(matrix)?;return Ok(Change::Preview);}return Ok(Change::View);
        }
        let Some(preview)=self.edit_preview.as_mut() else { return Ok(Change::View); };
        let offset=self.draft.as_ref().and_then(|d| d.points.first().zip(d.cursor)).map(|(a,b)| std::array::from_fn::<_,3,_>(|i| b[i]-a[i])).unwrap_or([0.;3]);
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?; Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self)->impl Iterator<Item=&Scene> {
        self.scenes().chain(self.transform_preview.iter().flat_map(|p|p.scenes())).chain(self.canvas_drag.iter().filter_map(|d|d.preview.as_ref()).flat_map(|p|p.scenes())).chain(self.edit_preview.iter().flat_map(|p|p.scenes())).chain(self.gizmo_edit.iter().flat_map(|g|g.preview.scenes()))
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene.objects().iter().filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id)).collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(&mut self, values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>) -> Result<bool, &'static str> {
        if values.is_empty() { return Ok(false); }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id) || !self.scene.objects().iter().any(|row| key.matches(row)) {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values.into_iter().map(|(key, bytes)|
            crate::rehydrate::PreparedReload::new(key, &bytes)).collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self.scenes().flat_map(|scene| scene.objects()).filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row)) { return Err("Reloaded source GUID or metadata does not match"); }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared { scene.hydrate(plan); }
        }
        if self.edit_preview.take().is_some() { self.draft=None; } self.gizmo_edit=None; self.snap_index = None; Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self.scenes().flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload()).filter_map(|row| row.origin().map(|origin| origin.id)).collect();
        for row in self.scenes().flat_map(|scene| scene.objects()) {
            if let Some(origin) = row.origin() {
                if !row.can_unload() { origins.remove(&origin.id); }
            }
        }
        if origins.is_empty() { return Err("No eligible imported sources to unload"); }
        let epoch = self.last_release.checked_add(1).ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() { scene.unload(&origins, epoch); }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        if !self.scene.contains(id) {return Err("Object not found");}
        let object=self.scene.objects().iter().find(|row|row.id==id);
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if object.is_some_and(|object|object.geometry().is_none()) {return Err("Reload editable sources before Move");}
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let ids=self.scene.with_descendants(&[id]);let delta=session_rust::Xform::translation(offset[0],offset[1],offset[2]);
        self.history.try_edit(&mut self.scene,|scene| {for id in ids {let matrix=&delta*&scene.placement(id).ok_or("Object not found")?;scene.place_following(id,matrix)?;}Ok(())})?;
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {return Err("Object not found");}
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if self.scene.objects().iter().find(|row| row.id == id).is_some_and(|row| row.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {for id in ids {scene.remove(id);}});
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) { self.selected = self.objects.primary(); self.selected_part = None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() { return Vec::new(); }
        if self.objects.primary() == self.selected { self.objects.rows().to_vec() }
        else { self.selected.into_iter().filter(|id| self.scene.pick_id(id.value()).is_some()).collect() }
    }

    pub fn select_objects(&mut self, rows: impl IntoIterator<Item = ObjectId>, additive: bool) {
        if additive { let old = self.selected_objects(); self.objects.set(&self.scene, old, false); }
        self.selected_part = None; self.objects.set(&self.scene, rows, additive); self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self,ids:Vec<ObjectId>,add:bool,remove:bool) {
        if remove {let old=self.selected_objects();self.objects.set(&self.scene,old,false);self.objects.remove(&self.scene,ids);self.selected=self.objects.primary();self.selected_part=None;}
        else {self.select_objects(ids,add);}
    }

    fn finish_canvas(&mut self)->Result<Change,&'static str> {
        let Some(mut drag)=self.canvas_drag.take() else {return Ok(Change::View);};
        let (add,remove)=drag.released.unwrap_or((false,false));
        if let Some(preview)=drag.preview.take() {let changed=preview.commit(&mut self.scene,&mut self.history)?;if changed {self.snap_index=None;}return Ok(if changed {Change::Scene} else {Change::Preview});}
        if drag.missed {if let Some(rectangle)=drag.rectangle() {let ids=crate::rect_select::found(&self.scene,&self.camera.view_projection().m,drag.size,rectangle);self.select_rows(ids,add,remove);}}
        Ok(Change::View)
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene); self.selected = self.objects.primary();
        } else { self.objects.clear(); }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) { return Err("Move needs finite coordinates"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Err("Select an object before Move"); }
        if offset.iter().all(|&n| n == 0.) { return Ok(Change::View); }
        let mut preview=crate::edit_preview::Preview::new(&self.scene,ids)?;
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
        preview.commit(&mut self.scene,&mut self.history)?;Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() { return Err("Use Select Object before Delete"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Ok(Change::View); }
        if self.scene.objects().iter().any(|r| ids.contains(&r.id) && r.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| { for id in ids { scene.remove(id); } });
        self.objects.clear(); self.selected = None; Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected.map(|owner| crate::pick_sources::Target { owner, part: self.selected_part.unwrap_or(crate::pick_sources::Part::Object) })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> { self.selected.filter(|_| self.selected_part.is_none()) }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")) {
            return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape") {Action::GizmoCancel} else {Action::GizmoNumber(text.to_owned())}));
        }
        if self.plane_menu {return Some(crate::drawing_input::choice(text).ok_or("Choose XY, XZ, YZ, 3 Point, View or Cancel"));}
        let draft=self.draft.as_ref()?;
        if let Some(choice)=crate::drawing_input::choice(text) {return Some(Ok(choice));}
        if draft.plane_points.is_none() && draft.primitive.is_some() {
            let words:Vec<_>=text.split_whitespace().collect();let primitive=draft.primitive.as_ref().unwrap();
            if text.trim().is_empty() || primitive.shape.option_in(&words).is_some() || words.first().is_some_and(|word|word.parse::<f64>().is_ok() || crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@')) {return Some(Ok(Action::ToolText(text.to_owned())));}
            return None;
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count()==0 {return Some(Ok(Action::CloseTool));}
        if text.trim().is_empty() {return Some(Ok(if draft.tool.count()==0 && !self.plane_only {Action::FinishTool}else{Action::CancelTool}));}
        let word=text.trim();
        if word.split_whitespace().next().is_some_and(|s|s.eq_ignore_ascii_case("Sides")) && draft.tool.sides().is_some() {
            let parts:Vec<_>=word.split_whitespace().collect();
            return Some(match parts.as_slice() {[_name,value]=>value.parse::<usize>().ok().filter(|n|(3..crate::tool::MAX_POINTS).contains(n)).map(Action::ToolSides).ok_or("Polygon sides must be between 3 and 4095"),_=>Err("Use Sides 6")});
        }
        let coordinate=crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@') || word.starts_with('w') || word.starts_with('W') || word.split_whitespace().next().is_some_and(|p|crate::drawing_coords::parse_world(p).is_some());
        coordinate.then(|| {
            let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
            crate::drawing_input::points(word,self.construction_plane,draft.frame,points,draft.cursor).map(|_|Action::ToolText(word.to_owned()))
        })
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        let draft=self.draft.as_mut().ok_or("No point tool is running")?;
        if let Some(points)=draft.plane_points.as_mut() {
            let point=crate::tool_coords::checked(point)?;let mut picked=points.clone();picked.push(point);
            if picked.len()==3 {
                let p:Vec<_>=picked.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect();
                let plane=crate::cplane::CPlane::from_3_points(&p[0],&p[1],&p[2]).ok_or("The three points are in line; pick the third again")?;
                draft.plane_points=None;self.apply(Action::ConstructionPlane(Some(plane)))?;
                if self.plane_only {self.draft=None;self.plane_only=false;}return Ok(Change::View);
            }
            *points=picked;self.tool_status=draft.prompt();return Ok(Change::View);
        }
        if draft.primitive.is_some() {return self.primitive_answer(session_rust::Point::new(point[0],point[1],point[2]));}
        crate::drawing_coords::set_plane(self.construction_plane);
        let Some(built)=draft.submit(point)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    fn commit_tool(&mut self,built:crate::tool::Built) -> Result<Change,&'static str> {
        let name=self.draft.as_ref().ok_or("No point tool is running")?.tool.name();
        if let crate::tool::Built::Move(offset)=built {
            let preview=self.edit_preview.as_mut().ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
            let changed=preview.commit(&mut self.scene,&mut self.history)?;
            self.edit_preview=None;self.draft=None;self.tool_status="Move completed.".into();return Ok(if changed {Change::Scene}else{Change::Preview});
        }
        self.history.try_edit(&mut self.scene,|scene|match built {
            crate::tool::Built::Point(point)=>scene.insert_point(point).map(|_|()),
            crate::tool::Built::Line(line)=>scene.insert_line(line).map(|_|()),
            crate::tool::Built::Polyline(line)=>scene.insert_path(line).map(|_|()),
            crate::tool::Built::Geometry(geometry)=>crate::primitive_geometry::insert(scene,geometry).map(|_|()),
            crate::tool::Built::Move(_)=>unreachable!("Move uses its placement preview"),
        })?;
        self.draft=None;self.tool_status=format!("{name} created.");Ok(Change::Scene)
    }

    fn primitive_failure(&mut self,message:String)->&'static str {self.tool_status=message;"Shape input invalid"}
    fn primitive_answer(&mut self,point:session_rust::Point)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.accept(crate::primitive_shape::Answer::Point(point.clone()));
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points.push([point[0],point[1],point[2]]);draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn primitive_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.text(text,draft.cursor,self.construction_plane);
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if self.draft.as_ref().is_some_and(|d|d.primitive.is_some() && d.plane_points.is_none()) {return self.primitive_text(text);}
        let draft=self.draft.as_ref().ok_or("No point tool is running")?;
        let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
        let added=crate::drawing_input::points(text,self.construction_plane,draft.frame,points,draft.cursor)?;
        if draft.plane_points.is_some() {
            if added.len()!=1 {return Err("Supply one construction-plane point at a time");}return self.place_tool_point(added[0]);
        }
        let draft=self.draft.as_mut().unwrap();
        let Some(built)=draft.submit_many(&added)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    pub fn snap_point(&mut self, screen: [f32; 2], size: [f64; 2]) -> Option<crate::snap::Hit> {
        if let Some(primitive)=self.draft.as_ref()?.primitive.as_ref() {if primitive.next().is_some_and(|ask|ask.kind==crate::primitive_shape::Kind::Height) {return primitive.on_axis(&self.camera,screen).map(|point|crate::snap::Hit {point,kind:crate::snap::Kind::Near,owner:None});}}
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude=self.edit_preview.as_ref().filter(|_| from.is_some()).map_or_else(Vec::new,|p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| crate::snap_index::Index::new(&self.scene).excluding(&exclude));
            let viewport = crate::snap_math::Screen { matrix: self.camera.view_projection().m, size };
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        let own=self.draft.as_ref().filter(|d|d.primitive.is_none() && d.tool.count()==0 && d.points.len()>=2 && self.snapping.objects && self.snapping.modes&crate::snap::END!=0).and_then(|d|{
            let point=*d.points.first()?;let viewport=crate::snap_math::Screen {matrix:self.camera.view_projection().m,size};let p=viewport.point(point)?;
            let cursor=[(screen[0] as f64+1.)*size[0]/2.,(1.-screen[1] as f64)*size[1]/2.];
            ((p[0]-cursor[0]).hypot(p[1]-cursor[1])<=12.).then_some(crate::snap::Hit {point,kind:crate::snap::Kind::End,owner:None})
        });
        own.or(hit).or_else(|| {
            let draft=self.draft.as_ref()?;
            let frame=self.construction_plane.unwrap_or(draft.frame);
            let through=if self.construction_plane.is_none() {draft.points.last().copied()}else{None};
            let point=crate::drawing_input::on_at(&self.camera,screen,frame,through)?;
            let (x,y)=frame.axes();let o=frame.origin();
            let point=if self.snapping.grid {crate::snap_math::grid(point,[o[0],o[1],o[2]],[[x[0],x[1],x[2]],[y[0],y[1],y[2]]],self.snapping.step)?}else{point};
            Some(crate::snap::Hit {point,kind:if self.snapping.grid {crate::snap::Kind::Grid}else{crate::snap::Kind::Near},owner:None})
        })
    }

    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
            Action::Touch(actions)=> {let mut change=Change::View;for action in actions {let next=self.apply(action)?;if next!=Change::View {change=next;}}return Ok(change);},
            Action::CanvasStart {from,to,size,modified,pointer}=>{self.canvas_drag=crate::canvas_drag::Drag::new(from,to,size,modified);if let Some(drag)=self.canvas_drag.as_mut() {drag.pointer=pointer;}return Ok(Change::View);},
            Action::CanvasMove {screen,size}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.released.is_none()&&drag.size==size&&screen.iter().all(|v|v.is_finite()) {drag.to=screen;drag.update(&self.camera)?;}if drag.preview.is_some() {return Ok(Change::Preview);}}return Ok(Change::View);},
            Action::CanvasRelease {screen,size,add,remove}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.size!=size {self.canvas_drag=None;return Ok(Change::Preview);}drag.to=screen;drag.released=Some((add,remove));drag.update(&self.camera)?;if drag.preview.is_some()||drag.missed {return self.finish_canvas();}}return Ok(Change::View);},
            Action::CanvasHit(hit)=>{
                let Some(drag)=self.canvas_drag.as_ref() else {return Ok(Change::View);};let ids=hit.filter(|id|!drag.modified&&self.scene.pick_id(id.value()).is_some());
                if let Some(id)=ids {let mut rows=self.selected_objects();if !rows.contains(&id) {self.select_objects([id],false);rows=vec![id];}if let Err(error)=self.canvas_drag.as_mut().unwrap().grab(&self.scene,rows,&self.camera) {self.canvas_drag=None;return Err(error);}}
                else {self.canvas_drag.as_mut().unwrap().missed=true;}
                if self.canvas_drag.as_ref().unwrap().released.is_some() {return self.finish_canvas();}return Ok(Change::Preview);
            },
            Action::CanvasCancel=>{let preview=self.canvas_drag.take().is_some_and(|d|d.preview.is_some());return Ok(if preview {Change::Preview} else {Change::View});},
            Action::SelectRows {ids,add,remove}=>{self.select_rows(ids,add,remove);return Ok(Change::View);},
            _=>{}
        }
        self.canvas_drag=None;
        let keep_gizmo=matches!(&action,Action::GizmoPress {..}|Action::GizmoTouch {..}|Action::GizmoMove {..}|Action::GizmoRelease {..}|Action::GizmoCancel|Action::GizmoNumber(_)|Action::Zoom(_)|Action::ZoomAt {..}|Action::PanScreen {..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::Snap(_));
        let keep_edit=matches!(&action, Action::GizmoPress {..} | Action::GizmoTouch {..} | Action::GizmoMove {..} | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::BeginTool(_) | Action::Draw {..} | Action::ToolText(_) | Action::FinishTool | Action::CloseTool | Action::ConstructionPlane(_) | Action::PlaneMenu | Action::Plane3 | Action::ToolSides(_) | Action::PrimitiveBegin {..} | Action::ToolPoint(_) | Action::ToolScreen {..} | Action::ToolHover(_) | Action::CancelTool | Action::Snap(_) | Action::Zoom(_) | Action::ZoomAt {..} | Action::PanScreen {..} | Action::Pan(_,_) | Action::Orbit(_,_) | Action::Fit | Action::FitSelected | Action::Projection(_) | Action::ResetView | Action::Background | Action::NormalView(_) | Action::Grid(_));
        let mut change=self.apply_action(action)?;
        self.gizmo_hover=None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change==Change::View {change=Change::Preview;}
        if !keep_edit && self.edit_preview.take().is_some() { self.draft=None; if change==Change::View { change=Change::Preview; } }
        if change==Change::Scene { self.snap_index=None; }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {

        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        match action {
            Action::PrimitiveBegin {shape,option,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let mut primitive=crate::primitive_draft::Draft::new(shape,frame,option).map_err(|message|self.primitive_failure(message))?;
                let geometry=if text.trim().is_empty() {None}else{primitive.text(&text,None,self.construction_plane).map_err(|message|self.primitive_failure(message))?};
                self.apply(Action::BeginTool(crate::tool::Shape::Primitive(shape)))?;
                let draft=self.draft.as_mut().unwrap();draft.primitive=Some(primitive);draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();
                if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ToolSides(sides)=>{
                let draft=self.draft.as_mut().ok_or("Sides works while drawing a polygon")?;draft.tool.set_sides(sides)?;self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ConstructionPlane(plane)=>{
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=self.draft.as_mut().and_then(|draft|draft.primitive.as_mut()) {primitive.set_plane(frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                self.construction_plane=plane;crate::drawing_coords::set_plane(plane);self.plane_menu=false;
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(draft)=self.draft.as_mut() {draft.frame=frame;draft.cursor=None;draft.snapped=None;}
                self.tool_status="Construction plane changed.".into();return Ok(Change::View);
            },
            Action::PlaneMenu=>{self.plane_menu=true;self.tool_status="CPlane · XY XZ YZ 3 Point View Cancel".into();return Ok(Change::View);},
            Action::Plane3=>{
                self.plane_menu=false;self.plane_only=self.draft.is_none();
                if self.plane_only {self.apply(Action::BeginTool(crate::tool::Shape::Point))?;}
                let draft=self.draft.as_mut().unwrap();draft.plane_points=Some(Vec::new());draft.frame=crate::cplane::CPlane::Xy;
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::Draw {shape,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let added=crate::drawing_input::points(&text,self.construction_plane,frame,&[],None)?;
                let mut draft=crate::tool::Draft::new(shape);draft.frame=frame;
                let built=draft.submit_many(&added)?;
                let built=if built.is_none() && draft.tool.count()==0 && added.len()>=2 {Some(draft.finish()?)}else{built};
                self.apply(Action::BeginTool(shape))?;self.draft=Some(draft);
                if let Some(built)=built {return self.commit_tool(built);}self.tool_status=self.draft.as_ref().unwrap().prompt();return Ok(Change::View);
            },
            Action::ToolText(text)=>return self.tool_text(&text),
            Action::CloseTool=>{
                let draft=self.draft.as_ref().ok_or("No point tool is running")?;
                if draft.tool.count()!=0 || draft.points.len()<3 {return Err("Close needs at least three polyline points");}
                if draft.points.len()>=crate::tool::MAX_POINTS {return Err("Drawing commands accept at most 4096 points");}
                let mut points=draft.points.clone();points.push(points[0]);
                crate::drawing_coords::set_plane(self.construction_plane);
                let built=draft.build_world(&points)?;return self.commit_tool(built);
            },
            Action::FinishTool=>{crate::drawing_coords::set_plane(self.construction_plane);let built=self.draft.as_ref().ok_or("No point tool is running")?.finish()?;return self.commit_tool(built);},
            Action::LayerEdit {key,edit}=>{
                if matches!(edit,crate::layer_edit::Edit::Current) {crate::layer_edit::apply(&mut self.scene,&key,edit)?;return Ok(Change::View);}
                self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::apply(scene,&key,edit))?;self.repair_selection();
            },
            Action::LayerProperty {key,property}=>{crate::layer_properties::apply(&mut self.scene,&key,property)?;self.repair_selection();if self.selected.is_some_and(|id|self.scene.pick_id(id.value()).is_none()) {self.selected=None;self.selected_part=None;}self.snap_index=None;return Ok(Change::Preview);},
            Action::Hide=>{let ids=self.scene.with_descendants(&self.selected_objects());std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,true);self.select_objects([],false);self.snap_index=None;return Ok(Change::Preview);},
            Action::Show=>{std::rc::Rc::make_mut(&mut self.scene.view).show_all();self.snap_index=None;return Ok(Change::Preview);},
            Action::TreePick {ids,additive}=>{let eligible=ids.into_iter().filter(|id|self.scene.pick_id(id.value())==Some(*id)).collect::<Vec<_>>();if eligible.is_empty() {return Err("This tree row has no visible editable objects");}self.select_objects(eligible,additive);return self.apply(Action::FitSelected);},
            Action::GizmoTouch {pointer,screen,size}=>return self.gizmo_press_radius(pointer,screen,size,false,18.),
            Action::GizmoPress {pointer,screen,size,modified}=>return self.gizmo_press(pointer,screen,size,modified),
            Action::GizmoMove {pointer,screen,size,shift}=>return self.gizmo_move(pointer,screen,size,shift),
            Action::GizmoRelease {pointer,screen,size,shift}=>return self.gizmo_release(pointer,screen,size,shift),
            Action::GizmoCancel=>{self.gizmo_edit=None;self.tool_status="Handle edit cancelled.".into();return Ok(Change::Preview);},
            Action::GizmoNumber(text)=>return self.gizmo_number(&text),
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() { draft.cursor = None; draft.snapped = None; }
                return self.update_edit_preview();
            }
            Action::ToolScreen { screen, size, commit } => {
                let hit = self.snap_point(screen,size);
                if let Some(draft) = self.draft.as_mut() { draft.snapped = hit.filter(|h| h.owner.is_some() || h.kind == crate::snap::Kind::Grid).map(|h| h.kind); }
                if commit {
                    if hit.is_some_and(|h|self.draft.as_ref().is_some_and(|d|d.primitive.is_none() && d.tool.count()==0 && d.points.len()>=2 && d.points.first().is_some_and(|p|(0..3).all(|i|(p[i]-h.point[i]).abs()<=1e-12)))) {return self.apply(Action::CloseTool);}
                    return hit.map_or(Ok(Change::View), |hit| self.place_tool_point(hit.point));
                }
                if let Some(draft) = self.draft.as_mut() { draft.hover(hit.map(|h| h.point))?; }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                let preview=if matches!(shape,crate::tool::Shape::Move) {
                    if self.selected_part.is_some() { return Err("Use Select Object before Move"); }
                    Some(crate::edit_preview::Preview::new(&self.scene,self.selected_objects())?)
                } else { None };
                let changed=preview.is_some() || self.edit_preview.is_some(); self.edit_preview=preview;
                self.snap_index=None;let mut draft=crate::tool::Draft::new(shape);
                draft.frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=draft.primitive.as_mut() {primitive.set_plane(draft.frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                self.draft=Some(draft);self.tool_status=self.draft.as_ref().unwrap().prompt();
                return Ok(if changed { Change::Preview } else { Change::View });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => { let changed=self.edit_preview.take().is_some(); self.draft=None;self.plane_menu=false;self.plane_only=false; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
                self.select_objects(target.filter(|t| t.part == crate::pick_sources::Part::Object && t.valid(&self.scene)).map(|t| t.owner), additive);
            }
            Action::SelectAll => {
                let ids: Vec<_> = self.scene.objects().iter().map(|r| r.id).chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id)).chain(self.scene.points().iter().map(|r| r.id)).chain(self.scene.texts().iter().map(|r|r.id)).chain(self.scene.clouds().iter().map(|r|r.id)).chain(self.scene.streams().iter().map(|r|r.id)).chain(self.scene.sheets().iter().map(|r|r.id)).collect();
                self.select_objects(ids, false);return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) = target.filter(|t| t.part == crate::pick_sources::Part::Object) { self.objects.set(&self.scene, [target.owner], false); }
                self.selected_part = target.map(|target| target.part).filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => { if mode!=crate::pick_sources::Mode::Control {self.scene.cloud_controls(None);} self.pick_mode = mode; self.selected = None; self.selected_part = None; self.objects.clear(); }
            Action::Pick(screen) => {
                let id = self.camera.ray(screen).and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear(); self.selected = None;
            }
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddSheet=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::sheet_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original drawing sheet"))?;},
            Action::AddNestedInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::nested_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare nested original placements"))?;},
            Action::AddShapeReuse=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::cad_shape_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare congruent original CAD"))?;},
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
            Action::Live(ready)=>{self.history.clear();self.scene.restore(*ready);},
            Action::Adopt {scene:ready,..}=>{self.history.edit(&mut self.scene,|scene|scene.restore(*ready));},
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                    line.width = 6.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line = session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::Controls(visible) => {
                let parent=if visible {self.selected.filter(|id|self.scene.clouds().iter().any(|r|r.id==*id)||self.scene.streams().iter().any(|r|r.id==*id))} else {None};
                if parent.is_some() && self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.clouds().iter().any(|r|Some(r.id)==parent && r.controls_visible)||self.scene.streams().iter().any(|r|Some(r.id)==parent&&r.controls_visible)) {return Ok(Change::View);}
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {self.pick_mode=crate::pick_sources::Mode::Control;self.objects.clear();self.selected_part=None;}
                else if !visible && self.pick_mode==crate::pick_sources::Mode::Control {self.pick_mode=crate::pick_sources::Mode::Object;self.selected_part=None;self.select_objects(self.selected,false);}
            },
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::normal_example::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddColours(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(false, 2, &[session_rust::Point::new(-0.6, -0.4, 0.0),
                    session_rust::Point::new(0.0, 0.6, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                curve.width = 5.0; curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared = crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0; point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared = crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_point(prepared).map(|_| ()))?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared = crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_line(prepared).map(|_| ()))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => return self.move_selection(offset),
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => { let id = self.scene.next(self.selected); self.select_objects(id, false); }
            Action::Delete => return self.delete_selection(),
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Grid(settings) => {
                        if let Some(grid) = settings { grid.half()?; grid.reach(self.camera.target)?; }
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::ZoomAt {factor,screen}=>self.camera.zoom_at(factor,screen),
                    Action::PanScreen {delta,size}=>self.camera.pan_screen(delta,size),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(||self.display_scene().bounds()) {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(|| self.selected_part.and(self.selected).and_then(|id| self.scene.selected_bounds(id)))
                        {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::ResetView => {
                        let aspect = self.camera.aspect;
                        self.camera = Camera { aspect, grid: self.camera.grid,frame:self.camera.frame, ..Camera::default() };
                    }
                    _ => unreachable!("Document actions were handled above"),
                }
                return Ok(Change::View);
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.repair_selection();
        self.selected = self.selected.filter(|id| self.scene.contains(*id));
        if let (Some(owner), Some(part)) = (self.selected, self.selected_part) {
            if !(crate::pick_sources::Target { owner, part }).valid(&self.scene) { self.selected = None; }
        }
        if self.selected.is_none() { self.selected_part = None; }
        Ok(Change::Scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new("test:first".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), first)).unwrap();
        let origin = editor.scene.objects()[0].origin().unwrap().id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new("test:second".into()));
        editor.apply(Action::ImportAt(bytes, second)).unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(saved.objects()[0].origin().unwrap().id, origin);
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some()); assert!(copy.upgrade().is_none());
        assert!(editor.scene.objects()[0].geometry().is_some());
        assert!(editor.scene.objects()[3].geometry().is_none());
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err()); assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:epoch".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(editor.apply(Action::UnloadSources), Err("Release epochs exhausted"));
        assert!(editor.scene.objects()[0].geometry().is_some());
        editor.last_release = 0; editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(2));
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
        editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap(); editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform(); let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view); assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(crate::memory::cpu(editor.scenes(), std::iter::empty()), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(&row.mesh))).collect();
        let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(row.geometry().unwrap()))).collect();
        let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document)))).collect();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
        assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
        assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(editor.scene.objects().iter().all(|row| !old.contains(&row.id)));
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
        let mut editor = Editor::default();
        editor.camera.aspect = 2.0;
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::ResetView).unwrap();
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
    }

    #[test]
    fn undo_repairs_selection_and_empty_delete_preserves_redo() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        for _ in 0..3 {
            editor.apply(Action::SelectNext).unwrap();
        }
        let box_id = editor.selected.unwrap();
        editor.apply(Action::Undo).unwrap();
        assert!(editor.selected.is_none());
        editor.apply(Action::Delete).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(box_id));
    }

    #[test]
    fn picking_a_moved_view_uses_the_editors_current_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        let point = session_rust::Point::new(0.2, 0.3, 0.75);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert_eq!(editor.apply(Action::Pick([screen[0] as f32, screen[1] as f32])).unwrap(), Change::Scene);
        assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
        assert_eq!(editor.apply(Action::Zoom(2.0)).unwrap(), Change::View);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77g-carry-transformation-state-through-the-existing-drawing-owner-04.rs"
```

```text
--8<-- "journey/code/77g-carry-transformation-state-through-the-existing-drawing-owner-05.rs"
```

```text
--8<-- "journey/code/77g-carry-transformation-state-through-the-existing-drawing-owner-06.rs"
```

```text
--8<-- "journey/code/77g-carry-transformation-state-through-the-existing-drawing-owner-07.rs"
```

```text
--8<-- "journey/code/77g-carry-transformation-state-through-the-existing-drawing-owner-08.rs"
```

</details>

</details>

<a id="checkpoint-77h-validate-selected-originals-before-beginning-or-committing-a-transform"></a>

### Step 27 of 49 · Validate selected originals before beginning or committing a transform (`77h-validate-selected-originals-before-beginning-or-committing-a-transform`)

- **Goal:** Validate selected originals before beginning or committing a transform.
- **Data Flow Trace:** `selected original targets → checked preview → current tool or one commit`
- **Files You Touch:** `src/editor.rs` (*Connect checked original transformations to the current Editor transaction boundary.*)

Freeze the selected targets and prepare their original preview before replacing any active input owner.

> **Self-Check Question:** *Can Copy on a control point replace a valid unfinished tool?*  
> **Answer:** No. Copy requires whole objects and validates them before adopting its owner.

- **Hands-On Lab Experiment:** Try Copy on a curve control, an unavailable source and a locked row. Compare direct source-part Move with whole-object placement.

<details>
<summary>Implementation 77h-validate-selected-originals-before-beginning-or-committing-a-transform: Validate selected originals before beginning or committing a transform</summary>

Validate selected originals before beginning or committing a transform.

Data flow: selected original targets → checked preview → current tool or one commit.

### 1. `src/editor.rs`

Connect checked original transformations to the current Editor transaction boundary.

<details>
<summary>Locate the existing block</summary>

```rust
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77h-validate-selected-originals-before-beginning-or-committing-a-transform-01.rs"
```

</details>

<a id="checkpoint-77i-adopt-validated-outcomes-and-preserve-repeat-copy-transactions"></a>

### Step 28 of 49 · Adopt validated outcomes and preserve repeat copy transactions (`77i-adopt-validated-outcomes-and-preserve-repeat-copy-transactions`)

- **Goal:** Adopt validated outcomes and preserve repeat copy transactions.
- **Data Flow Trace:** `complete candidate batch → checked temporary scene → individual Copy transactions`
- **Files You Touch:** `src/editor.rs` (*Connect checked original transformations to the current Editor transaction boundary.*)

Preflight the full typed batch against a temporary scene, then record each Copy target as one transaction.

> **Self-Check Question:** *Does Escape undo completed Copy targets?*  
> **Answer:** No. It ends the unfinished tool; Undo removes each completed target copy in reverse order.

- **Hands-On Lab Experiment:** Enter a Copy base and two targets, finish, then Undo twice and Redo twice. Cancel another unfinished target and verify Redo remains.

<details>
<summary>Implementation 77i-adopt-validated-outcomes-and-preserve-repeat-copy-transactions: Adopt validated outcomes and preserve repeat copy transactions</summary>

Adopt validated outcomes and preserve repeat copy transactions.

Data flow: complete candidate batch → checked temporary scene → individual Copy transactions.

### 1. `src/editor.rs`

Connect checked original transformations to the current Editor transaction boundary.

<details>
<summary>Locate the existing block</summary>

```rust
        let mut draft=crate::tool::Draft::new(crate::tool::Shape::Transform(kind));draft.frame=frame;let transform=draft.transform.as_mut().unwrap();transform.frame=frame;transform.owners=owners;transform.target=target;
        self.edit_preview=None;self.transform_preview=preview;self.draft=Some(draft);self.snap_index=None;self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::Preview)
    }
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77i-adopt-validated-outcomes-and-preserve-repeat-copy-transactions-01.rs"
```

</details>

<a id="checkpoint-77j-route-transform-input-cancellation-and-current-plane-changes"></a>

### Step 29 of 49 · Route transform input cancellation and current plane changes (`77j-route-transform-input-cancellation-and-current-plane-changes`)

- **Goal:** Route transform input cancellation and current plane changes.
- **Data Flow Trace:** `owned event or typed input → current transform question → preview or cancellation`
- **Files You Touch:** `src/editor.rs` (*Connect current transformation behavior to the existing input and history owner.*)

Connect clicked and typed answers, current-plane changes and cancellation while keeping Polyline Close separate from transformation completion.

> **Self-Check Question:** *Can a transform click close a Polyline collector?*  
> **Answer:** No. Only the open point collector owns start-point closure. Transform questions determine their own completion.

- **Hands-On Lab Experiment:** Hover Scale and Rotate, change CPlane, type an invalid answer, cancel, and verify saved originals remain unchanged.

<details>
<summary>Implementation 77j-route-transform-input-cancellation-and-current-plane-changes: Route transform input cancellation and current plane changes</summary>

Route transform input cancellation and current plane changes.

Data flow: owned event or typed input → current transform question → preview or cancellation.

### 1. `src/editor.rs`

Connect current transformation behavior to the existing input and history owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            let viewport = crate::snap_math::Screen { matrix: self.camera.view_projection().m, size };
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        let own=self.draft.as_ref().filter(|d|d.primitive.is_none() && d.tool.count()==0 && d.points.len()>=2 && self.snapping.objects && self.snapping.modes&crate::snap::END!=0).and_then(|d|{
            let point=*d.points.first()?;let viewport=crate::snap_math::Screen {matrix:self.camera.view_projection().m,size};let p=viewport.point(point)?;
            let cursor=[(screen[0] as f64+1.)*size[0]/2.,(1.-screen[1] as f64)*size[1]/2.];
            ((p[0]-cursor[0]).hypot(p[1]-cursor[1])<=12.).then_some(crate::snap::Hit {point,kind:crate::snap::Kind::End,owner:None})
```

</details>

Replace it with:

```rust
            let viewport = crate::snap_math::Screen { matrix: self.camera.view_projection().m, size };
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        let own=self.draft.as_ref().filter(|d|d.primitive.is_none() && d.transform.is_none() && d.tool.count()==0 && d.points.len()>=2 && self.snapping.objects && self.snapping.modes&crate::snap::END!=0).and_then(|d|{
            let point=*d.points.first()?;let viewport=crate::snap_math::Screen {matrix:self.camera.view_projection().m,size};let p=viewport.point(point)?;
            let cursor=[(screen[0] as f64+1.)*size[0]/2.,(1.-screen[1] as f64)*size[1]/2.];
            ((p[0]-cursor[0]).hypot(p[1]-cursor[1])<=12.).then_some(crate::snap::Hit {point,kind:crate::snap::Kind::End,owner:None})
```

<details>
<summary>Locate this block</summary>

```rust
        let outcome=next.text(text,draft.cursor,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.transform_outcome(next,outcome)
    }
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if self.draft.as_ref().is_some_and(|d|d.primitive.is_some() && d.plane_points.is_none()) {return self.primitive_text(text);}
        let draft=self.draft.as_ref().ok_or("No point tool is running")?;
        let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
```

</details>

Replace it with:

```rust
        let outcome=next.text(text,draft.cursor,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.transform_outcome(next,outcome)
    }
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_text(text);}

        if self.draft.as_ref().is_some_and(|d|d.primitive.is_some() && d.plane_points.is_none()) {return self.primitive_text(text);}
        let draft=self.draft.as_ref().ok_or("No point tool is running")?;
        let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
```

<details>
<summary>Locate this block</summary>

```rust
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        let draft=self.draft.as_mut().ok_or("No point tool is running")?;
        if let Some(points)=draft.plane_points.as_mut() {
            let point=crate::tool_coords::checked(point)?;let mut picked=points.clone();picked.push(point);
```

</details>

Replace it with:

```rust
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_point(point);}

        let draft=self.draft.as_mut().ok_or("No point tool is running")?;
        if let Some(points)=draft.plane_points.as_mut() {
            let point=crate::tool_coords::checked(point)?;let mut picked=points.clone();picked.push(point);
```

<details>
<summary>Locate this block</summary>

```rust
            if text.trim().is_empty() || primitive.shape.option_in(&words).is_some() || words.first().is_some_and(|word|word.parse::<f64>().is_ok() || crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@')) {return Some(Ok(Action::ToolText(text.to_owned())));}
            return None;
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count()==0 {return Some(Ok(Action::CloseTool));}
        if text.trim().is_empty() {return Some(Ok(if draft.tool.count()==0 && !self.plane_only {Action::FinishTool}else{Action::CancelTool}));}
        let word=text.trim();
```

</details>

Replace it with:

```rust
            if text.trim().is_empty() || primitive.shape.option_in(&words).is_some() || words.first().is_some_and(|word|word.parse::<f64>().is_ok() || crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@')) {return Some(Ok(Action::ToolText(text.to_owned())));}
            return None;
        }
        if draft.plane_points.is_none() && draft.transform.is_some(){
            let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.contains('<')||word.starts_with('@')||word.starts_with('w')||word.starts_with('W')||["1D","2D","3D"].iter().any(|mode|word.eq_ignore_ascii_case(mode))){return Some(Ok(Action::ToolText(text.into())))}return None;
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count()==0 {return Some(Ok(Action::CloseTool));}
        if text.trim().is_empty() {return Some(Ok(if draft.tool.count()==0 && !self.plane_only {Action::FinishTool}else{Action::CancelTool}));}
        let word=text.trim();
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    StreamRefresh,
    PrimitiveBegin {shape:&'static crate::primitive_shape::Shape,option:usize,text:String},
    ToolSides(usize),
    ConstructionPlane(Option<crate::cplane::CPlane>), PlaneMenu, Plane3,
    Draw {shape:crate::tool::Shape,text:String}, ToolText(String), FinishTool, CloseTool,
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
    CanvasRelease {screen:[f32;2],size:[f64;2],add:bool,remove:bool},
    CanvasHit(Option<ObjectId>),CanvasCancel,
    SelectRows {ids:Vec<ObjectId>,add:bool,remove:bool},
    PanScreen {delta:[f64;2],size:[f64;2]},ZoomAt {factor:f32,screen:[f32;2]},
    Touch(Vec<Action>),
    LayerEdit {key:String,edit:crate::layer_edit::Edit},
    LayerProperty {key:String,property:crate::layer_properties::Property},
    TreePick {ids:Vec<ObjectId>,additive:bool},
    GizmoPress {pointer:i32,screen:[f32;2],size:[f64;2],modified:bool},
    GizmoTouch {pointer:i32,screen:[f32;2],size:[f64;2]},
    GizmoMove {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoRelease {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen { screen: [f32; 2], size: [f64; 2], commit: bool },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany { target: Option<crate::pick_sources::Target>, additive: bool },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Live(Box<Scene>),
    Adopt {scene:Box<Scene>,replace:bool},
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    ReplaceAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    Close,
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddCloudLod,
    AddCloud,
    AddCad,
    AddInstances,
    AddShapeReuse,
    AddNestedInstances,
    AddSheet,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
    AddPoint,
    AddCurve,
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
    Preview,
}

pub struct Editor {
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
    gizmo_hover:Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub transform_preview:Option<crate::gizmo_edit::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub construction_plane:Option<crate::cplane::CPlane>,
    pub plane_menu:bool,
    pub plane_only:bool,
    pub tool_status: String,
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
    pub pick_mode: crate::pick_sources::Mode,
    objects: crate::object_selection::Objects,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.transform_preview.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|p|p.scene()),|g|g.preview.scene()),|p|&p.scene)
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if self.scene.clouds().iter().any(|r|r.controls_visible)||self.scene.streams().iter().any(|r|r.controls_visible) {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
    }

    pub fn gizmo_hit(&self,screen:[f32;2],size:[f64;2])->Option<crate::gizmo::Handle> {self.gizmo_hit_radius(screen,size,8.)}

    pub fn gizmo_hit_radius(&self,screen:[f32;2],size:[f64;2],radius:f64)->Option<crate::gizmo::Handle> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
        let origin=self.gizmo_origin()?;
        let scale=crate::widget_layout::world_per_pixel(&self.camera.view_projection(),origin,size[1])?;
        let ray=self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2])).hit_with_radius(&ray.origin,&ray.direction,scale,radius)
    }

    pub fn number_prompt(&self,size:[f64;2])->Option<crate::number_box::NumberPrompt> {
        let edit=self.gizmo_edit.as_ref()?;let handle=edit.gizmo.typing?;
        let origin=edit.origin();let matrix=self.camera.view_projection();let scale=crate::widget_layout::world_per_pixel(&matrix,origin,size[1])?;
        let point=edit.gizmo.handle_point(handle,scale);
        let at=crate::snap_math::Screen {matrix:matrix.m,size}.point([point[0],point[1],point[2]])?;
        let unit=handle.labels().2;
        Some(crate::number_box::NumberPrompt {handle,title:handle.title(),unit,hint:if unit=="factor" {"1"} else {"0"},at:at.map(|v|v as f32)})
    }

    pub fn gizmo_active(&self)->f32 {crate::gizmo_edit::code(self.gizmo_edit.as_ref().and_then(|e|e.active()).or(self.gizmo_hover))}

    pub fn hover_gizmo(&mut self,screen:Option<([f32;2],[f64;2])>) {
        self.gizmo_hover=if self.gizmo_edit.is_some() {None} else {screen.and_then(|(point,size)|self.gizmo_hit(point,size))};
    }

    fn gizmo_press(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool)->Result<Change,&'static str> {
        self.gizmo_press_radius(pointer,screen,size,modified,8.)
    }

    fn gizmo_press_radius(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool,radius:f64)->Result<Change,&'static str> {
        if modified || self.draft.is_some() || self.gizmo_edit.as_ref().is_some_and(|g|g.pointer.is_some()) {return Ok(Change::View);}
        let Some(handle)=self.gizmo_hit_radius(screen,size,radius) else {return Ok(Change::View);};
        let origin=self.gizmo_origin().ok_or("No editable selection")?;
        let preview=crate::gizmo_edit::Preview::new(&self.scene,self.selected_objects(),self.selection().filter(|_|self.selected_part.is_some()))?;
        let ray=self.camera.ray(screen).ok_or("Invalid handle ray")?;
        let gizmo=crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2]));
        self.gizmo_edit=crate::gizmo_edit::Edit::begin(gizmo,handle,&ray,preview,pointer,screen);Ok(Change::View)
    }

    fn gizmo_move(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return Err("Invalid handle viewport");}
        let Some(ray)=self.camera.ray(screen) else {return Err("Invalid handle ray");};
        let Some(edit)=self.gizmo_edit.as_mut() else {return Ok(Change::View);};
        Ok(if edit.update(pointer,screen,size,&ray,shift)? {Change::Preview} else {Change::View})
    }

    fn gizmo_release(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if self.gizmo_edit.as_ref().is_none_or(|g|g.pointer!=Some(pointer)) {return Ok(Change::View);}
        self.gizmo_move(pointer,screen,size,shift)?;
        let edit=self.gizmo_edit.as_mut().unwrap();
        if edit.click() {self.tool_status=format!("{}: type a value in {}, Enter applies, Esc closes",edit.active().unwrap().title(),edit.active().unwrap().labels().2);return Ok(Change::Preview);}
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?; self.gizmo_edit=None;
        self.tool_status="Handle edit completed.".into();Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn gizmo_number(&mut self,text:&str)->Result<Change,&'static str> {
        let edit=self.gizmo_edit.as_mut().ok_or("No handle number is being requested")?;
        let handle=edit.gizmo.typing.ok_or("No handle number is being requested")?;
        let value=crate::gizmo::typed_value(handle,text).map_err(|_|"Enter a finite number; scale needs a positive factor")?;
        let Some(value)=value else {self.gizmo_edit=None;return Ok(Change::Preview);};
        edit.preview.set(edit.gizmo.typed(handle,value))?;
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?;self.gizmo_edit=None;
        self.tool_status=format!("{} {value} {}",handle.title(),handle.labels().2);Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        if let Some(transform)=self.draft.as_ref().and_then(|draft|draft.transform.as_ref()) {
            let matrix=transform.preview(self.draft.as_ref().and_then(|draft|draft.cursor)).unwrap_or_else(session_rust::Xform::identity);
            if let Some(preview)=self.transform_preview.as_mut(){preview.set(matrix)?;return Ok(Change::Preview);}return Ok(Change::View);
        }
        let Some(preview)=self.edit_preview.as_mut() else { return Ok(Change::View); };
        let offset=self.draft.as_ref().and_then(|d| d.points.first().zip(d.cursor)).map(|(a,b)| std::array::from_fn::<_,3,_>(|i| b[i]-a[i])).unwrap_or([0.;3]);
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?; Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self)->impl Iterator<Item=&Scene> {
        self.scenes().chain(self.transform_preview.iter().flat_map(|p|p.scenes())).chain(self.canvas_drag.iter().filter_map(|d|d.preview.as_ref()).flat_map(|p|p.scenes())).chain(self.edit_preview.iter().flat_map(|p|p.scenes())).chain(self.gizmo_edit.iter().flat_map(|g|g.preview.scenes()))
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene.objects().iter().filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id)).collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(&mut self, values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>) -> Result<bool, &'static str> {
        if values.is_empty() { return Ok(false); }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id) || !self.scene.objects().iter().any(|row| key.matches(row)) {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values.into_iter().map(|(key, bytes)|
            crate::rehydrate::PreparedReload::new(key, &bytes)).collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self.scenes().flat_map(|scene| scene.objects()).filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row)) { return Err("Reloaded source GUID or metadata does not match"); }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared { scene.hydrate(plan); }
        }
        if self.edit_preview.take().is_some() { self.draft=None; } self.gizmo_edit=None; self.snap_index = None; Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self.scenes().flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload()).filter_map(|row| row.origin().map(|origin| origin.id)).collect();
        for row in self.scenes().flat_map(|scene| scene.objects()) {
            if let Some(origin) = row.origin() {
                if !row.can_unload() { origins.remove(&origin.id); }
            }
        }
        if origins.is_empty() { return Err("No eligible imported sources to unload"); }
        let epoch = self.last_release.checked_add(1).ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() { scene.unload(&origins, epoch); }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        if !self.scene.contains(id) {return Err("Object not found");}
        let object=self.scene.objects().iter().find(|row|row.id==id);
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if object.is_some_and(|object|object.geometry().is_none()) {return Err("Reload editable sources before Move");}
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let ids=self.scene.with_descendants(&[id]);let delta=session_rust::Xform::translation(offset[0],offset[1],offset[2]);
        self.history.try_edit(&mut self.scene,|scene| {for id in ids {let matrix=&delta*&scene.placement(id).ok_or("Object not found")?;scene.place_following(id,matrix)?;}Ok(())})?;
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {return Err("Object not found");}
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if self.scene.objects().iter().find(|row| row.id == id).is_some_and(|row| row.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {for id in ids {scene.remove(id);}});
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) { self.selected = self.objects.primary(); self.selected_part = None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() { return Vec::new(); }
        if self.objects.primary() == self.selected { self.objects.rows().to_vec() }
        else { self.selected.into_iter().filter(|id| self.scene.pick_id(id.value()).is_some()).collect() }
    }

    pub fn select_objects(&mut self, rows: impl IntoIterator<Item = ObjectId>, additive: bool) {
        if additive { let old = self.selected_objects(); self.objects.set(&self.scene, old, false); }
        self.selected_part = None; self.objects.set(&self.scene, rows, additive); self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self,ids:Vec<ObjectId>,add:bool,remove:bool) {
        if remove {let old=self.selected_objects();self.objects.set(&self.scene,old,false);self.objects.remove(&self.scene,ids);self.selected=self.objects.primary();self.selected_part=None;}
        else {self.select_objects(ids,add);}
    }

    fn finish_canvas(&mut self)->Result<Change,&'static str> {
        let Some(mut drag)=self.canvas_drag.take() else {return Ok(Change::View);};
        let (add,remove)=drag.released.unwrap_or((false,false));
        if let Some(preview)=drag.preview.take() {let changed=preview.commit(&mut self.scene,&mut self.history)?;if changed {self.snap_index=None;}return Ok(if changed {Change::Scene} else {Change::Preview});}
        if drag.missed {if let Some(rectangle)=drag.rectangle() {let ids=crate::rect_select::found(&self.scene,&self.camera.view_projection().m,drag.size,rectangle);self.select_rows(ids,add,remove);}}
        Ok(Change::View)
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene); self.selected = self.objects.primary();
        } else { self.objects.clear(); }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) { return Err("Move needs finite coordinates"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Err("Select an object before Move"); }
        if offset.iter().all(|&n| n == 0.) { return Ok(Change::View); }
        let mut preview=crate::edit_preview::Preview::new(&self.scene,ids)?;
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
        preview.commit(&mut self.scene,&mut self.history)?;Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() { return Err("Use Select Object before Delete"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Ok(Change::View); }
        if self.scene.objects().iter().any(|r| ids.contains(&r.id) && r.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| { for id in ids { scene.remove(id); } });
        self.objects.clear(); self.selected = None; Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected.map(|owner| crate::pick_sources::Target { owner, part: self.selected_part.unwrap_or(crate::pick_sources::Part::Object) })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> { self.selected.filter(|_| self.selected_part.is_none()) }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")) {
            return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape") {Action::GizmoCancel} else {Action::GizmoNumber(text.to_owned())}));
        }
        if self.plane_menu {return Some(crate::drawing_input::choice(text).ok_or("Choose XY, XZ, YZ, 3 Point, View or Cancel"));}
        let draft=self.draft.as_ref()?;
        if let Some(choice)=crate::drawing_input::choice(text) {return Some(Ok(choice));}
        if draft.plane_points.is_none() && draft.primitive.is_some() {
            let words:Vec<_>=text.split_whitespace().collect();let primitive=draft.primitive.as_ref().unwrap();
            if text.trim().is_empty() || primitive.shape.option_in(&words).is_some() || words.first().is_some_and(|word|word.parse::<f64>().is_ok() || crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@')) {return Some(Ok(Action::ToolText(text.to_owned())));}
            return None;
        }
        if draft.plane_points.is_none() && draft.transform.is_some(){
            let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.contains('<')||word.starts_with('@')||word.starts_with('w')||word.starts_with('W')||["1D","2D","3D"].iter().any(|mode|word.eq_ignore_ascii_case(mode))){return Some(Ok(Action::ToolText(text.into())))}return None;
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count()==0 {return Some(Ok(Action::CloseTool));}
        if text.trim().is_empty() {return Some(Ok(if draft.tool.count()==0 && !self.plane_only {Action::FinishTool}else{Action::CancelTool}));}
        let word=text.trim();
        if word.split_whitespace().next().is_some_and(|s|s.eq_ignore_ascii_case("Sides")) && draft.tool.sides().is_some() {
            let parts:Vec<_>=word.split_whitespace().collect();
            return Some(match parts.as_slice() {[_name,value]=>value.parse::<usize>().ok().filter(|n|(3..crate::tool::MAX_POINTS).contains(n)).map(Action::ToolSides).ok_or("Polygon sides must be between 3 and 4095"),_=>Err("Use Sides 6")});
        }
        let coordinate=crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@') || word.starts_with('w') || word.starts_with('W') || word.split_whitespace().next().is_some_and(|p|crate::drawing_coords::parse_world(p).is_some());
        coordinate.then(|| {
            let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
            crate::drawing_input::points(word,self.construction_plane,draft.frame,points,draft.cursor).map(|_|Action::ToolText(word.to_owned()))
        })
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_point(point);}

        let draft=self.draft.as_mut().ok_or("No point tool is running")?;
        if let Some(points)=draft.plane_points.as_mut() {
            let point=crate::tool_coords::checked(point)?;let mut picked=points.clone();picked.push(point);
            if picked.len()==3 {
                let p:Vec<_>=picked.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect();
                let plane=crate::cplane::CPlane::from_3_points(&p[0],&p[1],&p[2]).ok_or("The three points are in line; pick the third again")?;
                draft.plane_points=None;self.apply(Action::ConstructionPlane(Some(plane)))?;
                if self.plane_only {self.draft=None;self.plane_only=false;}return Ok(Change::View);
            }
            *points=picked;self.tool_status=draft.prompt();return Ok(Change::View);
        }
        if draft.primitive.is_some() {return self.primitive_answer(session_rust::Point::new(point[0],point[1],point[2]));}
        crate::drawing_coords::set_plane(self.construction_plane);
        let Some(built)=draft.submit(point)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    fn commit_tool(&mut self,built:crate::tool::Built) -> Result<Change,&'static str> {
        let name=self.draft.as_ref().ok_or("No point tool is running")?.tool.name();
        if let crate::tool::Built::Move(offset)=built {
            let preview=self.edit_preview.as_mut().ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
            let changed=preview.commit(&mut self.scene,&mut self.history)?;
            self.edit_preview=None;self.draft=None;self.tool_status="Move completed.".into();return Ok(if changed {Change::Scene}else{Change::Preview});
        }
        self.history.try_edit(&mut self.scene,|scene|match built {
            crate::tool::Built::Point(point)=>scene.insert_point(point).map(|_|()),
            crate::tool::Built::Line(line)=>scene.insert_line(line).map(|_|()),
            crate::tool::Built::Polyline(line)=>scene.insert_path(line).map(|_|()),
            crate::tool::Built::Geometry(geometry)=>crate::primitive_geometry::insert(scene,geometry).map(|_|()),
            crate::tool::Built::Move(_)=>unreachable!("Move uses its placement preview"),
        })?;
        self.draft=None;self.tool_status=format!("{name} created.");Ok(Change::Scene)
    }

    fn primitive_failure(&mut self,message:String)->&'static str {self.tool_status=message;"Shape input invalid"}
    fn primitive_answer(&mut self,point:session_rust::Point)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.accept(crate::primitive_shape::Answer::Point(point.clone()));
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points.push([point[0],point[1],point[2]]);draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn primitive_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.text(text,draft.cursor,self.construction_plane);
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn apply_transform(&mut self,kind:crate::transform_draft::Kind,delta:&session_rust::Xform,ids:&[ObjectId],target:Option<crate::pick_sources::Target>)->Result<Change,&'static str> {
        if !crate::placement::valid(&delta.m){return Err("Transform must be finite, affine and invertible");}
        if kind==crate::transform_draft::Kind::Copy {
            if target.is_some(){return Err("Copy works on whole objects");}
            self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::copy_objects(scene,ids,delta).map(|_|()))?;return Ok(Change::Scene);
        }
        let mut preview=crate::gizmo_edit::Preview::new(&self.scene,ids.to_vec(),target)?;preview.set(delta.clone())?;let changed=preview.commit(&mut self.scene,&mut self.history)?;Ok(if changed{Change::Scene}else{Change::Preview})
    }
    fn begin_transform(&mut self,kind:crate::transform_draft::Kind)->Result<Change,&'static str> {
        let owners=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());
        let preview=if kind==crate::transform_draft::Kind::Copy {if target.is_some(){return Err("Copy works on whole objects");}let mut candidate=self.scene.clone();crate::layer_edit::copy_objects(&mut candidate,&owners,&session_rust::Xform::identity())?;None}else{Some(crate::gizmo_edit::Preview::new(&self.scene,owners.clone(),target)?)};
        let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
        let mut draft=crate::tool::Draft::new(crate::tool::Shape::Transform(kind));draft.frame=frame;let transform=draft.transform.as_mut().unwrap();transform.frame=frame;transform.owners=owners;transform.target=target;
        self.edit_preview=None;self.transform_preview=preview;self.draft=Some(draft);self.snap_index=None;self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::Preview)
    }
    fn transform_outcome(&mut self,next:crate::transform_draft::Draft,outcome:crate::transform_draft::Outcome)->Result<Change,&'static str> {
        // Validate the entire typed batch against a temporary scene before recording its separate Copy target transactions.
        let mut candidate=self.scene.clone();
        for delta in &outcome.transforms {if next.kind==crate::transform_draft::Kind::Copy {crate::layer_edit::copy_objects(&mut candidate,&next.owners,delta)?;}else{let mut preview=crate::gizmo_edit::Preview::new(&candidate,next.owners.clone(),next.target)?;preview.set(delta.clone())?;let mut history=crate::history::History::default();preview.commit(&mut candidate,&mut history)?;}}
        for delta in &outcome.transforms {self.apply_transform(next.kind,delta,&next.owners,next.target)?;}
        let changed=!outcome.transforms.is_empty();
        if outcome.finished {self.draft=None;self.transform_preview=None;self.tool_status=format!("{} completed.",next.kind.name());return Ok(if changed{Change::Scene}else{Change::Preview});}
        let draft=self.draft.as_mut().ok_or("No transform tool is running")?;draft.points=next.points.clone();draft.transform=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();
        if changed {self.snap_index=None;return Ok(Change::Scene);}self.update_edit_preview()
    }
    fn transform_point(&mut self,point:[f64;3])->Result<Change,&'static str> {
        let mut next=self.draft.as_ref().and_then(|draft|draft.transform.clone()).ok_or("No transform tool is running")?;
        let transform=next.accept(crate::primitive_shape::Answer::Point(session_rust::Point::new(point[0],point[1],point[2]))).map_err(|message|self.primitive_failure(message))?;
        let finished=transform.is_some()&&next.kind!=crate::transform_draft::Kind::Copy;self.transform_outcome(next,crate::transform_draft::Outcome{transforms:transform.into_iter().collect(),finished})
    }
    fn transform_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_ref().ok_or("No transform tool is running")?;let mut next=draft.transform.clone().ok_or("No transform tool is running")?;
        let outcome=next.text(text,draft.cursor,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.transform_outcome(next,outcome)
    }
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_text(text);}

        if self.draft.as_ref().is_some_and(|d|d.primitive.is_some() && d.plane_points.is_none()) {return self.primitive_text(text);}
        let draft=self.draft.as_ref().ok_or("No point tool is running")?;
        let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
        let added=crate::drawing_input::points(text,self.construction_plane,draft.frame,points,draft.cursor)?;
        if draft.plane_points.is_some() {
            if added.len()!=1 {return Err("Supply one construction-plane point at a time");}return self.place_tool_point(added[0]);
        }
        let draft=self.draft.as_mut().unwrap();
        let Some(built)=draft.submit_many(&added)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    pub fn snap_point(&mut self, screen: [f32; 2], size: [f64; 2]) -> Option<crate::snap::Hit> {
        if let Some(primitive)=self.draft.as_ref()?.primitive.as_ref() {if primitive.next().is_some_and(|ask|ask.kind==crate::primitive_shape::Kind::Height) {return primitive.on_axis(&self.camera,screen).map(|point|crate::snap::Hit {point,kind:crate::snap::Kind::Near,owner:None});}}
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude=self.edit_preview.as_ref().filter(|_| from.is_some()).map_or_else(Vec::new,|p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| crate::snap_index::Index::new(&self.scene).excluding(&exclude));
            let viewport = crate::snap_math::Screen { matrix: self.camera.view_projection().m, size };
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        let own=self.draft.as_ref().filter(|d|d.primitive.is_none() && d.transform.is_none() && d.tool.count()==0 && d.points.len()>=2 && self.snapping.objects && self.snapping.modes&crate::snap::END!=0).and_then(|d|{
            let point=*d.points.first()?;let viewport=crate::snap_math::Screen {matrix:self.camera.view_projection().m,size};let p=viewport.point(point)?;
            let cursor=[(screen[0] as f64+1.)*size[0]/2.,(1.-screen[1] as f64)*size[1]/2.];
            ((p[0]-cursor[0]).hypot(p[1]-cursor[1])<=12.).then_some(crate::snap::Hit {point,kind:crate::snap::Kind::End,owner:None})
        });
        own.or(hit).or_else(|| {
            let draft=self.draft.as_ref()?;
            let frame=self.construction_plane.unwrap_or(draft.frame);
            let through=if self.construction_plane.is_none() {draft.points.last().copied()}else{None};
            let point=crate::drawing_input::on_at(&self.camera,screen,frame,through)?;
            let (x,y)=frame.axes();let o=frame.origin();
            let point=if self.snapping.grid {crate::snap_math::grid(point,[o[0],o[1],o[2]],[[x[0],x[1],x[2]],[y[0],y[1],y[2]]],self.snapping.step)?}else{point};
            Some(crate::snap::Hit {point,kind:if self.snapping.grid {crate::snap::Kind::Grid}else{crate::snap::Kind::Near},owner:None})
        })
    }

    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
            Action::Touch(actions)=> {let mut change=Change::View;for action in actions {let next=self.apply(action)?;if next!=Change::View {change=next;}}return Ok(change);},
            Action::CanvasStart {from,to,size,modified,pointer}=>{self.canvas_drag=crate::canvas_drag::Drag::new(from,to,size,modified);if let Some(drag)=self.canvas_drag.as_mut() {drag.pointer=pointer;}return Ok(Change::View);},
            Action::CanvasMove {screen,size}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.released.is_none()&&drag.size==size&&screen.iter().all(|v|v.is_finite()) {drag.to=screen;drag.update(&self.camera)?;}if drag.preview.is_some() {return Ok(Change::Preview);}}return Ok(Change::View);},
            Action::CanvasRelease {screen,size,add,remove}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.size!=size {self.canvas_drag=None;return Ok(Change::Preview);}drag.to=screen;drag.released=Some((add,remove));drag.update(&self.camera)?;if drag.preview.is_some()||drag.missed {return self.finish_canvas();}}return Ok(Change::View);},
            Action::CanvasHit(hit)=>{
                let Some(drag)=self.canvas_drag.as_ref() else {return Ok(Change::View);};let ids=hit.filter(|id|!drag.modified&&self.scene.pick_id(id.value()).is_some());
                if let Some(id)=ids {let mut rows=self.selected_objects();if !rows.contains(&id) {self.select_objects([id],false);rows=vec![id];}if let Err(error)=self.canvas_drag.as_mut().unwrap().grab(&self.scene,rows,&self.camera) {self.canvas_drag=None;return Err(error);}}
                else {self.canvas_drag.as_mut().unwrap().missed=true;}
                if self.canvas_drag.as_ref().unwrap().released.is_some() {return self.finish_canvas();}return Ok(Change::Preview);
            },
            Action::CanvasCancel=>{let preview=self.canvas_drag.take().is_some_and(|d|d.preview.is_some());return Ok(if preview {Change::Preview} else {Change::View});},
            Action::SelectRows {ids,add,remove}=>{self.select_rows(ids,add,remove);return Ok(Change::View);},
            _=>{}
        }
        self.canvas_drag=None;
        let keep_gizmo=matches!(&action,Action::GizmoPress {..}|Action::GizmoTouch {..}|Action::GizmoMove {..}|Action::GizmoRelease {..}|Action::GizmoCancel|Action::GizmoNumber(_)|Action::Zoom(_)|Action::ZoomAt {..}|Action::PanScreen {..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::Snap(_));
        let keep_edit=matches!(&action, Action::GizmoPress {..} | Action::GizmoTouch {..} | Action::GizmoMove {..} | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::BeginTool(_) | Action::Draw {..} | Action::ToolText(_) | Action::FinishTool | Action::CloseTool | Action::ConstructionPlane(_) | Action::PlaneMenu | Action::Plane3 | Action::ToolSides(_) | Action::PrimitiveBegin {..} | Action::ToolPoint(_) | Action::ToolScreen {..} | Action::ToolHover(_) | Action::CancelTool | Action::Snap(_) | Action::Zoom(_) | Action::ZoomAt {..} | Action::PanScreen {..} | Action::Pan(_,_) | Action::Orbit(_,_) | Action::Fit | Action::FitSelected | Action::Projection(_) | Action::ResetView | Action::Background | Action::NormalView(_) | Action::Grid(_));
        let mut change=self.apply_action(action)?;
        self.gizmo_hover=None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change==Change::View {change=Change::Preview;}
        if !keep_edit && self.edit_preview.take().is_some() { self.draft=None; if change==Change::View { change=Change::Preview; } }
        if change==Change::Scene { self.snap_index=None; }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {

        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        match action {
            Action::PrimitiveBegin {shape,option,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let mut primitive=crate::primitive_draft::Draft::new(shape,frame,option).map_err(|message|self.primitive_failure(message))?;
                let geometry=if text.trim().is_empty() {None}else{primitive.text(&text,None,self.construction_plane).map_err(|message|self.primitive_failure(message))?};
                self.apply(Action::BeginTool(crate::tool::Shape::Primitive(shape)))?;
                let draft=self.draft.as_mut().unwrap();draft.primitive=Some(primitive);draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();
                if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ToolSides(sides)=>{
                let draft=self.draft.as_mut().ok_or("Sides works while drawing a polygon")?;draft.tool.set_sides(sides)?;self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ConstructionPlane(plane)=>{
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=self.draft.as_mut().and_then(|draft|draft.primitive.as_mut()) {primitive.set_plane(frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                self.construction_plane=plane;crate::drawing_coords::set_plane(plane);self.plane_menu=false;
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(draft)=self.draft.as_mut() {draft.frame=frame;draft.cursor=None;draft.snapped=None;}
                self.tool_status="Construction plane changed.".into();return Ok(Change::View);
            },
            Action::PlaneMenu=>{self.plane_menu=true;self.tool_status="CPlane · XY XZ YZ 3 Point View Cancel".into();return Ok(Change::View);},
            Action::Plane3=>{
                self.plane_menu=false;self.plane_only=self.draft.is_none();
                if self.plane_only {self.apply(Action::BeginTool(crate::tool::Shape::Point))?;}
                let draft=self.draft.as_mut().unwrap();draft.plane_points=Some(Vec::new());draft.frame=crate::cplane::CPlane::Xy;
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::Draw {shape,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let added=crate::drawing_input::points(&text,self.construction_plane,frame,&[],None)?;
                let mut draft=crate::tool::Draft::new(shape);draft.frame=frame;
                let built=draft.submit_many(&added)?;
                let built=if built.is_none() && draft.tool.count()==0 && added.len()>=2 {Some(draft.finish()?)}else{built};
                self.apply(Action::BeginTool(shape))?;self.draft=Some(draft);
                if let Some(built)=built {return self.commit_tool(built);}self.tool_status=self.draft.as_ref().unwrap().prompt();return Ok(Change::View);
            },
            Action::ToolText(text)=>return self.tool_text(&text),
            Action::CloseTool=>{
                let draft=self.draft.as_ref().ok_or("No point tool is running")?;
                if draft.tool.count()!=0 || draft.points.len()<3 {return Err("Close needs at least three polyline points");}
                if draft.points.len()>=crate::tool::MAX_POINTS {return Err("Drawing commands accept at most 4096 points");}
                let mut points=draft.points.clone();points.push(points[0]);
                crate::drawing_coords::set_plane(self.construction_plane);
                let built=draft.build_world(&points)?;return self.commit_tool(built);
            },
            Action::FinishTool=>{crate::drawing_coords::set_plane(self.construction_plane);let built=self.draft.as_ref().ok_or("No point tool is running")?.finish()?;return self.commit_tool(built);},
            Action::LayerEdit {key,edit}=>{
                if matches!(edit,crate::layer_edit::Edit::Current) {crate::layer_edit::apply(&mut self.scene,&key,edit)?;return Ok(Change::View);}
                self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::apply(scene,&key,edit))?;self.repair_selection();
            },
            Action::LayerProperty {key,property}=>{crate::layer_properties::apply(&mut self.scene,&key,property)?;self.repair_selection();if self.selected.is_some_and(|id|self.scene.pick_id(id.value()).is_none()) {self.selected=None;self.selected_part=None;}self.snap_index=None;return Ok(Change::Preview);},
            Action::Hide=>{let ids=self.scene.with_descendants(&self.selected_objects());std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,true);self.select_objects([],false);self.snap_index=None;return Ok(Change::Preview);},
            Action::Show=>{std::rc::Rc::make_mut(&mut self.scene.view).show_all();self.snap_index=None;return Ok(Change::Preview);},
            Action::TreePick {ids,additive}=>{let eligible=ids.into_iter().filter(|id|self.scene.pick_id(id.value())==Some(*id)).collect::<Vec<_>>();if eligible.is_empty() {return Err("This tree row has no visible editable objects");}self.select_objects(eligible,additive);return self.apply(Action::FitSelected);},
            Action::GizmoTouch {pointer,screen,size}=>return self.gizmo_press_radius(pointer,screen,size,false,18.),
            Action::GizmoPress {pointer,screen,size,modified}=>return self.gizmo_press(pointer,screen,size,modified),
            Action::GizmoMove {pointer,screen,size,shift}=>return self.gizmo_move(pointer,screen,size,shift),
            Action::GizmoRelease {pointer,screen,size,shift}=>return self.gizmo_release(pointer,screen,size,shift),
            Action::GizmoCancel=>{self.gizmo_edit=None;self.tool_status="Handle edit cancelled.".into();return Ok(Change::Preview);},
            Action::GizmoNumber(text)=>return self.gizmo_number(&text),
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() { draft.cursor = None; draft.snapped = None; }
                return self.update_edit_preview();
            }
            Action::ToolScreen { screen, size, commit } => {
                let hit = self.snap_point(screen,size);
                if let Some(draft) = self.draft.as_mut() { draft.snapped = hit.filter(|h| h.owner.is_some() || h.kind == crate::snap::Kind::Grid).map(|h| h.kind); }
                if commit {
                    if hit.is_some_and(|h|self.draft.as_ref().is_some_and(|d|d.primitive.is_none() && d.tool.count()==0 && d.points.len()>=2 && d.points.first().is_some_and(|p|(0..3).all(|i|(p[i]-h.point[i]).abs()<=1e-12)))) {return self.apply(Action::CloseTool);}
                    return hit.map_or(Ok(Change::View), |hit| self.place_tool_point(hit.point));
                }
                if let Some(draft) = self.draft.as_mut() { draft.hover(hit.map(|h| h.point))?; }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                let preview=if matches!(shape,crate::tool::Shape::Move) {
                    if self.selected_part.is_some() { return Err("Use Select Object before Move"); }
                    Some(crate::edit_preview::Preview::new(&self.scene,self.selected_objects())?)
                } else { None };
                let changed=preview.is_some() || self.edit_preview.is_some(); self.edit_preview=preview;
                self.snap_index=None;let mut draft=crate::tool::Draft::new(shape);
                draft.frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=draft.primitive.as_mut() {primitive.set_plane(draft.frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                self.draft=Some(draft);self.tool_status=self.draft.as_ref().unwrap().prompt();
                return Ok(if changed { Change::Preview } else { Change::View });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => { let changed=self.edit_preview.take().is_some(); self.draft=None;self.plane_menu=false;self.plane_only=false; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
                self.select_objects(target.filter(|t| t.part == crate::pick_sources::Part::Object && t.valid(&self.scene)).map(|t| t.owner), additive);
            }
            Action::SelectAll => {
                let ids: Vec<_> = self.scene.objects().iter().map(|r| r.id).chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id)).chain(self.scene.points().iter().map(|r| r.id)).chain(self.scene.texts().iter().map(|r|r.id)).chain(self.scene.clouds().iter().map(|r|r.id)).chain(self.scene.streams().iter().map(|r|r.id)).chain(self.scene.sheets().iter().map(|r|r.id)).collect();
                self.select_objects(ids, false);return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) = target.filter(|t| t.part == crate::pick_sources::Part::Object) { self.objects.set(&self.scene, [target.owner], false); }
                self.selected_part = target.map(|target| target.part).filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => { if mode!=crate::pick_sources::Mode::Control {self.scene.cloud_controls(None);} self.pick_mode = mode; self.selected = None; self.selected_part = None; self.objects.clear(); }
            Action::Pick(screen) => {
                let id = self.camera.ray(screen).and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear(); self.selected = None;
            }
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddSheet=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::sheet_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original drawing sheet"))?;},
            Action::AddNestedInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::nested_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare nested original placements"))?;},
            Action::AddShapeReuse=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::cad_shape_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare congruent original CAD"))?;},
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
            Action::Live(ready)=>{self.history.clear();self.scene.restore(*ready);},
            Action::Adopt {scene:ready,..}=>{self.history.edit(&mut self.scene,|scene|scene.restore(*ready));},
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                    line.width = 6.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line = session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::Controls(visible) => {
                let parent=if visible {self.selected.filter(|id|self.scene.clouds().iter().any(|r|r.id==*id)||self.scene.streams().iter().any(|r|r.id==*id))} else {None};
                if parent.is_some() && self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.clouds().iter().any(|r|Some(r.id)==parent && r.controls_visible)||self.scene.streams().iter().any(|r|Some(r.id)==parent&&r.controls_visible)) {return Ok(Change::View);}
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {self.pick_mode=crate::pick_sources::Mode::Control;self.objects.clear();self.selected_part=None;}
                else if !visible && self.pick_mode==crate::pick_sources::Mode::Control {self.pick_mode=crate::pick_sources::Mode::Object;self.selected_part=None;self.select_objects(self.selected,false);}
            },
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::normal_example::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddColours(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(false, 2, &[session_rust::Point::new(-0.6, -0.4, 0.0),
                    session_rust::Point::new(0.0, 0.6, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                curve.width = 5.0; curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared = crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0; point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared = crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_point(prepared).map(|_| ()))?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared = crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_line(prepared).map(|_| ()))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => return self.move_selection(offset),
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => { let id = self.scene.next(self.selected); self.select_objects(id, false); }
            Action::Delete => return self.delete_selection(),
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Grid(settings) => {
                        if let Some(grid) = settings { grid.half()?; grid.reach(self.camera.target)?; }
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::ZoomAt {factor,screen}=>self.camera.zoom_at(factor,screen),
                    Action::PanScreen {delta,size}=>self.camera.pan_screen(delta,size),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(||self.display_scene().bounds()) {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(|| self.selected_part.and(self.selected).and_then(|id| self.scene.selected_bounds(id)))
                        {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::ResetView => {
                        let aspect = self.camera.aspect;
                        self.camera = Camera { aspect, grid: self.camera.grid,frame:self.camera.frame, ..Camera::default() };
                    }
                    _ => unreachable!("Document actions were handled above"),
                }
                return Ok(Change::View);
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.repair_selection();
        self.selected = self.selected.filter(|id| self.scene.contains(*id));
        if let (Some(owner), Some(part)) = (self.selected, self.selected_part) {
            if !(crate::pick_sources::Target { owner, part }).valid(&self.scene) { self.selected = None; }
        }
        if self.selected.is_none() { self.selected_part = None; }
        Ok(Change::Scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new("test:first".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), first)).unwrap();
        let origin = editor.scene.objects()[0].origin().unwrap().id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new("test:second".into()));
        editor.apply(Action::ImportAt(bytes, second)).unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(saved.objects()[0].origin().unwrap().id, origin);
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some()); assert!(copy.upgrade().is_none());
        assert!(editor.scene.objects()[0].geometry().is_some());
        assert!(editor.scene.objects()[3].geometry().is_none());
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err()); assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:epoch".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(editor.apply(Action::UnloadSources), Err("Release epochs exhausted"));
        assert!(editor.scene.objects()[0].geometry().is_some());
        editor.last_release = 0; editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(2));
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
        editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap(); editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform(); let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view); assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(crate::memory::cpu(editor.scenes(), std::iter::empty()), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(&row.mesh))).collect();
        let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(row.geometry().unwrap()))).collect();
        let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document)))).collect();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
        assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
        assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(editor.scene.objects().iter().all(|row| !old.contains(&row.id)));
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
        let mut editor = Editor::default();
        editor.camera.aspect = 2.0;
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::ResetView).unwrap();
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
    }

    #[test]
    fn undo_repairs_selection_and_empty_delete_preserves_redo() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        for _ in 0..3 {
            editor.apply(Action::SelectNext).unwrap();
        }
        let box_id = editor.selected.unwrap();
        editor.apply(Action::Undo).unwrap();
        assert!(editor.selected.is_none());
        editor.apply(Action::Delete).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(box_id));
    }

    #[test]
    fn picking_a_moved_view_uses_the_editors_current_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        let point = session_rust::Point::new(0.2, 0.3, 0.75);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert_eq!(editor.apply(Action::Pick([screen[0] as f32, screen[1] as f32])).unwrap(), Change::Scene);
        assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
        assert_eq!(editor.apply(Action::Zoom(2.0)).unwrap(), Change::View);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77j-route-transform-input-cancellation-and-current-plane-changes-01.rs"
```

```text
--8<-- "journey/code/77j-route-transform-input-cancellation-and-current-plane-changes-02.rs"
```

```text
--8<-- "journey/code/77j-route-transform-input-cancellation-and-current-plane-changes-03.rs"
```

```text
--8<-- "journey/code/77j-route-transform-input-cancellation-and-current-plane-changes-04.rs"
```

</details>

</details>

<a id="checkpoint-77k-connect-transformation-lifecycle-and-part-move-dispatch"></a>

### Step 30 of 49 · Connect transformation lifecycle and part Move dispatch (`77k-connect-transformation-lifecycle-and-part-move-dispatch`)

- **Goal:** Connect transformation lifecycle and part Move dispatch.
- **Data Flow Trace:** `transform lifecycle action → current plane and selected part → checked original preview`
- **Files You Touch:** `src/editor.rs` (*Register transform lifecycle actions before connecting current input dispatch.*) · `src/selection_tests.rs` (*Keep invalid Move and parent-deletion guards when source-part Move becomes available.*) · `src/edit_preview_tests.rs` (*Allow part Move previews and cancellation while retaining locked, cold and stale target guards.*)

Connect current plane updates and successful cancellation while routing part Move through its original source preview.

> **Self-Check Question:** *Does changing CPlane rewrite the accepted world points?*  
> **Answer:** No. Keep accepted points in the world and use the new plane for subsequent reference measurements.

- **Hands-On Lab Experiment:** Begin a part Move, accept a base point, change CPlane, preview a target and cancel. Compare saved source and placement before and after.

<details>
<summary>Implementation 77k-connect-transformation-lifecycle-and-part-move-dispatch: Connect transformation lifecycle and part Move dispatch</summary>

Connect transformation lifecycle and part Move dispatch.

Data flow: transform lifecycle action → current plane and selected part → checked original preview.

### 1. `src/editor.rs`

Register transform lifecycle actions before connecting current input dispatch. Connect current transformation behavior to the existing input and history owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => return self.move_selection(offset),
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => { let id = self.scene.next(self.selected); self.select_objects(id, false); }
            Action::Delete => return self.delete_selection(),
```

</details>

Replace it with:

```rust
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => {
                if self.selected_part.is_some(){let target=self.selection();return self.apply_transform(crate::transform_draft::Kind::Move,&session_rust::Xform::translation(offset[0],offset[1],offset[2]),&[],target);}
                return self.move_selection(offset);
            },
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => { let id = self.scene.next(self.selected); self.select_objects(id, false); }
            Action::Delete => return self.delete_selection(),
```

<details>
<summary>Locate this block</summary>

```rust
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => { let changed=self.edit_preview.take().is_some(); self.draft=None;self.plane_menu=false;self.plane_only=false; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
```

</details>

Replace it with:

```rust
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => { let changed=self.edit_preview.take().is_some()|self.transform_preview.take().is_some(); self.draft=None;self.plane_menu=false;self.plane_only=false; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
```

<details>
<summary>Locate this block</summary>

```rust
                let hit = self.snap_point(screen,size);
                if let Some(draft) = self.draft.as_mut() { draft.snapped = hit.filter(|h| h.owner.is_some() || h.kind == crate::snap::Kind::Grid).map(|h| h.kind); }
                if commit {
                    if hit.is_some_and(|h|self.draft.as_ref().is_some_and(|d|d.primitive.is_none() && d.tool.count()==0 && d.points.len()>=2 && d.points.first().is_some_and(|p|(0..3).all(|i|(p[i]-h.point[i]).abs()<=1e-12)))) {return self.apply(Action::CloseTool);}
                    return hit.map_or(Ok(Change::View), |hit| self.place_tool_point(hit.point));
                }
                if let Some(draft) = self.draft.as_mut() { draft.hover(hit.map(|h| h.point))?; }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                let preview=if matches!(shape,crate::tool::Shape::Move) {
                    if self.selected_part.is_some() { return Err("Use Select Object before Move"); }
                    Some(crate::edit_preview::Preview::new(&self.scene,self.selected_objects())?)
```

</details>

Replace it with:

```rust
                let hit = self.snap_point(screen,size);
                if let Some(draft) = self.draft.as_mut() { draft.snapped = hit.filter(|h| h.owner.is_some() || h.kind == crate::snap::Kind::Grid).map(|h| h.kind); }
                if commit {
                    if hit.is_some_and(|h|self.draft.as_ref().is_some_and(|d|d.primitive.is_none() && d.transform.is_none() && d.tool.count()==0 && d.points.len()>=2 && d.points.first().is_some_and(|p|(0..3).all(|i|(p[i]-h.point[i]).abs()<=1e-12)))) {return self.apply(Action::CloseTool);}
                    return hit.map_or(Ok(Change::View), |hit| self.place_tool_point(hit.point));
                }
                if let Some(draft) = self.draft.as_mut() { draft.hover(hit.map(|h| h.point))?; }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                if matches!(shape,crate::tool::Shape::Move)&&self.selected_part.is_some(){return self.begin_transform(crate::transform_draft::Kind::Move);}
                self.transform_preview=None;
                let preview=if matches!(shape,crate::tool::Shape::Move) {
                    if self.selected_part.is_some() { return Err("Use Select Object before Move"); }
                    Some(crate::edit_preview::Preview::new(&self.scene,self.selected_objects())?)
```

<details>
<summary>Locate this block</summary>

```rust
            Action::ConstructionPlane(plane)=>{
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=self.draft.as_mut().and_then(|draft|draft.primitive.as_mut()) {primitive.set_plane(frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                self.construction_plane=plane;crate::drawing_coords::set_plane(plane);self.plane_menu=false;
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(draft)=self.draft.as_mut() {draft.frame=frame;draft.cursor=None;draft.snapped=None;}
```

</details>

Replace it with:

```rust
            Action::ConstructionPlane(plane)=>{
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=self.draft.as_mut().and_then(|draft|draft.primitive.as_mut()) {primitive.set_plane(frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                if let Some(transform)=self.draft.as_mut().and_then(|d|d.transform.as_mut()){transform.frame=frame;}
                self.construction_plane=plane;crate::drawing_coords::set_plane(plane);self.plane_menu=false;
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(draft)=self.draft.as_mut() {draft.frame=frame;draft.cursor=None;draft.snapped=None;}
```

<details>
<summary>Locate this block</summary>

```rust

        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        match action {
            Action::PrimitiveBegin {shape,option,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let mut primitive=crate::primitive_draft::Draft::new(shape,frame,option).map_err(|message|self.primitive_failure(message))?;
```

</details>

Replace it with:

```rust

        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        match action {
            Action::BeginTransformation(kind)=>return self.begin_transform(kind),
            Action::Transformation {kind,delta}=>{let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(kind,&delta,&ids,target);},
            Action::PrimitiveBegin {shape,option,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let mut primitive=crate::primitive_draft::Draft::new(shape,frame,option).map_err(|message|self.primitive_failure(message))?;
```

<details>
<summary>Locate this block</summary>

```rust
        }
        self.canvas_drag=None;
        let keep_gizmo=matches!(&action,Action::GizmoPress {..}|Action::GizmoTouch {..}|Action::GizmoMove {..}|Action::GizmoRelease {..}|Action::GizmoCancel|Action::GizmoNumber(_)|Action::Zoom(_)|Action::ZoomAt {..}|Action::PanScreen {..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::Snap(_));
        let keep_edit=matches!(&action, Action::GizmoPress {..} | Action::GizmoTouch {..} | Action::GizmoMove {..} | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::BeginTool(_) | Action::Draw {..} | Action::ToolText(_) | Action::FinishTool | Action::CloseTool | Action::ConstructionPlane(_) | Action::PlaneMenu | Action::Plane3 | Action::ToolSides(_) | Action::PrimitiveBegin {..} | Action::ToolPoint(_) | Action::ToolScreen {..} | Action::ToolHover(_) | Action::CancelTool | Action::Snap(_) | Action::Zoom(_) | Action::ZoomAt {..} | Action::PanScreen {..} | Action::Pan(_,_) | Action::Orbit(_,_) | Action::Fit | Action::FitSelected | Action::Projection(_) | Action::ResetView | Action::Background | Action::NormalView(_) | Action::Grid(_));
        let mut change=self.apply_action(action)?;
        self.gizmo_hover=None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change==Change::View {change=Change::Preview;}
        if !keep_edit && self.edit_preview.take().is_some() { self.draft=None; if change==Change::View { change=Change::Preview; } }
        if change==Change::Scene { self.snap_index=None; }
        Ok(change)
```

</details>

Replace it with:

```rust
        }
        self.canvas_drag=None;
        let keep_gizmo=matches!(&action,Action::GizmoPress {..}|Action::GizmoTouch {..}|Action::GizmoMove {..}|Action::GizmoRelease {..}|Action::GizmoCancel|Action::GizmoNumber(_)|Action::Zoom(_)|Action::ZoomAt {..}|Action::PanScreen {..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::Snap(_));
        let keep_edit=matches!(&action, Action::GizmoPress {..} | Action::GizmoTouch {..} | Action::GizmoMove {..} | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::BeginTool(_) | Action::Draw {..} | Action::ToolText(_) | Action::FinishTool | Action::CloseTool | Action::ConstructionPlane(_) | Action::PlaneMenu | Action::Plane3 | Action::ToolSides(_) | Action::BeginTransformation(_) | Action::Transformation {..} | Action::PrimitiveBegin {..} | Action::ToolPoint(_) | Action::ToolScreen {..} | Action::ToolHover(_) | Action::CancelTool | Action::Snap(_) | Action::Zoom(_) | Action::ZoomAt {..} | Action::PanScreen {..} | Action::Pan(_,_) | Action::Orbit(_,_) | Action::Fit | Action::FitSelected | Action::Projection(_) | Action::ResetView | Action::Background | Action::NormalView(_) | Action::Grid(_));
        let mut change=self.apply_action(action)?;
        self.gizmo_hover=None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change==Change::View {change=Change::Preview;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.transform.is_some()){self.transform_preview=None;self.draft=None;if change==Change::View{change=Change::Preview;}}
        if !keep_edit && self.edit_preview.take().is_some() { self.draft=None; if change==Change::View { change=Change::Preview; } }
        if change==Change::Scene { self.snap_index=None; }
        Ok(change)
```

<details>
<summary>Locate this block</summary>

```rust

pub enum Action {
    StreamRefresh,
    PrimitiveBegin {shape:&'static crate::primitive_shape::Shape,option:usize,text:String},
    ToolSides(usize),
    ConstructionPlane(Option<crate::cplane::CPlane>), PlaneMenu, Plane3,
```

</details>

Replace it with:

```rust

pub enum Action {
    StreamRefresh,
    BeginTransformation(crate::transform_draft::Kind),
    Transformation {kind:crate::transform_draft::Kind,delta:session_rust::Xform},
    PrimitiveBegin {shape:&'static crate::primitive_shape::Shape,option:usize,text:String},
    ToolSides(usize),
    ConstructionPlane(Option<crate::cplane::CPlane>), PlaneMenu, Plane3,
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    StreamRefresh,
    BeginTransformation(crate::transform_draft::Kind),
    Transformation {kind:crate::transform_draft::Kind,delta:session_rust::Xform},
    PrimitiveBegin {shape:&'static crate::primitive_shape::Shape,option:usize,text:String},
    ToolSides(usize),
    ConstructionPlane(Option<crate::cplane::CPlane>), PlaneMenu, Plane3,
    Draw {shape:crate::tool::Shape,text:String}, ToolText(String), FinishTool, CloseTool,
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
    CanvasRelease {screen:[f32;2],size:[f64;2],add:bool,remove:bool},
    CanvasHit(Option<ObjectId>),CanvasCancel,
    SelectRows {ids:Vec<ObjectId>,add:bool,remove:bool},
    PanScreen {delta:[f64;2],size:[f64;2]},ZoomAt {factor:f32,screen:[f32;2]},
    Touch(Vec<Action>),
    LayerEdit {key:String,edit:crate::layer_edit::Edit},
    LayerProperty {key:String,property:crate::layer_properties::Property},
    TreePick {ids:Vec<ObjectId>,additive:bool},
    GizmoPress {pointer:i32,screen:[f32;2],size:[f64;2],modified:bool},
    GizmoTouch {pointer:i32,screen:[f32;2],size:[f64;2]},
    GizmoMove {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoRelease {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen { screen: [f32; 2], size: [f64; 2], commit: bool },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany { target: Option<crate::pick_sources::Target>, additive: bool },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Live(Box<Scene>),
    Adopt {scene:Box<Scene>,replace:bool},
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    ReplaceAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    Close,
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddCloudLod,
    AddCloud,
    AddCad,
    AddInstances,
    AddShapeReuse,
    AddNestedInstances,
    AddSheet,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
    AddPoint,
    AddCurve,
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
    Preview,
}

pub struct Editor {
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
    gizmo_hover:Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub transform_preview:Option<crate::gizmo_edit::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub construction_plane:Option<crate::cplane::CPlane>,
    pub plane_menu:bool,
    pub plane_only:bool,
    pub tool_status: String,
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
    pub pick_mode: crate::pick_sources::Mode,
    objects: crate::object_selection::Objects,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            transform_preview:None,draft: None, tool_status: String::new(), construction_plane:None, plane_menu:false, plane_only:false,
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.transform_preview.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|p|p.scene()),|g|g.preview.scene()),|p|&p.scene)
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if self.scene.clouds().iter().any(|r|r.controls_visible)||self.scene.streams().iter().any(|r|r.controls_visible) {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
    }

    pub fn gizmo_hit(&self,screen:[f32;2],size:[f64;2])->Option<crate::gizmo::Handle> {self.gizmo_hit_radius(screen,size,8.)}

    pub fn gizmo_hit_radius(&self,screen:[f32;2],size:[f64;2],radius:f64)->Option<crate::gizmo::Handle> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
        let origin=self.gizmo_origin()?;
        let scale=crate::widget_layout::world_per_pixel(&self.camera.view_projection(),origin,size[1])?;
        let ray=self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2])).hit_with_radius(&ray.origin,&ray.direction,scale,radius)
    }

    pub fn number_prompt(&self,size:[f64;2])->Option<crate::number_box::NumberPrompt> {
        let edit=self.gizmo_edit.as_ref()?;let handle=edit.gizmo.typing?;
        let origin=edit.origin();let matrix=self.camera.view_projection();let scale=crate::widget_layout::world_per_pixel(&matrix,origin,size[1])?;
        let point=edit.gizmo.handle_point(handle,scale);
        let at=crate::snap_math::Screen {matrix:matrix.m,size}.point([point[0],point[1],point[2]])?;
        let unit=handle.labels().2;
        Some(crate::number_box::NumberPrompt {handle,title:handle.title(),unit,hint:if unit=="factor" {"1"} else {"0"},at:at.map(|v|v as f32)})
    }

    pub fn gizmo_active(&self)->f32 {crate::gizmo_edit::code(self.gizmo_edit.as_ref().and_then(|e|e.active()).or(self.gizmo_hover))}

    pub fn hover_gizmo(&mut self,screen:Option<([f32;2],[f64;2])>) {
        self.gizmo_hover=if self.gizmo_edit.is_some() {None} else {screen.and_then(|(point,size)|self.gizmo_hit(point,size))};
    }

    fn gizmo_press(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool)->Result<Change,&'static str> {
        self.gizmo_press_radius(pointer,screen,size,modified,8.)
    }

    fn gizmo_press_radius(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool,radius:f64)->Result<Change,&'static str> {
        if modified || self.draft.is_some() || self.gizmo_edit.as_ref().is_some_and(|g|g.pointer.is_some()) {return Ok(Change::View);}
        let Some(handle)=self.gizmo_hit_radius(screen,size,radius) else {return Ok(Change::View);};
        let origin=self.gizmo_origin().ok_or("No editable selection")?;
        let preview=crate::gizmo_edit::Preview::new(&self.scene,self.selected_objects(),self.selection().filter(|_|self.selected_part.is_some()))?;
        let ray=self.camera.ray(screen).ok_or("Invalid handle ray")?;
        let gizmo=crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2]));
        self.gizmo_edit=crate::gizmo_edit::Edit::begin(gizmo,handle,&ray,preview,pointer,screen);Ok(Change::View)
    }

    fn gizmo_move(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return Err("Invalid handle viewport");}
        let Some(ray)=self.camera.ray(screen) else {return Err("Invalid handle ray");};
        let Some(edit)=self.gizmo_edit.as_mut() else {return Ok(Change::View);};
        Ok(if edit.update(pointer,screen,size,&ray,shift)? {Change::Preview} else {Change::View})
    }

    fn gizmo_release(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if self.gizmo_edit.as_ref().is_none_or(|g|g.pointer!=Some(pointer)) {return Ok(Change::View);}
        self.gizmo_move(pointer,screen,size,shift)?;
        let edit=self.gizmo_edit.as_mut().unwrap();
        if edit.click() {self.tool_status=format!("{}: type a value in {}, Enter applies, Esc closes",edit.active().unwrap().title(),edit.active().unwrap().labels().2);return Ok(Change::Preview);}
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?; self.gizmo_edit=None;
        self.tool_status="Handle edit completed.".into();Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn gizmo_number(&mut self,text:&str)->Result<Change,&'static str> {
        let edit=self.gizmo_edit.as_mut().ok_or("No handle number is being requested")?;
        let handle=edit.gizmo.typing.ok_or("No handle number is being requested")?;
        let value=crate::gizmo::typed_value(handle,text).map_err(|_|"Enter a finite number; scale needs a positive factor")?;
        let Some(value)=value else {self.gizmo_edit=None;return Ok(Change::Preview);};
        edit.preview.set(edit.gizmo.typed(handle,value))?;
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?;self.gizmo_edit=None;
        self.tool_status=format!("{} {value} {}",handle.title(),handle.labels().2);Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        if let Some(transform)=self.draft.as_ref().and_then(|draft|draft.transform.as_ref()) {
            let matrix=transform.preview(self.draft.as_ref().and_then(|draft|draft.cursor)).unwrap_or_else(session_rust::Xform::identity);
            if let Some(preview)=self.transform_preview.as_mut(){preview.set(matrix)?;return Ok(Change::Preview);}return Ok(Change::View);
        }
        let Some(preview)=self.edit_preview.as_mut() else { return Ok(Change::View); };
        let offset=self.draft.as_ref().and_then(|d| d.points.first().zip(d.cursor)).map(|(a,b)| std::array::from_fn::<_,3,_>(|i| b[i]-a[i])).unwrap_or([0.;3]);
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?; Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self)->impl Iterator<Item=&Scene> {
        self.scenes().chain(self.transform_preview.iter().flat_map(|p|p.scenes())).chain(self.canvas_drag.iter().filter_map(|d|d.preview.as_ref()).flat_map(|p|p.scenes())).chain(self.edit_preview.iter().flat_map(|p|p.scenes())).chain(self.gizmo_edit.iter().flat_map(|g|g.preview.scenes()))
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene.objects().iter().filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id)).collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(&mut self, values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>) -> Result<bool, &'static str> {
        if values.is_empty() { return Ok(false); }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id) || !self.scene.objects().iter().any(|row| key.matches(row)) {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values.into_iter().map(|(key, bytes)|
            crate::rehydrate::PreparedReload::new(key, &bytes)).collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self.scenes().flat_map(|scene| scene.objects()).filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row)) { return Err("Reloaded source GUID or metadata does not match"); }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared { scene.hydrate(plan); }
        }
        if self.edit_preview.take().is_some() { self.draft=None; } self.gizmo_edit=None; self.snap_index = None; Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self.scenes().flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload()).filter_map(|row| row.origin().map(|origin| origin.id)).collect();
        for row in self.scenes().flat_map(|scene| scene.objects()) {
            if let Some(origin) = row.origin() {
                if !row.can_unload() { origins.remove(&origin.id); }
            }
        }
        if origins.is_empty() { return Err("No eligible imported sources to unload"); }
        let epoch = self.last_release.checked_add(1).ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() { scene.unload(&origins, epoch); }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        if !self.scene.contains(id) {return Err("Object not found");}
        let object=self.scene.objects().iter().find(|row|row.id==id);
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if object.is_some_and(|object|object.geometry().is_none()) {return Err("Reload editable sources before Move");}
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let ids=self.scene.with_descendants(&[id]);let delta=session_rust::Xform::translation(offset[0],offset[1],offset[2]);
        self.history.try_edit(&mut self.scene,|scene| {for id in ids {let matrix=&delta*&scene.placement(id).ok_or("Object not found")?;scene.place_following(id,matrix)?;}Ok(())})?;
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {return Err("Object not found");}
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if self.scene.objects().iter().find(|row| row.id == id).is_some_and(|row| row.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {for id in ids {scene.remove(id);}});
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) { self.selected = self.objects.primary(); self.selected_part = None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() { return Vec::new(); }
        if self.objects.primary() == self.selected { self.objects.rows().to_vec() }
        else { self.selected.into_iter().filter(|id| self.scene.pick_id(id.value()).is_some()).collect() }
    }

    pub fn select_objects(&mut self, rows: impl IntoIterator<Item = ObjectId>, additive: bool) {
        if additive { let old = self.selected_objects(); self.objects.set(&self.scene, old, false); }
        self.selected_part = None; self.objects.set(&self.scene, rows, additive); self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self,ids:Vec<ObjectId>,add:bool,remove:bool) {
        if remove {let old=self.selected_objects();self.objects.set(&self.scene,old,false);self.objects.remove(&self.scene,ids);self.selected=self.objects.primary();self.selected_part=None;}
        else {self.select_objects(ids,add);}
    }

    fn finish_canvas(&mut self)->Result<Change,&'static str> {
        let Some(mut drag)=self.canvas_drag.take() else {return Ok(Change::View);};
        let (add,remove)=drag.released.unwrap_or((false,false));
        if let Some(preview)=drag.preview.take() {let changed=preview.commit(&mut self.scene,&mut self.history)?;if changed {self.snap_index=None;}return Ok(if changed {Change::Scene} else {Change::Preview});}
        if drag.missed {if let Some(rectangle)=drag.rectangle() {let ids=crate::rect_select::found(&self.scene,&self.camera.view_projection().m,drag.size,rectangle);self.select_rows(ids,add,remove);}}
        Ok(Change::View)
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene); self.selected = self.objects.primary();
        } else { self.objects.clear(); }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) { return Err("Move needs finite coordinates"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Err("Select an object before Move"); }
        if offset.iter().all(|&n| n == 0.) { return Ok(Change::View); }
        let mut preview=crate::edit_preview::Preview::new(&self.scene,ids)?;
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
        preview.commit(&mut self.scene,&mut self.history)?;Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() { return Err("Use Select Object before Delete"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Ok(Change::View); }
        if self.scene.objects().iter().any(|r| ids.contains(&r.id) && r.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| { for id in ids { scene.remove(id); } });
        self.objects.clear(); self.selected = None; Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected.map(|owner| crate::pick_sources::Target { owner, part: self.selected_part.unwrap_or(crate::pick_sources::Part::Object) })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> { self.selected.filter(|_| self.selected_part.is_none()) }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")) {
            return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape") {Action::GizmoCancel} else {Action::GizmoNumber(text.to_owned())}));
        }
        if self.plane_menu {return Some(crate::drawing_input::choice(text).ok_or("Choose XY, XZ, YZ, 3 Point, View or Cancel"));}
        let draft=self.draft.as_ref()?;
        if let Some(choice)=crate::drawing_input::choice(text) {return Some(Ok(choice));}
        if draft.plane_points.is_none() && draft.primitive.is_some() {
            let words:Vec<_>=text.split_whitespace().collect();let primitive=draft.primitive.as_ref().unwrap();
            if text.trim().is_empty() || primitive.shape.option_in(&words).is_some() || words.first().is_some_and(|word|word.parse::<f64>().is_ok() || crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@')) {return Some(Ok(Action::ToolText(text.to_owned())));}
            return None;
        }
        if draft.plane_points.is_none() && draft.transform.is_some(){
            let first=text.split_whitespace().next();if text.trim().is_empty()||first.is_some_and(|word|word.parse::<f64>().is_ok()||word.contains(',')||word.contains('<')||word.starts_with('@')||word.starts_with('w')||word.starts_with('W')||["1D","2D","3D"].iter().any(|mode|word.eq_ignore_ascii_case(mode))){return Some(Ok(Action::ToolText(text.into())))}return None;
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count()==0 {return Some(Ok(Action::CloseTool));}
        if text.trim().is_empty() {return Some(Ok(if draft.tool.count()==0 && !self.plane_only {Action::FinishTool}else{Action::CancelTool}));}
        let word=text.trim();
        if word.split_whitespace().next().is_some_and(|s|s.eq_ignore_ascii_case("Sides")) && draft.tool.sides().is_some() {
            let parts:Vec<_>=word.split_whitespace().collect();
            return Some(match parts.as_slice() {[_name,value]=>value.parse::<usize>().ok().filter(|n|(3..crate::tool::MAX_POINTS).contains(n)).map(Action::ToolSides).ok_or("Polygon sides must be between 3 and 4095"),_=>Err("Use Sides 6")});
        }
        let coordinate=crate::drawing_coords::parse_world(word).is_some() || word.contains(',') || word.contains('<') || word.starts_with('@') || word.starts_with('w') || word.starts_with('W') || word.split_whitespace().next().is_some_and(|p|crate::drawing_coords::parse_world(p).is_some());
        coordinate.then(|| {
            let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
            crate::drawing_input::points(word,self.construction_plane,draft.frame,points,draft.cursor).map(|_|Action::ToolText(word.to_owned()))
        })
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_point(point);}

        let draft=self.draft.as_mut().ok_or("No point tool is running")?;
        if let Some(points)=draft.plane_points.as_mut() {
            let point=crate::tool_coords::checked(point)?;let mut picked=points.clone();picked.push(point);
            if picked.len()==3 {
                let p:Vec<_>=picked.iter().map(|p|session_rust::Point::new(p[0],p[1],p[2])).collect();
                let plane=crate::cplane::CPlane::from_3_points(&p[0],&p[1],&p[2]).ok_or("The three points are in line; pick the third again")?;
                draft.plane_points=None;self.apply(Action::ConstructionPlane(Some(plane)))?;
                if self.plane_only {self.draft=None;self.plane_only=false;}return Ok(Change::View);
            }
            *points=picked;self.tool_status=draft.prompt();return Ok(Change::View);
        }
        if draft.primitive.is_some() {return self.primitive_answer(session_rust::Point::new(point[0],point[1],point[2]));}
        crate::drawing_coords::set_plane(self.construction_plane);
        let Some(built)=draft.submit(point)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    fn commit_tool(&mut self,built:crate::tool::Built) -> Result<Change,&'static str> {
        let name=self.draft.as_ref().ok_or("No point tool is running")?.tool.name();
        if let crate::tool::Built::Move(offset)=built {
            let preview=self.edit_preview.as_mut().ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
            let changed=preview.commit(&mut self.scene,&mut self.history)?;
            self.edit_preview=None;self.draft=None;self.tool_status="Move completed.".into();return Ok(if changed {Change::Scene}else{Change::Preview});
        }
        self.history.try_edit(&mut self.scene,|scene|match built {
            crate::tool::Built::Point(point)=>scene.insert_point(point).map(|_|()),
            crate::tool::Built::Line(line)=>scene.insert_line(line).map(|_|()),
            crate::tool::Built::Polyline(line)=>scene.insert_path(line).map(|_|()),
            crate::tool::Built::Geometry(geometry)=>crate::primitive_geometry::insert(scene,geometry).map(|_|()),
            crate::tool::Built::Move(_)=>unreachable!("Move uses its placement preview"),
        })?;
        self.draft=None;self.tool_status=format!("{name} created.");Ok(Change::Scene)
    }

    fn primitive_failure(&mut self,message:String)->&'static str {self.tool_status=message;"Shape input invalid"}
    fn primitive_answer(&mut self,point:session_rust::Point)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.accept(crate::primitive_shape::Answer::Point(point.clone()));
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points.push([point[0],point[1],point[2]]);draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn primitive_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_mut().ok_or("No shape tool is running")?;
        let result=draft.primitive.as_mut().ok_or("No shape tool is running")?.text(text,draft.cursor,self.construction_plane);
        let geometry=result.map_err(|message|self.primitive_failure(message))?;
        let draft=self.draft.as_mut().unwrap();draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();draft.cursor=None;draft.snapped=None;
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn apply_transform(&mut self,kind:crate::transform_draft::Kind,delta:&session_rust::Xform,ids:&[ObjectId],target:Option<crate::pick_sources::Target>)->Result<Change,&'static str> {
        if !crate::placement::valid(&delta.m){return Err("Transform must be finite, affine and invertible");}
        if kind==crate::transform_draft::Kind::Copy {
            if target.is_some(){return Err("Copy works on whole objects");}
            self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::copy_objects(scene,ids,delta).map(|_|()))?;return Ok(Change::Scene);
        }
        let mut preview=crate::gizmo_edit::Preview::new(&self.scene,ids.to_vec(),target)?;preview.set(delta.clone())?;let changed=preview.commit(&mut self.scene,&mut self.history)?;Ok(if changed{Change::Scene}else{Change::Preview})
    }
    fn begin_transform(&mut self,kind:crate::transform_draft::Kind)->Result<Change,&'static str> {
        let owners=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());
        let preview=if kind==crate::transform_draft::Kind::Copy {if target.is_some(){return Err("Copy works on whole objects");}let mut candidate=self.scene.clone();crate::layer_edit::copy_objects(&mut candidate,&owners,&session_rust::Xform::identity())?;None}else{Some(crate::gizmo_edit::Preview::new(&self.scene,owners.clone(),target)?)};
        let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
        let mut draft=crate::tool::Draft::new(crate::tool::Shape::Transform(kind));draft.frame=frame;let transform=draft.transform.as_mut().unwrap();transform.frame=frame;transform.owners=owners;transform.target=target;
        self.edit_preview=None;self.transform_preview=preview;self.draft=Some(draft);self.snap_index=None;self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::Preview)
    }
    fn transform_outcome(&mut self,next:crate::transform_draft::Draft,outcome:crate::transform_draft::Outcome)->Result<Change,&'static str> {
        // Validate the entire typed batch against a temporary scene before recording its separate Copy target transactions.
        let mut candidate=self.scene.clone();
        for delta in &outcome.transforms {if next.kind==crate::transform_draft::Kind::Copy {crate::layer_edit::copy_objects(&mut candidate,&next.owners,delta)?;}else{let mut preview=crate::gizmo_edit::Preview::new(&candidate,next.owners.clone(),next.target)?;preview.set(delta.clone())?;let mut history=crate::history::History::default();preview.commit(&mut candidate,&mut history)?;}}
        for delta in &outcome.transforms {self.apply_transform(next.kind,delta,&next.owners,next.target)?;}
        let changed=!outcome.transforms.is_empty();
        if outcome.finished {self.draft=None;self.transform_preview=None;self.tool_status=format!("{} completed.",next.kind.name());return Ok(if changed{Change::Scene}else{Change::Preview});}
        let draft=self.draft.as_mut().ok_or("No transform tool is running")?;draft.points=next.points.clone();draft.transform=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();
        if changed {self.snap_index=None;return Ok(Change::Scene);}self.update_edit_preview()
    }
    fn transform_point(&mut self,point:[f64;3])->Result<Change,&'static str> {
        let mut next=self.draft.as_ref().and_then(|draft|draft.transform.clone()).ok_or("No transform tool is running")?;
        let transform=next.accept(crate::primitive_shape::Answer::Point(session_rust::Point::new(point[0],point[1],point[2]))).map_err(|message|self.primitive_failure(message))?;
        let finished=transform.is_some()&&next.kind!=crate::transform_draft::Kind::Copy;self.transform_outcome(next,crate::transform_draft::Outcome{transforms:transform.into_iter().collect(),finished})
    }
    fn transform_text(&mut self,text:&str)->Result<Change,&'static str> {
        let draft=self.draft.as_ref().ok_or("No transform tool is running")?;let mut next=draft.transform.clone().ok_or("No transform tool is running")?;
        let outcome=next.text(text,draft.cursor,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.transform_outcome(next,outcome)
    }
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
        if self.draft.as_ref().is_some_and(|d|d.transform.is_some()&&d.plane_points.is_none()){return self.transform_text(text);}

        if self.draft.as_ref().is_some_and(|d|d.primitive.is_some() && d.plane_points.is_none()) {return self.primitive_text(text);}
        let draft=self.draft.as_ref().ok_or("No point tool is running")?;
        let points=draft.plane_points.as_deref().unwrap_or(&draft.points);
        let added=crate::drawing_input::points(text,self.construction_plane,draft.frame,points,draft.cursor)?;
        if draft.plane_points.is_some() {
            if added.len()!=1 {return Err("Supply one construction-plane point at a time");}return self.place_tool_point(added[0]);
        }
        let draft=self.draft.as_mut().unwrap();
        let Some(built)=draft.submit_many(&added)? else {self.tool_status=draft.prompt();self.snap_index=None;return self.update_edit_preview();};
        self.commit_tool(built)
    }

    pub fn snap_point(&mut self, screen: [f32; 2], size: [f64; 2]) -> Option<crate::snap::Hit> {
        if let Some(primitive)=self.draft.as_ref()?.primitive.as_ref() {if primitive.next().is_some_and(|ask|ask.kind==crate::primitive_shape::Kind::Height) {return primitive.on_axis(&self.camera,screen).map(|point|crate::snap::Hit {point,kind:crate::snap::Kind::Near,owner:None});}}
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude=self.edit_preview.as_ref().filter(|_| from.is_some()).map_or_else(Vec::new,|p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| crate::snap_index::Index::new(&self.scene).excluding(&exclude));
            let viewport = crate::snap_math::Screen { matrix: self.camera.view_projection().m, size };
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        let own=self.draft.as_ref().filter(|d|d.primitive.is_none() && d.transform.is_none() && d.tool.count()==0 && d.points.len()>=2 && self.snapping.objects && self.snapping.modes&crate::snap::END!=0).and_then(|d|{
            let point=*d.points.first()?;let viewport=crate::snap_math::Screen {matrix:self.camera.view_projection().m,size};let p=viewport.point(point)?;
            let cursor=[(screen[0] as f64+1.)*size[0]/2.,(1.-screen[1] as f64)*size[1]/2.];
            ((p[0]-cursor[0]).hypot(p[1]-cursor[1])<=12.).then_some(crate::snap::Hit {point,kind:crate::snap::Kind::End,owner:None})
        });
        own.or(hit).or_else(|| {
            let draft=self.draft.as_ref()?;
            let frame=self.construction_plane.unwrap_or(draft.frame);
            let through=if self.construction_plane.is_none() {draft.points.last().copied()}else{None};
            let point=crate::drawing_input::on_at(&self.camera,screen,frame,through)?;
            let (x,y)=frame.axes();let o=frame.origin();
            let point=if self.snapping.grid {crate::snap_math::grid(point,[o[0],o[1],o[2]],[[x[0],x[1],x[2]],[y[0],y[1],y[2]]],self.snapping.step)?}else{point};
            Some(crate::snap::Hit {point,kind:if self.snapping.grid {crate::snap::Kind::Grid}else{crate::snap::Kind::Near},owner:None})
        })
    }

    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
            Action::Touch(actions)=> {let mut change=Change::View;for action in actions {let next=self.apply(action)?;if next!=Change::View {change=next;}}return Ok(change);},
            Action::CanvasStart {from,to,size,modified,pointer}=>{self.canvas_drag=crate::canvas_drag::Drag::new(from,to,size,modified);if let Some(drag)=self.canvas_drag.as_mut() {drag.pointer=pointer;}return Ok(Change::View);},
            Action::CanvasMove {screen,size}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.released.is_none()&&drag.size==size&&screen.iter().all(|v|v.is_finite()) {drag.to=screen;drag.update(&self.camera)?;}if drag.preview.is_some() {return Ok(Change::Preview);}}return Ok(Change::View);},
            Action::CanvasRelease {screen,size,add,remove}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.size!=size {self.canvas_drag=None;return Ok(Change::Preview);}drag.to=screen;drag.released=Some((add,remove));drag.update(&self.camera)?;if drag.preview.is_some()||drag.missed {return self.finish_canvas();}}return Ok(Change::View);},
            Action::CanvasHit(hit)=>{
                let Some(drag)=self.canvas_drag.as_ref() else {return Ok(Change::View);};let ids=hit.filter(|id|!drag.modified&&self.scene.pick_id(id.value()).is_some());
                if let Some(id)=ids {let mut rows=self.selected_objects();if !rows.contains(&id) {self.select_objects([id],false);rows=vec![id];}if let Err(error)=self.canvas_drag.as_mut().unwrap().grab(&self.scene,rows,&self.camera) {self.canvas_drag=None;return Err(error);}}
                else {self.canvas_drag.as_mut().unwrap().missed=true;}
                if self.canvas_drag.as_ref().unwrap().released.is_some() {return self.finish_canvas();}return Ok(Change::Preview);
            },
            Action::CanvasCancel=>{let preview=self.canvas_drag.take().is_some_and(|d|d.preview.is_some());return Ok(if preview {Change::Preview} else {Change::View});},
            Action::SelectRows {ids,add,remove}=>{self.select_rows(ids,add,remove);return Ok(Change::View);},
            _=>{}
        }
        self.canvas_drag=None;
        let keep_gizmo=matches!(&action,Action::GizmoPress {..}|Action::GizmoTouch {..}|Action::GizmoMove {..}|Action::GizmoRelease {..}|Action::GizmoCancel|Action::GizmoNumber(_)|Action::Zoom(_)|Action::ZoomAt {..}|Action::PanScreen {..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::Snap(_));
        let keep_edit=matches!(&action, Action::GizmoPress {..} | Action::GizmoTouch {..} | Action::GizmoMove {..} | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::BeginTool(_) | Action::Draw {..} | Action::ToolText(_) | Action::FinishTool | Action::CloseTool | Action::ConstructionPlane(_) | Action::PlaneMenu | Action::Plane3 | Action::ToolSides(_) | Action::BeginTransformation(_) | Action::Transformation {..} | Action::PrimitiveBegin {..} | Action::ToolPoint(_) | Action::ToolScreen {..} | Action::ToolHover(_) | Action::CancelTool | Action::Snap(_) | Action::Zoom(_) | Action::ZoomAt {..} | Action::PanScreen {..} | Action::Pan(_,_) | Action::Orbit(_,_) | Action::Fit | Action::FitSelected | Action::Projection(_) | Action::ResetView | Action::Background | Action::NormalView(_) | Action::Grid(_));
        let mut change=self.apply_action(action)?;
        self.gizmo_hover=None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change==Change::View {change=Change::Preview;}
        if !keep_edit && self.draft.as_ref().is_some_and(|d|d.transform.is_some()){self.transform_preview=None;self.draft=None;if change==Change::View{change=Change::Preview;}}
        if !keep_edit && self.edit_preview.take().is_some() { self.draft=None; if change==Change::View { change=Change::Preview; } }
        if change==Change::Scene { self.snap_index=None; }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {

        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        match action {
            Action::BeginTransformation(kind)=>return self.begin_transform(kind),
            Action::Transformation {kind,delta}=>{let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(kind,&delta,&ids,target);},
            Action::PrimitiveBegin {shape,option,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let mut primitive=crate::primitive_draft::Draft::new(shape,frame,option).map_err(|message|self.primitive_failure(message))?;
                let geometry=if text.trim().is_empty() {None}else{primitive.text(&text,None,self.construction_plane).map_err(|message|self.primitive_failure(message))?};
                self.apply(Action::BeginTool(crate::tool::Shape::Primitive(shape)))?;
                let draft=self.draft.as_mut().unwrap();draft.primitive=Some(primitive);draft.points=draft.primitive.as_ref().unwrap().answers.iter().filter_map(|answer|match answer {crate::primitive_shape::Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect();
                if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ToolSides(sides)=>{
                let draft=self.draft.as_mut().ok_or("Sides works while drawing a polygon")?;draft.tool.set_sides(sides)?;self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ConstructionPlane(plane)=>{
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=self.draft.as_mut().and_then(|draft|draft.primitive.as_mut()) {primitive.set_plane(frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                if let Some(transform)=self.draft.as_mut().and_then(|d|d.transform.as_mut()){transform.frame=frame;}
                self.construction_plane=plane;crate::drawing_coords::set_plane(plane);self.plane_menu=false;
                let frame=plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(draft)=self.draft.as_mut() {draft.frame=frame;draft.cursor=None;draft.snapped=None;}
                self.tool_status="Construction plane changed.".into();return Ok(Change::View);
            },
            Action::PlaneMenu=>{self.plane_menu=true;self.tool_status="CPlane · XY XZ YZ 3 Point View Cancel".into();return Ok(Change::View);},
            Action::Plane3=>{
                self.plane_menu=false;self.plane_only=self.draft.is_none();
                if self.plane_only {self.apply(Action::BeginTool(crate::tool::Shape::Point))?;}
                let draft=self.draft.as_mut().unwrap();draft.plane_points=Some(Vec::new());draft.frame=crate::cplane::CPlane::Xy;
                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::Draw {shape,text}=>{
                let frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                let added=crate::drawing_input::points(&text,self.construction_plane,frame,&[],None)?;
                let mut draft=crate::tool::Draft::new(shape);draft.frame=frame;
                let built=draft.submit_many(&added)?;
                let built=if built.is_none() && draft.tool.count()==0 && added.len()>=2 {Some(draft.finish()?)}else{built};
                self.apply(Action::BeginTool(shape))?;self.draft=Some(draft);
                if let Some(built)=built {return self.commit_tool(built);}self.tool_status=self.draft.as_ref().unwrap().prompt();return Ok(Change::View);
            },
            Action::ToolText(text)=>return self.tool_text(&text),
            Action::CloseTool=>{
                let draft=self.draft.as_ref().ok_or("No point tool is running")?;
                if draft.tool.count()!=0 || draft.points.len()<3 {return Err("Close needs at least three polyline points");}
                if draft.points.len()>=crate::tool::MAX_POINTS {return Err("Drawing commands accept at most 4096 points");}
                let mut points=draft.points.clone();points.push(points[0]);
                crate::drawing_coords::set_plane(self.construction_plane);
                let built=draft.build_world(&points)?;return self.commit_tool(built);
            },
            Action::FinishTool=>{crate::drawing_coords::set_plane(self.construction_plane);let built=self.draft.as_ref().ok_or("No point tool is running")?.finish()?;return self.commit_tool(built);},
            Action::LayerEdit {key,edit}=>{
                if matches!(edit,crate::layer_edit::Edit::Current) {crate::layer_edit::apply(&mut self.scene,&key,edit)?;return Ok(Change::View);}
                self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::apply(scene,&key,edit))?;self.repair_selection();
            },
            Action::LayerProperty {key,property}=>{crate::layer_properties::apply(&mut self.scene,&key,property)?;self.repair_selection();if self.selected.is_some_and(|id|self.scene.pick_id(id.value()).is_none()) {self.selected=None;self.selected_part=None;}self.snap_index=None;return Ok(Change::Preview);},
            Action::Hide=>{let ids=self.scene.with_descendants(&self.selected_objects());std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,true);self.select_objects([],false);self.snap_index=None;return Ok(Change::Preview);},
            Action::Show=>{std::rc::Rc::make_mut(&mut self.scene.view).show_all();self.snap_index=None;return Ok(Change::Preview);},
            Action::TreePick {ids,additive}=>{let eligible=ids.into_iter().filter(|id|self.scene.pick_id(id.value())==Some(*id)).collect::<Vec<_>>();if eligible.is_empty() {return Err("This tree row has no visible editable objects");}self.select_objects(eligible,additive);return self.apply(Action::FitSelected);},
            Action::GizmoTouch {pointer,screen,size}=>return self.gizmo_press_radius(pointer,screen,size,false,18.),
            Action::GizmoPress {pointer,screen,size,modified}=>return self.gizmo_press(pointer,screen,size,modified),
            Action::GizmoMove {pointer,screen,size,shift}=>return self.gizmo_move(pointer,screen,size,shift),
            Action::GizmoRelease {pointer,screen,size,shift}=>return self.gizmo_release(pointer,screen,size,shift),
            Action::GizmoCancel=>{self.gizmo_edit=None;self.tool_status="Handle edit cancelled.".into();return Ok(Change::Preview);},
            Action::GizmoNumber(text)=>return self.gizmo_number(&text),
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() { draft.cursor = None; draft.snapped = None; }
                return self.update_edit_preview();
            }
            Action::ToolScreen { screen, size, commit } => {
                let hit = self.snap_point(screen,size);
                if let Some(draft) = self.draft.as_mut() { draft.snapped = hit.filter(|h| h.owner.is_some() || h.kind == crate::snap::Kind::Grid).map(|h| h.kind); }
                if commit {
                    if hit.is_some_and(|h|self.draft.as_ref().is_some_and(|d|d.primitive.is_none() && d.transform.is_none() && d.tool.count()==0 && d.points.len()>=2 && d.points.first().is_some_and(|p|(0..3).all(|i|(p[i]-h.point[i]).abs()<=1e-12)))) {return self.apply(Action::CloseTool);}
                    return hit.map_or(Ok(Change::View), |hit| self.place_tool_point(hit.point));
                }
                if let Some(draft) = self.draft.as_mut() { draft.hover(hit.map(|h| h.point))?; }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                if matches!(shape,crate::tool::Shape::Move)&&self.selected_part.is_some(){return self.begin_transform(crate::transform_draft::Kind::Move);}
                self.transform_preview=None;
                let preview=if matches!(shape,crate::tool::Shape::Move) {
                    if self.selected_part.is_some() { return Err("Use Select Object before Move"); }
                    Some(crate::edit_preview::Preview::new(&self.scene,self.selected_objects())?)
                } else { None };
                let changed=preview.is_some() || self.edit_preview.is_some(); self.edit_preview=preview;
                self.snap_index=None;let mut draft=crate::tool::Draft::new(shape);
                draft.frame=self.construction_plane.unwrap_or_else(||crate::cplane::CPlane::view(&self.camera.orientation.rotate_vector(session_rust::Vector::y_axis())));
                if let Some(primitive)=draft.primitive.as_mut() {primitive.set_plane(draft.frame).map_err(|message|{self.tool_status=message;"Shape input invalid"})?;}
                self.draft=Some(draft);self.tool_status=self.draft.as_ref().unwrap().prompt();
                return Ok(if changed { Change::Preview } else { Change::View });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => { let changed=self.edit_preview.take().is_some()|self.transform_preview.take().is_some(); self.draft=None;self.plane_menu=false;self.plane_only=false; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
                self.select_objects(target.filter(|t| t.part == crate::pick_sources::Part::Object && t.valid(&self.scene)).map(|t| t.owner), additive);
            }
            Action::SelectAll => {
                let ids: Vec<_> = self.scene.objects().iter().map(|r| r.id).chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id)).chain(self.scene.points().iter().map(|r| r.id)).chain(self.scene.texts().iter().map(|r|r.id)).chain(self.scene.clouds().iter().map(|r|r.id)).chain(self.scene.streams().iter().map(|r|r.id)).chain(self.scene.sheets().iter().map(|r|r.id)).collect();
                self.select_objects(ids, false);return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) = target.filter(|t| t.part == crate::pick_sources::Part::Object) { self.objects.set(&self.scene, [target.owner], false); }
                self.selected_part = target.map(|target| target.part).filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => { if mode!=crate::pick_sources::Mode::Control {self.scene.cloud_controls(None);} self.pick_mode = mode; self.selected = None; self.selected_part = None; self.objects.clear(); }
            Action::Pick(screen) => {
                let id = self.camera.ray(screen).and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear(); self.selected = None;
            }
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddSheet=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::sheet_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original drawing sheet"))?;},
            Action::AddNestedInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::nested_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare nested original placements"))?;},
            Action::AddShapeReuse=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::cad_shape_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare congruent original CAD"))?;},
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
            Action::Live(ready)=>{self.history.clear();self.scene.restore(*ready);},
            Action::Adopt {scene:ready,..}=>{self.history.edit(&mut self.scene,|scene|scene.restore(*ready));},
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                    line.width = 6.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line = session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::Controls(visible) => {
                let parent=if visible {self.selected.filter(|id|self.scene.clouds().iter().any(|r|r.id==*id)||self.scene.streams().iter().any(|r|r.id==*id))} else {None};
                if parent.is_some() && self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.clouds().iter().any(|r|Some(r.id)==parent && r.controls_visible)||self.scene.streams().iter().any(|r|Some(r.id)==parent&&r.controls_visible)) {return Ok(Change::View);}
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {self.pick_mode=crate::pick_sources::Mode::Control;self.objects.clear();self.selected_part=None;}
                else if !visible && self.pick_mode==crate::pick_sources::Mode::Control {self.pick_mode=crate::pick_sources::Mode::Object;self.selected_part=None;self.select_objects(self.selected,false);}
            },
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::normal_example::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddColours(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(false, 2, &[session_rust::Point::new(-0.6, -0.4, 0.0),
                    session_rust::Point::new(0.0, 0.6, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                curve.width = 5.0; curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared = crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0; point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared = crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_point(prepared).map(|_| ()))?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared = crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_line(prepared).map(|_| ()))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => {
                if self.selected_part.is_some(){let target=self.selection();return self.apply_transform(crate::transform_draft::Kind::Move,&session_rust::Xform::translation(offset[0],offset[1],offset[2]),&[],target);}
                return self.move_selection(offset);
            },
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => { let id = self.scene.next(self.selected); self.select_objects(id, false); }
            Action::Delete => return self.delete_selection(),
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Grid(settings) => {
                        if let Some(grid) = settings { grid.half()?; grid.reach(self.camera.target)?; }
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::ZoomAt {factor,screen}=>self.camera.zoom_at(factor,screen),
                    Action::PanScreen {delta,size}=>self.camera.pan_screen(delta,size),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(||self.display_scene().bounds()) {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(|| self.selected_part.and(self.selected).and_then(|id| self.scene.selected_bounds(id)))
                        {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::ResetView => {
                        let aspect = self.camera.aspect;
                        self.camera = Camera { aspect, grid: self.camera.grid,frame:self.camera.frame, ..Camera::default() };
                    }
                    _ => unreachable!("Document actions were handled above"),
                }
                return Ok(Change::View);
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.repair_selection();
        self.selected = self.selected.filter(|id| self.scene.contains(*id));
        if let (Some(owner), Some(part)) = (self.selected, self.selected_part) {
            if !(crate::pick_sources::Target { owner, part }).valid(&self.scene) { self.selected = None; }
        }
        if self.selected.is_none() { self.selected_part = None; }
        Ok(Change::Scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new("test:first".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), first)).unwrap();
        let origin = editor.scene.objects()[0].origin().unwrap().id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new("test:second".into()));
        editor.apply(Action::ImportAt(bytes, second)).unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(saved.objects()[0].origin().unwrap().id, origin);
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some()); assert!(copy.upgrade().is_none());
        assert!(editor.scene.objects()[0].geometry().is_some());
        assert!(editor.scene.objects()[3].geometry().is_none());
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err()); assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:epoch".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(editor.apply(Action::UnloadSources), Err("Release epochs exhausted"));
        assert!(editor.scene.objects()[0].geometry().is_some());
        editor.last_release = 0; editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(2));
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
        editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap(); editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform(); let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view); assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(crate::memory::cpu(editor.scenes(), std::iter::empty()), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(&row.mesh))).collect();
        let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(row.geometry().unwrap()))).collect();
        let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document)))).collect();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
        assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
        assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(editor.scene.objects().iter().all(|row| !old.contains(&row.id)));
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
        let mut editor = Editor::default();
        editor.camera.aspect = 2.0;
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::ResetView).unwrap();
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
    }

    #[test]
    fn undo_repairs_selection_and_empty_delete_preserves_redo() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        for _ in 0..3 {
            editor.apply(Action::SelectNext).unwrap();
        }
        let box_id = editor.selected.unwrap();
        editor.apply(Action::Undo).unwrap();
        assert!(editor.selected.is_none());
        editor.apply(Action::Delete).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(box_id));
    }

    #[test]
    fn picking_a_moved_view_uses_the_editors_current_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        let point = session_rust::Point::new(0.2, 0.3, 0.75);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert_eq!(editor.apply(Action::Pick([screen[0] as f32, screen[1] as f32])).unwrap(), Change::Scene);
        assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
        assert_eq!(editor.apply(Action::Zoom(2.0)).unwrap(), Change::View);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77k-connect-transformation-lifecycle-and-part-move-dispatch-01.rs"
```

```text
--8<-- "journey/code/77k-connect-transformation-lifecycle-and-part-move-dispatch-02.rs"
```

```text
--8<-- "journey/code/77k-connect-transformation-lifecycle-and-part-move-dispatch-03.rs"
```

```text
--8<-- "journey/code/77k-connect-transformation-lifecycle-and-part-move-dispatch-04.rs"
```

```text
--8<-- "journey/code/77k-connect-transformation-lifecycle-and-part-move-dispatch-05.rs"
```

```text
--8<-- "journey/code/77k-connect-transformation-lifecycle-and-part-move-dispatch-06.rs"
```

```text
--8<-- "journey/code/77k-connect-transformation-lifecycle-and-part-move-dispatch-07.rs"
```

```text
--8<-- "journey/code/77k-connect-transformation-lifecycle-and-part-move-dispatch-08.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/selection_tests.rs`

Keep invalid Move and parent-deletion guards when source-part Move becomes available.

<details>
<summary>Locate the existing block</summary>

```text
    editor.apply(Action::PickMode(Mode::Face)).unwrap(); editor.apply(Action::PickedTarget(Some(target))).unwrap();
    assert_eq!(editor.selection(), Some(target)); assert_eq!(editor.whole_selection(), None);
    assert!(editor.apply(Action::Translate([1., 0., 0.])).is_err());
    assert!(editor.apply(Action::Delete).is_err()); assert_eq!(editor.scene.objects().len(), 1);
    assert!(std::rc::Rc::ptr_eq(&source, &editor.scene.objects()[0].geometry().unwrap()));
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/77k-connect-transformation-lifecycle-and-part-move-dispatch-09.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/edit_preview_tests.rs`

Allow part Move previews and cancellation while retaining locked, cold and stale target guards.

<details>
<summary>Locate the existing block</summary>

```text
    e.apply(Action::SelectAll).unwrap(); e.selected_part=Some(crate::pick_sources::Part::Face(0)); assert!(e.apply(Action::BeginTool(Shape::Move)).is_err());
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/77k-connect-transformation-lifecycle-and-part-move-dispatch-10.rs"
```

</details>

</details>

<a id="checkpoint-77l-register-copy-scale-rotate-and-their-current-aliases"></a>

### Step 31 of 49 · Register Copy Scale Rotate and their current aliases (`77l-register-copy-scale-rotate-and-their-current-aliases`)

- **Goal:** Register Copy Scale Rotate and their current aliases.
- **Data Flow Trace:** `current verb or alias → checked parameter parser → Editor action`
- **Files You Touch:** `src/transform_commands.rs` (*Parse current direct and interactive transform names, options and aliases.*) · `src/lib.rs` (*Register the source transformation component.*) · `src/commands.rs` (*Register current transforms and preserve longest-name matching and typed aliases.*) · `src/editor.rs` (*Carry typed scale factors and world-axis rotation angles to the existing Editor.*)

Expose current transformation names, world-axis Rotate options and typed aliases through the same longest-name command registry.

> **Self-Check Question:** *Does the typed alias s mean the canvas shortcut S?*  
> **Answer:** Typed s invokes Scale. When the canvas owns the key, S runs Show; focused text input owns its own letters.

- **Hands-On Lab Experiment:** Try Copy 1,0,0, Scale 2, Rotate z 45, m 1,0,0, s 2 and rot z 45, with spaces separating the verb and parameters.

<details>
<summary>Implementation 77l-register-copy-scale-rotate-and-their-current-aliases: Register Copy Scale Rotate and their current aliases</summary>

Register Copy Scale Rotate and their current aliases.

Data flow: current verb or alias → checked parameter parser → Editor action.

### 1. `src/transform_commands.rs`

Parse current direct and interactive transform names, options and aliases.

Create the file and type:

```rust
--8<-- "journey/code/77l-register-copy-scale-rotate-and-their-current-aliases-01.rs"
```

### 2. `src/lib.rs`

Register the source transformation component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod source_part;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77l-register-copy-scale-rotate-and-their-current-aliases-02.rs" skip=22
```

### 3. `src/commands.rs`

Register current transforms and preserve longest-name matching and typed aliases.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
}

pub fn parse(line: &str) -> Result<Command, String> {
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action)=crate::primitive_registry::command(name,&words[count..].join(" ")) {return Ok(Command::Editor(action));}
    if name=="CPlane" && words.len()>count {return crate::drawing_input::choice(&words[count..].join(" ")).map(Command::Editor).ok_or_else(||"CPlane (XY XZ YZ 3 Point View Cancel)".into());}
    if matches!(name,"Point"|"Line"|"Curve"|"Polyline"|"Polyline Points"|"Polyline Rectangle"|"Polyline Polygon") && words.len()>count {
```

</details>

Replace it with:

```rust
}

pub fn parse(line: &str) -> Result<Command, String> {
    let alias=crate::transform_commands::alias(line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action)=crate::transform_commands::action(name,&words[count..]){return action.map(Command::Editor);}
    if let Some(action)=crate::primitive_registry::command(name,&words[count..].join(" ")) {return Ok(Command::Editor(action));}
    if name=="CPlane" && words.len()>count {return crate::drawing_input::choice(&words[count..].join(" ")).map(Command::Editor).ok_or_else(||"CPlane (XY XZ YZ 3 Point View Cancel)".into());}
    if matches!(name,"Point"|"Line"|"Curve"|"Polyline"|"Polyline Points"|"Polyline Rectangle"|"Polyline Polygon") && words.len()>count {
```

<details>
<summary>Locate this block</summary>

```rust
}

pub fn canonical(line: &str) -> String {
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(|| line.trim().to_owned(), |(name, count)| {
        std::iter::once(name).chain(words[count..].iter().copied()).collect::<Vec<_>>().join(" ")
```

</details>

Replace it with:

```rust
}

pub fn canonical(line: &str) -> String {
    let alias=crate::transform_commands::alias(line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(|| line.trim().to_owned(), |(name, count)| {
        std::iter::once(name).chain(words[count..].iter().copied()).collect::<Vec<_>>().join(" ")
```

<details>
<summary>Locate this block</summary>

```rust
];

pub fn names() -> Vec<&'static str> {
    HOSTS.iter().map(|(name, _)| *name).chain(ACTIONS.iter().map(|(name, _)| *name)).chain(crate::primitive_registry::ENTRIES.iter().map(|(name,_,_)|*name)).collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
```

</details>

Replace it with:

```rust
];

pub fn names() -> Vec<&'static str> {
    HOSTS.iter().map(|(name, _)| *name).chain(ACTIONS.iter().map(|(name, _)| *name)).chain(crate::primitive_registry::ENTRIES.iter().map(|(name,_,_)|*name)).chain(crate::transform_commands::NAMES.iter().copied()).collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::editor::Action;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Host { TextDemo, Help, Open, Replace, CancelOpen, Reload, CancelReload, Save, Report, PreviousReport, Selection, LayersOn, LayersOff, LayersAll }

pub enum Command { Editor(Action), Host(Host) }

pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Snap", || Action::Snap(crate::snap::Setting::Objects(None))),
    ("Snap On", || Action::Snap(crate::snap::Setting::Objects(Some(true)))),
    ("Snap Off", || Action::Snap(crate::snap::Setting::Objects(Some(false)))),
    ("Grid Snap", || Action::Snap(crate::snap::Setting::Grid(None))),
    ("Grid Snap On", || Action::Snap(crate::snap::Setting::Grid(Some(true)))),
    ("Grid Snap Off", || Action::Snap(crate::snap::Setting::Grid(Some(false)))),
    ("Snap End", || Action::Snap(crate::snap::Setting::Mode(crate::snap::END))),
    ("Snap Near", || Action::Snap(crate::snap::Setting::Mode(crate::snap::NEAR))),
    ("Snap Mid", || Action::Snap(crate::snap::Setting::Mode(crate::snap::MID))),
    ("Snap Center", || Action::Snap(crate::snap::Setting::Mode(crate::snap::CENTER))),
    ("Snap Perp", || Action::Snap(crate::snap::Setting::Mode(crate::snap::PERP))),
    ("Move", || Action::BeginTool(crate::tool::Shape::Move)),
    ("Point", || Action::BeginTool(crate::tool::Shape::Point)),
    ("Line", || Action::BeginTool(crate::tool::Shape::Line)),
    ("Curve",||Action::BeginTool(crate::tool::Shape::Curve)),
    ("Polyline",||Action::BeginTool(crate::tool::Shape::Polyline)),
    ("Polyline Points",||Action::BeginTool(crate::tool::Shape::Polyline)),
    ("Polyline Rectangle",||Action::BeginTool(crate::tool::Shape::Rectangle)),
    ("Polyline Polygon",||Action::BeginTool(crate::tool::Shape::Polygon {sides:6})),
    ("CPlane",||Action::PlaneMenu),
    ("Hide",||Action::Hide),
    ("Show",||Action::Show),
    ("Example Box", || Action::AddBox),
    ("Example Cloud LOD",||Action::AddCloudLod),
    ("Example Cloud",||Action::AddCloud),
    ("Example CAD",||Action::AddCad),
    ("Example Instances",||Action::AddInstances),
    ("Example Shape Reuse",||Action::AddShapeReuse),
    ("Example Nested Instances",||Action::AddNestedInstances),
    ("Example Sheet",||Action::AddSheet),
    ("Example Scene Text",||Action::AddText(crate::text::TextPlacement::WorldPlane {world:[-1.5,0.,1.],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.22})),
    ("Example Billboard",||Action::AddText(crate::text::TextPlacement::WorldBillboard {world:[-1.5,0.,1.],world_height:0.22})),
    ("Example Anchor",||Action::AddText(crate::text::TextPlacement::Anchor {world:[0.,0.,1.],offset:[8.,-12.]})),
    ("Example Nameplate",||Action::AddText(crate::text::TextPlacement::Nameplate {world:[0.,0.,1.],padding:[17.,4.],rounded:true})),
    ("Example Surface",||Action::AddSurface),
    ("Example Nested",||Action::AddNested),
    ("Example Line", || Action::AddLine),
    ("Example Point", || Action::AddPoint),
    ("Example Curve", || Action::AddCurve),
    ("Example Object Color", || Action::AddColours(crate::colour::Mode::Object)),
    ("Example Point Colors", || Action::AddColours(crate::colour::Mode::Points)),
    ("Example Face Colors", || Action::AddColours(crate::colour::Mode::Faces)),
    ("View Normals On", || Action::NormalView(true)),
    ("View Normals Off", || Action::NormalView(false)),
    ("View Grid On", || Action::Grid(Some(crate::grid::Settings::default()))),
    ("View Grid Off", || Action::Grid(None)),
    ("Example Sharp", || Action::AddNormals(crate::normal_example::Mode::Sharp)),
    ("Example Smooth", || Action::AddNormals(crate::normal_example::Mode::Smooth)),
    ("Example Winding", || Action::AddNormals(crate::normal_example::Mode::Winding)),
    ("Controls", || Action::Controls(true)),
    ("Controls On", || Action::Controls(true)),
    ("Controls Off", || Action::Controls(false)),
    ("Example Polyline", || Action::AddPolyline),
    ("Example Arrow", || Action::AddArrow),
    ("Example Triangle", || Action::ToggleExtra),
    ("Select Next", || Action::SelectNext),
    ("Select All", || Action::SelectAll),
    ("Select None", || Action::Picked(None)),
    ("Select Object", || Action::PickMode(crate::pick_sources::Mode::Object)),
    ("Select Face", || Action::PickMode(crate::pick_sources::Mode::Face)),
    ("Select Edge", || Action::PickMode(crate::pick_sources::Mode::Edge)),
    ("Select Control", || Action::PickMode(crate::pick_sources::Mode::Control)),
    ("Delete", || Action::Delete),
    ("Undo", || Action::Undo),
    ("Redo", || Action::Redo),
    ("Background", || Action::Background),
    ("Zoom In", || Action::Zoom(2.0)),
    ("Zoom Out", || Action::Zoom(0.5)),
    ("Pan Left", || Action::Pan(-0.25, 0.0)),
    ("Pan Right", || Action::Pan(0.25, 0.0)),
    ("Orbit Right", || Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0)),
    ("Orbit Up", || Action::Orbit(0.0, std::f64::consts::FRAC_PI_6)),
    ("View Isometric", || Action::Isometric),
    ("Fit", || Action::Fit),
    ("Fit Selected", || Action::FitSelected),
    ("View Reset", || Action::ResetView),
    ("View Perspective", || Action::Projection(crate::camera::Projection::Perspective)),
    ("View Orthographic", || Action::Projection(crate::camera::Projection::Orthographic)),
    ("Close", || Action::Close),
    ("Unload Sources", || Action::UnloadSources),
];

pub const HOSTS: &[(&str, Host)] = &[
    ("Layers On",Host::LayersOn),("Layers Off",Host::LayersOff),("Layers All",Host::LayersAll),
    ("Help", Host::Help),
    ("Open", Host::Open),
    ("Open Replace", Host::Replace),
    ("Cancel Open", Host::CancelOpen),
    ("Reload Sources", Host::Reload),
    ("Cancel Reload", Host::CancelReload),
    ("Save", Host::Save),
    ("Example Text",Host::TextDemo),
    ("Report", Host::Report),
    ("Report Previous", Host::PreviousReport),
    ("Selection", Host::Selection),
];

pub fn names() -> Vec<&'static str> {
    HOSTS.iter().map(|(name, _)| *name).chain(ACTIONS.iter().map(|(name, _)| *name)).chain(crate::primitive_registry::ENTRIES.iter().map(|(name,_,_)|*name)).chain(crate::transform_commands::NAMES.iter().copied()).collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
fn matched(words: &[&str]) -> Option<(&'static str, usize)> {
    names().into_iter().filter_map(|name| {
        let compact = name.split_whitespace().collect::<String>().to_ascii_lowercase();
        let mut rest = compact.as_str();
        for (index, word) in words.iter().enumerate() {
            let word = word.to_ascii_lowercase();
            rest = rest.strip_prefix(&word)?;
            if rest.is_empty() { return Some((name, index + 1, compact.len())); }
        }
        None
    }).max_by_key(|(_, _, letters)| *letters).map(|(name, count, _)| (name, count))
}

pub fn canonical(line: &str) -> String {
    let alias=crate::transform_commands::alias(line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(|| line.trim().to_owned(), |(name, count)| {
        std::iter::once(name).chain(words[count..].iter().copied()).collect::<Vec<_>>().join(" ")
    })
}

pub fn parse(line: &str) -> Result<Command, String> {
    let alias=crate::transform_commands::alias(line);let line=alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action)=crate::transform_commands::action(name,&words[count..]){return action.map(Command::Editor);}
    if let Some(action)=crate::primitive_registry::command(name,&words[count..].join(" ")) {return Ok(Command::Editor(action));}
    if name=="CPlane" && words.len()>count {return crate::drawing_input::choice(&words[count..].join(" ")).map(Command::Editor).ok_or_else(||"CPlane (XY XZ YZ 3 Point View Cancel)".into());}
    if matches!(name,"Point"|"Line"|"Curve"|"Polyline"|"Polyline Points"|"Polyline Rectangle"|"Polyline Polygon") && words.len()>count {
        let shape=match name {"Point"=>crate::tool::Shape::Point,"Curve"=>crate::tool::Shape::Curve,"Line"=>crate::tool::Shape::Line,"Polyline Rectangle"=>crate::tool::Shape::Rectangle,"Polyline Polygon"=>crate::tool::Shape::Polygon {sides:6},_=>crate::tool::Shape::Polyline};
        return Ok(Command::Editor(Action::Draw {shape,text:words[count..].join(" ")}));
    }
    if name == "Move" && words.len()>count {
        return crate::offset::parse(&format!("Move {}", words[count..].join(" ")))
            .map(|offset| Command::Editor(Action::Translate(offset))).map_err(str::to_owned);
    }
    if name == "Grid Snap" && words.len() > count {
        if words.len() != count+1 { return Err("Grid Snap (On Off) or a spacing".into()); }
        let step = words[count].parse::<f64>().map_err(|_| "Grid Snap (On Off) or a spacing")?;
        if !step.is_finite() || step<=0. { return Err("The grid spacing must be above zero · Example: Grid Snap 50".into()); }
        return Ok(Command::Editor(Action::Snap(crate::snap::Setting::Spacing(step))));
    }
    if words.len() != count { return Err(format!("{name} takes no arguments")); }
    if let Some((_, make)) = ACTIONS.iter().find(|(key, _)| *key == name) { return Ok(Command::Editor(make())); }
    let (_, host) = HOSTS.iter().find(|(key, _)| *key == name).ok_or("Command registry is incomplete")?;
    Ok(Command::Host(*host))
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77l-register-copy-scale-rotate-and-their-current-aliases-03.rs"
```

```text
--8<-- "journey/code/77l-register-copy-scale-rotate-and-their-current-aliases-04.rs"
```

```text
--8<-- "journey/code/77l-register-copy-scale-rotate-and-their-current-aliases-05.rs"
```

</details>

### 4. `src/editor.rs`

Carry typed scale factors and world-axis rotation angles to the existing Editor.

<details>
<summary>Locate the existing block</summary>

```rust
    StreamRefresh,
    BeginTransformation(crate::transform_draft::Kind),
    Transformation {kind:crate::transform_draft::Kind,delta:session_rust::Xform},
    PrimitiveBegin {shape:&'static crate::primitive_shape::Shape,option:usize,text:String},
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77l-register-copy-scale-rotate-and-their-current-aliases-06.rs"
```

</details>

<a id="checkpoint-77m-apply-direct-scale-and-rotate-about-the-current-selection-center"></a>

### Step 32 of 49 · Apply direct Scale and Rotate about the current selection center (`77m-apply-direct-scale-and-rotate-about-the-current-selection-center`)

- **Goal:** Apply direct Scale and Rotate about the current selection center.
- **Data Flow Trace:** `typed current factor or world axis → selection center → original transform`
- **Files You Touch:** `src/editor.rs` (*Apply direct current Scale and Rotate about the selection gizmo center.*)

Validate the direct factor or world-axis angle and apply it about the current gizmo center, falling back to the world origin when absent.

> **Self-Check Question:** *Does Rotate z90 use the construction plane normal?*  
> **Answer:** The explicit z form uses the world Z axis. Interactive Rotate uses the current construction plane normal.

- **Hands-On Lab Experiment:** Rotate a selected line about its center, then compare an interactive XZ reference turn and Undo each independently.

<details>
<summary>Implementation 77m-apply-direct-scale-and-rotate-about-the-current-selection-center: Apply direct Scale and Rotate about the current selection center</summary>

Apply direct Scale and Rotate about the current selection center.

Data flow: typed current factor or world axis → selection center → original transform.

### 1. `src/editor.rs`

Apply direct current Scale and Rotate about the selection gizmo center.

<details>
<summary>Locate the existing block</summary>

```rust
        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        match action {
            Action::BeginTransformation(kind)=>return self.begin_transform(kind),
            Action::Transformation {kind,delta}=>{let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(kind,&delta,&ids,target);},
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77m-apply-direct-scale-and-rotate-about-the-current-selection-center-01.rs"
```

</details>

<a id="checkpoint-77n-retain-earlier-selection-checks-with-source-part-support"></a>

### Step 33 of 49 · Retain earlier selection checks with source part support (`77n-retain-earlier-selection-checks-with-source-part-support`)

- **Goal:** Retain earlier selection checks with source part support.
- **Data Flow Trace:** `current selection → validated source-part preview → unchanged history on cancel`
- **Files You Touch:** `src/edit_preview_tests.rs` (*Copy retained selection/preview fixtures with current source-part Move support.*)

Strengthen the earlier part-preview check: cancellation must restore the original placements. The selection and parent-deletion guards now change in 77k, alongside part Move.

> **Self-Check Question:** *Does enabling part Move allow Delete to remove its parent implicitly?*  
> **Answer:** No. Delete still requires whole-object selection. Invalid edits and cancelled previews leave selection history unchanged.

- **Hands-On Lab Experiment:** Select an original part, start Move and cancel with Escape. Confirm its placement is restored, then select its parent and compare Delete/Undo. Read the retained cold, locked and stale-owner cases as optional boundary validation.

<details>
<summary>Implementation 77n-retain-earlier-selection-checks-with-source-part-support: Retain earlier selection checks with source part support</summary>

Retain earlier selection checks with source part support.

Data flow: current selection → validated source-part preview → unchanged history on cancel.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/edit_preview_tests.rs`

Copy retained selection/preview fixtures with current source-part Move support.

<details>
<summary>Locate the existing block</summary>

```text

#[test]
fn preview_rejects_parts_locked_or_cold_sources_and_stale_targets() {
    let mut e=editor(); let id=e.scene.objects()[0].id;
    assert!(Preview::new(&e.scene,vec![]).is_err()); let mut preview=Preview::new(&e.scene,vec![id,id]).unwrap(); assert_eq!(preview.ids(),&[id]);
    preview.set(Xform::translation(1.,0.,0.)).unwrap(); e.scene.translate(id,[2.,0.,0.]).unwrap();
    assert!(preview.commit(&mut e.scene,&mut crate::history::History::default()).is_err());
    e.apply(Action::SelectAll).unwrap(); e.selected_part=Some(crate::pick_sources::Part::Face(0)); e.apply(Action::BeginTool(Shape::Move)).unwrap(); assert!(e.transform_preview.is_some()); e.apply(Action::CancelTool).unwrap();
    let mut mesh=session_rust::Mesh::create_box(1.,1.,1.); mesh.is_locked=true;
    let locked=e.scene.insert(crate::prepared::PreparedMesh::new(mesh).unwrap()).unwrap(); assert!(Preview::new(&e.scene,vec![locked]).is_err());
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/77n-retain-earlier-selection-checks-with-source-part-support-02.rs"
```

</details>

</details>

<a id="checkpoint-77o-exercise-original-copy-transforms-history-and-exact-persistence"></a>

### Step 34 of 49 · Exercise original copy transforms history and exact persistence (`77o-exercise-original-copy-transforms-history-and-exact-persistence`)

- **Goal:** Exercise original copy transforms history and exact persistence.
- **Data Flow Trace:** `original selection → current Copy Scale Rotate → history → original Save/Open`
- **Files You Touch:** `src/transform_tests.rs` (*Copy integrated current copy/transform/original-edit/history/nested SaveOpen fixtures.*) · `src/lib.rs` (*Register the source transformation component.*)

Copy the current integrated creation, preview, source edit and nested placement fixtures, then use the registered commands to create the rendered exercise.

> **Self-Check Question:** *What survives a whole-object Copy and rotation?*  
> **Answer:** Keep the original geometry allocation and its source data. The new row has a distinct identity and placement; its parent, name and colours remain.

- **What You Should See:** Three original boxes retain their shared source and independent placements. Copy repeats with separate target transactions; Scale modes and Rotate references preserve original geometry. Source-part edits retain rational weights, knots and validated CAD boundaries. Undo/Redo and Save/Open preserve original identity and nested placement.
- **Hands-On Lab Experiment:** Type Close, CPlane XY, Box w0,0,0 1 1 1, Select All, Copy 2,0,0, Scale 0.8, Rotate z 30, Copy -2,0,0, View Isometric, Select None and Fit, with spaces after Copy, Scale and Rotate. Try repeated Copy targets, Scale1D/2D/3D, a failed input, Escape and Undo/Redo. Save and reopen.

<details>
<summary>Implementation 77o-exercise-original-copy-transforms-history-and-exact-persistence: Exercise original copy transforms history and exact persistence</summary>

Exercise original copy transforms history and exact persistence.

Data flow: original selection → current Copy Scale Rotate → history → original Save/Open.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/transform_tests.rs`

Copy integrated current copy/transform/original-edit/history/nested SaveOpen fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/77o-exercise-original-copy-transforms-history-and-exact-persistence-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register the source transformation component.

<details>
<summary>Locate the existing block</summary>

```text

pub mod transform_commands;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77o-exercise-original-copy-transforms-history-and-exact-persistence-02.rs" skip=29
```

</details>

</details>

<a id="checkpoint-77zza-describe-curve-gathering-answers"></a>

### Step 35 of 49 · Describe curve gathering answers (`77zza-describe-curve-gathering-answers`)

- **Goal:** Describe ordered curve questions and typed answers.
- **Data Flow Trace:** `original curve picks → ordered answers → Session kernel recipe`
- **Files You Touch:** `src/gather_input.rs` (*Type the shared curve-gathering declarations.*) · `src/lib.rs` (*Register these helpers for the later kernel surfacing recipes.*)

Describe ordered curve questions and typed answers. Keep original curves and their placed corner data separate from command progress.

> **Self-Check Question:** *Why preserve both a NURBS curve and optional corners?*  
> **Answer:** Kernel builders use the curve; line and polyline previews can retain their exact corners.

- **Hands-On Lab Experiment:** Trace a curve-pick step followed by two point answers. Identify which values belong to original geometry.

<details>
<summary>Implementation 77zza-describe-curve-gathering-answers: Describe curve gathering answers</summary>

Describe ordered curve questions and typed answers.

Data flow: original curve picks → ordered answers → Session kernel recipe.

### 1. `src/gather_input.rs`

Type the shared curve-gathering declarations.

Create the file and type:

```rust
--8<-- "journey/code/77zza-describe-curve-gathering-answers-1.rs"
```

### 2. `src/lib.rs`

Register these helpers for the later kernel surfacing recipes.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod transform_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77zza-describe-curve-gathering-answers-2.rs" skip=34
```

</details>

<a id="checkpoint-77zzb-read-curve-gathering-values"></a>

### Step 36 of 49 · Read curve gathering values (`77zzb-read-curve-gathering-values`)

- **Goal:** Read finite numbers, absolute points and distance vectors through the existing coordinate reader.
- **Data Flow Trace:** `original curve picks → ordered answers → Session kernel recipe`
- **Files You Touch:** `src/gather_input.rs` (*Insert the next gathering behavior after its existing declarations.*)

Read finite numbers, absolute points and distance vectors through the existing coordinate reader. Reject curve picks supplied as words.

> **Self-Check Question:** *Why does a distance accept either a number or a vector?*  
> **Answer:** A number follows the recipe axis; a vector states the displacement directly.

- **Hands-On Lab Experiment:** Try a finite angle, an out-of-range angle and a distance vector. Predict the accepted answer variants.

<details>
<summary>Implementation 77zzb-read-curve-gathering-values: Read curve gathering values</summary>

Read finite numbers, absolute points and distance vectors through the existing coordinate reader.

Data flow: original curve picks → ordered answers → Session kernel recipe.

### 1. `src/gather_input.rs`

Insert the next gathering behavior after its existing declarations.

<details>
<summary>Locate the existing block</summary>

```rust
    }, // low < value <= high; Enter takes the default
    Distance(&'static str), // along the recipe's axis: a click, a signed number or a typed vector
}
```

</details>

After that block, type:

```rust
impl Step {
    /// What the step asks for.
    pub fn prompt(&self) -> &'static str {
        match self {
            Step::Curves { prompt, .. } | Step::Number { prompt, .. } => prompt,
            Step::Point(prompt) | Step::Distance(prompt) => prompt,
        }
    }

    /// True for a step answered by picking curves.
    pub fn curves(&self) -> bool {
        matches!(self, Step::Curves { .. })
    }

    /// A typed word as this step's answer.
    pub fn read(&self, word: &str) -> Result<Answer, String> {
        match self {
            Step::Number {
                prompt, low, high, ..
            } => match typed_number(word) {
                Some(value) if value > *low && value <= *high => Ok(Answer::Number(value)),
                Some(_) => Err(format!(
                    "The {prompt} must be above {low} and at most {high}"
                )),
                None => Err(format!("`{word}` is not a number")),
            },
            Step::Point(_) => match coords::absolute(word) {
                Some([x, y, z]) => Ok(Answer::Point(Point::new(x, y, z))),
                None => Err(format!("`{word}` is not a point x,y,z")),
            },
            Step::Distance(_) => match (typed_number(word), coords::parse_world(word)) {
                (Some(value), _) => Ok(Answer::Number(value)),
                (None, Some((coords::Typed::Absolute { x, y, z }, world))) => {
                    let [x, y, z] = coords::offset([x, y, z.unwrap_or(0.0)], world);
                    Ok(Answer::Vector(Vector::new(x, y, z)))
                }
                _ => Err(format!("`{word}` is not a distance or a vector x,y,z")),
            },
            Step::Curves { .. } => Err("Click the curves".into()),
        }
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzb-read-curve-gathering-values-1.rs"
```

</details>

</details>

<a id="checkpoint-77zzc-connect-gathering-recipes"></a>

### Step 37 of 49 · Connect gathering recipes (`77zzc-connect-gathering-recipes`)

- **Goal:** Connect typed input to a recipe, its questions and kernel builder.
- **Data Flow Trace:** `original curve picks → ordered answers → Session kernel recipe`
- **Files You Touch:** `src/gather_input.rs` (*Insert the next gathering behavior after its existing declarations.*)

Connect typed input to a recipe, its questions and kernel builder. Reuse the existing option matcher and expose typed point, number and axis helpers.

> **Self-Check Question:** *Why reuse the option matcher?*  
> **Answer:** Solid and gathering commands use the same labels and case-insensitive multiword matching rules.

- **Hands-On Lab Experiment:** Trace a numeric distance along an axis and compare it with an explicit vector.

<details>
<summary>Implementation 77zzc-connect-gathering-recipes: Connect gathering recipes</summary>

Connect typed input to a recipe, its questions and kernel builder.

Data flow: original curve picks → ordered answers → Session kernel recipe.

### 1. `src/gather_input.rs`

Insert the next gathering behavior after its existing declarations.

<details>
<summary>Locate the existing block</summary>

```rust
    pub choice: &'static str,     // the chosen option, empty when none
    pub normal: Vector,           // the construction plane the camera faces
}
```

</details>

After that block, type:

```rust
impl Input {
    /// The Distance answer as a vector along `axis`.
    pub fn along(&self, index: usize, axis: &Vector) -> Option<Vector> {
        match self.answers.get(index)? {
            Answer::Number(value) => Some(axis * *value),
            Answer::Vector(vector) => Some(vector.clone()),
            Answer::Point(_) => None,
        }
    }

    /// The Point answer at `index`.
    pub fn point(&self, index: usize) -> Option<Point> {
        match self.answers.get(index)? {
            Answer::Point(p) => Some(p.clone()),
            _ => None,
        }
    }

    /// The Number answer at `index`.
    pub fn number(&self, index: usize) -> Option<f64> {
        match self.answers.get(index)? {
            Answer::Number(value) => Some(*value),
            _ => None,
        }
    }
}

/// What a build made and what to say about it.
pub struct Made {
    pub geometries: Vec<Geometry>, // added as one undo step
    pub message: String,           // e.g. `Lofted 3 curves into a NURBS surface`
}

/// A command that gathers curves and answers, then builds.
pub struct Recipe {
    pub name: &'static str,                             // shown name
    pub chips: &'static [(&'static str, &'static str)], // buttons: options, then Finish and Cancel
    pub steps: &'static [Step],                         // what it asks, in order
    pub axis: fn(&Input) -> (Point, Vector), // where a Distance step slides, unit direction
    pub build: fn(&Input) -> Result<Made, String>, // the objects from the answers
}

impl std::fmt::Debug for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

impl Recipe {
    /// The option labels, e.g. Open and Closed.
    pub fn choices(&self) -> impl Iterator<Item = &'static str> {
        options::choices(self.chips)
    }

    /// The option `words` start with and how many words spell it, ignoring case and spaces.
    pub fn option_in(&self, words: &[&str]) -> Option<(usize, usize)> {
        options::find(self.chips, words)
    }

    /// The steps that are not curve picks.
    pub fn others(&self) -> impl Iterator<Item = &'static Step> {
        self.steps.iter().filter(|step| !step.curves())
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzc-connect-gathering-recipes-1.rs"
```

</details>

</details>

<a id="checkpoint-77zzd-parse-gathering-command-words"></a>

### Step 38 of 49 · Parse gathering command words (`77zzd-parse-gathering-command-words`)

- **Goal:** Read options anywhere in a command and consume other answers in order.
- **Data Flow Trace:** `original curve picks → ordered answers → Session kernel recipe`
- **Files You Touch:** `src/gather_input.rs` (*Insert the next gathering behavior after its existing declarations.*)

Read options anywhere in a command and consume other answers in order. Apply remaining numeric defaults only when the typed prefix permits it.

> **Self-Check Question:** *Why cannot an omitted point receive a numeric default?*  
> **Answer:** The remaining sequence must consist entirely of number questions before default completion is valid.

- **Hands-On Lab Experiment:** Compare a partially typed revolve command with a missing axis point. Explain which can complete from defaults.

<details>
<summary>Implementation 77zzd-parse-gathering-command-words: Parse gathering command words</summary>

Read options anywhere in a command and consume other answers in order.

Data flow: original curve picks → ordered answers → Session kernel recipe.

### 1. `src/gather_input.rs`

Insert the next gathering behavior after its existing declarations.

<details>
<summary>Locate the existing block</summary>

```rust
    /// The steps that are not curve picks.
    pub fn others(&self) -> impl Iterator<Item = &'static Step> {
        self.steps.iter().filter(|step| !step.curves())
    }
}
```

</details>

After that block, type:

```rust
/// Read typed words: options anywhere, the last one winning, and answers in step order.
pub fn read(recipe: &Recipe, words: &[&str]) -> Result<(usize, Vec<Answer>), String> {
    let others: Vec<&Step> = recipe.others().collect();
    let mut choice = 0;
    let mut answers = Vec::new();
    let mut rest = words;

    while !rest.is_empty() {
        if let Some((index, count)) = recipe.option_in(rest) {
            choice = index;
            rest = &rest[count..];
            continue;
        }

        let Some(step) = others.get(answers.len()) else {
            let options: Vec<_> = recipe.choices().collect();
            let listed = if options.is_empty() {
                String::new()
            } else {
                format!("; its options are {}", options.join(", "))
            };
            return Err(format!(
                "`{}` is not a value of {}{listed}",
                rest[0], recipe.name
            ));
        };
        answers.push(step.read(rest[0])?);
        rest = &rest[1..];
    }

    // typed values take the defaults of the numbers left, e.g. a full turn
    if !answers.is_empty()
        && others[answers.len()..]
            .iter()
            .all(|step| matches!(step, Step::Number { .. }))
    {
        for step in &others[answers.len()..] {
            if let Step::Number { default, .. } = step {
                answers.push(Answer::Number(*default));
            }
        }
    }

    Ok((choice, answers))
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzd-parse-gathering-command-words-1.rs"
```

</details>

</details>

<a id="checkpoint-77zze-track-ordered-curve-picks"></a>

### Step 39 of 49 · Track ordered curve picks (`77zze-track-ordered-curve-picks`)

- **Goal:** Track preselected curve rows and skip answers already typed.
- **Data Flow Trace:** `original curve picks → ordered answers → Session kernel recipe`
- **Files You Touch:** `src/gather_input.rs` (*Add the next gathering behavior to the existing helpers.*)

Track preselected curve rows and skip answers already typed. Keep curve-pick order while advancing only after the required minimum.

> **Self-Check Question:** *Why retain a separate count of typed answers?*  
> **Answer:** Curve picks and other answers occupy different sequences; the count lets progress skip already supplied values.

- **Hands-On Lab Experiment:** Trace two preselected rows through separate rail and profile steps. Identify where progress waits.

<details>
<summary>Implementation 77zze-track-ordered-curve-picks: Track ordered curve picks</summary>

Track preselected curve rows and skip answers already typed.

Data flow: original curve picks → ordered answers → Session kernel recipe.

### 1. `src/gather_input.rs`

Add the next gathering behavior to the existing helpers.

<details>
<summary>Locate the existing block</summary>

```rust
/// A picked curve in world coordinates.
#[derive(Clone, Debug)]
pub struct Picked {
    pub curve: NurbsCurve,           // clamped, domain 0..1
    pub corners: Option<Vec<Point>>, // a line's or polyline's points
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77zze-track-ordered-curve-picks-1.rs"
```

</details>

<a id="checkpoint-77zzf-advance-and-cancel-curve-picks"></a>

### Step 40 of 49 · Advance and cancel curve picks (`77zzf-advance-and-cancel-curve-picks`)

- **Goal:** Toggle a picked curve, reject duplicates across groups, and advance on a full group or Enter.
- **Data Flow Trace:** `original curve picks → ordered answers → Session kernel recipe`
- **Files You Touch:** `src/gather_input.rs` (*Insert the next gathering behavior after its existing declarations.*)

Toggle a picked curve, reject duplicates across groups, and advance on a full group or Enter. Accept defaults and typed answers through one progress state.

> **Self-Check Question:** *Why truncate answers before adding a new one?*  
> **Answer:** The current progress position defines the accepted prefix; a replacement must discard later stale answers.

- **Hands-On Lab Experiment:** Trace picking the same row twice, picking it in another group and pressing Enter below the minimum.

<details>
<summary>Implementation 77zzf-advance-and-cancel-curve-picks: Advance and cancel curve picks</summary>

Toggle a picked curve, reject duplicates across groups, and advance on a full group or Enter.

Data flow: original curve picks → ordered answers → Session kernel recipe.

### 1. `src/gather_input.rs`

Insert the next gathering behavior after its existing declarations.

<details>
<summary>Locate the existing block</summary>

```rust
    /// Every row picked so far, in step and pick order.
    pub fn picked(&self) -> Vec<u32> {
        self.rows.iter().flatten().copied().collect()
    }
```

</details>

After that block, type:

```rust
    /// Pick `row` for the current Curves step; picking it again takes it back.
    pub fn pick(&mut self, steps: &[Step], row: u32) -> Result<(), String> {
        let Some(Step::Curves { max, .. }) = steps.get(self.step) else {
            return Err("Not picking curves now".into());
        };
        let rows = &mut self.rows[self.step];

        if let Some(at) = rows.iter().position(|r| *r == row) {
            rows.remove(at);
            return Ok(());
        }

        if self.rows.iter().flatten().any(|r| *r == row) {
            return Err("That curve is picked already".into());
        }

        self.rows[self.step].push(row);

        if self.rows[self.step].len() >= *max {
            self.step += 1;
            self.skip(steps);
        }

        Ok(())
    }

    /// Enter: a Curves step with enough picks or a Number step's default moves on.
    pub fn next(&mut self, steps: &[Step], answers: &mut Vec<Answer>) -> Result<(), String> {
        match steps.get(self.step) {
            Some(Step::Curves { min, .. }) if self.rows[self.step].len() < *min => {
                Err(format!("Select at least {}", count(*min, "curve")))
            }
            Some(Step::Curves { .. }) => {
                self.step += 1;
                self.skip(steps);
                Ok(())
            }
            Some(Step::Number { default, .. }) => {
                self.answer(steps, answers, Answer::Number(*default));
                Ok(())
            }
            Some(step) => Err(format!("Give the {}", step.prompt())),
            None => Ok(()),
        }
    }

    /// Answer the current step.
    pub fn answer(&mut self, steps: &[Step], answers: &mut Vec<Answer>, answer: Answer) {
        answers.truncate(self.answered);
        answers.push(answer);
        self.answered += 1;
        self.step += 1;
        self.skip(steps);
    }

    /// True once every step is answered.
    pub fn done(&self, steps: &[Step]) -> bool {
        self.step >= steps.len()
    }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzf-advance-and-cancel-curve-picks-1.rs"
```

</details>

</details>

<a id="checkpoint-77zzg-prepare-placed-kernel-curves"></a>

### Step 41 of 49 · Prepare placed kernel curves (`77zzg-prepare-placed-kernel-curves`)

- **Goal:** Convert lines, polylines and NURBS curves to placed kernel curves.
- **Data Flow Trace:** `original curve picks → ordered answers → Session kernel recipe`
- **Files You Touch:** `src/gather_input.rs` (*Add the next gathering behavior to the existing helpers.*)

Convert lines, polylines and NURBS curves to placed kernel curves. Remove repeated polyline corners, clamp periodic input and normalize the domain for shared surfacing recipes.

> **Self-Check Question:** *Why normalize every gathered curve to the same domain?*  
> **Answer:** Loft and sweep need compatible input conventions even when the original curves use different parameter ranges.

- **Hands-On Lab Experiment:** Compare a placed polyline with a periodic NURBS curve. Trace their distinct preparation paths and shared validity checks.

<details>
<summary>Implementation 77zzg-prepare-placed-kernel-curves: Prepare placed kernel curves</summary>

Convert lines, polylines and NURBS curves to placed kernel curves.

Data flow: original curve picks → ordered answers → Session kernel recipe.

### 1. `src/gather_input.rs`

Add the next gathering behavior to the existing helpers.

<details>
<summary>Locate the existing block</summary>

```rust
/// "1 curve" or "3 curves".
pub fn count(count: usize, noun: &str) -> String {
    match count {
        1 => format!("1 {noun}"),
        _ => format!("{count} {noun}s"),
    }
}

pub const SAMPLES: usize = 48; // points per curve for normals and previews
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77zzg-prepare-placed-kernel-curves-1.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/gather_input.rs`

Supply the original gathering-input validation fixtures separately.

<details>
<summary>Locate the existing block</summary>

```text
pub const SAMPLES: usize = 48; // points per curve for normals and previews
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77zzg-prepare-placed-kernel-curves-2.rs" skip=76
```

</details>

</details>

<a id="checkpoint-77zzh-check-original-loops-and-surface-controls"></a>

### Step 42 of 49 · Check original loops and surface controls (`77zzh-check-original-loops-and-surface-controls`)

- **Goal:** Find a loop’s unit normal and scale-aware planarity tolerance.
- **Data Flow Trace:** `original curves → consistent section frames → validated kernel surface`
- **Files You Touch:** `src/surfacing.rs` (*Type loop geometry, planarity and finite surface validation.*) · `src/lib.rs` (*Compile the shared surfacing owner while its recipes are prepared.*)

Find a loop’s unit normal and scale-aware planarity tolerance. Check that a resulting surface is valid and has finite control points before wrapping it as original geometry. Reuse the curve collector instead of another sampling path.

> **Self-Check Question:** *Why scale the planarity tolerance by the loop bounds?*  
> **Answer:** A fixed tolerance would behave differently for small and large CAD outlines.

- **Hands-On Lab Experiment:** Trace a planar loop, a collapsed loop and a surface with a non-finite control point through their separate checks.

<details>
<summary>Implementation 77zzh-check-original-loops-and-surface-controls: Check original loops and surface controls</summary>

Find a loop’s unit normal and scale-aware planarity tolerance.

Data flow: original curves → consistent section frames → validated kernel surface.

### 1. `src/surfacing.rs`

Type loop geometry, planarity and finite surface validation.

Create the file and type:

```rust
--8<-- "journey/code/77zzh-check-original-loops-and-surface-controls-1.rs"
```

### 2. `src/lib.rs`

Compile the shared surfacing owner while its recipes are prepared.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn validate_source_json(text: &str) -> Result<(), JsValue> {
    source_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;
```

</details>

After that block, type:

```rust

pub mod surfacing;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzh-check-original-loops-and-surface-controls-2.rs"
```

</details>

</details>

<a id="checkpoint-77zzi-align-original-sections-and-loft-through-them"></a>

### Step 43 of 49 · Align original sections and loft through them (`77zzi-align-original-sections-and-loft-through-them`)

- **Goal:** Align open curve directions and closed curve seams before asking the Session kernel for a loft.
- **Data Flow Trace:** `original curves → consistent section frames → validated kernel surface`
- **Files You Touch:** `src/surfacing.rs` (*Add section alignment and lofting before the existing result validator.*)

Align open curve directions and closed curve seams before asking the Session kernel for a loft. Keep section count and degree rules in the same helper. Supply the original validation fixtures separately; the construction recipes connect this owner next.

> **Self-Check Question:** *Why align sections before building the loft?*  
> **Answer:** Opposite directions or mismatched closed seams can twist the surface between otherwise valid original curves.

- **Hands-On Lab Experiment:** Trace two open sections with opposite directions, then a closed section with a shifted seam. The existing browser drawing remains unchanged until the following recipe connection.

<details>
<summary>Implementation 77zzi-align-original-sections-and-loft-through-them: Align original sections and loft through them</summary>

Align open curve directions and closed curve seams before asking the Session kernel for a loft.

Data flow: original curves → consistent section frames → validated kernel surface.

### 1. `src/surfacing.rs`

Add section alignment and lofting before the existing result validator.

<details>
<summary>Locate the existing block</summary>

```rust
/// The surface as a named geometry, when valid with finite control points.
pub fn checked(mut surface: NurbsSurface, name: &str) -> Result<Geometry, String> {
    let finite = (0..surface.cv_count(0)).all(|i| {
        (0..surface.cv_count(1)).all(|j| {
            surface
                .get_cv(i, j)
                .is_some_and(|p| (0..3).all(|k| p[k].is_finite()))
        })
    });

    if !surface.is_valid() || !finite {
        return Err(format!("The {name} surface could not be built"));
    }

    surface.name = name.into();
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}

/// The world curves of one step.
pub fn curves(picked: &[Picked]) -> Vec<NurbsCurve> {
    picked.iter().map(|picked| picked.curve.clone()).collect()
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77zzi-align-original-sections-and-loft-through-them-1.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/surfacing.rs`

Supply the five original surfacing validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
))
}

/// The world curves of one step.
pub fn curves(picked: &[Picked]) -> Vec<NurbsCurve> {
    picked.iter().map(|picked| picked.curve.clone()).collect()
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77zzi-align-original-sections-and-loft-through-them-2.rs" skip=160
```

</details>

</details>

<a id="checkpoint-77zzj-find-extrusion-axes-and-cap-eligibility"></a>

### Step 44 of 49 · Find extrusion axes and cap eligibility (`77zzj-find-extrusion-axes-and-cap-eligibility`)

- **Goal:** Reuse the original loop normal and planarity helpers to choose an extrusion axis.
- **Data Flow Trace:** `original curves → offset and cap rules → shared BRep topology`
- **Files You Touch:** `src/extrude_geometry.rs` (*Type the shared extrusion axis, orientations and cap eligibility.*) · `src/lib.rs` (*Compile the original extrusion helpers before their command registration.*)

Reuse the original loop normal and planarity helpers to choose an extrusion axis. Orient that axis with the construction plane. Classify open, bent and flat outlines before building a cap, keeping the reason separate from geometry construction.

> **Self-Check Question:** *Why retain the reason an outline cannot be capped?*  
> **Answer:** An open extrusion may still be valid. The command can report the missing cap without discarding its side geometry.

- **Hands-On Lab Experiment:** Trace an open curve, a bent loop and an offset parallel to the outline plane.

<details>
<summary>Implementation 77zzj-find-extrusion-axes-and-cap-eligibility: Find extrusion axes and cap eligibility</summary>

Reuse the original loop normal and planarity helpers to choose an extrusion axis.

Data flow: original curves → offset and cap rules → shared BRep topology.

### 1. `src/extrude_geometry.rs`

Type the shared extrusion axis, orientations and cap eligibility.

Create the file and type:

```rust
--8<-- "journey/code/77zzj-find-extrusion-axes-and-cap-eligibility-1.rs"
```

### 2. `src/lib.rs`

Compile the original extrusion helpers before their command registration.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn validate_source_json(text: &str) -> Result<(), JsValue> {
    source_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;

pub mod surfacing;
```

</details>

After that block, type:

```rust

pub mod extrude_geometry;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzj-find-extrusion-axes-and-cap-eligibility-2.rs"
```

</details>

</details>

<a id="checkpoint-77zzk-map-cap-curves-to-surface-parameters"></a>

### Step 45 of 49 · Map cap curves to surface parameters (`77zzk-map-cap-curves-to-surface-parameters`)

- **Goal:** Build a planar cap patch around the original control points.
- **Data Flow Trace:** `original curves → offset and cap rules → shared BRep topology`
- **Files You Touch:** `src/extrude_geometry.rs` (*Extend this original owner with the day’s whole extrusion helpers.*)

Build a planar cap patch around the original control points. Map its boundary into surface parameters, determine its winding and attach the boundary’s pcurve to the existing BRep edge. Reuse that edge when creating the cap face.

> **Self-Check Question:** *Why reuse the existing boundary edge?*  
> **Answer:** The side and cap must share one topological boundary. Two coincident edges would leave separate faces rather than a connected shell.

- **Hands-On Lab Experiment:** Sketch one shared edge and its two surface-parameter curves. Trace the cap orientation through its winding.

<details>
<summary>Implementation 77zzk-map-cap-curves-to-surface-parameters: Map cap curves to surface parameters</summary>

Build a planar cap patch around the original control points.

Data flow: original curves → offset and cap rules → shared BRep topology.

### 1. `src/extrude_geometry.rs`

Extend this original owner with the day’s whole extrusion helpers.

<details>
<summary>Locate the existing block</summary>

```rust
    let along = normal.dot(offset);

    if along.abs() <= 1e-9 * offset.magnitude() {
        return Err(Skip::Flat);
    }

    Ok(if along < 0.0 { -normal } else { normal })
}
```

</details>

After that block, type:

```rust
/// A straight pcurve from (u0, v0) to (u1, v1).
fn uv_line(u0: f64, v0: f64, u1: f64, v1: f64) -> NurbsCurve {
    NurbsCurve::create(
        false,
        1,
        &[Point::new(u0, v0, 0.0), Point::new(u1, v1, 0.0)],
    )
}

/// A flat patch in the plane through `origin` with outward `normal` that covers `points`.
fn cap_patch(points: &[Point], origin: &Point, normal: &Vector) -> NurbsSurface {
    let mut x = normal.cross(&Vector::new(0.0, 0.0, 1.0));

    if !x.normalize_self() {
        x = Vector::new(1.0, 0.0, 0.0);
    }

    let y = normal.cross(&x);
    let (mut low, mut high) = ([f64::MAX; 2], [f64::MIN; 2]);

    for p in points {
        let d = p - origin;
        let (u, v) = (d.dot(&x), d.dot(&y));
        low = [low[0].min(u), low[1].min(v)];
        high = [high[0].max(u), high[1].max(v)];
    }

    let pad = 0.01 * (high[0] - low[0]).max(high[1] - low[1]);
    let corner = |u: f64, v: f64| origin + &(&x * u + &y * v);
    let (u0, v0, u1, v1) = (low[0] - pad, low[1] - pad, high[0] + pad, high[1] + pad);
    let points = [
        corner(u0, v0),
        corner(u0, v1),
        corner(u1, v0),
        corner(u1, v1),
    ];
    NurbsSurface::create(false, false, 1, 1, 2, 2, &points).unwrap_or_default()
}

/// The curve in the patch's parameters: the affine image of its control points.
fn on_patch(curve: &NurbsCurve, patch: &NurbsSurface) -> NurbsCurve {
    let p00 = patch.get_cv(0, 0).unwrap_or_default();
    let eu = &patch.get_cv(1, 0).unwrap_or_default() - &p00;
    let ev = &patch.get_cv(0, 1).unwrap_or_default() - &p00;
    let ((u0, u1), (v0, v1)) = (
        patch.domain(0).unwrap_or((0.0, 1.0)),
        patch.domain(1).unwrap_or((0.0, 1.0)),
    );
    let mut flat = curve.clone();

    for i in 0..curve.cv_count() {
        let (wx, wy, wz, w) = curve.get_cv_4d(i).unwrap_or((0.0, 0.0, 0.0, 1.0));
        let d = &Point::new(wx / w, wy / w, wz / w) - &p00;
        let u = u0 + (u1 - u0) * d.dot(&eu) / eu.dot(&eu);
        let v = v0 + (v1 - v0) * d.dot(&ev) / ev.dot(&ev);
        flat.set_cv_4d(i, u * w, v * w, 0.0, w);
    }

    flat
}

/// Twice the signed area a closed pcurve encloses, counter-clockwise positive.
fn winding(curve: &NurbsCurve) -> f64 {
    let points = curve
        .divide_by_count((curve.cv_count() * 4).max(16), true)
        .0;
    points
        .windows(2)
        .map(|pair| pair[0][0] * pair[1][1] - pair[1][0] * pair[0][1])
        .sum()
}

/// A cap face bounded by one closed edge lying on `patch`.
fn cap_face(brep: &mut BRep, patch: &NurbsSurface, curve: &NurbsCurve, edge: usize) -> usize {
    let surface = brep.add_surface(patch);
    let flat = on_patch(curve, patch);
    let orientation = if winding(&flat) > 0.0 { F } else { R };
    let pcurve = brep.add_curve_2d(&flat);
    brep.add_pcurve(edge, surface, pcurve as i32, -1);
    let wire = brep.add_wire(&[BRepRef::new(edge as i32, orientation)]);
    brep.add_face(surface as i32, &[BRepRef::new(wire as i32, F)], 0.0)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzk-map-cap-curves-to-surface-parameters-1.rs"
```

</details>

</details>

<a id="checkpoint-77zzl-build-closed-curved-extrusion-topology"></a>

### Step 46 of 49 · Build closed curved extrusion topology (`77zzl-build-closed-curved-extrusion-topology`)

- **Goal:** Build the side surface from an oriented original curve.
- **Data Flow Trace:** `original curves → offset and cap rules → shared BRep topology`
- **Files You Touch:** `src/extrude_geometry.rs` (*Extend this original owner with the day’s whole extrusion helpers.*)

Build the side surface from an oriented original curve. Share the lower, upper and seam edges, attach their surface-parameter curves, and join the side and both caps into one solid. Preserve the original curve precision throughout.

> **Self-Check Question:** *Why does the seam edge need two parameter curves?*  
> **Answer:** The periodic side meets the same three-dimensional seam at both ends of its surface domain. Each side of that seam needs its own parameter location.

- **Hands-On Lab Experiment:** Trace the seam through the side wire twice, once forward and once reversed. These helpers remain preparatory until the recipe connects.

<details>
<summary>Implementation 77zzl-build-closed-curved-extrusion-topology: Build closed curved extrusion topology</summary>

Build the side surface from an oriented original curve.

Data flow: original curves → offset and cap rules → shared BRep topology.

### 1. `src/extrude_geometry.rs`

Extend this original owner with the day’s whole extrusion helpers.

<details>
<summary>Locate the existing block</summary>

```rust
    brep.add_pcurve(edge, surface, pcurve as i32, -1);
    let wire = brep.add_wire(&[BRepRef::new(edge as i32, orientation)]);
    brep.add_face(surface as i32, &[BRepRef::new(wire as i32, F)], 0.0)
}
```

</details>

After that block, type:

```rust
/// A closed planar curve extruded into a solid: one side face with a seam and two flat caps.
fn capped(curve: &NurbsCurve, offset: &Vector, normal: &Vector) -> Result<BRep, String> {
    let mut bottom = curve.clone();

    // the side faces outward when the curve turns counter-clockwise about the offset
    let outline = bottom.divide_by_count(64, false).0;

    if loop_normal(&outline).is_some_and(|own| own.dot(normal) < 0.0) {
        bottom.reverse();
    }

    bottom.set_domain(0.0, 1.0);
    let top = bottom.transformed(&Xform::translation(offset[0], offset[1], offset[2]));
    let body = Primitives::create_extrusion(&bottom, offset);

    if !body.is_valid() {
        return Err("The curve could not be extruded".into());
    }

    let mut brep = BRep::new();
    let low = bottom.point_at_start();
    let high = &low + offset;
    let start = brep.add_vertex(&low, 0.0) as i32;
    let end = brep.add_vertex(&high, 0.0) as i32;
    let curve_low = brep.add_curve_3d(&bottom) as i32;
    let edge_low = brep.add_edge(curve_low, start, start);
    let curve_high = brep.add_curve_3d(&top) as i32;
    let edge_high = brep.add_edge(curve_high, end, end);
    let curve_seam = brep.add_curve_3d(&NurbsCurve::create(false, 1, &[low.clone(), high])) as i32;
    let edge_seam = brep.add_edge(curve_seam, start, end);
    let surface = brep.add_surface(&body);
    let (u0, u1) = body.domain(0).unwrap_or((0.0, 1.0));
    let (v0, v1) = body.domain(1).unwrap_or((0.0, 1.0));
    let pcurve_low = brep.add_curve_2d(&uv_line(u0, v0, u1, v0));
    brep.add_pcurve(edge_low, surface, pcurve_low as i32, -1);
    let pcurve_high = brep.add_curve_2d(&uv_line(u0, v1, u1, v1));
    brep.add_pcurve(edge_high, surface, pcurve_high as i32, -1);
    let right = brep.add_curve_2d(&uv_line(u1, v0, u1, v1));
    let left = brep.add_curve_2d(&uv_line(u0, v0, u0, v1));
    brep.add_pcurve(edge_seam, surface, right as i32, left as i32);
    let wire = brep.add_wire(&[
        BRepRef::new(edge_low as i32, F),
        BRepRef::new(edge_seam as i32, F),
        BRepRef::new(edge_high as i32, R),
        BRepRef::new(edge_seam as i32, R),
    ]);
    let side = brep.add_face(surface as i32, &[BRepRef::new(wire as i32, F)], 0.0);
    let points: Vec<Point> = (0..bottom.cv_count())
        .filter_map(|i| bottom.get_cv(i))
        .collect();
    let lifted: Vec<Point> = points.iter().map(|p| p + offset).collect();
    let floor = cap_face(
        &mut brep,
        &cap_patch(&points, &low, &-normal),
        &bottom,
        edge_low as usize,
    );
    let roof = cap_face(
        &mut brep,
        &cap_patch(&lifted, &(&low + offset), normal),
        &top,
        edge_high as usize,
    );
    let shell = brep.add_shell(&[
        BRepRef::new(side as i32, F),
        BRepRef::new(floor as i32, F),
        BRepRef::new(roof as i32, F),
    ]);
    brep.add_solid(&[BRepRef::new(shell as i32, F)]);
    Ok(brep)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzl-build-closed-curved-extrusion-topology-1.rs"
```

</details>

</details>

<a id="checkpoint-77zzm-extrude-original-corners-and-curves"></a>

### Step 47 of 49 · Extrude original corners and curves (`77zzm-extrude-original-corners-and-curves`)

- **Goal:** Dispatch original corner chains and curved sections through the shared extrusion helpers.
- **Data Flow Trace:** `original curves → offset and cap rules → shared BRep topology`
- **Files You Touch:** `src/extrude_geometry.rs` (*Extend this original owner with the day’s whole extrusion helpers.*)

Dispatch original corner chains and curved sections through the shared extrusion helpers. Build polygon sides from their original points, add eligible caps and validate the resulting BRep. Curved inputs use the kernel surface or the closed topology builder.

> **Self-Check Question:** *Why preserve the original curve when building a curved extrusion?*  
> **Answer:** Sampling it into a polygon would change the source geometry and its later editable control points.

- **Hands-On Lab Experiment:** Compare a polygon outline with a curved outline and trace Cap On and Cap Off through each branch.

<details>
<summary>Implementation 77zzm-extrude-original-corners-and-curves: Extrude original corners and curves</summary>

Dispatch original corner chains and curved sections through the shared extrusion helpers.

Data flow: original curves → offset and cap rules → shared BRep topology.

### 1. `src/extrude_geometry.rs`

Extend this original owner with the day’s whole extrusion helpers.

<details>
<summary>Locate the existing block</summary>

```rust
    let along = normal.dot(offset);

    if along.abs() <= 1e-9 * offset.magnitude() {
        return Err(Skip::Flat);
    }

    Ok(if along < 0.0 { -normal } else { normal })
}
```

</details>

After that block, type:

```rust
/// One picked curve extruded by `offset`, capped when asked and possible.
fn extrude(picked: &Picked, offset: &Vector, cap: bool) -> Result<(Geometry, Skip), String> {
    let named = |mut brep: BRep| {
        brep.name = "extrusion".into();
        Geometry::BRep(Rc::new(brep))
    };

    match &picked.corners {
        Some(corners) if corners.len() > 2 => {
            let closed = corners.len() >= 4
                && corners[0].distance(&corners[corners.len() - 1], None) <= 1e-12;
            let lift = |p: &Point| p + offset;
            let mut faces: Vec<Polyline> = corners
                .windows(2)
                .map(|pair| {
                    Polyline::new(vec![
                        pair[0].clone(),
                        pair[1].clone(),
                        lift(&pair[1]),
                        lift(&pair[0]),
                        pair[0].clone(),
                    ])
                })
                .collect();
            let skip = match cap_normal(&corners[..corners.len() - 1], closed, offset) {
                Ok(_) if cap => {
                    faces.push(Polyline::new(corners.clone()));
                    faces.push(Polyline::new(corners.iter().map(lift).collect()));
                    Skip::None
                }
                Ok(_) => Skip::None,
                Err(skip) => skip,
            };
            let brep = BRep::from_polylines(&faces, &[]);

            if brep.face_count() == 0 {
                return Err("The polyline could not be extruded".into());
            }

            Ok((named(brep), skip))
        }
        _ => {
            let outline = picked.outline();
            let normal = cap_normal(&outline, picked.curve.is_closed(), offset);

            if let (true, Ok(normal)) = (cap && picked.corners.is_none(), &normal) {
                return Ok((named(capped(&picked.curve, offset, normal)?), Skip::None));
            }

            let surface = Primitives::create_extrusion(&picked.curve, offset);
            let skip = normal.err().unwrap_or(Skip::None);
            Ok((checked(surface, "extrusion")?, skip))
        }
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzm-extrude-original-corners-and-curves-1.rs"
```

</details>

</details>

<a id="checkpoint-77zzn-connect-the-original-extrusion-recipe"></a>

### Step 48 of 49 · Connect the original extrusion recipe (`77zzn-connect-the-original-extrusion-recipe`)

- **Goal:** Complete the original recipe with curve selection, signed distance and cap choices.
- **Data Flow Trace:** `original curves → offset and cap rules → shared BRep topology`
- **Files You Touch:** `src/extrude_geometry.rs` (*Extend this original owner with the day’s whole extrusion helpers.*)

Complete the original recipe with curve selection, signed distance and cap choices. Validate the offset before building every selected original as one result. Keep omitted-cap messages and the full output in the same undo transaction.

> **Self-Check Question:** *Why validate the offset before building any selected curve?*  
> **Answer:** A non-finite or zero offset must reject the whole candidate before any partial result can be published.

- **Hands-On Lab Experiment:** Trace two selected curves through one extrusion result and one Undo. The existing command registry connects this prepared recipe in the following checkpoint.

<details>
<summary>Implementation 77zzn-connect-the-original-extrusion-recipe: Connect the original extrusion recipe</summary>

Complete the original recipe with curve selection, signed distance and cap choices.

Data flow: original curves → offset and cap rules → shared BRep topology.

### 1. `src/extrude_geometry.rs`

Extend this original owner with the day’s whole extrusion helpers.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    ]);
    brep.add_solid(&[BRepRef::new(shell as i32, F)]);
    Ok(brep)
```

</details>

Replace it with:

```rust
    ]);
    brep.add_solid(&[BRepRef::new(shell as i32, F)]);
    Ok(brep)
}

/// Every picked curve extruded by the answered distance, as one undo step.
fn build(input: &Input) -> Result<Made, String> {
    let (_, direction) = axis(input);
    let offset = input
        .along(0, &direction)
        .ok_or("Extrude needs a distance")?;

    if !(offset.magnitude() > 1e-12 && (0..3).all(|i| offset[i].is_finite())) {
        return Err("The extrusion distance must not be zero".into());
    }

    let cap = input.choice != "Cap Off";
    let mut geometries = Vec::new();
    let mut skipped = [0; 3];

    for picked in &input.curves[0] {
        let (geometry, skip) = extrude(picked, &offset, cap)?;

        match skip {
            Skip::Open if cap => skipped[0] += 1,
            Skip::Bent if cap => skipped[1] += 1,
            Skip::Flat if cap => skipped[2] += 1,
            _ => {}
        }

        geometries.push(geometry);
    }

    let solids = geometries
        .iter()
        .filter(|geometry| matches!(geometry, Geometry::BRep(brep) if brep.is_solid()))
        .count();
    let mut message = format!(
        "Extruded {}: {}, {}",
        count(geometries.len(), "curve"),
        count(solids, "solid"),
        count(geometries.len() - solids, "open surface")
    );

    for (number, why) in skipped
        .iter()
        .zip(["open", "not planar", "along its plane"])
    {
        if *number > 0 {
            message.push_str(&format!(" · no cap on {} ({why})", count(*number, "curve")));
        }
    }

    Ok(Made {
        geometries,
        message,
    })
```

<details>
<summary>Locate this block</summary>

```rust
const F: BRepOrientation = BRepOrientation::Forward;
const R: BRepOrientation = BRepOrientation::Reversed;
```

</details>

Replace it with:

```rust
pub static RECIPE: Recipe = Recipe {
    name: "Extrude",
    chips: &[
        ("Cap On", "Cap On"),
        ("Cap Off", "Cap Off"),
        ("Finish", ""),
        ("Cancel", "Escape"),
    ],
    steps: &[
        Step::Curves {
            prompt: "select curves to extrude",
            min: 1,
            max: usize::MAX,
        },
        Step::Distance("distance"),
    ],
    axis,
    build,
};

const F: BRepOrientation = BRepOrientation::Forward;
const R: BRepOrientation = BRepOrientation::Reversed;
```

<details>
<summary>Complete updated block reference</summary>

```rust
--8<-- "journey/code/77zzn-connect-the-original-extrusion-recipe-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/extrude_geometry.rs`

Supply the four original extrusion validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
/// Every picked curve extruded by the answered distance, as one undo step.
fn build(input: &Input) -> Result<Made, String> {
    let (_, direction) = axis(input);
    let offset = input
        .along(0, &direction)
        .ok_or("Extrude needs a distance")?;

    if !(offset.magnitude() > 1e-12 && (0..3).all(|i| offset[i].is_finite())) {
        return Err("The extrusion distance must not be zero".into());
    }

    let cap = input.choice != "Cap Off";
    let mut geometries = Vec::new();
    let mut skipped = [0; 3];

    for picked in &input.curves[0] {
        let (geometry, skip) = extrude(picked, &offset, cap)?;

        match skip {
            Skip::Open if cap => skipped[0] += 1,
            Skip::Bent if cap => skipped[1] += 1,
            Skip::Flat if cap => skipped[2] += 1,
            _ => {}
        }

        geometries.push(geometry);
    }

    let solids = geometries
        .iter()
        .filter(|geometry| matches!(geometry, Geometry::BRep(brep) if brep.is_solid()))
        .count();
    let mut message = format!(
        "Extruded {}: {}, {}",
        count(geometries.len(), "curve"),
        count(solids, "solid"),
        count(geometries.len() - solids, "open surface")
    );

    for (number, why) in skipped
        .iter()
        .zip(["open", "not planar", "along its plane"])
    {
        if *number > 0 {
            message.push_str(&format!(" · no cap on {} ({why})", count(*number, "curve")));
        }
    }

    Ok(Made {
        geometries,
        message,
    })
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77zzn-connect-the-original-extrusion-recipe-2.rs" skip=1531
```

</details>

</details>

<a id="checkpoint-77zzo-revolve-original-profiles-around-an-axis"></a>

### Step 49 of 49 · Revolve original profiles around an axis (`77zzo-revolve-original-profiles-around-an-axis`)

- **Goal:** Turn original curves into exact kernel surfaces around a chosen axis.
- **Data Flow Trace:** `original profile → axis and angle → kernel surface → one result`
- **Files You Touch:** `src/revolve_geometry.rs` (*Type the original axis validation, exact revolution and complete recipe result.*) · `src/lib.rs` (*Compile the prepared original revolution recipe.*)

Describe one shared recipe for profile selection, two axis points and a bounded angle. Reject a zero-length axis before calling the kernel. Convert degrees to radians once, validate every resulting surface, and return all selected profiles in one undo transaction.

> **Self-Check Question:** *Why revolve the original curve instead of its display polyline?*  
> **Answer:** The kernel retains its exact rational surface and editable source geometry. The display polyline is only a tessellation for drawing.

- **Hands-On Lab Experiment:** Trace a 90-degree turn and a complete turn of a straight profile. Compare the surface seam and radius. The following registry checkpoint connects this prepared recipe to the browser command.

<details>
<summary>Implementation 77zzo-revolve-original-profiles-around-an-axis: Revolve original profiles around an axis</summary>

Turn original curves into exact kernel surfaces around a chosen axis.

Data flow: original profile → axis and angle → kernel surface → one result.

### 1. `src/revolve_geometry.rs`

Type the original axis validation, exact revolution and complete recipe result.

Create the file and type:

```rust
--8<-- "journey/code/77zzo-revolve-original-profiles-around-an-axis-1.rs"
```

### 2. `src/lib.rs`

Compile the prepared original revolution recipe.

<details>
<summary>Locate the existing block</summary>

```rust
    source_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;

pub mod surfacing;

pub mod extrude_geometry;
```

</details>

After that block, type:

```rust

pub mod revolve_geometry;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzo-revolve-original-profiles-around-an-axis-2.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/revolve_geometry.rs`

Supply the unchanged original radius, seam and axis validation fixture.

<details>
<summary>Locate the existing block</summary>

```text
use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{checked, count};
use session_rust::{NurbsCurve, NurbsSurface, Point, Primitives};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface Revolve",
    chips: &[("Finish", ""), ("Cancel", "Escape")],
    steps: &[
        Step::Curves {
            prompt: "select profile curves",
            min: 1,
            max: usize::MAX,
        },
        Step::Point("axis start"),
        Step::Point("axis end"),
        Step::Number {
            prompt: "angle",
            default: 360.0,
            low: 0.0,
            high: 360.0,
        },
    ],
    axis: crate::gather_input::no_axis,
    build,
};

/// `profile` turned `degrees` right-handed about the axis from `start` to `end`.
pub fn revolve(
    profile: &NurbsCurve,
    start: &Point,
    end: &Point,
    degrees: f64,
) -> Result<NurbsSurface, String> {
    let axis = end - start;

    if axis.magnitude() <= 1e-12 {
        return Err("The axis start and end must differ".into());
    }

    if !(degrees > 0.0 && degrees <= 360.0) {
        return Err("The angle must be above 0 and at most 360".into());
    }

    Ok(Primitives::create_revolve(
        profile,
        start,
        &axis,
        degrees.to_radians(),
    ))
}

/// One surface per profile, all in one undo step.
fn build(input: &Input) -> Result<Made, String> {
    let (Some(start), Some(end), Some(degrees)) = (input.point(0), input.point(1), input.number(2))
    else {
        return Err("Nurbs Surface Revolve needs an axis and an angle".into());
    };
    let geometries = input.curves[0]
        .iter()
        .map(|picked| checked(revolve(&picked.curve, &start, &end, degrees)?, "revolve"))
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Made {
        message: format!(
            "Revolved {} by {degrees}°",
            count(geometries.len(), "curve")
        ),
        geometries,
    })
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77zzo-revolve-original-profiles-around-an-axis-3.rs" skip=1946
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 77zzo-revolve-original-profiles-around-an-axis
npm --prefix ../session_tests run course -- check 77zzo-revolve-original-profiles-around-an-axis
npm --prefix ../session_tests run course -- save 77zzo-revolve-original-profiles-around-an-axis
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

- **Final Lesson Experiment:** Trace a 90-degree turn and a complete turn of a straight profile. Compare the surface seam and radius. The following registry checkpoint connects this prepared recipe to the browser command.

Continue with [Profile collection and one surfacing transaction](../../consolidation/78-copy-current-curve-collection-and-surfacing-recipes.md).
