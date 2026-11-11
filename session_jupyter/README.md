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

`show` uploads the session to the viewer's storage (Cloudflare R2, bucket `session-viewer-data`) as
the scene `my_net` and returns a frame with
<https://petrasvestartas.github.io/session/?scene=view_my_net.yaml>. Run the cell again and the open
viewer reloads.

| Call | What it does |
| --- | --- |
| `show(session, name)` | upload, then show the viewer in the notebook |
| `show(session, name, view="top", height=700)` | choose the opening camera (`top`, `front`, `iso`, …) and the frame height |
| `show(session, name, upload=False)` | only show the scene already published under that name |
| `publish(session, name)` | upload and return the viewer address |
| `url(name)` | the viewer address of a published scene |

## Install

Inside the `session` monorepo, beside `session_py`:

```bash
uv pip install -e session_jupyter
```

## Upload rights

Uploading needs the `[r2]` profile in `~/.aws/credentials`, the same one `bash/view_put.sh` uses.
Without it `show` raises a clear error; `upload=False` still shows what is published.

A scene name is a public address and the storage keeps no history of the scene file: publishing a
name again replaces what that name showed. The geometry itself is stored under its content hash and
is never overwritten.

## Settings

`SESSION_VIEWER`, `R2_BUCKET`, `R2_ACCOUNT`, `R2_ENDPOINT`, `R2_PUBLIC`, `R2_PROFILE` and
`R2_NOTIFY` environment variables replace the defaults; they match `bash/lib/view.sh`.
