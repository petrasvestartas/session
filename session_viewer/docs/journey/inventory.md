# Current source ownership inventory

Inventory captured at Session `2f96b35d225015cfcb9bfe5e8ec66565ef99aa28`; Wood `6e8ca14f7cbfcecba37d60caa153d91354f55e74`. [Final upstream check](latest-commit-audit.md). Wood was subsequently fast-forwarded to `4916884a45e2558d57dd1a0175bca1bd8a627410`; its Rust subtree and every inventoried input are unchanged.

The October 11 snapshot includes the preserved local hierarchy-path and parent Element-lamp changes. The [October 9 inventory](current-source-inventory.json) remains historical.

[Exact file hashes and ownership](current-source-inventory-2026-10-11.json) covers every Rust source and declared build entry. Verification also fingerprints the live maintained viewer and Wood source/build/assets dependencies; changing them invalidates dependent results. These hashes identify sources, not executed browser behavior.

| Source group | Files | Read with |
| --- | ---: | --- |
| kernel: Original geometry and serialization | 109 | source chapters,96 |
| viewer: Build, startup and application ownership | 8 | 00–02,34–35,94,96 |
| viewer: Camera, snapping and construction planes | 2 | 26–27,72–76,85,96 |
| viewer: Command grammar, defaults and tools | 101 | drawing/editing chapters,85,92,93,96 |
| viewer: Derived display, text and rendering infrastructure | 2 | core GPU chapters,60–65,94 |
| viewer: Element attributes and interactions | 2 | 52–54,92,96,97 |
| viewer: Frame pacing and performance | 1 | 95 |
| viewer: GPU pipelines and current rendering | 84 | 86–94 |
| viewer: Interaction, selection and editing state | 14 | 26–33,72–85,92,96 |
| viewer: Loading, source ownership, archives and routing | 59 | 43–54,66–71,85,96 |
| viewer: Maintained native rendering and correctness reference | 4 | 94–96 |
| viewer: Original source walking and shared display | 16 | 55–71,92,94,95 |
| viewer: Panels, command input and keyboard focus | 13 | 03,35,85,96 |
| wood: Element builders and features | 13 | 92,96 |

[Dated current comparison](current-viewer-2026-10-11.json) preserves prior hashes and records changes. [Command grammar](current-command-grammar.md), [current behavior](current-viewer.md), and [remaining implementation differences](rendering-differences.md) have distinct scopes.
