"""Persistent stdio JSONL client for `session-viewer-cli serve --stdio`."""

from __future__ import annotations

import json
import os
import pathlib
import subprocess
from typing import Any


class ViewerOpError(RuntimeError):
    """Structured error returned by `viewer_core::op::execute`."""

    def __init__(self, code: str, message: str) -> None:
        super().__init__(f"[{code}] {message}")
        self.code = code
        self.op_message = message


def find_cli_binary() -> pathlib.Path:
    """Locate `session-viewer-cli` via `SESSION_VIEWER_CLI` or workspace Cargo target directories."""
    env = os.environ.get("SESSION_VIEWER_CLI")
    if env:
        p = pathlib.Path(env)
        if p.is_file():
            return p
        raise FileNotFoundError(f"SESSION_VIEWER_CLI={env!r} does not exist")

    session_root = pathlib.Path(__file__).resolve().parents[3]
    viewer_dir = session_root / "session_viewer"
    candidates = [
        viewer_dir / "target" / "x86_64-unknown-linux-gnu" / "release" / "session-viewer-cli",
        viewer_dir / "target" / "x86_64-unknown-linux-gnu" / "debug" / "session-viewer-cli",
        viewer_dir / "target" / "release" / "session-viewer-cli",
        viewer_dir / "target" / "debug" / "session-viewer-cli",
    ]
    for candidate in candidates:
        if candidate.is_file():
            return candidate
    raise FileNotFoundError(
        "session-viewer-cli binary not found; build it with "
        "`cargo build --target x86_64-unknown-linux-gnu --bin session-viewer-cli`"
    )


class ViewerCliClient:
    """Spawns `session-viewer-cli serve --stdio` and exchanges typed `Op` JSON lines."""

    def __init__(
        self,
        binary: str | os.PathLike[str] | None = None,
        open_path: str | os.PathLike[str] | None = None,
    ) -> None:
        self.binary = pathlib.Path(binary) if binary else find_cli_binary()
        cmd = [str(self.binary), "serve", "--stdio"]
        if open_path is not None:
            cmd.extend(["--open", str(open_path)])
        self._proc = subprocess.Popen(
            cmd,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
        )
        self._seq = 0

    def execute(self, op: str, **fields: Any) -> dict[str, Any]:
        """Send one `Request` line and return its `Reply` dictionary (or raise `ViewerOpError`)."""
        if self._proc.poll() is not None or self._proc.stdin is None or self._proc.stdout is None:
            stderr = self._proc.stderr.read() if self._proc.stderr else ""
            raise RuntimeError(f"session-viewer-cli exited unexpectedly: {stderr}")

        self._seq += 1
        req_id = f"mcp-{self._seq}"
        payload = {"id": req_id, "op": op}
        for k, v in fields.items():
            if v is not None:
                payload[k] = v

        self._proc.stdin.write(json.dumps(payload, separators=(",", ":")) + "\n")
        self._proc.stdin.flush()

        raw = self._proc.stdout.readline()
        if not raw:
            stderr = self._proc.stderr.read() if self._proc.stderr else ""
            raise RuntimeError(f"EOF from session-viewer-cli: {stderr}")

        resp = json.loads(raw)
        if not resp.get("ok", False):
            err = resp.get("error") or {}
            raise ViewerOpError(err.get("code", "unknown"), err.get("message", "operation failed"))
        return resp["reply"]

    def close(self) -> None:
        if self._proc.poll() is None:
            if self._proc.stdin:
                try:
                    self._proc.stdin.close()
                except OSError:
                    pass
            try:
                self._proc.wait(timeout=3.0)
            except subprocess.TimeoutExpired:
                self._proc.kill()
                self._proc.wait(timeout=3.0)

    def __enter__(self) -> ViewerCliClient:
        return self

    def __exit__(self, *_exc: object) -> None:
        self.close()
