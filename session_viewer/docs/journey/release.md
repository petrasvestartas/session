# Course status

- The course has **1,310 active checkpoints across 109 reader subjects**. Kernel/editor and viewport/input now each have one runnable endpoint; history owns fallible rollback. Their [native, wasm and six browser checks](release/history-editor-viewport-windows-20261114.json) pass. The [documentation check](release/consolidation-docs-windows-20261114.json) passes 110 tests, the strict site build and 88 browser routes. Upright runtime captures retain fixed-camera cropping.
- The [mathematics subject](03a-mathematics.md) has 17 matching core modules, 24 passing native tests, wasm compilation and desktop/upright browser evidence. Windows software-WebGPU failed; upright emulation is not physical-phone acceptance. Cumulative integration remains pending.
- Finished-subject ownership and production coverage remain incomplete: 25 inventory gaps, 4,840 later-subject rewrites across 305 files, and endpoint parity of 323 missing, 707 extra and three changed files. The [completion contract](completion-contract.md) defines the remaining acceptance; [historical evidence](../archived/journey-release-evidence.md) does not establish current acceptance.

## Reproduce

The retained reference checkpoints use the [historical bundle-server instructions](../archived/journey-release-evidence.md#reproduce). Those commands reproduce the reference, not current production acceptance. Use the [mathematics lesson's commands](03a-mathematics.md#run-and-observe) for the full stage-core checkpoint, and [Maintaining](../maintaining.md) for current course checks.
