# Equilibrium deck

```bash
cd session/session_viewer

# check every step: no errors, no overlaps
cargo test --lib stage::

# local preview, reloads on YAML save
bash ../../wood/tools/buildslot.sh stage env CARGO_BUILD_JOBS=8 trunk serve
# open http://127.0.0.1:8080/?stage=decks/equilibrium/index.yaml&data=off

# publish to the live site (no rebuild; then reload ?stage=equilibrium)
aws s3 sync examples/stage s3://session-viewer-data/stage/ --delete --exclude '*' --include '*.yaml' --content-type application/yaml --cache-control no-cache --profile r2 --endpoint-url https://0520459c6817bd96c1e25fcb49461c4e.r2.cloudflarestorage.com

# pictures, if any: separate sync, without --delete
aws s3 sync examples/stage s3://session-viewer-data/stage/ --exclude '*' --include '*.png' --content-type image/png --profile r2 --endpoint-url https://0520459c6817bd96c1e25fcb49461c4e.r2.cloudflarestorage.com
```

Keys: → / Space next step, ← back, ↓ / ↑ next/previous slide.
Deck format: ../README.md
