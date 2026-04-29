#!/bin/bash
# Lint and reformat source and test files across all 3 languages.
# Enforces consistent style for coordinate data, constructor args, and MINI_CHECK chains.
#
# Usage:
#   ./bash/lint.sh                         # all languages, tests + src
#   ./bash/lint.sh --py                    # Python tests + src
#   ./bash/lint.sh --rust                  # Rust tests + src
#   ./bash/lint.sh --cpp                   # C++ tests + src
#   ./bash/lint.sh --py --tests-only       # Python tests only
#   ./bash/lint.sh --py --src-only         # Python src only
#   ./bash/lint.sh --rust path/to/file.rs  # single file

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Rules applied to ALL files (tests + src)
RULES_COMMON='
## RULE 1 — One item per line in coordinate/object arrays
Any list/array/vec! containing 2 or more Point, Vector, Color, Line, Plane, Polyline, or
BoundingBox constructors: always put each item on its own indented line, regardless of
total line length.

BEFORE (C++):  {{1.28955,0,1.127558},{0.85791,0,0.225512}}
AFTER:
    {
        {1.28955, 0, 1.127558},
        {0.85791, 0, 0.225512},
    }

BEFORE (Python):  [Point(0,0,0), Point(1,0,0)]
AFTER:
    [
        Point(0, 0, 0),
        Point(1, 0, 0),
    ]

BEFORE (Rust):  vec![Point::new(0.0,0.0,0.0), Point::new(1.0,0.0,0.0)]
AFTER:
    vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
    ]

Exception: 2-element bounding box min/max pairs that are clearly min/max may stay inline
if it reads more clearly (use judgement — when in doubt, expand).

## RULE 2 — Spaces after commas in ALL constructor arguments
Every Point, Vector, Color, Line, Plane, Polyline, NurbsCurve, BoundingBox constructor
must have a space after each comma between arguments. Applies inside array literals too.

BEFORE:  Point(1.28955,0.0,1.127558)    →   AFTER:  Point(1.28955, 0.0, 1.127558)
BEFORE:  Point::new(1.0,2.0,3.0)        →   AFTER:  Point::new(1.0, 2.0, 3.0)
BEFORE:  {1.28955,0.0,1.127558}         →   AFTER:  {1.28955, 0.0, 1.127558}
BEFORE:  Color(255,0,0,255)              →   AFTER:  Color(255, 0, 0, 255)
BEFORE:  [0,0,0,0,1,1,1,1]              →   AFTER:  [0, 0, 0, 0, 1, 1, 1, 1]   (nurbsknot vectors)
BEFORE:  vec![0.0,0.0,1.0,1.0]          →   AFTER:  vec![0.0, 0.0, 1.0, 1.0]

## RULE 3 — One Line/Polyline per line in arrays
Any array/list/vec! containing 2 or more Line::from_points()/Line.from_points()/Polyline(...)
constructors: always put each constructor on its own line (same logic as Rule 1).

## RULE 4 — NurbsCurve/NurbsSurface constructor point lists
NurbsCurve.create()/NurbsCurve::create()/NurbsSurface factory methods with a points list
as first argument containing 2 or more points: apply Rule 1 to that points list.

## RULE 5 — Polyline with many points
Polyline([...])/Polyline::new(vec![...]) with more than 4 points: always expand each
point to its own line (apply Rule 1).
'

# Additional rules applied only to test files (contain MINI_CHECK)
RULES_TESTS='
## RULE 6 — Split long MINI_CHECK condition chains
Any MINI_CHECK/MINI_CHECK! that chains multiple independent conditions with && or "and"
on a line longer than ~100 chars: split into one MINI_CHECK call per condition.

BEFORE (C++):   MINI_CHECK(TOLERANCE.is_close(k[0], 0.0) && TOLERANCE.is_close(k[1], 0.5) && TOLERANCE.is_close(k[2], 1.0));
AFTER:
    MINI_CHECK(TOLERANCE.is_close(k[0], 0.0));
    MINI_CHECK(TOLERANCE.is_close(k[1], 0.5));
    MINI_CHECK(TOLERANCE.is_close(k[2], 1.0));

BEFORE (Python):  MINI_CHECK(a == 1 and b == 2 and c == 3 and d == 4 and e == 5)
AFTER:
    MINI_CHECK(a == 1)
    MINI_CHECK(b == 2)
    MINI_CHECK(c == 3)
    MINI_CHECK(d == 4)
    MINI_CHECK(e == 5)

Exception: keep chains that fit under ~100 chars total on one line.

## RULE 7 — Split long string search chains in MINI_CHECK
MINI_CHECK/MINI_CHECK! checking multiple string.find()/contains()/"x" in s conditions
with && or "and" on a line longer than ~100 chars: split to one MINI_CHECK per check.

BEFORE (C++):   MINI_CHECK(s.find("10") != std::string::npos && s.find("20") != std::string::npos && s.find("name") != std::string::npos);
AFTER:
    MINI_CHECK(s.find("10") != std::string::npos);
    MINI_CHECK(s.find("20") != std::string::npos);
    MINI_CHECK(s.find("name") != std::string::npos);

BEFORE (Python):  MINI_CHECK("10" in s and "20" in s and "name" in s)
AFTER:
    MINI_CHECK("10" in s)
    MINI_CHECK("20" in s)
    MINI_CHECK("name" in s)

## RULE 8 — MINI_TEST names must use spaces, not underscores
The second argument of MINI_TEST/MINI_TEST! (the test name) must use spaces between words,
not underscores. The class name (first argument) is unaffected.

BEFORE (C++):    MINI_TEST("Mesh", "From_polygon_with_holes") {
AFTER (C++):     MINI_TEST("Mesh", "From Polygon With Holes") {

BEFORE (Python): @MINI_TEST("Mesh", "From_polygon_with_holes")
AFTER (Python):  @MINI_TEST("Mesh", "From Polygon With Holes")

BEFORE (Rust):   MINI_TEST!("From_polygon_with_holes", {
AFTER (Rust):    MINI_TEST!("From Polygon With Holes", {

Rules for converting underscore names to space-separated title case:
- Replace every underscore with a space
- Capitalize the first letter of each word
- Preserve existing capitalization of acronyms (e.g. "Json" stays "Json", "Bvh" stays "Bvh")
- Exception: if the name is already using spaces, leave it unchanged

Also update the corresponding REGISTER lines to match the new names:
BEFORE (C++):    REGISTER("From_polygon_with_holes", run_mesh_from_polygon_with_holes);
AFTER (C++):     REGISTER("From Polygon With Holes", run_mesh_from_polygon_with_holes);

BEFORE (Python): REGISTER_MINI_TEST("From_polygon_with_holes", run_mesh_from_polygon_with_holes)
AFTER (Python):  REGISTER_MINI_TEST("From Polygon With Holes", run_mesh_from_polygon_with_holes)

BEFORE (Rust):   REGISTER_MINI_TEST!("From_polygon_with_holes", run_mesh_from_polygon_with_holes);
AFTER (Rust):    REGISTER_MINI_TEST!("From Polygon With Holes", run_mesh_from_polygon_with_holes);

## RULE 9 — Remove redundant bool literal comparisons in MINI_CHECK
Any MINI_CHECK/MINI_CHECK! that compares a simple variable to a bool literal: drop the
comparison and use the bare variable or its negation.

BEFORE (C++):    MINI_CHECK(eq == true);      AFTER:  MINI_CHECK(eq);
BEFORE (C++):    MINI_CHECK(sq2 == false);    AFTER:  MINI_CHECK(!sq2);
BEFORE (Python): MINI_CHECK(eq == True)       AFTER:  MINI_CHECK(eq)
BEFORE (Python): MINI_CHECK(sq2 == False)     AFTER:  MINI_CHECK(not sq2)
BEFORE (Rust):   MINI_CHECK!(ok == true);     AFTER:  MINI_CHECK!(ok);
BEFORE (Rust):   MINI_CHECK!(ok == false);    AFTER:  MINI_CHECK!(!ok);

Only apply when the subject is a simple identifier (variable name). Do not strip when
the subject is a method call or a complex expression — leave those unchanged.

## RULE 10 — Exactly one blank line before first MINI_CHECK in each test block
The last variable/assignment line in each test block must be followed by exactly one
blank line before the first MINI_CHECK/MINI_CHECK! call. C++ is ground truth.

BEFORE (missing blank line):
    let d = ml.duplicate();
    MINI_CHECK!(m.rows == 2);

AFTER:
    let d = ml.duplicate();

    MINI_CHECK!(m.rows == 2);

BEFORE (excess blank lines):
    let d = ml.duplicate();


    MINI_CHECK!(m.rows == 2);

AFTER:
    let d = ml.duplicate();

    MINI_CHECK!(m.rows == 2);
'

PROMPT_TESTS="Read the file at the given path. Apply ALL rules below. Do NOT change code logic, variable names, or program behavior. Only apply rules to lines that actually need it.
$RULES_COMMON
$RULES_TESTS
File to lint:"

PROMPT_SRC="Read the file at the given path. Apply ALL rules below. Do NOT change function signatures, class/struct definitions, algorithm logic, or docstrings. Only reformat data literals and long constructor chains.
$RULES_COMMON
File to lint:"

PARALLEL=${PARALLEL:-20}
MODEL=${MODEL:-claude-haiku-4-5-20251001}

# Single-file reformat (used for explicit file arg)
reformat() {
    local file="$1"
    local prompt="$2"
    echo "  $(basename "$file")"
    claude --dangerously-skip-permissions --model "$MODEL" -p "$prompt $file"
}

# Run a batch of files in parallel; collect outputs and print in order
run_batch() {
    local prompt="$1"; shift
    local files=("$@")
    local tmps=()
    for f in "${files[@]}"; do
        local tmp
        tmp=$(mktemp)
        tmps+=("$tmp")
        { echo "  $(basename "$f")"; claude --dangerously-skip-permissions -p "$prompt $f"; } \
            > "$tmp" 2>&1 &
    done
    wait
    for tmp in "${tmps[@]}"; do
        cat "$tmp"
        rm -f "$tmp"
    done
}

run_tests() {
    local lang="$1"
    local files=()
    case "$lang" in
        py)   for f in "$ROOT"/session_py/src/session_py/*_test.py;  do [ -f "$f" ] && files+=("$f"); done ;;
        rust) for f in "$ROOT"/session_rust/src/*_test.rs;            do [ -f "$f" ] && files+=("$f"); done ;;
        cpp)  for f in "$ROOT"/session_cpp/src/*_test.cpp;            do [ -f "$f" ] && files+=("$f"); done ;;
    esac
    local i=0
    while [ $i -lt ${#files[@]} ]; do
        run_batch "$PROMPT_TESTS" "${files[@]:$i:$PARALLEL}"; (( i += PARALLEL ))
    done
}

run_src() {
    local lang="$1"
    echo "  [src]"
    local files=()
    case "$lang" in
        py)
            for f in "$ROOT"/session_py/src/session_py/*.py; do
                base="$(basename "$f")"
                [[ "$base" == *_test.py ]] && continue
                [[ "$base" == __init__.py ]] && continue
                [[ "$base" == mini_test.py ]] && continue
                [ -f "$f" ] && files+=("$f")
            done
            ;;
        rust)
            for f in "$ROOT"/session_rust/src/*.rs; do
                base="$(basename "$f")"
                [[ "$base" == *_test.rs ]] && continue
                [[ "$base" == mini_test.rs ]] && continue
                [[ "$base" == lib.rs ]] && continue
                [ -f "$f" ] && files+=("$f")
            done
            ;;
        cpp)
            for f in "$ROOT"/session_cpp/src/*.cpp "$ROOT"/session_cpp/src/*.h; do
                base="$(basename "$f")"
                [[ "$base" == *_test.cpp ]] && continue
                [ -f "$f" ] && files+=("$f")
            done
            ;;
    esac
    local i=0
    while [ $i -lt ${#files[@]} ]; do
        run_batch "$PROMPT_SRC" "${files[@]:$i:$PARALLEL}"
        (( i += PARALLEL ))
    done
}

run_lang() {
    local lang="$1"
    local mode="${2:---all}"
    echo "[$lang]"
    [[ "$mode" != "--src-only"   ]] && run_tests "$lang"
    [[ "$mode" != "--tests-only" ]] && run_src "$lang"
}

# Parse args
MODE="--all-langs"
FILTER=""
FILE=""

for arg in "$@"; do
    case "$arg" in
        --py|--rust|--cpp) FILTER="$arg" ;;
        --tests-only|--src-only) FILTER2="$arg" ;;
        --all) ;;
        *) FILE="$arg" ;;
    esac
done

if [ -n "$FILE" ]; then
    # Single file mode — detect language from extension
    case "$FILE" in
        *.py)  reformat "$FILE" "$PROMPT_TESTS" ;;
        *.rs)  reformat "$FILE" "$PROMPT_TESTS" ;;
        *.cpp|*.h) reformat "$FILE" "$PROMPT_TESTS" ;;
    esac
elif [ -n "$FILTER" ]; then
    lang="${FILTER#--}"
    run_lang "$lang" "${FILTER2:---all}"
else
    run_lang "cpp" "${FILTER2:---all}"
    run_lang "py"  "${FILTER2:---all}"
    run_lang "rust" "${FILTER2:---all}"
fi

echo "Done."
