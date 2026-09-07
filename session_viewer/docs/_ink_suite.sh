#!/usr/bin/env bash
# Every check the ink rule must pass, in one run. Prints PASS/FAIL per check and exits nonzero
# on the first failure. The floor census needs the floor model, fetched from the bucket into
# $SCRATCH/pb when absent (INK_SUITE_NO_FETCH=1 skips that check instead).
#
#   docs/_ink_suite.sh
set -uo pipefail
cd "$(dirname "$0")/.."
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target}" REGEN_PROTO=0
T=x86_64-unknown-linux-gnu
B=$CARGO_TARGET_DIR/$T/release/examples
OUT="${INK_SUITE_OUT:-${SCRATCH:-/tmp}/ink_suite}"
mkdir -p "$OUT"
DATA=https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev

check() {
    local name=$1; shift
    if "$@" > "$OUT/$name.log" 2>&1; then echo "PASS $name"; else echo "FAIL $name (see $OUT/$name.log)"; exit 1; fi
}

# The close-up is the one check that must count its ink: it exists to catch edges thinning or
# dropping, which a render that exits 0 still does. 242406 non-background pixels measured at
# 1400x900, MSAA 4, zoom 5 on the Intel iGPU; the band is _gate.sh's, wide enough for a
# driver's rounding and far narrower than a lost edge.
closeup() {
    local output ink
    output=$(env VIEWER_W=1400 VIEWER_H=900 VIEWER_ZOOM=5 VIEWER_MSAA=4 "$B/selftest" "$OUT/close.ppm" assets/pb/view_local_boxes.pb 2>&1) || { echo "$output"; return 1; }
    echo "$output"
    ink=$(printf '%s\n' "$output" | grep -o "non-background pixels: [0-9]*" | grep -o "[0-9]*$")
    [ "${ink:-0}" -ge 240000 ] && [ "$ink" -le 245000 ] || { echo "close-up ink ${ink:-none} (expected 240000..245000, measured 242406)"; return 1; }
}

check build cargo build -q --release --target $T --examples
check xtest cargo xtest -q
check wasm cargo check -q --target wasm32-unknown-unknown
check clippy cargo clippy -q --release --all-targets --target $T -- -D warnings
check gate docs/_gate.sh
check probe_matrix python3 docs/_probe_matrix.py "$B/selftest" "$B/mk_hidden_line_probe" "$OUT/matrix"
check lifecycle "$B/check_hidden_line_lifecycle" "$OUT/lifecycle" "$OUT/matrix/regular.pb" "$OUT/matrix/warped.pb" "$OUT/matrix/authored.pb"
check determinism "$B/check_determinism" "$OUT/matrix/regular.pb" "$OUT/matrix/warped.pb" assets/pb/view_local_boxes.pb
check closeup closeup
check brep_probe "$B/mk_brep_probe" "$OUT/brep_ok.pb"
check brep_probe_flipped env BREP_PROBE_FLIPPED=1 "$B/mk_brep_probe" "$OUT/brep_flipped.pb"
check orbit_ok python3 docs/_orbit_check.py "$B/selftest" "$OUT/brep_ok.pb" "$OUT/orbit_ok"
check orbit_flipped python3 docs/_orbit_check.py "$B/selftest" "$OUT/brep_flipped.pb" "$OUT/orbit_flipped"

"$B/mk_joint_probe" "$OUT/joint.pb" > /dev/null
for cam in "iso VIEWER_NOTHING=1" "down VIEWER_ORBIT=0,209" "front VIEWER_ORBIT=0,60" "side VIEWER_ORBIT=300,120" "top VIEWER_VIEW=top" "tilt VIEWER_ORBIT=0,5 VIEWER_VIEW=top"; do
    set -- $cam; name=$1; shift
    for d in 1 4; do
        env VIEWER_W=1800 VIEWER_H=1400 VIEWER_NO_GRID=1 VIEWER_MSAA=4 VIEWER_DISTANCE_SCALE=$d "$@" "$B/selftest" "$OUT/joint_${name}_$d.ppm" "$OUT/joint.pb" > /dev/null 2>&1
        check "joint_${name}_$d" python3 docs/_stroke_weight.py "$OUT/joint_${name}_$d.ppm"
    done
done


FLOOR="${SCRATCH:-/tmp}/pb/view_mixed_floor_model.pb"
if [ ! -f "$FLOOR" ] && [ -z "${INK_SUITE_NO_FETCH:-}" ]; then
    mkdir -p "$(dirname "$FLOOR")" && curl -sS -o "$FLOOR" "$DATA/pb/view_mixed_floor_model.pb"
fi
if [ -f "$FLOOR" ]; then
    check floor_census python3 docs/_hidden_line_matrix.py "$B/selftest" "$B/census_plates" "$FLOOR" "$OUT/floor" --require-zero-scales 1,4
else
    echo "SKIP floor_census (no floor model)"
fi
echo "ink suite OK"
