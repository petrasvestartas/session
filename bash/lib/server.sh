#!/usr/bin/env bash
# Manage Vue dev server lifecycle

# Only set SCRIPT_DIR if not already set (when sourced from parent)
if [[ -z "${SCRIPT_DIR:-}" ]] || [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    source "${SCRIPT_DIR}/common.sh"
fi

PORT=8769
# Hash-route form: the app (createWebHashHistory) reads suite only from the fragment, so a
# pre-hash ?suite= would sit in the address bar as a stale duplicate that never updates.
URL="http://localhost:${PORT}/session/tests#/tests?suite=point_test"

start_server() {
    local repo_root="$1"
    local tests_dir="${repo_root}/session_tests"

    if port_in_use $PORT; then
        log "Dev server already running on port ${PORT}"
        log "Vite will hot-reload changes automatically"
        log "View at: ${URL}"
        return 0
    fi

    log "Starting development server..."
    (cd "${tests_dir}" && npm run dev >/dev/null 2>&1 &)
    sleep 2

    log "Server started at ${URL}"

    # Open browser (chrome with the WebGPU flags - see open_url in common.sh)
    open_url "${URL}"
}

stop_server() {
    if port_in_use $PORT; then
        log "Stopping dev server on port ${PORT}..."
        kill_port $PORT
        log "Server stopped"
    else
        log "No server running on port ${PORT}"
    fi
}

ensure_node_deps() {
    local repo_root="$1"
    local tests_dir="${repo_root}/session_tests"

    if ! command -v npm >/dev/null 2>&1; then
        log "npm not found. Please install Node.js first."
        return 1
    fi

    local lock="${tests_dir}/node_modules/.package-lock.json"
    if [[ -f "$lock" && "$lock" -nt "${tests_dir}/package.json" ]]; then
        log "npm deps up-to-date, skipping install"
        return 0
    fi

    log "Installing npm dependencies..."
    (cd "${tests_dir}" && npm install --silent) || {
        log "npm install failed, retrying..."
        rm -rf "${tests_dir}/node_modules" "${tests_dir}/package-lock.json"
        (cd "${tests_dir}" && npm install) || return 1
    }
}

# Command dispatch when run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")

    case "${1:-}" in
        start)
            ensure_node_deps "$REPO_ROOT" "${2:-false}"
            start_server "$REPO_ROOT"
            ;;
        stop)
            stop_server
            ;;
        restart)
            stop_server
            sleep 1
            ensure_node_deps "$REPO_ROOT" "${2:-false}"
            start_server "$REPO_ROOT"
            ;;
        *)
            echo "Usage: server.sh {start|stop|restart} [--fast]"
            exit 1
            ;;
    esac
fi
