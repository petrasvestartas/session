#!/usr/bin/env bash
# Shared functions for minitest system

# Single source of truth for class names (sorted alphabetically)
CLASS_NAMES=("color" "knot" "line" "mesh" "nurbscurve" "nurbssurface" "plane" "point" "pointcloud" "polyline" "primitives" "tolerance" "trimmedsurface" "vector" "xform")

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
