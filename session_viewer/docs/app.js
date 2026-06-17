// Standalone docs site for the viewer build-log lessons. Independent of session_tests.
// Flow: fetch sections.json (served dynamically by serve.py) → build the sidebar →
// fetch the selected .md → render with marked → highlight code with highlight.js.
// The active section is kept in the URL hash so links are shareable.

const navEl = document.getElementById('nav');
const contentEl = document.getElementById('content');

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

async function renderSection(sections, id) {
  const s = sections.find((x) => x.id === id) || sections[0];
  if (!s) { contentEl.innerHTML = '<p class="loading">No lessons found.</p>'; return; }
  const res = await fetch(s.file, { cache: 'no-store' });
  const raw = await res.text();
  contentEl.innerHTML = marked.parse(raw);
  contentEl.querySelectorAll('pre code').forEach((b) => {
    try { hljs.highlightElement(b); } catch (_) { /* unknown lang → leave plain */ }
  });
  document.querySelector('main').scrollTop = 0;
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
  await renderSection(sections, current());
  window.addEventListener('hashchange', () => renderSection(sections, current()));
}

main();
