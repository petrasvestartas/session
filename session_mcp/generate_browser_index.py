#!/usr/bin/env python3
"""
Generate browser-compatible API index for Vue application.

This creates a JavaScript file that sets window.API_INDEX with all
Session API methods for client-side search in the Vue test viewer.

Usage:
    python session_mcp/generate_browser_index.py
"""

import json
from pathlib import Path
from .server import APIIndex


def generate():
    """Generate browser index and write to session_tests/public/apiIndex.js"""
    index = APIIndex()
    data = index.to_browser_format()

    output_dir = Path(__file__).parent.parent / "session_tests" / "public"
    output_dir.mkdir(parents=True, exist_ok=True)
    output_file = output_dir / "apiIndex.js"

    js_content = f"// Auto-generated API index for browser search\nwindow.API_INDEX = {json.dumps(data, indent=2)};\n"
    output_file.write_text(js_content)

    print(f"Generated {output_file}")
    print(f"  - {len(data['concepts'])} methods indexed")
    print(f"  - Classes: {', '.join(sorted(set(c['name'].split('.')[0] for c in data['concepts'])))}")


if __name__ == "__main__":
    generate()
