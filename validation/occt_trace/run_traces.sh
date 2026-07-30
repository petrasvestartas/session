#!/usr/bin/env bash
# Captures the trace corpus into occt_trace/traces/.
set -u
cd "$(dirname "$0")"
BIN=./build/occt_trace
OUT=traces
mkdir -p "$OUT"
: > "$OUT/_summary.txt"

run() { # run <name> <op> <a> <b>
  echo "=== $1"
  "$BIN" --op "$2" --a "$3" --b "$4" --name "$1" --out "$OUT/$1.trace"
  grep -h '^SUMMARY' "$OUT/$1.trace" >> "$OUT/_summary.txt" 2>/dev/null
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

# 6. box x box control
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
wc -l "$OUT"/*.trace | tail -1
