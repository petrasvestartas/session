#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
python3 - "$here" <<'PY'
from pathlib import Path
import hashlib
import json
import sys
import urllib.request

here = Path(sys.argv[1])
agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36"
for face in json.loads((here / "fonts.json").read_text())["faces"]:
    request = urllib.request.Request(face["url"], headers={"User-Agent": agent})
    data = urllib.request.urlopen(request).read()
    digest = hashlib.sha256(data).hexdigest()
    if digest != face["sha256"]:
        raise ValueError(f'{face["file"]}: served {digest}, fonts.json records {face["sha256"]}')
    (here / face["file"]).write_bytes(data)
PY

# description: re-download the committed woff2 faces and fail if either differs from fonts.json.
# directory: cd ~/code/code_cpp/wood_research/session/session_viewer
# run: docs/stylesheets/fonts/fetch.sh
