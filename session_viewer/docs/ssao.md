# Ambient occlusion

Type `Arctic`, press Enter, then choose On or Off; `Arctic On` and `Arctic Off` also run directly, and G toggles it when the viewport has keyboard focus. Arctic starts off, and switching it never moves the camera.

Arctic keeps authored colours under neutral sky and ground lighting, with soft contact shadows on surfaces and on a virtual floor under the lowest visible solid. Hidden objects, sheets, lines and points do not lower that floor. It is a screen-space effect: an object that is hidden or off screen casts nothing.

Turning Arctic on also turns the black surface outlines on. `Outline Off` hides them and keeps the shading, `Outline On` shows them without Arctic, and O toggles them. Turning Arctic off leaves the outline setting as it is. Clipping sections draw their own cut boundaries whatever the outline setting; see [lesson 18b](18b-clipping.md).

The code is built in [lesson 32](32-colors-lighting.md) (the pass and its shaders) and [lesson 33](33-contact-shadows.md) (the `Arctic` and `Outline` commands).

## Navigation

The same occlusion runs during orbit, pan, zoom and still frames: drag tiers never switch it off or lower its resolution. While Arctic is on, the drag tier stays at full quality for lines and outlines too, and the automatic canvas-scale and MSAA downgrade after slow frames is held back, so nothing thins while moving and thickens on release. That can cost more than the cheaper drag rendering, especially in line-heavy scenes.

While the camera moves, the y blur reprojects last frame's shading onto the same surface points to reduce shimmer. A depth check rejects newly exposed geometry, and the reused value is clamped to the range of its neighbours. A geometry edit, a toggle or a resize discards the history. When the camera stops, the last image stays: there is no settling frame.

A frame that can reuse history evaluates every second horizon slice and every second ground direction, chosen by pixel position, so the blur's neighbourhood still sees every direction in one frame. A frame without history, the first after a toggle or an edit, evaluates all of them. The pattern is fixed in screen space, because a pattern that changes per frame shimmers through the history blend.

Surface occlusion runs at `clamp(1 / DPR, 0.25, 0.5)` times the canvas size, about one AO pixel per CSS pixel: 960 x 540 for a 1920 x 1080 canvas at DPR 1. The ground field uses half that resolution with more directions, and the surface pass samples it. These resolutions stay fixed while navigating.

## Rendering

- Four cosine-weighted horizon slices, six steps to each side, estimate how much sky a surface point sees, read from a six-level linear-depth pyramid. Triangle normals stay faceted.
- Ground contacts use twenty-four directions of six steps, with height and distance fades, and keep each object's radius, 5% of its half-diagonal.
- The pyramid stores, beside depth, each pixel's radius and its place in its 4 x 4 block, so a coarse level still rebuilds the exact occluder position. The first level also stores an octahedral RG8 normal, which the horizon and blur passes read; the full-resolution upsample derives exact per-sample normals from the triangles.
- Two bilateral blurs, reprojection and a depth-aware upsample produce a full-resolution R8 cache that a blended full-screen draw lays over the faces.
- At 4x MSAA the cache holds the lightest of a pixel's samples. Samples that see another surface store their difference in a correction buffer, and an indirect draw shades them only in the 16 x 16 tiles that need it. Both blends happen before the MSAA resolve.
- Only the screen rectangle around the visible solids, plus a margin for the 64 px ground reach, is shaded. A still camera and unchanged geometry reuse the cache and only blend it.
- In the browser, the 1x and 4x pipelines compile after geometry appears, in idle callbacks, or after a 50 ms timeout where `requestIdleCallback` is missing. They survive toggles and resizes; camera movement creates no textures, bind groups or pipelines.

The horizon integral follows [Jimenez et al., Practical Real-Time Strategies for Accurate Indirect Occlusion](https://www.activision.com/cdn/research/Practical_Real_Time_Strategies_for_Accurate_Indirect_Occlusion_NEW%20VERSION_COLOR.pdf); [Intel's XeGTAO](https://github.com/GameTechDev/XeGTAO) is a reference implementation. This one combines fixed samples and spatial filtering with reprojection and needs no separate temporal antialiasing pass.

## Memory

At 1920 x 1080 and DPR 1 the AO textures take 8,942,580 bytes: the six-level R32 depth and R16 radius pyramid, a half-resolution RG8 normal image, two half-resolution R8 working images, quarter-resolution ground and history-depth images, a coarse occupancy mask and the full-resolution R8 cache. The figure leaves out driver alignment.

The buffers add 348 bytes at 1x and 8,360,016 bytes at 4x, where the per-sample corrections and the tile lists live. Turning Arctic off releases every texture and buffer at once; the compiled pipelines stay. Normals and radii come from the vertex, index, owner and instance buffers the faces already use, so Arctic copies no scene data. The `?inspect=1` counters include all of it.

## Measurements

Native GPU timestamp medians on the dragon at 1920 x 1080, DPR 1, before and after the current sampling, in milliseconds, with outlines off. The laptop has an Intel i9-13900HX with Raptor Lake-S UHD graphics and an NVIDIA RTX 4080 Laptop GPU.

| GPU | MSAA | Cached AO | Moved AO | Drag AO |
| --- | ---: | ---: | ---: | ---: |
| Intel Raptor Lake-S UHD | 1x | 0.345 → 0.193 | 26.593 → 3.304 | 11.720 → 3.304 |
| Intel Raptor Lake-S UHD | 4x | 10.934 → 0.652 | 45.055 → 5.804 | 17.671 → 5.814 |
| NVIDIA RTX 4080 Laptop | 1x | 0.020 → 0.014 | 1.113 → 0.467 | 0.449 → 0.456 |
| NVIDIA RTX 4080 Laptop | 4x | 0.292 → 0.042 | 1.812 → 0.607 | 0.640 → 0.499 |

Moving AO on the Intel GPU takes 3.30 ms at 1x and 5.80 ms at 4x, within the 4 ms and 6 ms goals. Whole Intel GPU frames during a drag take 11.44 ms at 1x and 14.90 ms at 4x; 1.3 to 1.7 ms of the 4x figure is the first read of the multisampled depth and id textures, which any pass reading them pays once per frame. The NVIDIA rows predate the halved sample sets. The figures measure GPU work, not input latency, and cover the whole effect: pyramid, ground, blurs, history, MSAA upsample and composite.

In a native rotation test, reprojection cuts the variation of shadows at fixed ground points by about 54% against blurring alone and keeps about 97% of the mean shadow strength. To measure again, run the ignored `benchmark_arctic` test in `src/engine/gpu/ssao.rs` with `AO_SCENE` and `AO_OUTPUT` set, one run at a time.

## Check the look

Open `view_mixed`, run `Layers On`, select the floor-model group and press F. Clear the selection with Escape, hide the panel with `Layers Off`, and compare `Arctic On` and `Arctic Off` at the column bases. Repeat with the plates and with the solids and BReps to see creases, the cone and the torus. The framing must not change.

![Reference floor contacts](screenshots/ssao-floor.png)

![Reference plate contacts](screenshots/ssao-plates.png)

The browser checks are in `tests/`: `ambient-lighting.cjs` checks real WebGPU, released memory and the controls (`AMBIENT_SPIN=1` adds continuous rotation), `ambient-details.cjs` small-part shadows and plate smoothness, `ambient-floor.cjs` the floor scene, late loading and unchanged framing, `ambient-motion.cjs` orbiting, and `ambient-scenes.cjs` every published scene. The native tests in `ssao.rs` cover both projections, 1x and 4x, contact halos, the same image through every drag tier, history rejected after an edit, and no pipeline compiled across twenty toggles and twenty resizes.
