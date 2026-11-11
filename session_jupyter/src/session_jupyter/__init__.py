"""Show a session_py Session in a Jupyter notebook with the session viewer.

    from session_jupyter import show

    show(session, "my_scene")

show() returns a frame with the viewer page:

    with upload rights (the [r2] profile in ~/.aws/credentials)   upload as the scene `name`, a lasting address
    without them, also in a browser notebook (JupyterLite)       send it to the upload service: no keys needed,
                                                                 stored once by content, deleted after one day
    if the service cannot be reached                             the geometry travels inside the frame's address
"""

from __future__ import annotations

import base64
import gzip
import hashlib
import hmac
import json
import os
import re
import sys
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
# The upload service anyone may use (session_jupyter/worker): files live one day, each content once.
UPLOAD = os.environ.get("SESSION_UPLOAD", "https://session-upload.petrasvestartas.workers.dev/upload")
# The viewer that reads a scene from its own address (#session=...), so nothing has to be uploaded.
INLINE_VIEWER = os.environ.get("INLINE_VIEWER", "https://brg-teaching.github.io/Teaching/CAS/website/session-viewer/")

VIEWS = ("top", "bottom", "front", "back", "left", "right", "iso", "isoback")


def url(name: str) -> str:
    """The viewer address of the published scene `name`."""
    return f"{SESSION_VIEWER.rstrip('/')}/?scene=view_{name}.yaml"


def inline_url(session: Session) -> str:
    """The viewer address that carries the session itself; it is drawn in the page, nothing is uploaded."""
    payload = base64.b64encode(session.pb_dumps()).decode("ascii")
    return f"{INLINE_VIEWER.rstrip('/')}/?scene=notebook.yaml&data=off&live=off#session={payload}"


def share(session: Session, view: str = "iso", grid: bool = False) -> str:
    """Send the session to the upload service and return its viewer address. No keys; kept for one day."""
    if not UPLOAD:
        raise OSError("No upload service is set (SESSION_UPLOAD)")
    if view not in VIEWS:
        raise ValueError(f"view must be one of {', '.join(VIEWS)}")
    body = gzip.compress(canonical(session.pb_dumps()), compresslevel=6, mtime=0)
    address = f"{UPLOAD}?view={view}&grid={int(grid)}"
    return json.loads(_post(address, body))["url"]


_GUID = re.compile(rb"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")


def canonical(data: bytes) -> bytes:
    """The session bytes with its random object ids replaced by ids numbered in order of appearance.

    Every run of a notebook makes new random ids, so the same geometry gave different bytes, and a stored
    copy for every run. Each id keeps its length, so the encoding stays valid, and equal ids stay equal,
    so the links between objects are kept.
    """
    numbers: dict[bytes, bytes] = {}

    def number(match: re.Match) -> bytes:
        found = match.group(0)
        if found not in numbers:
            numbers[found] = f"{len(numbers):08x}-0000-4000-8000-000000000000".encode()
        return numbers[found]

    numbered = _GUID.sub(number, data)
    # Some parts are written in the order of their ids: written again, that order follows the new ids.
    return _GUID.sub(number, Session.pb_loads(numbered).pb_dumps())


def _post(address: str, body: bytes) -> str:
    """POST bytes and return the reply text; in a browser notebook through the browser itself."""
    if sys.platform == "emscripten":
        from js import Uint8Array, XMLHttpRequest  # the kernel runs in a web worker, where a blocking request is allowed

        request = XMLHttpRequest.new()
        request.open("POST", address, False)
        request.send(Uint8Array.new(body))
        if request.status != 200:
            raise OSError(f"Upload answered HTTP {request.status}: {request.responseText}")
        return request.responseText

    request = urllib.request.Request(address, data=body, method="POST", headers={"Content-Type": "application/octet-stream", "User-Agent": "session_jupyter"})  # Cloudflare refuses Python's default agent
    with urllib.request.urlopen(request, timeout=60) as reply:
        return reply.read().decode()


def can_upload() -> bool:
    """Whether this notebook may upload: not in a browser notebook, and the [r2] profile is present."""
    if sys.platform == "emscripten":
        return False
    try:
        _credentials()
    except (PermissionError, KeyError):
        return False
    return True


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
    upload: bool | None = None,
) -> IFrame:
    """Show the session in a notebook frame.

    upload=None (default): upload as the scene `name` with upload rights (can_upload()); else send it to the
    upload service (share()); if that cannot be reached, draw it from the frame's address (inline_url()).
    upload=True always uploads as `name`; upload=False only shows the scene already published under `name`.
    """
    if upload is None:
        if can_upload():
            address = publish(session, name, view, grid)
        else:
            try:
                address = share(session, view, grid)
            except (OSError, ValueError, KeyError):
                address = inline_url(session)
    else:
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
