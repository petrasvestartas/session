#!/usr/bin/env python3
"""Validate and materialize a scene whose uploaded payload is an immutable revision."""
import json
import math
from pathlib import Path
import sys
import tomllib


def read(path):
    """Read the viewer's existing TOML, JSON or YAML manifest formats."""
    raw = Path(path).read_bytes()
    if len(raw) > 4 * 1024 * 1024:
        raise ValueError("manifest exceeds 4 MiB")
    if Path(path).suffix == ".toml":
        data = tomllib.loads(raw.decode())
    else:
        try:
            data = json.loads(raw)
        except ValueError:
            import yaml
            data = yaml.safe_load(raw)
    if not isinstance(data, dict) or not isinstance(data.get("items"), list) or not data["items"]:
        raise ValueError("manifest must contain a nonempty items array")
    for item in data["items"]:
        if not isinstance(item, dict) or not isinstance(item.get("file"), str) or not item["file"].strip():
            raise ValueError("each item must name a file")
    return data


def value(item):
    """Write the finite scalar/array values used by Session TOML scene manifests."""
    if isinstance(item, bool):
        return "true" if item else "false"
    if isinstance(item, str):
        return json.dumps(item, ensure_ascii=False)
    if isinstance(item, (int, float)) and math.isfinite(item):
        return repr(item)
    if isinstance(item, list):
        return "[" + ", ".join(value(entry) for entry in item) + "]"
    if isinstance(item, dict):
        return "{ " + ", ".join(json.dumps(key) + " = " + value(entry) for key, entry in item.items()) + " }"
    raise ValueError("manifest contains an unsupported or non-finite value")


def write(data, path):
    """Retain every authored placement/style field while changing only the payload reference."""
    if Path(path).suffix == ".toml":
        lines = [json.dumps(key) + " = " + value(entry) for key, entry in data.items() if key != "items"]
        for item in data["items"]:
            lines.extend(["", "[[items]]"])
            lines.extend(json.dumps(key) + " = " + value(entry) for key, entry in item.items())
        text = "\n".join(lines) + "\n"
    else:
        text = json.dumps(data, ensure_ascii=False, indent=2, allow_nan=False) + "\n"
    Path(path).write_text(text)


def main():
    """Print referenced keys, or rewrite the supplied geometry reference to a content hash."""
    data = read(sys.argv[1])
    if len(sys.argv) == 2:
        for item in data["items"]:
            print(item["file"])
        return
    geometry, revision, output = sys.argv[2:]
    names = {Path(geometry).name, "view_live.pb"}
    matched = False
    for item in data["items"]:
        if item["file"] == revision or (Path(item["file"]).name in names and not item["file"].startswith(("http://", "https://"))):
            item["file"] = revision
            matched = True
    if not matched:
        raise ValueError("manifest does not reference the supplied geometry or view_live.pb")
    write(data, output)


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, ImportError) as error:
        sys.exit("manifest: " + str(error))

# Workspace root: python3 bash/lib/view_manifest.py scene.toml scan.pb pb/revisions/<sha>.pb /tmp/view_live.toml
# Local publisher helper; public scene URL remains ?scene=view_live.toml.
