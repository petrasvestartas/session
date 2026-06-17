#!/usr/bin/env python3
"""Standalone server for the viewer build-log docs.

Serves this folder statically AND generates /sections.json on the fly by scanning
the *.md lessons (numbered NN-title.md; README and _-prefixed files are ignored). So
adding a lesson needs no manifest step — just drop in NN-title.md and refresh.

    cd session_viewer/docs && python serve.py      # http://localhost:8771
"""
import json
import os
import re
import http.server
import socketserver
from pathlib import Path

PORT = 8771
ROOT = Path(__file__).parent

# Fixed MIME map. We DO NOT use the mimetypes module: on Python 3.14 / Windows it
# reads the registry on first use, which is extremely slow and can hang the server.
EXT = {
    ".html": "text/html; charset=utf-8",
    ".js": "text/javascript; charset=utf-8",
    ".css": "text/css; charset=utf-8",
    ".json": "application/json; charset=utf-8",
    ".md": "text/plain; charset=utf-8",
    ".svg": "image/svg+xml",
    ".png": "image/png",
    ".ico": "image/x-icon",
    ".txt": "text/plain; charset=utf-8",
    ".wgsl": "text/plain; charset=utf-8",
    ".toml": "text/plain; charset=utf-8",
    ".rs": "text/plain; charset=utf-8",
    ".lock": "text/plain; charset=utf-8",
    ".ttf": "font/ttf",
    ".woff2": "font/woff2",
}


def list_sections():
    sections = []
    for md in sorted(ROOT.glob("*.md")):
        # Lessons are numbered (NN-title.md). This skips README.md and _-prefixed files.
        if not md.stem[:1].isdigit():
            continue
        text = md.read_text(encoding="utf-8", errors="replace")
        m = re.search(r"^#\s+(.+)$", text, re.MULTILINE)
        title = m.group(1).strip() if m else md.stem
        num = re.match(r"(\d+)", md.stem)
        order = int(num.group(1)) if num else 0
        sections.append({"id": md.stem, "title": title, "file": md.name, "order": order})
    sections.sort(key=lambda s: s["order"])
    return sections


class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(ROOT), **kwargs)

    def guess_type(self, path):
        ext = os.path.splitext(str(path))[1].lower()
        return EXT.get(ext, "application/octet-stream")

    def do_GET(self):
        if self.path.split("?")[0] == "/sections.json":
            body = json.dumps(list_sections()).encode("utf-8")
            self.send_response(200)
            self.send_header("Content-Type", "application/json; charset=utf-8")
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(body)
            return
        super().do_GET()

    def log_message(self, *args):
        pass  # quiet


if __name__ == "__main__":
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("", PORT), Handler) as httpd:
        print(f"viewer docs -> http://localhost:{PORT}")
        httpd.serve_forever()
