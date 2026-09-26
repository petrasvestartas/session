// Vite plugin: the viewer course (session_viewer/docs/*.md) as lazy Vue routes.
// Resolves every --8<-- include the way pymdownx.snippets does (named sections and line ranges,
// against the MkDocs base paths docs/ and ..), strips marker lines, fails the build naming the page
// and include when a file or section is missing, renders with marked + a greyscale Shiki theme,
// copies the images and files the pages link to, and builds the search index.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import type { Plugin, ViteDevServer } from 'vite';
import { Marked, type Tokens } from 'marked';
import { createHighlighter, type Highlighter } from 'shiki';
import MiniSearch from 'minisearch';
import { greyTheme, tokenClass, THEME_NAME } from '../src/greyTheme';
import { SEARCH_OPTIONS } from '../src/searchOptions';
import { kernelClasses } from './kernel';

const HERE = path.dirname(fileURLToPath(import.meta.url));
const SESSION = path.resolve(HERE, '../..');
const VIEWER = path.join(SESSION, 'session_viewer');
const DOCS = path.join(VIEWER, 'docs');
const BASES = [DOCS, SESSION];
const GITHUB = 'https://github.com/petrasvestartas/session/blob/main/';
const SKIP_DIRS = new Set(['lessons', 'hooks', 'overrides', 'node_modules', '__pycache__', 'diagrams', 'reconstruction']);

const LANGS = ['rust', 'wgsl', 'html', 'shellscript', 'toml', 'yaml', 'markdown', 'proto', 'json', 'cpp', 'python', 'javascript'];
const ALIAS: Record<string, string> = { sh: 'shellscript', bash: 'shellscript', shell: 'shellscript', console: 'shellscript', rs: 'rust', yml: 'yaml', md: 'markdown', js: 'javascript', py: 'python', protobuf: 'proto' };

const V_COURSE = 'virtual:course';
const V_PAGE = 'virtual:course-page/';
const V_SEARCH = 'virtual:search-index';
const V_KERNEL = 'virtual:kernel';

interface Page {
  slug: string;
  file: string;
  title: string;
  html: string;
  toc: { level: number; id: string; text: string }[];
  ids: Set<string>;
  search: boolean;
}

interface Group {
  title: string;
  slugs: string[];
}

interface Link {
  from: string;
  href: string;
  slug: string;
  frag: string;
}

const esc = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
const unesc = (s: string) => s.replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&quot;/g, '"').replace(/&#39;/g, "'").replace(/&amp;/g, '&');
const posix = (p: string) => p.split(path.sep).join('/');
const stripTags = (s: string) => unesc(s.replace(/<[^>]+>/g, ' ')).replace(/\s+/g, ' ').trim();

/** Python-Markdown's toc slugify plus its _1, _2 suffixes, so MkDocs anchors keep working. */
function slugify(text: string, ids: Set<string>): string {
  let id = text.normalize('NFKD').replace(/[^\x00-\x7f]/g, '');
  id = id.replace(/[^\w\s-]/g, '').trim().toLowerCase().replace(/[-\s]+/g, '-');
  let unique = id;
  for (let n = 1; ids.has(unique); n++) unique = `${id}_${n}`;
  ids.add(unique);
  return unique;
}

function pageSlug(file: string): string {
  const rel = file.startsWith(DOCS + path.sep) ? path.relative(DOCS, file) : path.basename(file);
  return posix(rel).replace(/\.md$/, '').toLowerCase();
}

function listPages(): string[] {
  const out: string[] = [];
  const walk = (dir: string, depth: number) => {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, e.name);
      if (e.isDirectory() && depth === 0 && !SKIP_DIRS.has(e.name)) walk(p, 1);
      else if (e.isFile() && e.name.endsWith('.md')) out.push(p);
    }
  };
  walk(DOCS, 0);
  const arch = path.join(VIEWER, 'ARCHITECTURE.md');
  if (fs.existsSync(arch)) out.push(arch);
  return out;
}

/** Nav groups: "Course" in SERIES.txt order, the others from the mkdocs.yml nav while it exists. */
function navGroups(bySlug: Map<string, string>): Group[] {
  const groups: Group[] = [];
  const course: string[] = [];
  const series = path.join(DOCS, 'lessons/SERIES.txt');
  if (fs.existsSync(series)) {
    for (const line of fs.readFileSync(series, 'utf8').split('\n')) {
      const file = line.trim().split(/\s+/)[2];
      const slug = file && pageSlug(path.join(DOCS, file));
      if (slug && bySlug.has(slug) && !course.includes(slug)) course.push(slug);
    }
  }
  const mk = path.join(VIEWER, 'mkdocs.yml');
  let current: Group | null = null;
  if (fs.existsSync(mk)) {
    const text = fs.readFileSync(mk, 'utf8');
    const nav = text.slice(text.search(/^nav:/m) + 4).split('\n');
    for (const line of nav) {
      if (/^\S/.test(line)) break;
      const g = line.match(/^ {2}- (?:"([^"]+)"|([^:"]+)):\s*$/);
      if (g) {
        current = { title: g[1] || g[2], slugs: [] };
        groups.push(current);
        continue;
      }
      const item = line.match(/^ {4}- (?:"[^"]+"|[^:"]+):\s*(\S+\.md)\s*$/);
      if (item && current) {
        const slug = pageSlug(path.join(VIEWER, item[1]));
        if (bySlug.has(slug)) current.slugs.push(slug);
      }
    }
  }
  const courseGroup = groups.find((g) => g.title === 'Course');
  if (courseGroup) {
    for (const s of courseGroup.slugs) if (!course.includes(s)) course.push(s);
    courseGroup.slugs = course;
  } else {
    groups.push({ title: 'Course', slugs: course });
  }
  const listed = new Set(groups.flatMap((g) => g.slugs));
  const rest = [...bySlug.keys()].filter((s) => !listed.has(s) && !s.includes('/')).sort();
  if (rest.length) groups.push({ title: 'More', slugs: rest });
  return groups.filter((g) => g.slugs.length);
}

const SNIPPET = /^([ \t]*)-{1,}8<-{1,}[ \t]+("(?:\\"|[^"\n\r])+?"|'(?:\\'|[^'\n\r])+?')[ \t]*$/;
const MARKER = /-{1,}8<-{1,}[ \t]+\[[ \t]*(start|end)[ \t]*:[ \t]*([a-z][-_0-9a-z]*)[ \t]*\]/i;

function resolveFile(p: string): string | null {
  for (const base of BASES) {
    const f = path.join(base, p);
    if (fs.existsSync(f) && fs.statSync(f).isFile()) return f;
  }
  return null;
}

/** Lines of one include spec, or an error string. */
export function include(spec: string, sources: Set<string>): string[] | string {
  const m = spec.match(/^(.*?)(?::(\d*):(\d*)|:(\d+)|:([a-z][-_0-9a-z]*))?$/i)!;
  const file = resolveFile(m[1]);
  if (file) sources.add(file);
  if (!file) return 'file not found (looked in session_viewer/docs/ and session/)';
  let lines = fs.readFileSync(file, 'utf8').replace(/\r\n/g, '\n').split('\n');
  if (lines[lines.length - 1] === '') lines.pop();
  if (m[2] !== undefined || m[4] !== undefined) {
    const start = parseInt(m[2] ?? m[4], 10) || 1;
    const end = m[3] ? parseInt(m[3], 10) : m[4] !== undefined ? start : lines.length;
    if (start > lines.length) return `line ${start} is past the end (${lines.length} lines)`;
    lines = lines.slice(start - 1, end);
  } else if (m[5]) {
    const name = m[5];
    const out: string[] = [];
    let inside = false;
    let found = false;
    for (const l of lines) {
      const k = l.match(MARKER);
      if (k) {
        if (k[2] === name && k[1].toLowerCase() === 'start') {
          inside = true;
          found = true;
        } else if (k[2] === name && inside) {
          break;
        }
        continue;
      }
      if (inside) out.push(l);
    }
    if (!found) return `section "${name}" not found`;
    return out;
  }
  return lines.filter((l) => !MARKER.test(l));
}

function resolveSnippets(text: string, page: string, errors: string[], sources: Set<string>): string {
  const out: string[] = [];
  for (const line of text.split('\n')) {
    const m = line.match(SNIPPET);
    if (!m) {
      out.push(line);
      continue;
    }
    const spec = m[2].slice(1, -1);
    const got = include(spec, sources);
    if (typeof got === 'string') {
      errors.push(`${page}: --8<-- "${spec}": ${got}`);
      continue;
    }
    for (const l of got) out.push(l.length ? m[1] + l : l);
  }
  return out.join('\n');
}

export default function coursePlugin(): Plugin {
  let base = '/';
  let hl: Highlighter | null = null;
  let pages = new Map<string, Page>();
  let groups: Group[] = [];
  let assets = new Map<string, string>();
  let searchJson = '';
  let sources = new Set<string>();
  let rendering: Promise<void> | null = null;
  let isBuild = false;

  const highlight = (code: string, lang: string): string => {
    const id = ALIAS[lang] ?? lang;
    if (!hl || !LANGS.includes(id)) return esc(code);
    const { tokens } = hl.codeToTokens(code, { lang: id as any, theme: THEME_NAME });
    return tokens
      .map((line) => {
        let out = '';
        let cls = '';
        let buf = '';
        const flush = () => {
          if (buf) out += cls ? `<span class="${cls}">${esc(buf)}</span>` : esc(buf);
          buf = '';
        };
        for (const t of line) {
          const c = tokenClass(t.color);
          if (c !== cls) {
            flush();
            cls = c;
          }
          buf += t.content;
        }
        flush();
        return out;
      })
      .join('\n');
  };

  const renderPage = (file: string, slug: string, bySlug: Map<string, string>, links: Link[], errors: string[]): Page => {
    const rel = posix(path.relative(SESSION, file));
    let raw = fs.readFileSync(file, 'utf8').replace(/\r\n/g, '\n');
    let search = true;
    const front = raw.match(/^---\n([\s\S]*?)\n---\n/);
    if (front) {
      search = !/exclude:\s*true/.test(front[1]);
      raw = raw.slice(front[0].length);
    }
    raw = resolveSnippets(raw, rel, errors, sources);

    const ids = new Set<string>();
    const toc: Page['toc'] = [];
    const dir = path.dirname(file);

    const rewrite = (href: string): { url: string; external: boolean } => {
      if (/^[a-z][a-z0-9+.-]*:|^\/\//i.test(href)) return { url: href, external: true };
      if (href.startsWith('#/')) return { url: href, external: false };
      if (href.startsWith('#')) {
        links.push({ from: rel, href, slug, frag: href.slice(1) });
        return { url: `#/course/${slug}${href}`, external: false };
      }
      const [p, frag = ''] = href.split('#');
      const abs = path.resolve(dir, decodeURI(p));
      const target = bySlug.get(pageSlug(abs)) === abs ? pageSlug(abs) : '';
      if (p.endsWith('.md')) {
        links.push({ from: rel, href, slug: target, frag });
        return { url: `#/course/${target || pageSlug(abs)}${frag ? '#' + frag : ''}`, external: false };
      }
      if (fs.existsSync(abs) && fs.statSync(abs).isFile()) {
        if (abs.startsWith(VIEWER + path.sep)) {
          const key = posix(path.relative(VIEWER, abs));
          assets.set(key, abs);
          return { url: `${base}course/${key}`, external: false };
        }
        if (abs.startsWith(SESSION + path.sep)) return { url: GITHUB + posix(path.relative(SESSION, abs)), external: true };
      }
      links.push({ from: rel, href, slug: '', frag: '' });
      return { url: href, external: false };
    };

    const md = new Marked({ gfm: true });
    md.use({
      renderer: {
        heading(this: any, { tokens, depth }: Tokens.Heading) {
          const inner = this.parser.parseInline(tokens);
          const text = stripTags(inner);
          const id = slugify(text, ids);
          if (depth === 2 || depth === 3) toc.push({ level: depth, id, text });
          return `<h${depth} id="${id}">${inner}<a class="anchor" href="#/course/${slug}#${id}" aria-label="Link to this section">#</a></h${depth}>\n`;
        },
        code({ text, lang }: Tokens.Code) {
          const l = (lang || '').trim().split(/\s+/)[0].toLowerCase();
          // Pygments drops blank lines at both ends; an include that starts on a blank line would show one.
          const body = text.replace(/^\n+|\n+$/g, '');
          return `<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>${highlight(body, l)}</code></pre></div>\n`;
        },
        link(this: any, { href, title, tokens }: Tokens.Link) {
          const r = rewrite(href);
          const t = title ? ` title="${esc(title)}"` : '';
          const ext = r.external ? ' target="_blank" rel="noopener"' : '';
          return `<a href="${esc(r.url)}"${t}${ext}>${this.parser.parseInline(tokens)}</a>`;
        },
        image(this: any, { href, title, tokens }: Tokens.Image) {
          const r = rewrite(href);
          const t = title ? ` title="${esc(title)}"` : '';
          const alt = this.parser.parseInline(tokens, this.parser.textRenderer);
          return `<img src="${esc(r.url)}" alt="${esc(unesc(alt))}"${t} loading="lazy" decoding="async">`;
        },
      },
      // marked reads "~`x`" as literal text (a GFM strikethrough edge case); lex it again with the
      // tilde escaped so the code span renders, as in Python-Markdown.
      walkTokens(token) {
        if (token.type === 'text' && !('tokens' in token && token.tokens) && /~`/.test(token.raw)) {
          (token as Tokens.Text).tokens = md.Lexer.lexInline(token.raw.replace(/~`/g, '\\~`'));
        }
      },
    });

    // pymdownx admonitions (!!! and ??? blocks) become placeholders, rendered in place afterwards.
    const blocks: string[] = [];
    const lines = raw.split('\n');
    const kept: string[] = [];
    let fence = '';
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      const f = line.match(/^\s*(`{3,}|~{3,})/);
      if (f && (!fence || f[1].startsWith(fence))) fence = fence ? '' : f[1];
      const a = !fence && line.match(/^(\?{3}\+?|!{3})\s+([\w-]+)(?:\s+"(.*)")?\s*$/);
      if (!a) {
        kept.push(line);
        continue;
      }
      const body: string[] = [];
      while (i + 1 < lines.length && (lines[i + 1].trim() === '' || /^( {4}|\t)/.test(lines[i + 1]))) body.push(lines[++i].replace(/^( {4}|\t)/, ''));
      const title = md.parseInline(a[3] ?? a[2].charAt(0).toUpperCase() + a[2].slice(1)) as string;
      const inner = md.parse(body.join('\n')) as string;
      blocks.push(
        a[1] === '!!!'
          ? `<div class="note"><p class="note-title">${title}</p>${inner}</div>`
          : `<details class="note"${a[1].endsWith('+') ? ' open' : ''}><summary>${title}</summary>${inner}</details>`,
      );
      kept.push('', `@@BLOCK${blocks.length - 1}@@`, '');
    }
    let html = md.parse(kept.join('\n')) as string;
    html = html.replace(/<p>@@BLOCK(\d+)@@<\/p>/g, (_m, n) => blocks[+n]);
    const h1 = html.match(/<h1 id="[^"]*">([\s\S]*?)<a class="anchor"/);
    const title = h1 ? stripTags(h1[1]) : slug;
    return { slug, file, title, html, toc, ids, search };
  };

  const buildSearch = (list: Page[]) => {
    const docs: { id: string; title: string; page: string; route: string; text: string }[] = [];
    for (const p of list) {
      if (!p.search) continue;
      const html = p.html.replace(/<div class="code">[\s\S]*?<\/pre><\/div>/g, ' ');
      const re = /<h([1-3]) id="([^"]+)">([\s\S]*?)<\/h\1>/g;
      const heads = [...html.matchAll(re)];
      heads.forEach((h, i) => {
        const end = i + 1 < heads.length ? heads[i + 1].index! : html.length;
        const text = stripTags(html.slice(h.index! + h[0].length, end));
        const title = stripTags(h[3].replace(/<a class="anchor"[\s\S]*?<\/a>/, ''));
        const route = h[1] === '1' ? `/course/${p.slug}` : `/course/${p.slug}#${h[2]}`;
        docs.push({ id: route, title, page: p.title, route, text });
      });
    }
    for (const c of kernelClasses(SESSION)) {
      docs.push({ id: `k:${c.suite}`, title: c.label, page: 'Kernel API', route: `/tests?suite=${c.suite}`, text: c.description });
      for (const t of c.tests) {
        const route = `/tests?suite=${c.suite}&test=${encodeURIComponent(t)}`;
        docs.push({ id: `k:${c.suite}:${t}`, title: t, page: `Kernel API · ${c.label}`, route, text: '' });
      }
    }
    const ms = new MiniSearch(SEARCH_OPTIONS);
    ms.addAll(docs);
    return JSON.stringify(ms);
  };

  const renderAll = async () => {
    hl ??= await createHighlighter({ themes: [greyTheme as any], langs: LANGS as any });
    const t0 = Date.now();
    const files = listPages();
    sources = new Set([...files, path.join(VIEWER, 'mkdocs.yml'), path.join(DOCS, 'lessons/SERIES.txt')]);
    const bySlug = new Map(files.map((f) => [pageSlug(f), f]));
    const links: Link[] = [];
    const errors: string[] = [];
    assets = new Map();
    const next = new Map<string, Page>();
    for (const [slug, file] of bySlug) next.set(slug, renderPage(file, slug, bySlug, links, errors));
    if (errors.length) throw new Error(`course: ${errors.length} unresolved --8<-- include(s):\n  ${errors.join('\n  ')}`);
    const broken: string[] = [];
    for (const l of links) {
      const target = l.slug && next.get(l.slug);
      if (!target) broken.push(`${l.from}: ${l.href}`);
      else if (l.frag && !target.ids.has(l.frag)) broken.push(`${l.from}: ${l.href} (no anchor #${l.frag})`);
    }
    if (broken.length) {
      const msg = `course: ${broken.length} broken link(s):\n  ${broken.join('\n  ')}`;
      if (process.env.DOCS_STRICT_LINKS) throw new Error(msg);
      console.warn(msg);
    }
    pages = next;
    groups = navGroups(bySlug);
    searchJson = buildSearch([...pages.values()]);
    console.log(`course: ${pages.size} pages, ${assets.size} assets, search ${(searchJson.length / 1024).toFixed(0)} KB, ${Date.now() - t0} ms`);
  };

  const ensure = () => (rendering ??= renderAll());

  const courseModule = () => {
    const order = groups.flatMap((g) => g.slugs);
    const meta: Record<string, { title: string; prev?: string; next?: string }> = {};
    for (const p of pages.values()) meta[p.slug] = { title: p.title };
    order.forEach((s, i) => {
      if (i > 0) meta[s].prev = order[i - 1];
      if (i + 1 < order.length) meta[s].next = order[i + 1];
    });
    const loaders = [...pages.keys()].map((s) => `${JSON.stringify(s)}: () => import(${JSON.stringify(V_PAGE + s)})`);
    return `export const groups = ${JSON.stringify(groups)};\nexport const pages = ${JSON.stringify(meta)};\nexport const loaders = {${loaders.join(',\n')}};\n`;
  };

  const kernelModule = () => {
    const out: Record<string, { label: string; description: string }> = {};
    for (const c of kernelClasses(SESSION)) out[c.suite] = { label: c.label, description: c.description };
    return `export default ${JSON.stringify(out)};\n`;
  };

  return {
    name: 'session-course',
    configResolved(config) {
      base = config.base;
      isBuild = config.command === 'build';
    },
    async buildStart() {
      await ensure();
    },
    resolveId(id) {
      if (id === V_COURSE || id === V_SEARCH || id === V_KERNEL || id.startsWith(V_PAGE)) return '\0' + id;
    },
    async load(id) {
      if (!id.startsWith('\0virtual:')) return;
      await ensure();
      const v = id.slice(1);
      if (v === V_COURSE) return courseModule();
      if (v === V_KERNEL) return kernelModule();
      if (v === V_SEARCH) return `export default ${JSON.stringify(searchJson)};\n`;
      const page = pages.get(v.slice(V_PAGE.length));
      if (page) return `export default ${JSON.stringify({ title: page.title, html: page.html, toc: page.toc })};\n`;
    },
    generateBundle() {
      if (!isBuild) return;
      for (const [key, abs] of assets) this.emitFile({ type: 'asset', fileName: `course/${key}`, source: fs.readFileSync(abs) });
    },
    configureServer(server: ViteDevServer) {
      const prefix = `${base}course/`;
      const types: Record<string, string> = { '.svg': 'image/svg+xml', '.png': 'image/png', '.jpg': 'image/jpeg', '.webp': 'image/webp', '.gif': 'image/gif' };
      server.middlewares.use((req, res, next) => {
        const url = decodeURI((req.url || '').split('?')[0]);
        const abs = url.startsWith(prefix) && assets.get(url.slice(prefix.length));
        if (!abs) return next();
        res.setHeader('Content-Type', types[path.extname(abs)] || 'application/octet-stream');
        fs.createReadStream(abs).pipe(res);
      });
      // Watch only the files the pages read: the lesson folders hold thousands more (and targets).
      let timer: ReturnType<typeof setTimeout> | undefined;
      const watch = () => server.watcher.add([...sources]);
      ensure().then(watch, () => {});
      server.watcher.on('all', (_e, file) => {
        if (!sources.has(file)) return;
        clearTimeout(timer);
        timer = setTimeout(async () => {
          rendering = null;
          try {
            await ensure();
          } catch (e) {
            console.error(String(e));
            return;
          }
          watch();
          for (const mod of server.moduleGraph.idToModuleMap.values()) {
            if (mod.id?.startsWith('\0virtual:')) server.moduleGraph.invalidateModule(mod);
          }
          server.ws.send({ type: 'full-reload', path: '*' });
        }, 300);
      });
    },
  };
}
