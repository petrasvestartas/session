# Research: Potree-class techniques for CAD (2026-08-24)

Three GitHub/literature sweeps: (a) heavy 2D drawings, (b) large meshes, (c) picking/selection/editing.
Compiled from agent research; repos verified active unless noted. See per-section sources.

## a) Heavy 2D CAD drawings

1. **vello** (github.com/linebender/vello) — Rust+wgpu, WGSL compute vector rasterizer
   (binning → coarse → fine over 16x16 tiles, prefix-sum everything). 177 fps on a 30k-path
   scene; GPU-side stroking with real joins/caps. Watch **vello_hybrid "sparse strips"**
   (issue #670): CPU flattens to run-length coverage strips, GPU composites — strips are
   RETAINABLE, i.e. a cached rasterized sheet re-composites for free on pan. Best fit as the
   sheet rasterizer feeding impostor tiles.
2. **Sheet impostors / tile+LOD** (maplibre architecture, pdf.js tiling): rasterize a sheet
   once into a texture/tile pyramid, draw one quad; re-rasterize visible tiles on zoom
   threshold. Turns "N sheets x 350k segments" into "visible tiles of ONE sheet" per frame.
   Also: maplibre-tile-spec (Jan 2026) for columnar/delta-encoded vector geometry on disk.
3. **m-schuetz/webgpu_wireframe_thicklines** — vertex pulling: draw(6*segments), no vertex
   buffer, endpoints from storage; same philosophy as our splat lane. ~1 WGSL shader to port.
4. **Batch-AABB indirect culling** (nvpro gl_occlusion_culling recipe): 4k-segment batches
   with AABBs, compute pass writes indirect draw args. We have all the infrastructure.
5. Dead/skip: pathfinder (unmaintained, no wgpu — but steal its tile-occlusion idea:
   opaque tiles kill everything beneath), forma (archived, no strokes), rive-renderer
   (C++ pixel-local storage, not portable to wgpu), MSDF (glyphs only).
6. Compute-rasterize segments at far zoom (Schutz-style DDA + atomics, reusing our splat
   pixel buffers): kills hatching overdraw the same way splatting killed cloud fit-views.

## b) Large meshes

Effort ladder (from Bevy virtual geometry, nanite-webgpu, meshoptimizer, nvpro):
1. **Quantized vertex formats** (days): snorm16 positions dequantized by object/meshlet box
   + oct-encoded normals in one u32 → 32 B/vertex → 12-16 B. wgpu supports natively.
   Bevy PR #15643 measured 109.97 → 63.61 MB (~42%) with identical quality.
2. **Meshlets + per-cluster cull** (1-2 weeks): meshopt::build_meshlets (~64v/124t, cone +
   bounds out), one compute pass frustum/cone-tests clusters → compacted indirect draws.
   nvpro's CAD demo (gl_vk_meshlet_cadscene): 4.0-4.6 ms → 1.0-1.3 ms on dense parts.
   SKIP the full Nanite LOD DAG (METIS, seam locking — months); use 2-3 discrete
   meshopt_simplify levels picked by screen size (~50 lines).
3. **HiZ occlusion culling** (1-2 weeks): depth pyramid from last frame, test AABBs, two-phase
   variant = Bevy PR #17413 (canonical wgpu implementation). nanite-webgpu does the simpler
   single-phase (one-frame popping). Biggest win in high-depth-complexity assemblies.
4. **meshopt codecs for transport** (gltfpack -cc / EXT_meshopt_compression): ~3x file size,
   3-6 GB/s decode, per-bufferView = streamable. Skip Draco (slow decode, scrambles order).
5. **3D-Tiles-style chunked LOD streaming** for scans too big for RAM: octree chunks, per-chunk
   LODs, refine by screen-space geometric error. The mesh analog of Potree's octree.
Key repos: Scthe/nanite-webgpu (WebGPU Nanite clone: 1.7B tris interactive in Chrome; documents
the no-64-bit-atomics + 128MB-buffer pains we know), Bevy meshlet module (only production-ish
Rust+wgpu implementation; MIT), zeux/meshoptimizer (+ meshopt crate).

## c) Picking / selection / editing

1. **three-mesh-bvh** (gkjohnson) — the ideas to port to our scene BVH:
   - 32-byte packed nodes, implicit left child (next in buffer) → Vec<Node>, zero pointers;
   - ONE `shapecast(intersectsBounds, intersectsTriangle)` visitor powers ray/box/marquee;
     marquee accepts WHOLE subtrees fully inside the rect — that's what makes it cheap;
   - `refit(hints)` bottom-up during drags, full rebuild on commit;
   - `indirect` triangle-index buffer so the BVH never permutes the GPU index buffer.
   Numbers: 500 rays vs 80k tris @60fps.
2. **GPU id-buffer picking** (webgpufundamentals lesson; Bevy gpu_readback is the wgpu code
   to crib): r32uint id target, copy 1px (hover) or the marquee rect, ring of 2-3 mapped
   buffers → 1-2 frame latency, zero stalls. Complement to CPU ray (which keeps topological
   resolution) — marquee via id-rect is O(pixels), independent of object count.
3. **SolveSpace** hit-testing UX (draw.cpp/mouse.cpp): 10 px pick radius, z-priority
   points > lines > faces, 3 px drag threshold, marquee counts crossing segments. ~200 lines,
   no cleverness — the Rhino feel.
4. **Entity resolution**: emit per-triangle face-id / per-segment edge-id arrays at
   tessellation time → a hit resolves to a topological entity O(1) (CADmium/truck lesson).
5. **Arena**: Bevy MeshAllocator pattern — growable slabs + pcwalton/offset-allocator crate
   (O(1) alloc/free), dedicated buffer above a size threshold, GPU-side copies, NON-relocating
   growth (PR #17793) so offsets stay valid. Measured: frame 8.74 → 5.53 ms on Bistro.
   Transforms in a separate matrix buffer = 64-byte writes, geometry untouched (= our design).
6. Read for architecture, not code: Fornjot fj-core (immutable topology, Handle<T>, edits
   reuse unchanged sub-objects), blackjack (halfedge → wgpu regen), OpenCADStudio (young,
   active Rust/wgpu CAD to watch).

## Mapping onto our lesson chain
- 52/53 (BVH/frustum): adopt three-mesh-bvh node layout + shapecast + subtree-accept marquee.
- 48 (arena): offset-allocator + slab pattern IS our per-object arena lesson, with numbers.
- 55-58 (picking): CPU ray stays primary; id-buffer pass is the marquee/hover upgrade (58+).
- 73-76 (GPU tessellation): meshlet/quantization ladder slots after; HiZ = 88's lever 2.
- Sheets: impostor tiles + vello_hybrid strips = the future heavy-sheet lesson; batch culling
  first, measure at 53 before building any of it.
