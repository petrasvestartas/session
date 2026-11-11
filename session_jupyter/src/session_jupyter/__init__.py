"""Show a session_py Session in a Jupyter notebook with the session viewer.

    from session_jupyter import show

    show(session, "my_scene")

show() uploads the session to the viewer's storage and returns a frame with the viewer page.
"""

from __future__ import annotations

import gzip
import hashlib
import hmac
import json
import os
import re
import urllib.error
import urllib.request
from configparser import ConfigParser
from datetime import datetime
from datetime import timezone
from pathlib import Path

from IPython.display import IFrame
from session_py import Session

# The viewer page and its storage. Every value can be replaced by an environment variable of the same name.
SESSION_VIEWER = os.environ.get("SESSION_VIEWER", "https://petrasvestartas.github.io/session/")
R2_BUCKET = os.environ.get("R2_BUCKET", "session-viewer-data")
R2_ACCOUNT = os.environ.get("R2_ACCOUNT", "0520459c6817bd96c1e25fcb49461c4e")
R2_ENDPOINT = os.environ.get("R2_ENDPOINT", f"https://{R2_ACCOUNT}.r2.cloudflarestorage.com")
R2_PUBLIC = os.environ.get("R2_PUBLIC", "https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev")
R2_PROFILE = os.environ.get("R2_PROFILE", "r2")
R2_NOTIFY = os.environ.get("R2_NOTIFY", "https://ntfy.sh/wood-live-84eaac4a04729911")

VIEWS = ("top", "bottom", "front", "back", "left", "right", "iso", "isoback")


def url(name: str) -> str:
    """The viewer address of the published scene `name`."""
    return f"{SESSION_VIEWER.rstrip('/')}/?scene=view_{name}.yaml"


def publish(session: Session, name: str, view: str = "iso", grid: bool = False) -> str:
    """Upload the session as the scene `name` and return its viewer address.

    The geometry goes to a key named by its content, so it is never overwritten; only the small
    scene file view_<name>.yaml is replaced. An open viewer page reloads by itself.
    Needs the [r2] profile in ~/.aws/credentials.
    """
    if not re.fullmatch(r"[A-Za-z0-9_-]+", name):
        raise ValueError("A scene name may contain letters, digits, underscore and hyphen")
    if view not in VIEWS:
        raise ValueError(f"view must be one of {', '.join(VIEWS)}")

    geometry = session.pb_dumps()
    payload = gzip.compress(geometry, compresslevel=6, mtime=0)
    revision = f"pb/revisions/{hashlib.sha256(payload).hexdigest()}.pb"

    if not _is_published(revision):
        _put(revision, payload, "application/octet-stream", "public, max-age=31536000, immutable", "gzip")

    scene = {
        "name": name,
        "live": True,
        "view": view,
        "grid": grid,
        "items": [
            {
                "file": revision,
                "name": name,
                "at": [0, 0, 0],
                "encoding": "gzip",
                "size": len(geometry),
            }
        ],
    }
    text = json.dumps(scene, indent=2) + "\n"
    _put(f"scenes/view_{name}.yaml", text.encode(), "application/yaml", "no-cache", "")
    _notify(f"view_{name}.yaml")

    return url(name)


def show(
    session: Session,
    name: str,
    view: str = "iso",
    grid: bool = False,
    height: int = 600,
    upload: bool = True,
) -> IFrame:
    """Publish the session as the scene `name` and return the viewer in a notebook frame.

    upload=False only shows the scene that is already published under that name.
    """
    address = publish(session, name, view, grid) if upload else url(name)

    return IFrame(src=address, width="100%", height=height)


def _credentials() -> tuple[str, str]:
    """The key id and secret of the R2 profile in ~/.aws/credentials."""
    parser = ConfigParser()
    parser.read(Path.home() / ".aws" / "credentials")

    if not parser.has_section(R2_PROFILE):
        raise PermissionError(
            f"No [{R2_PROFILE}] profile in ~/.aws/credentials: the scene cannot be uploaded. "
            "Use show(session, name, upload=False) to look at the scene that is already published."
        )

    return parser[R2_PROFILE]["aws_access_key_id"], parser[R2_PROFILE]["aws_secret_access_key"]


def _is_published(key: str) -> bool:
    """Whether the public address already serves this key."""
    request = urllib.request.Request(f"{R2_PUBLIC}/{key}", method="HEAD", headers={"User-Agent": "curl/8"})

    try:
        with urllib.request.urlopen(request, timeout=30) as reply:
            return reply.status == 200
    except urllib.error.HTTPError:
        return False


def _put(key: str, data: bytes, content_type: str, cache: str, encoding: str) -> None:
    """Upload bytes to one key, signed the way S3 storage asks (AWS Signature Version 4)."""
    key_id, secret = _credentials()
    host = R2_ENDPOINT.split("://", 1)[1]
    path = f"/{R2_BUCKET}/{key}"
    now = datetime.now(timezone.utc)
    stamp = now.strftime("%Y%m%dT%H%M%SZ")
    day = now.strftime("%Y%m%d")
    digest = hashlib.sha256(data).hexdigest()

    signed = "host;x-amz-content-sha256;x-amz-date"
    request_text = "\n".join(
        [
            "PUT",
            path,
            "",
            f"host:{host}",
            f"x-amz-content-sha256:{digest}",
            f"x-amz-date:{stamp}",
            "",
            signed,
            digest,
        ]
    )
    scope = f"{day}/auto/s3/aws4_request"
    to_sign = "\n".join(
        ["AWS4-HMAC-SHA256", stamp, scope, hashlib.sha256(request_text.encode()).hexdigest()]
    )

    key_bytes = ("AWS4" + secret).encode()
    for part in (day, "auto", "s3", "aws4_request"):
        key_bytes = hmac.new(key_bytes, part.encode(), hashlib.sha256).digest()
    signature = hmac.new(key_bytes, to_sign.encode(), hashlib.sha256).hexdigest()

    headers = {
        "Authorization": f"AWS4-HMAC-SHA256 Credential={key_id}/{scope}, SignedHeaders={signed}, Signature={signature}",
        "x-amz-content-sha256": digest,
        "x-amz-date": stamp,
        "Content-Type": content_type,
        "Cache-Control": cache,
    }
    if encoding:
        headers["Content-Encoding"] = encoding

    request = urllib.request.Request(f"{R2_ENDPOINT}{path}", data=data, method="PUT", headers=headers)

    with urllib.request.urlopen(request, timeout=180) as reply:
        if reply.status != 200:
            raise OSError(f"Upload of {key} answered HTTP {reply.status}")


def _notify(message: str) -> None:
    """Tell open viewer pages to reload now. Best effort: they also reload on their own."""
    try:
        request = urllib.request.Request(R2_NOTIFY, data=message.encode(), method="POST")
        urllib.request.urlopen(request, timeout=5).close()
    except OSError:
        pass
