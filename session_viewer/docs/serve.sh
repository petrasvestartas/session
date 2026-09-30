#!/usr/bin/env bash
set -euo pipefail
site=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../session_tests" && pwd)
mode=${1:-serve}
if (($#)); then shift; fi

case "$mode" in
    serve) port=8769 ;;
    build) exec env DOCS_STRICT_LINKS=1 npm --prefix "$site" run build -- "$@" ;;
    preview) port=8788 ;;
    *) echo 'Usage: docs/serve.sh [serve|build|preview] [Vite options]' >&2; exit 2 ;;
esac

# Respect a custom port before stopping its listener; leave client connections alone.
args=("$@")
for ((i=0; i<${#args[@]}; i++)); do
    case "${args[i]}" in
        --port) port=${args[i+1]:-} ;;
        --port=*) port=${args[i]#--port=} ;;
    esac
done
if [[ ! "$port" =~ ^[0-9]{1,5}$ ]] || ((10#$port < 1 || 10#$port > 65535)); then
    echo 'Port must be a number between 1 and 65535.' >&2
    exit 2
fi
port=$((10#$port))
command -v lsof >/dev/null || { echo 'Install lsof to stop an existing server.' >&2; exit 1; }
mapfile -t listeners < <(lsof -t -iTCP:"$port" -sTCP:LISTEN)
if ((${#listeners[@]})); then
    echo "Stopping server on port $port (PID ${listeners[*]})."
    kill -TERM -- "${listeners[@]}"
    for attempt in {1..50}; do
        if ! lsof -t -iTCP:"$port" -sTCP:LISTEN >/dev/null; then break; fi
        sleep 0.1
    done
    if lsof -t -iTCP:"$port" -sTCP:LISTEN >/dev/null; then
        echo "Port $port is still occupied after stopping the server." >&2
        exit 1
    fi
fi

case "$mode" in
    serve) exec npm --prefix "$site" run dev -- --host 127.0.0.1 --port "$port" --strictPort "$@" ;;
    preview)
        exec npm --prefix "$site" run preview -- --host 127.0.0.1 --port "$port" --strictPort \
            --base /docs/ --outDir ../session_viewer/target/docs/vue "$@"
        ;;
esac
