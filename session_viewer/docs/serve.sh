#!/usr/bin/env bash
# Render the maintained Markdown course with explicit Rust syntax highlighting.
set -euo pipefail
viewer_docs_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
viewer_docs_mode=${1:-serve}
if (($#)); then shift; fi
case "$viewer_docs_mode" in serve|build) ;; *) echo 'Usage: docs/serve.sh [serve|build] [MkDocs options]' >&2; exit 2 ;; esac
mkdir -p "$viewer_docs_root/target/docs/source"
cat > "$viewer_docs_root/target/docs/source/index.html" <<'HTML'
<!doctype html><html lang="en"><meta charset="utf-8"><meta http-equiv="refresh" content="0;url=docs/"><title>Session Viewer course</title><a href="docs/">Open the Session Viewer course</a></html>
HTML
ln -sfn "$viewer_docs_root/docs" "$viewer_docs_root/target/docs/source/docs"
ln -sfn "$viewer_docs_root/ARCHITECTURE.md" "$viewer_docs_root/target/docs/source/ARCHITECTURE.md"
ln -sfn "$viewer_docs_root/tests" "$viewer_docs_root/target/docs/source/tests"
mkdir -p "$viewer_docs_root/target/docs/source/assets/text"
ln -sfn "$viewer_docs_root/assets/text/README.md" "$viewer_docs_root/target/docs/source/assets/text/README.md"
exec uvx --with mkdocs-material==9.7.4 --with pygments==2.19.2 mkdocs==1.6.1 "$viewer_docs_mode" --config-file "$viewer_docs_root/mkdocs.yml" "$@"
