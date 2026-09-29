# Course release: journey-2

The cumulative lessons progress from a canvas through indexed geometry, scene ownership, stable identity, picking, undo/redo, a perspective orbit camera, a lit kernel box, shared editor actions, synchronized resizing, cancellable pointer gestures, focused keyboard/wheel navigation and small mesh-session imports. Imported objects retain their source session and GUID; one history transaction covers the whole file. Rust checks inspect shape coverage, landmark pixels and state invariants. The background lesson also proves that two toggles restore the original pixels. The course route lists the current lessons and estimates; use the commands below to check their build evidence.

**All 24 published checkpoints passed their scripted browser checks in headed Chrome 153.0.8010.36 on September 29.** Each lesson includes its actual browser page. The run uses Linux, a 900 × 760 CSS-pixel initial viewport, device-pixel ratio 1, and the WebGPU launch flags in `open-chrome.sh`. Source and bundle fingerprints, browser version and canvas hashes are recorded in `screenshots/journey/browser.json`. This proves the tested cases, not every interaction or another browser.

Lesson 24 adds scene fitting. Its 35 Rust tests include fitting box corners across scales, orientations and aspect ratios, preserving document history, and empty/point bounds. Chrome checks repeated Fit, recovery after panning, import Undo/Redo after fitting, and geometry margins at both 900 × 760 and 480 × 900 browser sizes. The native readback separately checks every vertex against the view and the imported beam's visible colour.

Checks include the background’s mouse/keyboard round trip, button and picking actions, window resizing, right-drag orbit and release, wheel zoom without page scrolling, focused keys without navigation outside the canvas, and file import with whole-import Undo/Redo. The production viewer also passed a separate Chrome test that destroys its GPU device, checks that GPU work stops, downloads a diagnostic report, and recovers that report after reload. The reported Firefox driver crash has not been reproduced or proven fixed.

Lesson 04’s earlier failure came from a button’s keyboard focus outline overlapping 197 pixels of the canvas border. The GPU drawing was restored correctly. Rendering assertions now compare pixels read from the canvas itself; the full-page screenshots retain the real interface and its focus indicators.

Native GPU readbacks remain separate supporting evidence. The generated lesson pages prefer current browser screenshots and fall back to explicitly labelled native output when browser evidence no longer matches the checkpoint source.

## Command-line revision in progress

The published record above belongs to the button-based revision. The [real command-dock preview](command-line.md) has separate source fingerprints and Chrome captures. The revised typing lessons and a fresh complete course-browser run are still pending; the preview does not replace the published 24-lesson record.

## What the similar pictures show

These lessons deliberately keep the same small scene so you can compare one change. Their images are different, but a still image cannot establish that an interaction works. The scripted browser actions provide that additional evidence; the captions explain what to look for.

| Lesson | Visible difference | Browser action checked |
| --- | --- | --- |
| [16 · Orbit](16-orbit.md) | Two triangles seen from an isometric camera | Isometric button |
| [17 · Solid](17-solid.md) | A grey box joins the triangles | Add box and Isometric buttons |
| [18 · Light](18-light.md) | Three different face brightnesses on the same box | Shader output in Chrome |
| [19 · Actions](19-actions.md) | The selected box is yellow | Buttons reaching the shared Editor path |
| [20 · Resize](20-resize.md) | The canvas grows with the browser window | Resize from 900 × 760 to 1000 × 800 at DPR 1 |
| [21 · Gestures](21-gestures.md) | The scene rotates after a right drag | Drag changes pixels; movement after release does not |

On September 29, all six source PNGs were compared byte-for-byte with fresh PNG encodings of their saved GPU readbacks. The raw pixels matched their verification records, and the two documentation builds contained the same six distinct assets. This audit verifies the files; it does not verify what a running localhost server or browser cache returns.

## Fixed inputs

The package pins wgpu 29.0.4 and the browser bindings. `journey/release/Cargo.lock` fixes the initial transitive dependencies. Lesson 15 introduces the existing Rust geometry kernel and its separate `15-perspective.lock`. Lesson 23 adds direct protobuf and JavaScript dependencies using `23-import.lock`; their versions were already used transitively. The dependency command preserves the old lock and installs the versions for that stage. The init command supplies only the first lock; all viewer implementation code appears in the lessons. Verification also fingerprints the kernel source used by the later checkpoints.

The September 28 viewer is fingerprinted separately in `destination.json`; production source has not been replaced by the teaching renderer. September 29 browser diagnostics and device-loss guards are newer than that reference. Its fingerprint and source-parity checks intentionally report this difference until the fixes enter the checkpoints. The shorter course has not yet reached the full viewer.

## Reproduce the checks

From `session_viewer`, the authoring checks are:

```sh
npm --prefix ../session_tests run course -- structure
npm --prefix ../session_tests run course -- verify
npm --prefix ../session_tests run course -- verify --stored
```

The TypeScript command validates lesson edits, displayed listings and the destination inventory. Rust performs the executable pixel assertions. The second reconstructs reference projects under `target/course-ID` and keeps browser bundles in `target/course-checks`, builds browser bundles and checks native frames. The third rejects stale build evidence. None writes to `workspace/journey`.

For a browser run, first complete those builds. Then serve their parent directory:

```sh
npm --prefix ../session_tests run course -- serve
```

Install the capture dependencies once from `session_viewer`; Chrome must also be installed:

```sh
npm install --prefix target/course-tools playwright@1.58.2 pngjs@7.0.0
```

In a second terminal:

```sh
npm --prefix ../session_tests run course -- capture
```

Set `JOURNEY_URL` if serving on another address. The tool checks startup errors, distinct canvas results, button activation by mouse and keyboard, and the two-toggle round trip. It writes actual page screenshots and source/bundle fingerprints to `docs/screenshots/journey`. Regenerate the lesson pages with `npm --prefix ../session_tests run course -- generate` after a successful capture to include current browser screenshots.

## Before declaring the full course finished

Every planned feature needs cumulative lessons, a working browser checkpoint and the acceptance cases in [the destination contract](destination.md). Compare representative scenes at fixed camera poses and viewport sizes. Record hardware and performance conditions. Source coverage alone cannot establish equivalent visual quality or responsiveness.

[Return to the course](../journey.md)
