#!/usr/bin/env bash
# HARNESS NEGATIVE CONTROL.
#
# selftest.sh proves the tool reports nothing when the two sides are identical. On its own that
# is also what a tool which reports nothing EVER would print, so it certifies nothing. This
# script injects one known defect at a time into the v2 side of a case the harness calls clean
# (box_box_cut) and asserts that the NAMED stage -- the one that owns that record -- turns FAIL.
# A mutation that slips through is a blind spot in the harness and is printed as MISS.
#
#   mutation_test.sh [--case box_box_cut] [--occt <dir>] [--v2 <dir>]
set -u
cd "$(dirname "$0")"
CASE=box_box_cut
OCCT=../occt_trace/traces
V2=traces
while [ $# -gt 0 ]; do
  case "$1" in
    --case) CASE="$2"; shift 2;;
    --occt) OCCT="$2"; shift 2;;
    --v2) V2="$2"; shift 2;;
    *) echo "mutation_test.sh: unknown option $1"; exit 2;;
  esac
done

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

python3 - "$V2/$CASE.trace" "$TMP" <<'PY'
import sys
src, out = sys.argv[1], sys.argv[2]
L = open(src).read().split("\n")

def first(pred):
    for n, l in enumerate(L):
        if pred(l):
            return n
    return -1

def emit(name, n, newline):
    """newline None -> drop the record"""
    if n < 0:
        print("SKIP %s (no such record)" % name)
        return
    m = list(L)
    if newline is None:
        del m[n]
    else:
        m[n] = newline
    open("%s/%s.trace" % (out, name), "w").write("\n".join(m))

def bump(l, key, delta):
    """add delta to the FIRST component of a key=a,b,c or key=<scalar> field"""
    parts = l.split(" ")
    for j, p in enumerate(parts):
        if p.startswith(key + "="):
            v = p[len(key) + 1:]
            if "," in v:
                c = v.split(",")
                c[0] = repr(float(c[0]) + delta)
                parts[j] = key + "=" + ",".join(c)
            else:
                parts[j] = key + "=" + repr(float(v) + delta)
            break
    return " ".join(parts)

n = first(lambda l: l.startswith("RESVERT "));    emit("res_vert_moved", n, bump(L[n], "p", 1e-3) if n >= 0 else None)
n = first(lambda l: l.startswith("RES type="));   emit("res_vol_changed", n, bump(L[n], "vol", 0.5) if n >= 0 else None)
n = first(lambda l: l.startswith("RES type="));   emit("res_naked_changed", n, bump(L[n], "naked", 1) if n >= 0 else None)
n = first(lambda l: l.startswith("RES type="));   emit("res_nface_changed", n, bump(L[n], "nface", 1) if n >= 0 else None)
n = first(lambda l: l.startswith("RESFACE "));    emit("res_face_dropped", n, None)
n = first(lambda l: l.startswith("DSVERT "));     emit("ds_vert_moved", n, bump(L[n], "p", 1e-2) if n >= 0 else None)
n = first(lambda l: l.startswith("SECPB "));      emit("sec_pblock_dropped", n, None)
n = first(lambda l: l.startswith("SEC tag="));    emit("sec_end_moved", n, bump(L[n], "p0", 1e-2) if n >= 0 else None)
n = first(lambda l: l.startswith("SEC tag="));    emit("sec_len_changed", n, bump(L[n], "len", 0.1) if n >= 0 else None)
n = first(lambda l: l.startswith("ARG i=0"));     emit("arg_nvert_changed", n, bump(L[n], "nvert", 1) if n >= 0 else None)
n = first(lambda l: l.startswith("ARG i=0"));     emit("arg_vol_changed", n, bump(L[n], "vol", 0.5) if n >= 0 else None)
n = first(lambda l: l.startswith("AVERT a=0"));   emit("arg_vert_moved", n, bump(L[n], "p", 1e-2) if n >= 0 else None)
PY

# mutation -> the stage that MUST report it
declare -A WANT=(
  [res_vert_moved]=res.vert_positions
  [res_vol_changed]=res.volume
  [res_naked_changed]=res.naked
  [res_nface_changed]=res.faces
  [res_face_dropped]=res.face_areas
  [ds_vert_moved]=ds.vertices
  [sec_pblock_dropped]=sec.pblocks
  [sec_end_moved]=sec.endpoints
  [sec_len_changed]=sec.lengths
  [arg_nvert_changed]=input.counts.arg0
  [arg_vol_changed]=input.volume.arg0
  [arg_vert_moved]=input.vertices.arg0
)

echo "=== NEGATIVE CONTROL on $CASE (harness must DETECT each injected defect) ==="
printf "%-22s %-24s %-6s %s\n" MUTATION EXPECTED_STAGE VERDICT ALL_FAILING_STAGES
att=0; caught=0
for m in "${!WANT[@]}"; do :; done
for m in $(printf '%s\n' "${!WANT[@]}" | sort); do
  f="$TMP/$m.trace"
  [ -f "$f" ] || { printf "%-22s %-24s %-6s %s\n" "$m" "${WANT[$m]}" SKIP "mutation not produced"; continue; }
  att=$((att + 1))
  allf=$(python3 ./v2_trace_diff.py "$OCCT/$CASE.trace" "$f" --row | awk '{print $NF}')
  if echo ",$allf," | grep -q ",${WANT[$m]},"; then
    caught=$((caught + 1)); v=CAUGHT
  else
    v=MISS
  fi
  printf "%-22s %-24s %-6s %s\n" "$m" "${WANT[$m]}" "$v" "$allf"
done

# and the unmutated original must still be clean
base=$(python3 ./v2_trace_diff.py "$OCCT/$CASE.trace" "$V2/$CASE.trace" --row | awk '{print $NF}')
echo "CONTROL unmutated=$base"
echo "MUTATION attempted=$att caught=$caught"
if [ "$att" -eq "$caught" ] && [ "$base" = "-" ]; then echo "MUTATION PASS"; else echo "MUTATION FAIL"; fi
