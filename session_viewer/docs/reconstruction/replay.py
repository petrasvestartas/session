#!/usr/bin/env python3
"""Reconstruct exact tutorial checkpoints and optionally verify each in a real browser."""

import argparse
import functools
import hashlib
import http.server
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import threading


HERE = Path(__file__).resolve().parent


def digest(path):
    """Identify supplied source bytes independently of their modification time."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_json(path):
    """Read one checked-in reconstruction contract."""
    return json.loads(path.read_text())


def safe_path(root, relative):
    """Reject manifest paths escaping the requested reconstruction workspace."""
    path = root / relative
    if Path(relative).is_absolute() or ".." in Path(relative).parts:
        raise ValueError(f"unsafe workspace path: {relative}")
    return path


def run(command, cwd, env, log):
    """Run an exact argument vector and retain errors with the checkpoint evidence."""
    print("Running", " ".join(map(str, command)), flush=True)
    log.parent.mkdir(parents=True, exist_ok=True)
    with log.open("w") as output:
        subprocess.run(command, cwd=cwd, env=env, stdout=output,
                       stderr=subprocess.STDOUT, check=True)


def check_files(workspace, expected):
    """Fail on a missing or edited required source before advancing a checkpoint."""
    for relative, checksum in sorted(expected.items()):
        path = safe_path(workspace, relative)
        if not path.is_file() or digest(path) != checksum:
            raise ValueError(f"checkpoint source differs: {relative}")


def initialize(workspace):
    """Extract the pinned unmodified Rust kernel into a new, empty workspace."""
    if workspace.exists() and any(workspace.iterdir()):
        raise ValueError(f"output is not empty: {workspace}; choose a new path or --advance")
    workspace.mkdir(parents=True, exist_ok=True)
    metadata = read_json(HERE / "kernel-base.json")
    archive = HERE / metadata["archive"]
    if digest(archive) != metadata["sha256"]:
        raise ValueError("kernel base archive checksum failed")
    kernel = workspace / "session_rust"
    kernel.mkdir()
    with tarfile.open(archive) as stream:
        stream.extractall(kernel, filter="data")
    for source in metadata.get("parity_sources", []):
        archive = HERE / source["archive"]
        if digest(archive) != source["sha256"]:
            raise ValueError(f"parity base checksum failed: {source['directory']}")
        destination = safe_path(workspace, source["directory"])
        destination.mkdir()
        with tarfile.open(archive) as stream:
            stream.extractall(destination, filter="data")


def apply_step(workspace, step, env, evidence):
    """Check a complete patch, apply it, and copy explicitly inventoried binary assets."""
    patch = HERE / step["patch"]
    if digest(patch) != step["patch_sha256"]:
        raise ValueError(f"patch checksum failed: {step['id']}")
    run(["git", "apply", "--check", str(patch)], workspace, env,
        evidence / "patch-check.log")
    run(["git", "apply", str(patch)], workspace, env, evidence / "patch-apply.log")
    copy_assets(workspace, step)
    check_files(workspace, step["files"])
    (workspace / ".reconstruction-state.json").write_text(json.dumps({
        "checkpoint": step["id"], "files": step["files"],
    }, indent=2) + "\n")


def copy_assets(workspace, step):
    """Install the explicitly inventoried binary inputs for automatic or manual edits."""
    for asset in step.get("assets", []):
        source = (HERE / asset["source"]).resolve()
        if digest(source) != asset["sha256"]:
            raise ValueError(f"asset checksum failed: {asset['source']}")
        target = safe_path(workspace, asset["target"])
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source, target)


class QuietHandler(http.server.SimpleHTTPRequestHandler):
    """Serve a checkpoint without mixing resource requests into build diagnostics."""

    def log_message(self, *_args):
        """Keep request logs in the browser evidence instead of standard output."""
        pass


def verify_step(workspace, step, env, evidence):
    """Build the locked WASM application and verify its submitted browser frame."""
    viewer = workspace / "session_viewer"
    run(["cargo", "check", "--locked", "--lib"], viewer, env,
        evidence / "wasm-check.log")
    run(["trunk", "build", "--release"], viewer, env,
        evidence / "trunk-build.log")
    handler = functools.partial(QuietHandler, directory=str(viewer / "dist"))
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    browser_env = dict(env, VIEWER_URL=f"http://127.0.0.1:{server.server_port}/",
                       VIEWER_TEST_OUTPUT=str(evidence),
                       VIEWER_TUTORIAL_STEP=step["id"])
    try:
        run(["node", str(HERE / "smoke.cjs")], viewer, browser_env,
            evidence / "browser.log")
    finally:
        server.shutdown()
        server.server_close()
        thread.join()
    check_files(workspace, step["files"])


def selected_steps(series, through):
    """Select the ordered prefix ending at an existing numbered checkpoint."""
    available = [step["id"] for step in series["steps"]]
    target = through or available[-1]
    if target not in available:
        raise ValueError(f"unknown checkpoint {target}; available: {', '.join(available)}")
    return series["steps"][:available.index(target) + 1]


def main():
    """Replay a clean prefix, advance a verified prefix, or test all clean reconstructions."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", required=True, type=Path,
                        help="new reconstruction workspace; never the production checkout")
    parser.add_argument("--through", help="last numbered checkpoint, for example 04")
    parser.add_argument("--initialize-only", action="store_true",
                        help="extract pinned shared prerequisites before typing checkpoint 00")
    parser.add_argument("--copy-assets", action="store_true",
                        help="copy only the hash-checked binary inputs before adopting manual edits")
    parser.add_argument("--advance", action="store_true",
                        help="advance a previously reconstructed, unchanged workspace")
    parser.add_argument("--adopt", action="store_true",
                        help="verify manually completed --through sources and record that checkpoint")
    parser.add_argument("--verify", action="store_true",
                        help="locked WASM build and browser smoke after every applied step")
    parser.add_argument("--verify-clean", action="store_true",
                        help="reconstruct each prefix in its own new directory and browser-test it")
    parser.add_argument("--from-step", help="first prefix to test with --verify-clean; each tested prefix still starts from pinned bases")
    parser.add_argument("--target-dir", type=Path,
                        help="optional reusable Cargo cache outside the source snapshots")
    args = parser.parse_args()
    if args.from_step and not args.verify_clean:
        parser.error("--from-step requires --verify-clean")
    if sum((args.advance, args.verify_clean, args.adopt, args.initialize_only, args.copy_assets)) > 1:
        parser.error("choose one reconstruction mode")
    workspace = args.output.resolve()
    if args.initialize_only:
        initialize(workspace)
        print(f"Pinned prerequisites ready; type checkpoint 00 beneath {workspace}")
        return
    series = read_json(HERE / "series.json")
    steps = selected_steps(series, args.through)
    if args.from_step and args.from_step not in [step["id"] for step in steps]:
        parser.error("--from-step must name a checkpoint at or before --through")
    if args.copy_assets:
        if not (workspace / "session_viewer").is_dir():
            parser.error("--copy-assets requires an existing manual viewer workspace")
        for step in steps:
            copy_assets(workspace, step)
        print(f"Binary inputs copied through {steps[-1]['id']}; source edits remain untouched")
        return
    env = dict(os.environ, REGEN_PROTO="0", NO_COLOR="true", PYTHONDONTWRITEBYTECODE="1")
    if args.target_dir:
        env["CARGO_TARGET_DIR"] = str(args.target_dir.resolve())
    if args.adopt:
        final = steps[-1]
        check_files(workspace, final["files"])
        if args.verify:
            verify_step(workspace, final, env, workspace / "evidence" / final["id"])
        (workspace / ".reconstruction-state.json").write_text(json.dumps({
            "checkpoint": final["id"], "files": final["files"],
        }, indent=2) + "\n")
        print(f"Verified manually reconstructed checkpoint {final['id']}")
        return
    if args.verify_clean:
        if workspace.exists() and any(workspace.iterdir()):
            raise ValueError(f"clean-verification output must be new: {workspace}")
        workspace.mkdir(parents=True, exist_ok=True)
        for index, final in enumerate(steps):
            if args.from_step and final["id"] < args.from_step:
                continue
            checkpoint = workspace / final["id"]
            initialize(checkpoint)
            for step in steps[:index + 1]:
                apply_step(checkpoint, step, env, checkpoint / "evidence" / step["id"])
            verify_step(checkpoint, final, env, checkpoint / "evidence" / final["id"])
        tested = sum(not args.from_step or step["id"] >= args.from_step for step in steps)
        print(f"PASS {tested} clean checkpoint reconstructions and browser checks")
        return
    start = 0
    if args.advance:
        previous = read_json(workspace / ".reconstruction-state.json")
        check_files(workspace, previous["files"])
        ids = [step["id"] for step in steps]
        if previous["checkpoint"] not in ids:
            raise ValueError("requested checkpoint precedes the existing workspace")
        start = ids.index(previous["checkpoint"]) + 1
    else:
        initialize(workspace)
    for step in steps[start:]:
        evidence = workspace / "evidence" / step["id"]
        apply_step(workspace, step, env, evidence)
        if args.verify:
            verify_step(workspace, step, env, evidence)
    print(f"Reconstructed checkpoint {steps[-1]['id']}: {workspace / 'session_viewer'}")


if __name__ == "__main__":
    main()
