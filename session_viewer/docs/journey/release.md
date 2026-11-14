# Course status

- Local integration has **1,311 checkpoints across 110 reader subjects**. [Cumulative mathematics](03a-mathematics.md) passes 27 native tests; [commands that change the view](04-input.md) retains mathematics and passes 29. Both have wasm and desktop/upright browser evidence.
- The last published mathematics revision passes 112 tooling tests, strict documentation, reader navigation and CI. Command-checkpoint evidence covers its own sources and interactions. Native reference compilation passes; final-reference wasm startup remains unverified. Upright viewport emulation does not establish physical-phone acceptance.
- Completion remains open: **97 unconsolidated subjects**, 25 inventory gaps, 4,842 later-subject rewrites across 306 files, and endpoint parity of 299 missing, 709 extra and five changed files. See the [completion contract](completion-contract.md).

## Reproduce

The retained reference checkpoints use the [historical bundle-server instructions](../archived/journey-release-evidence.md#reproduce). Those commands reproduce the reference, not current production acceptance. Use the [mathematics lesson's commands](03a-mathematics.md#run-and-observe) for the full stage-core checkpoint, and [Maintaining](../maintaining.md) for current course checks.

[Earlier consolidated runtime evidence](release/history-editor-viewport-windows-20261114.json) and [documentation evidence](release/consolidation-docs-windows-20261114.json) retain their original source scope; they do not accept the changed cumulative graph.
