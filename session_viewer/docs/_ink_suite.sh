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
# dropping, which a render that exits 0 still does. 242720 non-background pixels measured at
# 1400x900, MSAA 4, zoom 5 on the Intel iGPU; the band is _gate.sh's, wide enough for a
# driver's rounding and far narrower than a lost edge.
closeup() {
    local output ink
    output=$(env VIEWER_W=1400 VIEWER_H=900 VIEWER_ZOOM=5 VIEWER_MSAA=4 "$B/selftest" "$OUT/close.ppm" assets/pb/view_local_boxes.pb 2>&1) || { echo "$output"; return 1; }
    echo "$output"
    ink=$(printf '%s\n' "$output" | grep -o "non-background pixels: [0-9]*" | grep -o "[0-9]*$")
    [ "${ink:-0}" -ge 240000 ] && [ "$ink" -le 245000 ] || { echo "close-up ink ${ink:-none} (expected 240000..245000, measured 242720)"; return 1; }
}

check build cargo build -q --release --target $T --examples
check xtest cargo xtest -q
check wasm cargo check -q --target wasm32-unknown-unknown
check clippy cargo clippy -q --release --all-targets --target $T -- -D warnings
check gate docs/_gate.sh
check probe_matrix python3 docs/_probe_matrix.py "$B/selftest" "$B/mk_hidden_line_probe" "$OUT/matrix"
check lifecycle "$B/check_hidden_line_lifecycle" "$OUT/lifecycle" "$OUT/matrix/regular.pb" "$OUT/matrix/warped.pb" "$OUT/matrix/authored.pb"
check determinism "$B/check_determinism" "$OUT/matrix/regular.pb" "$OUT/matrix/warped.pb" assets/pb/view_local_boxes.pb
check mixed_scene "$B/mk_mixed_solids" "$OUT/mixed.pb"
# Determinism of the unwelded walk on a torus and a block with hole. NOT the sphere and NOT the
# mixed scene: the kernel's grid mesher averages a pole's normal over its fan in HashMap order,
# so a sphere's two pole normals differ in their last bits between loads (measured 2026-09-08,
# five loads of sphere.pb, `verts` flaky every time; torus and hole deterministic in three
# loads each). The weld used to drop those normals; the analytic normals now reach the GPU.
# The fix is a sorted accumulation in the kernel (phase 2 part B), after which the sphere and
# the mixed scene join this line.
check probe_torus "$B/mk_shade_probe" "$OUT/torus.pb" torus
check probe_hole "$B/mk_shade_probe" "$OUT/hole.pb" hole
check probe_determinism "$B/check_determinism" "$OUT/torus.pb" "$OUT/hole.pb"

# Smooth shading: a sphere alone, headlight on, no ink. The largest second difference of luma
# across its centre scanline measured 1.43 with per-vertex normals against 4.07 with the
# welded flat facets (1400x900, MSAA 4, Intel iGPU, 2026-09-08); the floor is 2.5, the
# quantisation stair of an 8-bit gradient with room for a driver's rounding. Zero back-face
# pixels from above: a face wound inside out shows the shader's red.
check shade_probe "$B/mk_shade_probe" "$OUT/sphere.pb" sphere
check render_shade env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_NO_EDGES=1 "$B/selftest" "$OUT/sphere.ppm" "$OUT/sphere.pb"
check shade_scanline python3 docs/_shade_scanline.py "$OUT/sphere.ppm" --max-second-diff 2.5 --max-backface 0
check render_mixed_top env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_VIEW=top "$B/selftest" "$OUT/mixed_top.ppm" "$OUT/mixed.pb"
check mixed_backface python3 docs/_shade_scanline.py "$OUT/mixed_top.ppm" --max-backface 0

# A hidden line behind a curved BRep: zero magenta at distance 1 and 4, read from the whole
# colour frame.
check cylinder_hidden "$B/mk_cylinder_hidden_probe" "$OUT/cyl_hidden.pb"
for d in 1 4; do
    check "render_cylinder_hidden_$d" env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_MSAA=4 VIEWER_ORBIT=0,60 VIEWER_DISTANCE_SCALE=$d "$B/selftest" "$OUT/cyl_hidden_$d.ppm" "$OUT/cyl_hidden.pb"
    check "cylinder_hidden_$d" bash -c "set -- \$(python3 docs/_count_colors.py '$OUT/cyl_hidden_$d.ppm'); [ \"\$4\" -eq 0 ]"
done

# The teapot: 32 bicubic patches, four open shells, a pole and a tip. Every one of its 72
# non-degenerated edges chained off the tessellation, and no back-face pixel from above. The
# red the top view does show is the shader's "inside of an open solid": the ring between the
# pot's mouth (r = 1.4 units) and the smaller lid (r = 1.3) and the spout's open tip, both
# openings of Newell's data. _shade_scanline.py measures BACKFACE_COLOR in linear bytes and
# the frame is sRGB, so that red is outside its window - the line is a floor, not a proof.
check teapot "$B/mk_teapot" "$OUT/teapot.pb"
check teapot_census bash -c "'$B/mk_teapot' '$OUT/teapot.pb' | grep -q ' unchained 0'"
check render_teapot_top env VIEWER_W=1400 VIEWER_H=900 VIEWER_NO_GRID=1 VIEWER_VIEW=top "$B/selftest" "$OUT/teapot_top.ppm" "$OUT/teapot.pb"
check teapot_backface python3 docs/_shade_scanline.py "$OUT/teapot_top.ppm" --max-backface 0

check closeup closeup
check brep_probe "$B/mk_brep_probe" "$OUT/brep_ok.pb"
check brep_probe_flipped env BREP_PROBE_FLIPPED=1 "$B/mk_brep_probe" "$OUT/brep_flipped.pb"
check orbit_ok python3 docs/_orbit_check.py "$B/selftest" "$OUT/brep_ok.pb" "$OUT/orbit_ok"
check orbit_flipped python3 docs/_orbit_check.py "$B/selftest" "$OUT/brep_flipped.pb" "$OUT/orbit_flipped" --compare "$OUT/orbit_ok"

# Six cameras. VIEWER_VIEW is applied after VIEWER_ORBIT and replaces it, so a "tilted top"
# renders byte for byte identical to top; the near-edge-on camera is an orbit alone. `tilt` is
# 0.60 degrees off straight down - the harness logs eye (3017.605, -56.110, 6479.761) mm
# against target (3050, 0, 278.5) mm, an elevation of 89.40 degrees. A unit is 0.005 rad and
# the pitch is applied to the iso camera's 30 degrees of elevation, so 207.35 units is 89.40:
# nothing is clamped, and 312 units would pitch past vertical and come back down the far side
# at 60.6 degrees. The id frame is what the weights are measured from - one group of pixels per
# stroke, whatever antialiasing did to its colour.
check joint_scene "$B/mk_joint_probe" "$OUT/joint.pb"
for name in iso down front side top tilt; do
    case $name in
        iso) cam=() ;;
        down) cam=(VIEWER_ORBIT=0,209) ;;
        front) cam=(VIEWER_ORBIT=0,60) ;;
        side) cam=(VIEWER_ORBIT=300,120) ;;
        top) cam=(VIEWER_VIEW=top) ;;
        tilt) cam=(VIEWER_ORBIT=0,207.35) ;;
    esac
    for d in 1 4; do
        check "render_${name}_$d" env VIEWER_W=1800 VIEWER_H=1400 VIEWER_NO_GRID=1 VIEWER_MSAA=4 VIEWER_DISTANCE_SCALE=$d VIEWER_IDS="$OUT/joint_${name}_$d.ids" "${cam[@]}" "$B/selftest" "$OUT/joint_${name}_$d.ppm" "$OUT/joint.pb"
        check "joint_${name}_$d" python3 docs/_stroke_weight.py "$OUT/joint_${name}_$d.ppm" "$OUT/joint_${name}_$d.ids"
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
