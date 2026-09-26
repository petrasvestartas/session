# session_tests

The one documentation site, Vue 3 + Vite: the viewer course, the kernel API (the minitest page)
and install notes. Production lives at `/session/docs/`, next to the viewer at `/session/`.

```bash
npm install                          # once
npm run dev                          # localhost:8769/session/  (course pages reload on save)
npm run build                        # dist/ for /session/docs/  (DOCS_BASE=/other/ to move it)
```

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
