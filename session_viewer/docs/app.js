// Standalone docs site for the viewer build-log lessons. Independent of session_tests.
// Flow: fetch sections.json (served dynamically by serve.py) → build the sidebar →
// fetch the selected .md → render with marked → highlight code with highlight.js.
// The active section is kept in the URL hash so links are shareable.

const navEl = document.getElementById('nav');
const contentEl = document.getElementById('content');
const toggleEl = document.getElementById('navToggle');

// Sidebar collapse — remembers its state and can also be toggled with the `\` key.
function setNavCollapsed(collapsed) {
  document.body.classList.toggle('nav-collapsed', collapsed);
  toggleEl.textContent = collapsed ? '»' : '«';
  localStorage.setItem('navCollapsed', collapsed ? '1' : '0');
}
setNavCollapsed(localStorage.getItem('navCollapsed') === '1');
toggleEl.addEventListener('click', () =>
  setNavCollapsed(!document.body.classList.contains('nav-collapsed')));
window.addEventListener('keydown', (e) => {
  if (e.key === '\\' && !/^(INPUT|TEXTAREA)$/.test(e.target.tagName))
    setNavCollapsed(!document.body.classList.contains('nav-collapsed'));
});

marked.setOptions({ gfm: true, breaks: false });

// highlight.js has no built-in WGSL grammar, so register a minimal one. Covers the
// constructs the lessons use: keywords, scalar/vector/matrix/texture types, builtin
// functions, @attributes (@vertex, @location, …), comments, numbers, strings.
hljs.registerLanguage('wgsl', (hljs) => ({
  name: 'WGSL',
  keywords: {
    keyword:
      'fn let var const struct return if else for while loop break continue switch ' +
      'case default fallthrough discard type alias enable requires override const_assert',
    type:
      'bool i32 u32 f32 f16 vec2 vec3 vec4 mat2x2 mat2x3 mat2x4 mat3x2 mat3x3 mat3x4 ' +
      'mat4x2 mat4x3 mat4x4 array atomic ptr sampler sampler_comparison ' +
      'texture_1d texture_2d texture_2d_array texture_3d texture_cube texture_cube_array ' +
      'texture_multisampled_2d texture_storage_1d texture_storage_2d texture_storage_2d_array ' +
      'texture_storage_3d texture_depth_2d texture_depth_2d_array texture_depth_cube ' +
      'texture_depth_cube_array texture_depth_multisampled_2d',
    literal: 'true false',
    built_in:
      'abs acos asin atan atan2 ceil clamp cos cross degrees distance dot exp exp2 floor ' +
      'fma fract inverseSqrt length log log2 max min mix modf normalize pow radians reflect ' +
      'refract round sign sin smoothstep sqrt step tan select arrayLength dpdx dpdy fwidth ' +
      'textureSample textureLoad textureStore textureDimensions',
  },
  contains: [
    hljs.C_LINE_COMMENT_MODE,
    hljs.C_BLOCK_COMMENT_MODE,
    hljs.QUOTE_STRING_MODE,
    hljs.C_NUMBER_MODE,
    { className: 'meta', begin: /@[A-Za-z_]+/ },                 // @vertex @location @group …
    { className: 'function', beginKeywords: 'fn', end: /(?=\()/, excludeBegin: true,
      contains: [hljs.UNDERSCORE_TITLE_MODE] },
  ],
}));

async function loadSections() {
  const res = await fetch('sections.json', { cache: 'no-store' });
  if (!res.ok) throw new Error(`sections.json → ${res.status}`);
  return res.json(); // [{ id, title, file }]
}

function buildNav(sections, activeId) {
  // keep the title, drop any prior links
  navEl.querySelectorAll('a, .grp').forEach((n) => n.remove());
  for (const s of sections) {
    const a = document.createElement('a');
    a.href = `#${s.id}`;
    a.textContent = s.title;
    if (s.id === activeId) a.classList.add('active');
    navEl.appendChild(a);
  }
}

// Scroll memory: a refresh must land where you were reading. The offset is stored per section,
// so reloading lesson 35 half-way down comes back half-way down lesson 35 — and only on RELOAD:
// clicking a nav link still opens the new lesson at its top.
const mainEl = document.querySelector('main');
const scrollKey = (id) => `scroll:${id}`;
let activeId = null;
let scrollQueued = false;
mainEl.addEventListener('scroll', () => {
  if (scrollQueued || !activeId) return;
  scrollQueued = true;                       // one write per frame, not one per scroll event
  requestAnimationFrame(() => {
    scrollQueued = false;
    if (activeId) localStorage.setItem(scrollKey(activeId), String(mainEl.scrollTop));
  });
});

async function renderSection(sections, id, restore = false) {
  const s = sections.find((x) => x.id === id) || sections[0];
  if (!s) { contentEl.innerHTML = '<p class="loading">No lessons found.</p>'; return; }
  const res = await fetch(s.file, { cache: 'no-store' });
  const raw = await res.text();
  contentEl.innerHTML = marked.parse(raw);
  contentEl.querySelectorAll('pre code').forEach((b) => {
    try { hljs.highlightElement(b); } catch (_) { /* unknown lang → leave plain */ }
  });
  // One copy button per code block: the refactor lessons ship whole files as listings, and a
  // listing that has to be selected by hand across a scrolling page is a listing that gets
  // typed wrong. Clipboard access needs a secure context; localhost counts.
  contentEl.querySelectorAll('pre').forEach((pre) => {
    const btn = document.createElement('button');
    btn.className = 'copy';
    btn.type = 'button';
    btn.textContent = 'copy';
    btn.addEventListener('click', async () => {
      const code = pre.querySelector('code');
      try {
        await navigator.clipboard.writeText(code ? code.textContent : pre.textContent);
        btn.textContent = 'copied';
      } catch (_) {
        btn.textContent = 'select + ctrl-c';
      }
      setTimeout(() => { btn.textContent = 'copy'; }, 1500);
    });
    pre.appendChild(btn);
  });
  activeId = s.id;
  // After paint: highlight.js has finished reflowing the code blocks, so the saved offset means
  // the same thing it did when it was written (an overshoot past the end clamps by itself).
  const saved = restore ? parseFloat(localStorage.getItem(scrollKey(s.id))) : 0;
  requestAnimationFrame(() => { mainEl.scrollTop = Number.isFinite(saved) ? saved : 0; });
  buildNav(sections, s.id);
}

async function main() {
  let sections;
  try {
    sections = await loadSections();
  } catch (e) {
    contentEl.innerHTML =
      `<h1>Can't load lessons</h1><p class="loading">${e}</p>` +
      `<p>Run this site through its server so directory listing works:</p>` +
      `<pre><code>cd session_viewer/docs &amp;&amp; python serve.py</code></pre>`;
    return;
  }
  const current = () => decodeURIComponent(location.hash.replace(/^#/, '')) || (sections[0] && sections[0].id);
  await renderSection(sections, current(), true);   // page load / refresh → resume where you were
  window.addEventListener('hashchange', () => renderSection(sections, current()));
}

main();
