# session_jupyter

Show [session](https://github.com/petrasvestartas/session) geometry in a Jupyter notebook with the
session viewer. One function, no paths:

```python
from session_py import Line, Session
from session_jupyter import show

session = Session("net")
session.add_line(Line(0, 0, 0, 1, 0, 0))

show(session, "my_net")
```

With upload rights, `show` uploads the session to the viewer's storage (Cloudflare R2, bucket `session-viewer-data`) as
the scene `my_net` and returns a frame with
<https://petrasvestartas.github.io/session/?scene=view_my_net.yaml>. Run the cell again and the open
viewer reloads.

| Call | What it does |
| --- | --- |
| `show(session, name)` | with upload rights: upload as `name`; without: send to the upload service (one day) |
| `show(session, name, upload=True)` | always upload, then show the viewer in the notebook |
| `show(session, name, view="top", height=700)` | choose the opening camera (`top`, `front`, `iso`, …) and the frame height |
| `show(session, name, upload=False)` | only show the scene already published under that name |
| `publish(session, name)` | upload and return the viewer address |
| `share(session)` | send it to the upload service, no keys, kept one day; return the viewer address |
| `inline_url(session)` | the viewer address that carries the session itself, nothing uploaded |
| `url(name)` | the viewer address of a published scene |

## Install

```bash
pip install session-jupyter
```

Every change to `session_jupyter/` on `main` is released by `.github/workflows/session-jupyter-publish.yml`.
For development inside the `session` monorepo, beside `session_py`:

```bash
uv pip install -e session_jupyter
```

## Without keys: the upload service

Students have no keys. `show(session, name)` then sends the scene to the upload service in
[`worker/`](worker/), a Cloudflare Worker anyone may call, also from a browser notebook on a Chromebook:

- each scene is stored under the hash of its content (`tmp/<hash>.pb` and `tmp/<hash>.yaml` in the bucket
  `session-viewer-data`), so the same scene is stored once, however often a cell runs. `canonical()` gives the
  objects fixed ids first, because every run makes new random ones;
- the bucket deletes everything under `tmp/` one day after it was last sent;
- a body may be at most 10 MB.

`SESSION_UPLOAD` is the service address (default `https://session-upload.petrasvestartas.workers.dev/upload`). When the service cannot be reached, the geometry
travels inside the frame's address instead (`inline_url`).

Put the service online once (from `worker/`):

```bash
wrangler login
wrangler deploy
wrangler r2 bucket lifecycle add session-viewer-data tmp-one-day tmp/ --expire-days 1
```

## Upload rights

Uploading needs the `[r2]` profile in `~/.aws/credentials`, the same one `bash/view_put.sh` uses.
Without it, and always in a browser notebook (JupyterLite), `show` uploads nothing: the geometry
travels inside the frame's address (`#session=...`) to the viewer at `INLINE_VIEWER`, which draws it in
the page. Every student sees their own result, with no paths and no keys. `show(..., upload=True)`
without the profile raises a clear error; `upload=False` still shows what is published.

A scene name is a public address and the storage keeps no history of the scene file: publishing a
name again replaces what that name showed. The geometry itself is stored under its content hash and
is never overwritten.

## Settings

`SESSION_VIEWER`, `R2_BUCKET`, `R2_ACCOUNT`, `R2_ENDPOINT`, `R2_PUBLIC`, `R2_PROFILE` and
`R2_NOTIFY`, `SESSION_UPLOAD` and `INLINE_VIEWER` environment variables replace the defaults; they match `bash/lib/view.sh`.
