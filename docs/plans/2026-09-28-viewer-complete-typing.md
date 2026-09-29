# Complete typing course — 2026-09-28

The latest revision follows the learner’s choice: type each addition once and allow longer steps. Each of the 43 tutorials is now one complete buildable step, with its explanation, code and checks on the page. Four steps review existing code. The eight introductory Rust programs remain separate runnable foundations.

The arbitrary 60-line fragments, repeated instructions, per-file diagrams and source-workbook screenshots have been removed from the viewer tutorials. Complete additions retain their comments and blank lines. A repeated image is rejected by its content hash. The course now presents 52 distinct images, including a fresh offscreen render from the final step.

The course still covers 97,631 typed lines in 351 text files, expressed as 978 complete additions. Every production Rust/WGSL file is covered; final parity is exact after teaching comments are removed. The write-once check reports zero violations. The learner keeps one `workspace/handwritten` project. Setup copies no implementation, and source comparison remains read-only. Step IDs such as `00` now identify complete checkpoints.

Every section now opens with estimated active study hours, its local purpose, its role in the whole viewer, a data path, links to starting files and an understanding question. An answer follows the final checks. The eight foundations also have time ranges and viewer connections. The overview explains loading, viewing and editing as three concrete journeys and gives course milestones and a section time table. Estimates are explicitly unmeasured planning assumptions, with instructions for calibrating them to the learner's own pace. The complete typing course is substantial: roughly 1,000–2,100 active hours under this model, plus 8–21 hours for foundations.

Validation is repeatable with `python3 docs/check_steps.py`. It reconstructs the code shown in every tutorial, builds the WebAssembly library, runs native tests, and checks a GPU frame whenever the renderer exists. All 43 builds and native suites passed; all 42 renderer steps produced the expected frame. The final native suite reported 487 passed and 53 ignored; ignored tests are not counted as passing.

Additional rendering checks passed: changing the step-01 shader changed white pixels to grey, and the final fixture rendered 92,899 non-background pixels with five visible objects confirmed by the GPU ID buffer. Build logs, commands and source fingerprints are in `session_viewer/target/course-steps`. `check_steps.py --check` rejects missing or stale step results.

The strict Vue build passed with 77 pages, 83 assets and a 903 KB search index. Links and file-map anchors resolve to listings inside complete tutorials. The release Trunk bundle includes this documentation. The new presentation was not browser-tested: no browser is connected in this environment. Earlier browser checks do not constitute validation of this revision’s layout or interactions.

Work remains local. No commit, push or deployment was made. Production viewer and kernel implementation were not changed by this tutorial revision.
