#!/usr/bin/env bash
# Shared functions for minitest system

# Single source of truth for class names (sorted alphabetically)
CLASS_NAMES=("aabb" "boolean_polyline" "brep" "closest" "color" "convex_hull" "element" "element_beam" "element_column" "element_plate" "file_encoders" "file_obj" "file_step" "graph" "intersection" "io" "nurbsknot" "line" "instance_ref" "matrix" "mesh" "mesh_offset" "nurbscurve" "nurbssurface" "obb" "objects" "plane" "point" "pointcloud" "polyline" "primitives" "quaternion" "remesh_cdt" "remesh_nurbssurface_grid" "remesh_nurbssurface_adaptive" "session" "session_config" "spatial_aabbtree" "spatial_bvh" "spatial_kdtree" "spatial_rtree" "tolerance" "tree" "nurbssurface_trimmed" "vector" "xform")

# Classes with NO test source in a given language, so they legitimately produce no json.
# These are reported as SKIP; every other class in CLASS_NAMES must still emit json or the
# run fails, which is the whole point of the missing-class guard in print_class_summary.
# Delete an entry the moment its test file lands (C++: src/<cls>_test.cpp,
# Python: src/session_py/<cls>_test.py, Rust: src/<cls>_test.rs).
# ("pdf" is not a class: the Rust-only PDF import test lives in the Io suite, feature-gated.)
NOT_IMPLEMENTED_cpp=""
NOT_IMPLEMENTED_py="file_step"
NOT_IMPLEMENTED_rust="file_step"

# Resolve repo root from script location
resolve_repo_root() {
    local script_path="$1"
    local script_dir="$(cd "$(dirname "$script_path")" && pwd)"
    if [[ "$script_dir" == */lib ]]; then
        echo "$(dirname "$(dirname "$script_dir")")"
    elif [[ "$script_dir" == */bash ]]; then
        echo "$(dirname "$script_dir")"
    else
        echo "$script_dir"
    fi
}

# Detect platform
detect_platform() {
    if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]] || [[ -n "$MSYSTEM" ]]; then
        echo "windows"
    else
        echo "unix"
    fi
}

# Convert MSYS/Cygwin path to Windows path for native tools (pip, cmake, etc.)
to_windows_path() {
    local path="$1"
    if [[ "$(detect_platform)" == "windows" ]]; then
        # Use cygpath if available, otherwise manual conversion
        if command -v cygpath >/dev/null 2>&1; then
            cygpath -w "$path"
        else
            # Manual: /d/foo -> D:\foo
            echo "$path" | sed -e 's|^/\([a-zA-Z]\)/|\1:\\|' -e 's|/|\\|g'
        fi
    else
        echo "$path"
    fi
}

# Get Python executable path
get_python_path() {
    local repo_root="$1"
    local platform=$(detect_platform)
    if [[ "$platform" == "windows" ]]; then
        echo "${repo_root}/uvsession/Scripts/python.exe"
    else
        echo "${repo_root}/uvsession/bin/python"
    fi
}

# Check if port is in use
port_in_use() {
    local port=$1
    local platform=$(detect_platform)
    if [[ "$platform" == "windows" ]]; then
        netstat -ano 2>/dev/null | grep ":${port}" | grep -q "LISTENING"
    else
        lsof -i ":${port}" >/dev/null 2>&1
    fi
}

# Kill process on port
kill_port() {
    local port=$1
    local platform=$(detect_platform)
    if [[ "$platform" == "windows" ]]; then
        local pids=$(netstat -ano 2>/dev/null | grep ":${port}" | grep "LISTENING" | awk '{print $5}' | sort -u)
        for pid in $pids; do
            [[ -n "$pid" && "$pid" != "0" ]] && taskkill //F //PID "$pid" >/dev/null 2>&1 || true
        done
    else
        lsof -ti ":${port}" 2>/dev/null | xargs kill -9 2>/dev/null || true
    fi
}

# Logging functions
log() {
    echo "[mini] $*"
}

log_lang() {
    local lang="$1"
    shift
    echo "[$lang] $*"
}

# Check if uv is available
has_uv() {
    command -v uv >/dev/null 2>&1
}

# Get number of CPU cores
get_jobs() {
    echo "${MINITEST_JOBS:-$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)}"
}

# Print per-class pass/fail summary by reading JSON output files.
# Iterates CLASS_NAMES -- it must NEVER glob. Globbing counted JSON left behind by DELETED
# classes as passing tests (elementfeature 10 + reciprocal 1 = 11 phantom C++ tests, plus
# phantom py/rust file_step) and could not notice a class that produced no JSON at all
# (C++ has no `io` tests although py and rust do). Here a missing class and a stale
# artifact are both FAILURES, not silence. Set MINITEST_LENIENT=1 to downgrade the
# missing/stale anomalies to warnings (real test failures still fail). Pass the binary's own aggregate as arg 4 and it is reconciled against the
# per-class sum instead of being added to it (adding them is where "1531" came from).
# Usage: print_class_summary <lang_prefix> <json_dir> <python_exe> [aggregate_total]
print_class_summary() {
    local lang="$1"
    local json_dir="$2"
    local python_exe="${3:-python3}"
    local aggregate="${4:-}"
    [[ -d "$json_dir" ]] || return 0
    [[ -x "$python_exe" ]] || python_exe="python3"
    local skip_var="NOT_IMPLEMENTED_${lang}"
    # $python_exe is a NATIVE Windows interpreter under MSYS bash, so it cannot open the
    # MSYS-style /d/a/... path that $json_dir carries there: every class then looks missing
    # and the run fails with "0/0 classes" while the tests themselves all passed.
    local json_dir_native=$(to_windows_path "$json_dir")
    MINITEST_SKIP="${!skip_var:-}" \
    "$python_exe" - "$json_dir_native" "$lang" "$aggregate" "${CLASS_NAMES[@]}" <<'EOF'
import json, glob, os, sys
d, lang, aggregate = sys.argv[1], sys.argv[2], sys.argv[3]
lenient = os.environ.get('MINITEST_LENIENT') == '1'
tag = 'WARN' if lenient else 'ERROR'
not_implemented = set(os.environ.get('MINITEST_SKIP', '').split())
classes = sys.argv[4:]
total = passed = 0
missing, stale, failed, seen, skipped = [], [], [], set(), []
for cls in classes:
    f = os.path.join(d, cls + '_test.json')
    if not os.path.exists(f):
        (skipped if cls in not_implemented else missing).append(cls)
        continue
    seen.add(os.path.basename(f))
    tests = json.load(open(f))
    n = len(tests)
    p = sum(1 for t in tests if t.get('passed', False))
    total += n
    passed += p
    if p != n:
        failed.append(cls)
    print(f'[{lang}-{cls}] {p}/{n} ' + ('passed' if p == n else 'FAILED'))
for f in sorted(glob.glob(os.path.join(d, '*_test.json'))):
    if os.path.basename(f) not in seen:
        stale.append(os.path.basename(f).replace('_test.json', ''))
print(f'[{lang}] TOTAL {passed}/{total} over {len(classes) - len(missing) - len(skipped)}/'
      f'{len(classes) - len(skipped)} classes')
rc = 0
if skipped:
    print(f'[{lang}] SKIP: {len(skipped)} class(es) not implemented in this language: '
          f'{" ".join(skipped)}')
if missing:
    print(f'[{lang}] {tag}: {len(missing)} class(es) in CLASS_NAMES produced NO json - '
          f'this language does not implement them or they did not run: '
          f'{" ".join(missing)}')
    rc = rc if lenient else 1
if stale:
    print(f'[{lang}] {tag}: {len(stale)} json file(s) with no entry in CLASS_NAMES - '
          f'either add the class to CLASS_NAMES (if its test source exists) or delete '
          f'the file; it is NOT a passing test: {" ".join(stale)}')
    rc = rc if lenient else 1
if failed:
    print(f'[{lang}] ERROR: failing classes: {" ".join(failed)}')
    rc = 1
if aggregate:
    try:
        exp = int(aggregate)
    except ValueError:
        exp = None
    if exp is not None and exp != total:
        print(f'[{lang}] ERROR: binary aggregate {exp} != per-class sum {total} - these '
              f'count the SAME tests and must reconcile, never be summed')
        rc = 1
sys.exit(rc)
EOF
}

# Setup Windows tool paths for MINGW64/MSYS2
setup_windows_paths() {
    [[ "$(detect_platform)" != "windows" ]] && return 0

    local user_home="/c/Users/${USERNAME:-$USER}"
    local paths_to_add=""

    # Cargo/Rust
    if ! command -v cargo >/dev/null 2>&1; then
        local cargo_bin="${user_home}/.cargo/bin"
        [[ -d "$cargo_bin" ]] && paths_to_add="${paths_to_add}:${cargo_bin}"
    fi

    # CMake - check common locations
    if ! command -v cmake >/dev/null 2>&1; then
        local cmake_paths=(
            "/c/Program Files/CMake/bin"
            "/c/Program Files (x86)/CMake/bin"
            "${user_home}/AppData/Local/CMake/bin"
            "/c/Program Files/Microsoft Visual Studio/2022/Community/Common7/IDE/CommonExtensions/Microsoft/CMake/CMake/bin"
            "/c/Program Files/Microsoft Visual Studio/2022/Professional/Common7/IDE/CommonExtensions/Microsoft/CMake/CMake/bin"
            "/c/Program Files/Microsoft Visual Studio/2022/BuildTools/Common7/IDE/CommonExtensions/Microsoft/CMake/CMake/bin"
        )
        for p in "${cmake_paths[@]}"; do
            if [[ -f "${p}/cmake.exe" ]]; then
                paths_to_add="${paths_to_add}:${p}"
                break
            fi
        done
    fi

    # Node/npm
    if ! command -v npm >/dev/null 2>&1; then
        local node_paths=(
            "/c/Program Files/nodejs"
            "${user_home}/AppData/Roaming/nvm/current"
            "${user_home}/AppData/Local/Programs/nodejs"
        )
        # Add nvm-windows versions (find latest)
        local nvm_dir="${user_home}/AppData/Local/nvm"
        if [[ -d "$nvm_dir" ]]; then
            local latest_nvm=$(ls -d "${nvm_dir}"/v* 2>/dev/null | sort -V | tail -1)
            [[ -n "$latest_nvm" ]] && node_paths+=("$latest_nvm")
        fi
        for p in "${node_paths[@]}"; do
            if [[ -f "${p}/npm.cmd" ]] || [[ -f "${p}/npm" ]]; then
                paths_to_add="${paths_to_add}:${p}"
                break
            fi
        done
    fi

    # Add found paths
    if [[ -n "$paths_to_add" ]]; then
        export PATH="${PATH}${paths_to_add}"
    fi
}

# Auto-setup paths when sourced
setup_windows_paths
