#!/usr/bin/env python3
"""Verify final tutorial sources against the frozen production bytes."""
import argparse
import hashlib
import json
from pathlib import Path
import replay


HERE = Path(__file__).resolve().parent


def packaging_reason(name):
    """Allow only the documented local-input packaging differences."""
    if name == "session_viewer/assets/view_local.yaml":
        return "Deterministic local interaction scene replaces the user's local working datasets."
    if name.startswith("session_viewer/assets/fonts/"):
        return "Retained imported-document font artifacts; PDF outline geometry is baked into the source meshes."
    return None


def verify(workspace, production=None):
    """Require every frozen runtime byte and reject an extra tutorial implementation."""
    baseline = replay.read_json(HERE / "baseline.json")
    series = replay.read_json(HERE / "series.json")
    if series["production_tree"] != baseline["tree_sha256"]:
        raise ValueError("tutorial series targets a different production freeze")
    final = series["steps"][-1]
    if final["id"] != "16":
        raise ValueError("production convergence requires checkpoint 16")
    replay.check_files(workspace, final["files"])
    same, packaging = {}, {}
    for name, checksum in sorted(baseline["files"].items()):
        if final["files"].get(name) != checksum:
            reason = packaging_reason(name)
            if reason is None:
                raise ValueError(f"runtime differs from production: {name}")
            packaging[name] = reason
        else:
            same[name] = checksum
        if production is not None:
            source = production.parent / name
            if not source.is_file() or replay.digest(source) != checksum:
                raise ValueError(f"production changed after freeze: {name}")
    expected_source = {name for name in baseline["files"] if name.startswith("session_viewer/src/")}
    actual_source = {"session_viewer/" + path.relative_to(workspace / "session_viewer").as_posix()
                     for path in (workspace / "session_viewer/src").rglob("*") if path.is_file()}
    if actual_source != expected_source:
        raise ValueError(f"extra/missing runtime source: {sorted(actual_source ^ expected_source)}")
    encoding = "".join(name + "\0" + checksum + "\n" for name, checksum in sorted(baseline["files"].items()))
    if hashlib.sha256(encoding.encode()).hexdigest() != baseline["tree_sha256"]:
        raise ValueError("production inventory hash is invalid")
    return {"passed": True, "production_tree": baseline["tree_sha256"],
            "runtime_source_files": len(expected_source), "files": same,
            "packaging_differences": packaging,
            "tutorial_input_additions": ["session_viewer/assets/pb/interaction.pb",
                                         "session_viewer/assets/pb/interaction.pb.json"],
            "excluded_runtime_mismatches": {}}


def main():
    """Inspect an already reconstructed checkpoint without changing its sources."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workspace", type=Path, required=True)
    parser.add_argument("--production", type=Path, help="also compare the live production viewer checkout")
    parser.add_argument("--output", type=Path, help="write the complete checked inventory")
    args = parser.parse_args()
    report = verify(args.workspace.resolve(), args.production.resolve() if args.production else None)
    if args.output:
        args.output.write_text(json.dumps(report, sort_keys=True, indent=2) + "\n")
    print(f"PASS production convergence: {report['runtime_source_files']} runtime files, "
          f"{len(report['files'])} identical frozen files; only documented input packaging differs")


if __name__ == "__main__":
    main()
