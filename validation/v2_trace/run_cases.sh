#!/usr/bin/env bash
# Captures a v2 trace for every case that has an OCCT trace, then diffs the pair.
#
#   run_cases.sh [--bin <main_20>] [--occt <dir>] [--out <dir>] [--tol 1e-6] [--timeout 300]
#
# The case list and the spec strings are COPIED VERBATIM from validation/occt_trace/run_traces.sh,
# so the two tracers are driven by the same operands; a divergence can never be a difference in
# what was built.
#
# EVERY case records its exit status in $OUT/_status.txt as "<name> exit=<rc>". A trace whose
# producer did not exit 0 is NOT counted anywhere -- "attempted" and "completed" are different
# numbers and both are printed.
set -u
cd "$(dirname "$0")"

BIN=../../session_cpp/build_v2diff/main_20
OCCT=../occt_trace/traces
OUT=traces
TOL=1e-6
TMO=300

while [ $# -gt 0 ]; do
  case "$1" in
    --bin) BIN="$2"; shift 2;;
    --occt) OCCT="$2"; shift 2;;
    --out) OUT="$2"; shift 2;;
    --tol) TOL="$2"; shift 2;;
    --timeout) TMO="$2"; shift 2;;
    *) echo "run_cases.sh: unknown option $1"; exit 2;;
  esac
done

if [ ! -x "$BIN" ]; then echo "run_cases.sh: no v2 tracer at $BIN"; exit 2; fi
mkdir -p "$OUT"
: > "$OUT/_summary.txt"
: > "$OUT/_status.txt"

echo "BIN $BIN"
sha1sum "$BIN"
env | grep '^SESSION_' | sort || true

ATT=0; DONE=0
run() { # run <name> <op> <a> <b>
  ATT=$((ATT + 1))
  timeout "$TMO" "$BIN" --op "$2" --a "$3" --b "$4" --name "$1" --out "$OUT/$1.trace"
  local rc=$?
  echo "$1 exit=$rc" >> "$OUT/_status.txt"
  if [ $rc -eq 0 ]; then
    DONE=$((DONE + 1))
    grep -h '^SUMMARY' "$OUT/$1.trace" >> "$OUT/_summary.txt" 2>/dev/null
  else
    echo "=== $1  EXIT=$rc"
  fi
}

SPH=sphere,r=2.5
TANG=23.578178478201835

# 1. sphere x cylinder through the centre, tilt about Y (23.578... = exact pole tangency)
for A in 0 20 23 $TANG 24 25 30 45; do
  TAG=$(echo "$A" | cut -c1-6 | tr -d '.')
  for OP in cut common; do
    run "sph_cyl_roty${TAG}_${OP}" "$OP" "$SPH" "cylinder,r=1,h=8,center,roty=$A"
  done
done

# 2. sphere x sphere, two poses
run sph_sph_p1_cut    cut    "$SPH" "sphere,r=2,tx=3"
run sph_sph_p1_common common "$SPH" "sphere,r=2,tx=3"
run sph_sph_p2_cut    cut    "$SPH" "sphere,r=2,tx=1.5,ty=1,tz=1.2"
run sph_sph_p2_common common "$SPH" "sphere,r=2,tx=1.5,ty=1,tz=1.2"

# 3. box x cone, two poses
run box_cone_p1_cut    cut    "box,dx=4,dy=4,dz=4,center" "cone,r1=2,r2=0,h=5,center"
run box_cone_p1_common common "box,dx=4,dy=4,dz=4,center" "cone,r1=2,r2=0,h=5,center"
run box_cone_p2_cut    cut    "box,dx=4,dy=4,dz=4,center" "cone,r1=2,r2=0,h=5,center,roty=30,tx=0.7"
run box_cone_p2_common common "box,dx=4,dy=4,dz=4,center" "cone,r1=2,r2=0,h=5,center,roty=30,tx=0.7"

# 4. cone x cone, two poses
run cone_cone_p1_cut    cut    "cone,r1=2,r2=0,h=5,center" "cone,r1=2,r2=0,h=5,center,rotx=90"
run cone_cone_p1_common common "cone,r1=2,r2=0,h=5,center" "cone,r1=2,r2=0,h=5,center,rotx=90"
run cone_cone_p2_cut    cut    "cone,r1=2,r2=0,h=5,center" "cone,r1=2,r2=0,h=5,center,roty=35,tx=1"
run cone_cone_p2_common common "cone,r1=2,r2=0,h=5,center" "cone,r1=2,r2=0,h=5,center,roty=35,tx=1"

# 5. cylinder x cylinder, one pose
run cyl_cyl_cut    cut    "cylinder,r=1.5,h=8,center" "cylinder,r=1,h=8,center,rotx=90"
run cyl_cyl_common common "cylinder,r=1.5,h=8,center" "cylinder,r=1,h=8,center,rotx=90"

# 6. box x box control -- both kernels are exact here, so a divergence indicts the harness
run box_box_cut    cut    "box,dx=4,dy=4,dz=4,center" "box,dx=2,dy=2,dz=6,center"
run box_box_common common "box,dx=4,dy=4,dz=4,center" "box,dx=2,dy=2,dz=6,center"

# 7. sphere x box: the seam-crossing / pole cases (common = the 7-face split)
run sph_box_common common "$SPH" "box,dx=4,dy=4,dz=4,center"
run sph_box_cut    cut    "$SPH" "box,dx=4,dy=4,dz=4,center"
run sph_box_fuse   fuse   "$SPH" "box,dx=4,dy=4,dz=4,center"

# 8. coincident faces/edges -- the only configuration that produces BOPDS_CommonBlocks
run box_box_touch_fuse   fuse   "box,dx=4,dy=4,dz=4,center" "box,dx=4,dy=4,dz=4,center,tx=4"
run box_box_touch_common common "box,dx=4,dy=4,dz=4,center" "box,dx=4,dy=4,dz=4,center,tx=4"
run box_box_half_fuse    fuse   "box,dx=4,dy=4,dz=4,center" "box,dx=4,dy=4,dz=2,center,tx=4"

sort -o "$OUT/_summary.txt" "$OUT/_summary.txt"
echo "CASES attempted=$ATT completed=$DONE"

echo
echo "=========================== DIVERGENCE TABLE ==========================="
printf "%-24s %-7s %-22s %-13s %-13s %s\n" CASE OP FIRST_DIVERGENCE OCCT V2 DETAIL
for t in "$OUT"/*.trace; do
  n=$(basename "$t" .trace)
  if ! grep -q "^$n exit=0\$" "$OUT/_status.txt"; then
    printf "%-24s %-7s %s\n" "$n" "-" "TRACER DID NOT EXIT 0 -- not counted"
    continue
  fi
  if [ -f "$OCCT/$n.trace" ]; then
    python3 ./v2_trace_diff.py "$OCCT/$n.trace" "$t" --tol "$TOL" --row
  else
    printf "%-24s %-7s %s\n" "$n" "-" "NO OCCT TRACE at $OCCT/$n.trace"
  fi
done
