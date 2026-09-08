#!/usr/bin/env python3
"""Remove the standalone point producer in a scratch build; verify other CAD families.

No production source is edited. --target-dir may reuse a Cargo artifact cache; otherwise
build artifacts are isolated beneath the scratch directory. Requires Trunk and Playwright
through NODE_PATH exactly like tests/interaction.cjs.
"""
import argparse
import functools
import hashlib
import http.server
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import threading


def replace_once(source, old, new):
    """Fail explicitly when the source registration contract has changed."""
    if source.count(old) != 1:
        raise RuntimeError(f"module contract changed: expected one occurrence of {old!r}")
    return source.replace(old, new, 1)


def run(command, cwd, env, log):
    """Run one checked build or browser command with a retained diagnostic log."""
    print("Running", " ".join(map(str, command)), flush=True)
    with log.open("w") as output:
        subprocess.run(command, cwd=cwd, env=env, stdout=output, stderr=subprocess.STDOUT, check=True)


class QuietHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, *_args):
        """Keep fixture HTTP requests out of the test result stream."""
        pass


def main():
    """Prepare an isolated removal build and verify the remaining source families."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, help="new scratch directory; must not already exist")
    parser.add_argument("--target-dir", type=Path, help="optional shared Cargo artifact cache")
    parser.add_argument("--prepare-only", action="store_true", help="prepare/build now; run browser separately after other GPU tests")
    args = parser.parse_args()
    repository = Path(__file__).resolve().parents[1]
    scratch = args.output.resolve() if args.output else Path(tempfile.mkdtemp(prefix="viewer-modularity-"))
    if args.output:
        scratch.mkdir(parents=True, exist_ok=False)
    viewer = scratch / "session_viewer"
    viewer.mkdir()
    (scratch / "session_rust").symlink_to(repository.parent / "session_rust", target_is_directory=True)
    for filename in ("Cargo.toml", "Cargo.lock", "Trunk.toml", "index.html"):
        shutil.copy2(repository / filename, viewer / filename)
    for directory in ("src", ".cargo"):
        shutil.copytree(repository / directory, viewer / directory)
    (viewer / "examples").mkdir()
    shutil.copy2(repository / "examples/interaction_fixture.rs", viewer / "examples/interaction_fixture.rs")
    (viewer / "assets/pb").mkdir(parents=True)
    shutil.copytree(repository / "assets/text", viewer / "assets/text")
    shutil.copy2(repository / "assets/text-quality.html", viewer / "assets/text-quality.html")
    (viewer / "assets/view_local.yaml").write_text('name: Removed standalone points\nitems:\n  - file: "pb/interaction.pb"\n')

    original_point = repository / "src/app/walk/points.rs"
    original_hash = hashlib.sha256(original_point.read_bytes()).hexdigest()
    (viewer / "src/app/walk/points.rs").unlink()
    dispatch = viewer / "src/app/walk/mod.rs"
    text = dispatch.read_text()
    text = replace_once(text, "use points::walk_point;\n", "")
    text = replace_once(text, "pub mod points;\n", "")
    text = replace_once(text, "        Geometry::Point(p) => walk_point(w.glyph, p, cx.row),",
                        "        // Standalone points are deliberately unavailable in this scratch configuration.\n"
                        "        Geometry::Point(_) => Row::thin(Aabb::empty()),")
    text = replace_once(text, "pub fn is_drawable(geom: &Geometry) -> bool {\n    match geom {",
                        "pub fn is_drawable(geom: &Geometry) -> bool {\n    match geom {\n"
                        "        // Skip before allocating an object row or visiting the removed producer.\n"
                        "        Geometry::Point(_) => false,")
    dispatch.write_text(text)
    generator = viewer / "examples/interaction_fixture.rs"
    generator.write_text(replace_once(generator.read_text(), "    scene.pb_dump(&output);",
                                     "    scene.add_point(Point::new(0.0, 0.0, 0.0), None);\n    scene.pb_dump(&output);"))
    fixture = viewer / "assets/pb/interaction.pb"
    env = dict(os.environ, REGEN_PROTO="0", NO_COLOR="true")
    env["CARGO_TARGET_DIR"] = str(args.target_dir.resolve() if args.target_dir else scratch / "target")
    run(["cargo", "run", "--locked", "--target", "x86_64-unknown-linux-gnu", "--example", "interaction_fixture", "--", str(fixture)], viewer, env, scratch / "fixture-build.log")
    run(["trunk", "build", "--release"], viewer, env, scratch / "wasm-build.log")
    assert not (viewer / "src/app/walk/points.rs").exists()
    assert hashlib.sha256(original_point.read_bytes()).hexdigest() == original_hash, "production point producer must remain untouched"
    metadata = {"removed": "src/app/walk/points.rs", "source_point_module_sha256": original_hash,
                "serialized_objects": 8, "expected_drawable_objects": 7, "fixture": str(fixture),
                "policy": "skip standalone Geometry::Point before row allocation; retain shared marker/cloud GPU lanes"}
    (scratch / "removal.json").write_text(json.dumps(metadata, indent=2))
    if args.prepare_only:
        print(f"PASS scratch WASM compilation; browser pending. Scratch: {scratch}")
        return
    handler = functools.partial(QuietHandler, directory=str(viewer / "dist"))
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    env.update(VIEWER_URL=f"http://127.0.0.1:{server.server_port}/", VIEWER_INTERACTION_FIXTURE=str(fixture), VIEWER_TEST_OUTPUT=str(scratch / "browser"))
    try:
        run(["node", str(repository / "tests/interaction.cjs")], viewer, env, scratch / "browser.log")
    finally:
        server.shutdown()
        server.server_close()
    for dpr in (1, 2):
        cases = json.loads((scratch / f"browser/interaction-dpr-{dpr}.json").read_text())
        assert len(cases) == 7
        assert all(case["state"]["objects"] == 7 for case in cases), "standalone point probe must not allocate a row"
    metadata["browser_verified_dpr"] = [1, 2]
    (scratch / "removal.json").write_text(json.dumps(metadata, indent=2))
    print(f"PASS standalone point producer removed; seven remaining source families render/select at DPR1/2. Scratch: {scratch}")


if __name__ == "__main__":
    main()
