# Timber-floor edge regression

`timber-floor.pb` is the published Session timber-floor scene downloaded on 2026-10-03 from https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/pb/view_live.pb.

SHA-256: `1d53ea1f6aaa533a566febe4f9e7ca77f1e9d7e4bf52bf473d408695090fa1f4` (4,975,446 bytes).

The ray oracle requires at least 99.9% of visible edge samples drawn at desktop and phone framebuffer sizes, fitted and farther perspective views, and orthographic projection. Disable decorations and translucency so they cannot fill missing samples. Existing near-plate tests separately enforce hidden-ink limits: the floor has curved edges intentionally raised above tessellation facets by geometric sag, so its pixel-neighbourhood leak counts are observational rather than that plate's strict planar limit.

## Vault dome navigation

`vault-dome.pb` is generated from the unchanged `wood/examples/templates_vault_4_dome.cpp` example at Wood commit `1bd60e99`. Only the example output directory was redirected; the fixture contains 800 voussoirs and 2320 contacts.

SHA-256: `b00bc3697b1572bd2735d0795f179980b9585c659376f6af4bcdd9360d482d26` (3,912,827 bytes).

Run the headed Chrome wheel regression against this fixture with `node tests/wheel-navigation.cjs tests/fixtures/vault-dome.pb 800`. It checks real wheel input, full-quality restoration and return to idle. Its advanced timing clock exercises adaptive navigation policy; it is not a frame-rate benchmark.
