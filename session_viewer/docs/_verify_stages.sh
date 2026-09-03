#!/usr/bin/env bash
# Prove a lesson: replay NN-*.md onto stage NN-1 and demand the result is BYTE-IDENTICAL to
# stage NN, then compile the stage. Every lesson must pass this before it is shipped.
#
#   docs/_verify_stages.sh 05          one lesson (stage 04 -> lesson 05 -> stage 05)
#   docs/_verify_stages.sh             every lesson in order
#
# Stages live in docs/_stages/NN_slug/ (src, Cargo.toml, .cargo, index.html, Trunk.toml, and from
# lesson 11 on examples/); stage 00 is the empty crate directory. `docs/_stages/session_rust` is a
# symlink so `path = "../session_rust"` resolves from every stage, exactly as it does for a reader
# building next to the kernel.
set -u
cd "$(dirname "$0")"
export REGEN_PROTO=0
work=$(mktemp -d) && trap 'rm -rf "$work"' EXIT

stage_dir() { ls -d _stages/$(printf '%02d' "$1")_* 2>/dev/null | head -1; }
lesson_md() { ls $(printf '%02d' "$1")-*.md 2>/dev/null | head -1; }

verify() {
    local n=$1 prev cur md
    prev=$(stage_dir $((n - 1))); cur=$(stage_dir "$n"); md=$(lesson_md "$n")
    [ -n "$cur" ] && [ -n "$md" ] || { echo "lesson $n: missing stage or lesson file"; return 1; }
    rm -rf "$work/w" "$work/snap"; mkdir -p "$work/snap"
    [ -n "$prev" ] && cp -r "$prev/." "$work/snap/"
    rm -rf "$work/snap/target"
    python3 _replay_check.py "$work/snap" "$work/w" "$md" > "$work/replay.log" 2>&1 || { echo "lesson $n: replay FAILED"; tail -20 "$work/replay.log"; return 1; }
    grep -qiE "fail|not found|no match|ambiguous" "$work/replay.log" && { echo "lesson $n: replay reported problems"; grep -iE "fail|not found|no match|ambiguous" "$work/replay.log" | head -10; return 1; }
    if ! diff -r -q -x target -x Cargo.lock "$work/w" "$cur" > "$work/diff.log"; then
        echo "lesson $n: replay of $md onto stage $((n - 1)) differs from stage $n:"; head -20 "$work/diff.log"; return 1
    fi
    if ! (cd "$cur" && CARGO_TARGET_DIR="$HOME/.cache/tstages/$(printf '%02d' "$n")" cargo check -q --target wasm32-unknown-unknown 2> "$work/check.log") || [ -s "$work/check.log" ]; then
        echo "lesson $n: stage $n does not compile cleanly (errors or warnings):"; head -30 "$work/check.log"; return 1
    fi
    echo "lesson $n OK ($md -> $cur)"
}

if [ $# -gt 0 ]; then verify "$1"; exit $?; fi
for md in [0-9][0-9]-*.md; do
    n=$((10#${md%%-*}))
    [ "$n" -eq 0 ] && continue
    verify "$n" || exit 1
done
echo "all lessons OK"
