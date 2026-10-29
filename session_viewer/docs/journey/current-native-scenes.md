# Actual maintained native scene checks

The current maintained renderer loaded and drew four existing immutable inputs: the checked-in comparison, `wood/data/floor_model.pb`, `tests/fixtures/timber-floor.pb` and `tests/fixtures/vault-dome.pb`. Each input kept its SHA-256 after rendering. The [earlier records](current-native-scenes.json) retain their original source/output hashes, settings and logs; they describe historical captures, not the replacement illustrations below.

The dome illustration uses `vault-dome.pb` (SHA-256 `b00bc3697b1572bd2735d0795f179980b9585c659376f6af4bcdd9360d482d26`), produced from the unchanged Wood dome example at `1bd60e99`. It contains 800 Elements and 2,320 graph contacts. It differs from the earlier dome input; the earlier 1,600-row count does not describe this fixture. The [upstream check](latest-commit-audit.md) retains the historical dependency context.

The floor illustration uses the published `timber-floor.pb` fixture (SHA-256 `1d53ea1f6aaa533a566febe4f9e7ca77f1e9d7e4bf52bf473d408695090fa1f4`), containing 529 Elements and 204 graph contacts. Its retained image filename is `current-native-floor-output.png`, but this fixture replaces the earlier private recovery output. This capture demonstrates viewer presentation and makes no claim about kernel geometry validation.

![Maintained NVIDIA Vulkan rendering of the published timber-floor fixture.](../screenshots/journey/current-native-floor-output.png)

![Maintained NVIDIA Vulkan rendering of the immutable 800-Element dome fixture.](../screenshots/journey/current-native-dome.png)

These frames were rendered on an NVIDIA GeForce RTX 4080 Laptop GPU using Vulkan at 1280 x 960 physical pixels (DPR 2), with opaque lit faces, AO/outlines enabled, grid disabled and MSAA 4. The four captured frames were visually reviewed at this framing. Finite display limits remain: the logs report 2 limited rows in the comparison, 800 in the dome, 237 in the floor input and 399 in the published floor fixture. These images do not imply that every row reached the requested refinement or that arbitrary zoom levels have been reviewed. The earlier CPU Vulkan/Lavapipe timings remain historical and do not measure these captures; use the [large-scene recipe](large-scenes.md) for performance investigation.

Use [subject97](97-finish-with-the-maintained-browser-runtime.md) for the maintained browser destination. Browser gestures, two-density visual acceptance, release-after-Close and matched Lenovo Chrome/Firefox measurements remain separate checks. No reference pages have been retired.
