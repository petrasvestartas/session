#!/usr/bin/env bash
# The viewer's gate: the tests, the local scene's ink count, and the depth probe - a plate whose
# bottom outline (magenta) must never surface through its top face seen from above.
#
#   docs/_gate.sh            -> prints "gate OK" or the first failure
#
# Numbers here are MEASURED (1400 x 900, Intel iGPU, 2026-09-03) and re-recorded with the reason
# whenever a declared pixel change moves them.
set -u
cd "$(dirname "$0")/.."
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HOME/.cache/tmain}" REGEN_PROTO=0
T=x86_64-unknown-linux-gnu
out=$(mktemp -d) && trap 'rm -rf "$out"' EXIT

cargo xtest -q >/dev/null 2>&1 || { echo "gate FAIL: cargo xtest"; exit 1; }
cargo build -q --release --target $T --example selftest --example mk_plate_outline || { echo "gate FAIL: build"; exit 1; }
B=$CARGO_TARGET_DIR/$T/release/examples/selftest

ink=$(VIEWER_W=1400 VIEWER_H=900 $B "$out/local.ppm" assets/view_local.toml 2>&1 | grep -o "non-background pixels: [0-9]*" | grep -o "[0-9]*$")
[ "${ink:-0}" -ge 60000 ] && [ "$ink" -le 65000 ] || { echo "gate FAIL: local scene ink $ink (expected 60000..65000)"; exit 1; }

$CARGO_TARGET_DIR/$T/release/examples/mk_plate_outline "$out/plate.pb" >/dev/null 2>&1
printf 'name = "plate"\n[[items]]\nfile = "plate.pb"\nname = "plate"\nat = [0, 0, 0]\n' > "$out/plate.toml"
probe() {
    local name=$1 z=$2; shift 2
    env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_EDGES=1 VIEWER_ZOOM=$z "$@" $B "$out/$name$z.ppm" "$out/plate.toml" >/dev/null 2>&1
    read -r _ blue _ magenta < <(python3 docs/_count_colors.py "$out/$name$z.ppm")
    [ "$magenta" -le 4 ] || { echo "gate FAIL: plate bottom outline visible from above ($name, zoom $z: $magenta px)"; exit 1; }
    [ "$blue" -gt 500 ] || { echo "gate FAIL: plate top outline missing ($name, zoom $z: $blue px)"; exit 1; }
}
for z in 0 -6; do probe top $z VIEWER_VIEW=top; done
for z in 0 -6 -12; do probe down $z VIEWER_ORBIT=0,209; done
echo "gate OK (local ink $ink)"
