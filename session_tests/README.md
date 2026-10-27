# session_tests

The one documentation site, Vue 3 + Vite: the viewer course, the kernel API (the minitest page)
and install notes. Production lives at `/session/docs/`, next to the viewer at `/session/`.

```bash
npm install                          # once
npm run dev                          # localhost:8769/session/  (course pages reload on save)
npm run build                        # dist/ for /session/docs/  (DOCS_BASE=/other/ to move it)
```

On Windows, run course builds and browser captures through the local resource guard:

```powershell
$env:DOCS_BASE = '/docs/'
$env:DOCS_STRICT_LINKS = '1'
$env:NODE_OPTIONS = '--max-old-space-size=1280'
.\scripts\run-guarded.cmd -ManifestPath ..\session_viewer\target\docs-build.json node node_modules/vite/bin/vite.js build
```

The guard admits one job, caps its combined committed memory at 2 GiB by default, restricts it to two
available processors and terminates its children on timeout or exit. The default timeout is
600 seconds; `-TimeoutSeconds` overrides it. Redirect the runner's output to retain a log.
Use `-MemoryLimitMiB 4096` for a 4 GiB job, or `6144` for a 6 GiB software-browser job, when authorized. The course runner accepts
the same limit through `$env:COURSE_MEMORY_MIB = '4096'` in PowerShell.
Its JSON manifest records the limits, elapsed time, peak commit and exit status. Windows
has no separate per-job swap quota, so this guard does not claim Linux's zero-swap guarantee.
The command launcher uses a process-local execution-policy override, without changing machine settings.

The guard waits for the command itself, then removes lingering descendants before releasing
the job lock. This includes MSVC background helpers. Run `node --test scripts/run-guarded.test.cjs`
to check exit-status preservation, child cleanup and timeout handling.

For isolated Windows software-WebGPU checks, use the pinned course tools and installed Chrome:

```powershell
$env:NODE_PATH = (Resolve-Path ../session_viewer/target/course-tools/node_modules).Path
$env:VIEWER_HEADLESS = '1'
$warp = powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts/warp-adapter.ps1 | ConvertFrom-Json
$env:VIEWER_CHROME_ARGS = ConvertTo-Json -Compress -InputObject @('--enable-unsafe-webgpu', '--ignore-gpu-blocklist', '--use-gl=angle', '--use-angle=d3d11', "--use-adapter-luid=$($warp.luid)")
.\scripts\run-guarded.cmd -TimeoutSeconds 90 -ManifestPath ../session_viewer/target/software-gpu.resources.json node scripts/check-software-webgpu.cjs http://127.0.0.1:8788/docs/ ../session_viewer/target/software-gpu.json
```

The probe requires Playwright 1.58.2, verifies the actual software adapter, checks exact texture
readback and checks a presented canvas screenshot. Windows selects the verified Microsoft WARP
adapter by its current LUID; obtain it again after reboot. The helper rejects other adapters.
Linux uses SwiftShader. The course capture script accepts `VIEWER_HEADLESS` and `VIEWER_CHROME_ARGS`.
Windows `d3d11-warp` marks the entire GL path as software and forces Dawn onto SwiftShader;
regular `d3d11` plus WARP's LUID keeps both rendering APIs on the intended CPU device.

`check-software-webgpu.cjs` also accepts a final `warp` or `swiftshader` argument and an explicit
`VIEWER_BROWSER_EXECUTABLE`. For a task-local pinned browser, install Chromium with the existing
Playwright 1.58.2 CLI and `PLAYWRIGHT_BROWSERS_PATH` pointing inside `session_viewer/target`.
The Windows pinned headless shell requires its compatible `dxcompiler.dll` and `dxil.dll` beside
that executable; this does not require installing DLLs globally. The probe records the actual browser,
adapter and launch flags and rejects a backend that does not match the requested identity.

The full Windows WARP shader diagnostic uses
`--enable-dawn-features=d3d_skip_shader_optimizations`; it leaves the application shaders unchanged.
`scripts/course/check-reload-grid.cjs` enables that setting only when
`VIEWER_DAWN_SKIP_OPTIMIZATIONS=1`. Keep the recorded compiler setting the same for both sides of a
performance comparison. Software-adapter timings are not hardware frame-rate measurements.

The final checkpoint's native GPU helpers retain their default readback deadlines. A guarded job can
set `VIEWER_GPU_WAIT_SECONDS=60` for measured deferred WARP compilation on first use; the accepted
range is 1–120 seconds. The outer job timeout still applies, and all pixel, selection and source
assertions remain unchanged.

The viewer's `trunk serve` / `trunk build` runs `scripts/build-viewer-docs.sh` first: it builds this
site with base `/docs/` into `session_viewer/target/docs/vue` (only when a source changed), and
Trunk copies it to `dist/docs`, so the viewer's corner link opens it locally too.

Routes (hash router):

- `#/` home: Viewer course, Kernel API, Install.
- `#/course` the lesson chain in `session_viewer/docs/lessons/SERIES.txt` order, plus the reference
  pages from the `mkdocs.yml` nav; `#/course/12-picking#step-1` is one page and heading.
- `#/tests?suite=color_test&test=Constructor` one kernel class, scrolled to one test.
- `#/install`.

`plugins/course.ts` turns `session_viewer/docs/*.md` into lazy page chunks at build time. It resolves
every `--8<--` include like pymdownx.snippets (named sections `path:name` and line ranges
`path:10:20`, against `session_viewer/docs/` then `session/`), drops marker lines, and fails the
build naming the page and include when a file or section is missing. Code is highlighted with the
greyscale theme in `src/greyTheme.ts`; images and files the pages link to are copied under
`course/`. The search index (MiniSearch) covers course headings and prose plus kernel class and
test names from `session_cpp/src/*_test.cpp`; it loads only when the search box opens (`/`).

Check a build in headless Chrome (pages, links, anchors, assets, search, deep link, phone width,
console errors, first paint):

```bash
# serve dist so it answers at /session/docs/, then
NODE_PATH=<dir with playwright> node scripts/check-site.mjs http://localhost:8795/session/docs/ [shots dir]
```
