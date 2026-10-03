# Timber-floor edge regression

`timber-floor.pb` is the published Session timber-floor scene downloaded on 2026-10-03 from https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/pb/view_live.pb.

SHA-256: `1d53ea1f6aaa533a566febe4f9e7ca77f1e9d7e4bf52bf473d408695090fa1f4` (4,975,446 bytes).

The ray oracle requires at least 99.9% of visible edge samples drawn at desktop and phone framebuffer sizes, fitted and farther perspective views, and orthographic projection. Disable decorations and translucency so they cannot fill missing samples. Existing near-plate tests separately enforce hidden-ink limits: the floor has curved edges intentionally raised above tessellation facets by geometric sag, so its pixel-neighbourhood leak counts are observational rather than that plate's strict planar limit.
