# Viewer changes to carry into the course

This review covers the viewer commits from September 28 through October 3, 2026, plus the current command-input work. It describes the source changes; the unfinished lessons below still need implementation and acceptance checks.

| Change | What the learner must understand | Tutorial destination |
| --- | --- | --- |
| `is_visible` on every drawable type (`556b9f1d`; element support began in `44b0e503`) | Load hidden objects as hidden rows, keep source geometry, and save the current visibility back to each object. Hiding is different from deleting. | 28 source records; 29 save/reopen; 53 visibility and layers |
| `is_locked` on every drawable type (`dbd71108`) | Restore the lock when loading and preserve it when saving. A locked object must remain visible while selection and editing respect its lock. | 29 save/reopen; 45 selection; 53 locking |
| Inherited tree colors (`e13c263e`) | An object's explicit override wins. Otherwise use its node color or the nearest colored parent. Keep display policy separate from stored geometry. | 40 color presentation; 52 nested tree; 53 layers |
| Browser decoding of tree color field 5 (`92a2bfca`) | A schema field does not reach the browser merely because the native kernel reads it. Verify the window decoder separately. | 28 source records; 30 loading; 52 nested tree |
| Curved BRep edges sampled from their own curves (`5523cc6c`, `c978aeb3`) | Surface triangulation and boundary sampling have different jobs. Curve sag gives hidden-line visibility a geometric tolerance. | 35 strokes; 38 curve sampling; 39 visibility; 56 shared boundaries |
| Display density capped at 2 by default, and fully covered strokes culled before fragment work (`d03926a7`) | CSS pixels and drawing pixels differ. Performance optimizations must preserve close-up ink and selected geometry. | 54 display density; 86 finite-triangle visibility; 95 performance |
| Congruent BReps reuse their display walk within one document ([661c766d](https://github.com/petrasvestartas/session/commit/661c766d042f99df3f01a21200cfc1d31627d750)) | Build a key from topology, curve/surface parameters, weights, colours and control points in a rigid local frame. Moved/rotated copies replay positions, normals, edge IDs and facing information for their own row. Mirrors and changed shapes take independent walks. Scope the recordings to document preparation and release them when its outer guard drops. | 56 boundaries; 69a congruent display walks; 95 performance |
| Full-circle edge sampling remains translation invariant (same commit) | A tiny rounding excess beyond360degrees must not turn90display chords into91 after moving a circle. Compare independently prepared and replayed geometry, rather than only measuring cache hits. | 38 curve sampling; 56 boundaries; 69a congruent display walks |
| Nearly solid face default | The October 2 default was 0.9 (`740888fe`). The requested desktop default is 0.95; the phone fix uses solid opacity 1. Query overrides and explicit opacity commands win over defaults, and automatic element dimming no longer overwrites them. | 93 opacity |
| Hybrid near/far triangle depth anchors | Huge near-clipped faces retain the homogeneous centre anchor; bounded far triangles use a divided first-corner reference to avoid cancellation. Preserve both near and far oracle acceptance, including portrait framing. | 39 visibility; 86 finite-triangle visibility; 94 comparison |
| Compressed scene publication and byte-based live revision identity | The manifest declares decoded size and encoding. Browser decoding must not range-probe compressed data, and validator changes alone must not replace unchanged scene bytes. First-frame preparation overlaps download while inactive Arctic remains cold. | 30 loading; 64–68 publication/streaming; 95 performance |
| Offscreen ribbon work and visibility-pool completion | Reject work conservatively without losing wide-pen fringes. Consume asynchronous overflow reports and redraw until capacity fits; return to idle afterward. | 35 strokes; 86 finite visibility; 95 performance |
| Saved diagnostics and actual failure-time freshness (`042b3ad1`) | Keep metadata independent of a stopped GPU, bound admission and retention, and date failure separately from heartbeat. Active other tabs and invalid/future chronology stay quiet; interrupted does not mean proven crash. | 34b–34fb reports/storage; following lifecycle/telemetry/recovery checkpoints |
| Browser redraw batches wait for GPU completion | Input continues updating camera state while the GPU is busy; completion draws the latest pose. Include UI uploads and picking in the fence, preserve sharpness and return to idle. | 33 lifetimes; 54 browser navigation; 95 performance |
| Keyboard feature commands and right-click repeat | Printable input opens the command dock without losing its first character. Mouse/phone gestures navigate. Right-click restarts an interactive tool with fresh arguments; coordinate responses do not replace that tool. | 03d typing; 22 wheel navigation; 47 parsing; 48 interactive tools; 54 ownership |

The close-up hidden-line correction (`ee3db14a`) and its adapter fallback (`ffbe7f2e`) were reverted on October 2 (`cf4f6c81`, `c5da88f2`). The foreground investigation reproduced 1,650 missing samples out of 7,609 visible points. The new correction retains near-clipped polygon coverage, derives depth from homogeneous corners, anchors strokes from their original endpoints, and adds coordinate-scaled rounding slack. The [regression report](line-visibility.md) records the explanation and checks. The corresponding cumulative rendering lessons remain to be authored.

The phone fixes are accepted by the user: appearance is correct, loading is faster and rotation is smooth. The numerical phone loading target is still unmeasured. [Phone evidence and remaining robustness work](../phone-performance.md) distinguish actual user acceptance from desktop emulation. Adjacent-face ownership, idle tap/selection preparation and ribbon variant reduction remain implementation work. CPU restructuring remains conditional on phone diagnostics.

The local protobuf checkout currently predates the kernel's committed visibility/locking bindings. Verification uses `REGEN_PROTO=0` to preserve those committed bindings. Updating a schema checkout must be deliberate; regenerating older bindings removes fields the kernel already uses. This checkout issue is separate from the viewer behavior the lessons teach.

- [x] Review the week's commit history and identify its teaching destinations, including the October3 congruent-BRep change integrated during publication.
- [ ] Teach and verify congruent BRep walk keys, replay, mirrors, per-row facing/IDs, scoped cache release and translation-invariant circle chords.
- [x] Verify current production typing, repeat and mouse/phone input in Chrome.
- [x] Revise and verify lessons 03d and 22–26.
- [x] Preserve original double coordinates, names and visibility/locking attributes through source preparation; share original imported geometry.
- [x] Verify exact source coordinates, names and visibility/locking through save/reopen; preserve object colour in the snapshot check.
- [ ] Implement visibility/locking drawing and selection policies, and inherited tree colour load/save fixtures.
- [x] Diagnose and fix current close-up line loss with a geometric oracle and browser views: eight native view/sample combinations and 24 Chrome zoom views pass.
- [ ] Verify the attributes and opacity defaults in the completed rendering chapters.

[Return to the complete lesson checklist](roadmap.md).
