#!/usr/bin/env bash
# Web - start all three servers:
#   8769  session_tests   (minitest.sh, session code docs/tests viewer)
#   8771  session_viewer/docs/serve.py (viewer build-log docs)
#   8770  session_viewer  (trunk serve)
# Usage:
#   ./web.sh              # minitest (all languages) + docs + trunk serve
#   ./web.sh --py         # args before -- are passed to minitest.sh
#   ./web.sh --skip-tests # skip minitest, only start the two viewer servers
#   ./web.sh --kill       # stop all three servers

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/common.sh"

REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")
VIEWER_DIR="${REPO_ROOT}/session_viewer"
DOCS_DIR="${VIEWER_DIR}/docs"

TESTS_PORT=8769
TRUNK_PORT=8770
DOCS_PORT=8771

SKIP_TESTS=false
KILL_ONLY=false
MINITEST_ARGS=()

for arg in "$@"; do
    case $arg in
        --skip-tests|--no-tests)
            SKIP_TESTS=true
            ;;
        --kill|-k)
            KILL_ONLY=true
            ;;
        *)
            MINITEST_ARGS+=("$arg")
            ;;
    esac
done

kill_all() {
    for p in $TESTS_PORT $TRUNK_PORT $DOCS_PORT; do
        if port_in_use $p; then
            log "Stopping server on port ${p}"
            kill_port $p
        fi
    done
}

if [[ "$KILL_ONLY" == "true" ]]; then
    kill_all
    exit 0
fi

# a) minitest.sh - session code tests + Vue viewer on 8769
if [[ "$SKIP_TESTS" == "false" ]]; then
    "${SCRIPT_DIR}/minitest.sh" "${MINITEST_ARGS[@]}"
fi

# Wait for a port to start LISTENING, then open it - same rule minitest.sh uses for 8769:
# only what this run started gets a tab, so a re-run does not pile up duplicates.
# The wait is not politeness: trunk compiles the whole wasm before it binds, so a fixed
# sleep opens the tab on a connection-refused page.
open_when_up() {
    local port=$1
    local url=$2
    local waited=0
    while ! port_in_use "$port"; do
        if (( waited >= $3 )); then
            log "Port ${port} never came up after ${3}s - open ${url} manually"
            return
        fi
        sleep 1
        waited=$((waited + 1))
    done
    open_url "${url}"
}

# b) viewer docs - python serve.py on 8771
DOCS_STARTED=false
if port_in_use $DOCS_PORT; then
    log "Docs server already running on port ${DOCS_PORT}"
else
    log "Starting viewer docs on port ${DOCS_PORT}..."
    PY=$(get_python_path "$REPO_ROOT" 2>/dev/null || echo python3)
    [[ -x "$PY" ]] || PY=python3
    (cd "${DOCS_DIR}" && "$PY" serve.py >/tmp/session_docs.log 2>&1 &)
    DOCS_STARTED=true
fi

# c) viewer - trunk serve on 8770
TRUNK_STARTED=false
if port_in_use $TRUNK_PORT; then
    log "Trunk already running on port ${TRUNK_PORT}"
else
    if ! command -v trunk >/dev/null 2>&1; then
        log "trunk not found - install with: cargo install trunk"
    else
        log "Starting trunk serve on port ${TRUNK_PORT}..."
        (cd "${VIEWER_DIR}" && trunk serve >/tmp/session_trunk.log 2>&1 &)
        TRUNK_STARTED=true
    fi
fi

sleep 2
# Docs is a plain static server - up in a moment. Trunk builds the wasm first, which on a cold
# target dir is minutes, so it gets a far longer budget.
# if, not `[[ ]] &&`: under `set -e` a false one-liner IS the script's last command, so a
# re-run where both servers were already up would exit 1 and swallow the URL summary below.
if [[ "$DOCS_STARTED" == "true" ]]; then
    open_when_up $DOCS_PORT "http://localhost:${DOCS_PORT}" 15
fi
if [[ "$TRUNK_STARTED" == "true" ]]; then
    open_when_up $TRUNK_PORT "http://localhost:${TRUNK_PORT}" 600
fi
log "Tests:  http://localhost:${TESTS_PORT}/session/tests"
log "Docs:   http://localhost:${DOCS_PORT}"
log "Viewer: http://localhost:${TRUNK_PORT}"
log "Stop all: ./bash/web.sh --kill"
