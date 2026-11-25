<template>
  <div class="course-page">
    <p v-if="missing" class="missing">
      There is no course page named "{{ slug }}". <router-link to="/course">All lessons</router-link>
    </p>
    <template v-else-if="page">
      <div v-if="activeMeta" class="tut-banner">
        <div class="tut-banner-left">
          <span class="tut-eyebrow">Tutorial {{ activeMeta.number }} of 17</span>
          <span v-if="activeMeta.fileCount" class="tut-count">{{ activeMeta.fileCount }} source files</span>
        </div>
        <div class="tut-actions">
          <button
            v-if="longCodeCount > 0"
            type="button"
            class="wood-btn"
            @click="toggleAllCode">
            {{ allExpanded ? 'Collapse long files' : `Expand all ${longCodeCount} long files` }}
          </button>
        </div>
      </div>

      <details v-if="page.toc.length" class="toc-inline">
        <summary>On this page</summary>
        <a v-for="t in page.toc" :key="t.id" :href="'#/course/' + slug + '#' + t.id" :class="'l' + t.level" @click.prevent="go(t.id)">{{ t.text }}</a>
      </details>

      <div class="columns">
        <article ref="articleEl" class="doc" v-html="page.html" @click="onClick"></article>
        <aside v-if="page.toc.length" class="toc" aria-label="On this page">
          <p class="toc-title">On this page</p>
          <a v-for="t in page.toc" :key="t.id" :href="'#/course/' + slug + '#' + t.id" :class="'l' + t.level" @click.prevent="go(t.id)">{{ t.text }}</a>
        </aside>
      </div>

      <nav class="pager" aria-label="Previous and next lesson">
        <router-link v-if="meta?.prev" :to="'/course/' + meta.prev" class="prev">
          <span>Previous</span>{{ pages[meta.prev].title }}
        </router-link>
        <router-link v-if="meta?.next" :to="'/course/' + meta.next" class="next">
          <span>Next</span>{{ pages[meta.next].title }}
        </router-link>
      </nav>

      <dialog ref="plateDialog" class="presentation-dialog" @click.self="closePlateModal">
        <header class="presentation-header">
          <span class="presentation-kicker">{{ lightboxAlt || `Tutorial ${activeMeta?.number}` }}</span>
          <div class="presentation-controls">
            <a v-if="lightboxSrc" :href="lightboxSrc" target="_blank" rel="noopener" class="presentation-ext">Raw PNG ↗</a>
            <button type="button" class="presentation-close" aria-label="Close figure preview" @click="closePlateModal">×</button>
          </div>
        </header>
        <div class="presentation-stage" @click.self="closePlateModal">
          <img v-if="lightboxSrc" :src="lightboxSrc" :alt="lightboxAlt" class="presentation-img" />
        </div>
      </dialog>
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { loaders, pages, tutorials, type CoursePage } from 'virtual:course';

const route = useRoute();
const router = useRouter();
const page = ref<CoursePage | null>(null);
const missing = ref(false);
const articleEl = ref<HTMLElement | null>(null);
const plateDialog = ref<HTMLDialogElement | null>(null);
const lightboxSrc = ref('');
const lightboxAlt = ref('');
const longCodeCount = ref(0);
const allExpanded = ref(false);

const slug = computed(() => String(route.params.slug || ''));
const meta = computed(() => pages[slug.value]);

const activeLookup = computed(() => {
  for (let i = 0; i < tutorials.length; i++) {
    const t = tutorials[i];
    if (t.slug === slug.value) return { item: t, idx: i };
    const ch = t.chapters.find((c) => c.slug === slug.value);
    if (ch) return { item: ch, idx: i };
  }
  return null;
});

const activeMeta = computed(() => activeLookup.value?.item ?? null);

const openPlateModal = (src: string, alt: string) => {
  lightboxSrc.value = src;
  lightboxAlt.value = alt;
  plateDialog.value?.showModal();
};

const closePlateModal = () => {
  if (plateDialog.value?.open) plateDialog.value.close();
  lightboxSrc.value = '';
};

const enhanceArticle = () => {
  const el = articleEl.value;
  if (!el) return;
  allExpanded.value = false;
  let longCount = 0;
  const codeBlocks = el.querySelectorAll<HTMLElement>('.code');
  codeBlocks.forEach((box) => {
    if (box.dataset.enhanced === '1') return;
    box.dataset.enhanced = '1';
    const pre = box.querySelector('pre');
    const text = pre?.textContent ?? '';
    const lines = text ? text.split('\n').length : 0;
    const isLong = lines > 42;
    if (isLong) {
      longCount++;
      box.classList.add('is-long', 'is-collapsed');
    }
    const bar = document.createElement('div');
    bar.className = 'code-bar';
    const info = document.createElement('span');
    info.className = 'code-lines';
    info.textContent = `${lines} ${lines === 1 ? 'line' : 'lines'}`;
    bar.appendChild(info);

    const actions = document.createElement('div');
    actions.className = 'code-bar-actions';
    if (isLong) {
      const toggle = document.createElement('button');
      toggle.type = 'button';
      toggle.className = 'code-toggle';
      toggle.textContent = `Show all ${lines} lines`;
      toggle.dataset.lines = String(lines);
      actions.appendChild(toggle);
    }
    const copyBtn = box.querySelector<HTMLButtonElement>('button.copy');
    if (copyBtn) {
      actions.appendChild(copyBtn);
    }
    bar.appendChild(actions);
    box.insertBefore(bar, box.firstChild);
  });
  longCodeCount.value = longCount;
};

const toggleAllCode = () => {
  const el = articleEl.value;
  if (!el) return;
  const nextExpanded = !allExpanded.value;
  allExpanded.value = nextExpanded;
  el.querySelectorAll<HTMLElement>('.code.is-long').forEach((box) => {
    box.classList.toggle('is-collapsed', !nextExpanded);
    const btn = box.querySelector<HTMLButtonElement>('button.code-toggle');
    if (btn) {
      btn.textContent = nextExpanded ? 'Collapse' : `Show all ${btn.dataset.lines} lines`;
    }
  });
};

const scrollToHash = () => {
  const id = decodeURIComponent(route.hash.slice(1));
  const el = id ? document.getElementById(id) : null;
  if (el) el.scrollIntoView({ block: 'start' });
  else document.getElementById('content')?.scrollTo({ top: 0 });
};

const go = (id: string) => {
  if (route.hash === '#' + id) scrollToHash();
  else router.push({ path: route.path, hash: '#' + id });
};

watch(
  slug,
  async (s) => {
    closePlateModal();
    const load = loaders[s];
    missing.value = !load;
    if (!load) {
      page.value = null;
      longCodeCount.value = 0;
      return;
    }
    const mod = await load();
    if (s !== slug.value) return;
    if (mod.default.redirect) {
      await router.replace(mod.default.redirect + (mod.default.redirect.includes('#') ? '' : route.hash));
      return;
    }
    page.value = mod.default;
    document.title = `${mod.default.title} · Session docs`;
    await nextTick();
    enhanceArticle();
    scrollToHash();
  },
  { immediate: true },
);

watch(() => route.hash, () => nextTick(scrollToHash));

// Delegated clicks inside the rendered page: figure lightbox, code expand/collapse, copy, and in-app links.
const onClick = (e: MouseEvent) => {
  const target = e.target as HTMLElement;
  const codeToggle = target.closest<HTMLButtonElement>('button.code-toggle');
  if (codeToggle) {
    const box = codeToggle.closest<HTMLElement>('.code.is-long');
    if (box) {
      const collapsed = box.classList.toggle('is-collapsed');
      codeToggle.textContent = collapsed ? `Show all ${codeToggle.dataset.lines} lines` : 'Collapse';
    }
    return;
  }
  const copy = target.closest<HTMLButtonElement>('button.copy');
  if (copy) {
    const box = copy.closest('.code');
    const text = box?.querySelector('pre')?.textContent ?? '';
    navigator.clipboard?.writeText(text).then(() => {
      copy.textContent = 'Copied';
      setTimeout(() => (copy.textContent = 'Copy'), 1200);
    });
    return;
  }
  const img = target.closest<HTMLImageElement>('img');
  if (img && !e.ctrlKey && !e.metaKey && !e.shiftKey && !img.src.endsWith('.svg')) {
    const parentLink = img.closest<HTMLAnchorElement>('a');
    if (!parentLink || /\.(png|webp|jpg|jpeg)(\?|$)/i.test(parentLink.getAttribute('href') || '')) {
      e.preventDefault();
      openPlateModal(img.src, img.alt || '');
      return;
    }
  }
  const a = target.closest<HTMLAnchorElement>('a[href^="#/"]');
  if (!a || e.ctrlKey || e.metaKey || e.shiftKey) return;
  e.preventDefault();
  const to = a.getAttribute('href')!.slice(1);
  if (to === route.fullPath) scrollToHash();
  else router.push(to);
};
</script>

<style scoped>
.course-page {
  padding: 1.25rem 0.75rem 4rem;
}

.missing {
  color: var(--fg);
}

.tut-banner {
  max-width: 1320px;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0 0 0.65rem;
  margin-bottom: 1.25rem;
  border-bottom: 1px solid var(--rule);
}

.tut-banner-left {
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  gap: 0.75rem;
}

.tut-eyebrow {
  font-size: 13.5px;
  font-weight: 500;
  color: var(--fg);
}

.tut-count {
  font-size: 13px;
  color: var(--muted);
}

.tut-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.wood-btn {
  display: inline-flex;
  align-items: center;
  padding: 0.3rem 0.75rem;
  font-family: inherit;
  font-size: 13px;
  font-weight: 400;
  border: 1px solid var(--rule);
  background: #ffffff;
  color: var(--fg);
  text-decoration: none;
  cursor: pointer;
  transition: border-color 0.14s ease, color 0.14s ease;
}

.wood-btn:hover {
  border-color: var(--accent);
  color: var(--accent);
}

.columns {
  display: flex;
  gap: 2.75rem;
  align-items: flex-start;
}

.doc {
  flex: 1 1 auto;
  min-width: 0;
  max-width: 1040px;
  overflow-wrap: anywhere;
  line-height: 1.65;
  font-size: 15.5px;
}

.toc {
  position: sticky;
  top: 1rem;
  flex: 0 0 232px;
  max-height: calc(100vh - 2rem);
  overflow-y: auto;
  font-size: 13px;
  line-height: 1.42;
  padding: 0.25rem 0 0.25rem 0.85rem;
  border-left: 2px solid var(--rule);
  scrollbar-width: none;
}

.toc::-webkit-scrollbar {
  display: none;
}

.toc-title {
  margin: 0 0 0.45rem;
  color: var(--muted);
  font-weight: 400;
  font-size: 13px;
}

.toc a,
.toc-inline a {
  display: block;
  padding: 0.18rem 0;
  color: var(--muted);
  text-decoration: none;
}

.toc a:hover,
.toc-inline a:hover {
  color: var(--accent);
}

.toc a.l3,
.toc-inline a.l3 {
  padding-left: 0.85rem;
}

.toc-inline {
  display: none;
  margin: 0 0 1rem;
  font-size: 14px;
  border-bottom: 1px solid var(--rule);
  padding-bottom: 0.5rem;
}

.toc-inline summary {
  cursor: pointer;
  color: var(--fg);
  font-weight: 500;
}

@media (max-width: 1100px) {
  .toc {
    display: none;
  }

  .toc-inline {
    display: block;
  }
}

.pager {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  max-width: 1040px;
  margin-top: 3.5rem;
  padding-top: 1.25rem;
  border-top: 1px solid var(--rule);
}

.pager a {
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
  padding: 0.7rem 1rem;
  border: 1px solid var(--rule);
  background: #ffffff;
  color: var(--fg);
  font-weight: 500;
  text-decoration: none;
  max-width: 48%;
  transition: border-color 0.14s ease, transform 0.14s ease, box-shadow 0.14s ease;
}

.pager a span {
  font-size: 12px;
  font-weight: 400;
  color: var(--muted);
}

.pager a:hover {
  border-color: var(--ink);
  box-shadow: 4px 4px 0 var(--accent), 4px 4px 0 1.5px var(--ink);
  transform: translate(-3px, -3px);
  text-decoration: none;
}

.pager .next {
  margin-left: auto;
  text-align: right;
}

/* Lightbox Dialog */
.presentation-dialog {
  inset: 0;
  width: min(96vw, 1680px);
  height: min(92vh, 980px);
  margin: auto;
  padding: 0;
  border: 1px solid var(--ink);
  background: #ffffff;
  color: var(--fg);
  box-shadow: 0 10px 30px rgba(22, 24, 26, 0.16);
}

.presentation-dialog[open] {
  display: flex;
  flex-direction: column;
}

.presentation-dialog::backdrop {
  background: rgba(22, 24, 26, 0.55);
}

.presentation-header {
  flex: none;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0.55rem 1rem;
  border-bottom: 1px solid var(--rule);
  background: #ffffff;
}

.presentation-kicker {
  font-size: 14px;
  font-weight: 500;
  color: var(--fg);
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 65vw;
}

.presentation-controls {
  display: flex;
  align-items: center;
  gap: 0.65rem;
}

.presentation-ext {
  font-size: 13px;
  font-weight: 500;
  color: var(--accent);
  text-decoration: none;
}

.presentation-ext:hover {
  text-decoration: underline;
}

.presentation-close {
  width: 32px;
  height: 32px;
  padding: 0;
  border: 1px solid var(--rule);
  background: #ffffff;
  color: var(--fg);
  font-size: 20px;
  line-height: 1;
  cursor: pointer;
}

.presentation-close:hover {
  border-color: var(--ink);
}

.presentation-stage {
  flex: 1 1 auto;
  min-height: 0;
  padding: 1.25rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #ffffff;
  overflow: hidden;
}

.presentation-img {
  max-width: 100%;
  max-height: 100%;
  width: auto;
  height: auto;
  object-fit: contain;
  display: block;
}

/* Rendered markdown (v-html) — wood.css typography & hairlines. */
.doc :deep(h1) {
  font-size: clamp(28px, 3vw, 38px);
  font-weight: 600;
  letter-spacing: -0.02em;
  line-height: 1.15;
  color: var(--fg);
  margin: 0 0 1.2rem;
}

.doc :deep(h2) {
  font-size: 23px;
  font-weight: 600;
  letter-spacing: -0.01em;
  color: var(--fg);
  margin: 2.6rem 0 0.75rem;
  padding: 0;
  border-bottom: none;
}

.doc :deep(h3) {
  font-size: 17.5px;
  font-weight: 500;
  color: var(--fg);
  margin: 1.9rem 0 0.45rem;
}

.doc :deep(h4) {
  font-size: 15.5px;
  font-weight: 500;
  margin: 1.4rem 0 0.4rem;
}

.doc :deep(h1),
.doc :deep(h2),
.doc :deep(h3),
.doc :deep(h4) {
  scroll-margin-top: 1rem;
  overflow-wrap: anywhere;
}

.doc :deep(.anchor) {
  margin-left: 0.4rem;
  color: var(--faint);
  text-decoration: none;
  font-weight: 400;
  visibility: hidden;
}

.doc :deep(h1:hover .anchor),
.doc :deep(h2:hover .anchor),
.doc :deep(h3:hover .anchor),
.doc :deep(h4:hover .anchor),
.doc :deep(.anchor:focus) {
  visibility: visible;
}

.doc :deep(p),
.doc :deep(ul),
.doc :deep(ol) {
  margin: 0.75rem 0;
}

.doc :deep(li) {
  margin: 0.22rem 0;
}

.doc :deep(a) {
  color: var(--accent);
  font-weight: 500;
  text-decoration: none;
}

.doc :deep(a:hover) {
  color: var(--accent);
  text-decoration: underline;
  text-decoration-color: var(--mark);
  text-decoration-thickness: 2px;
  text-underline-offset: 3px;
}

.doc :deep(img) {
  max-width: 100%;
  width: 100%;
  height: auto;
  display: block;
  margin: 1rem 0;
  border: 1px solid var(--rule);
  background: #ffffff;
  box-shadow: none;
  cursor: zoom-in;
}

.doc :deep(img:not([src*="explain-"])) {
  max-height: 310px;
  width: auto;
  padding: 10px;
  object-fit: contain;
}

.doc :deep(img[src*="explain-"]) {
  border: 0;
  padding: 0;
  margin: 0.9rem 0 1.4rem;
}

.doc :deep(img[src*=".svg"]) {
  border: 0;
  padding: 0;
}

@media (min-width: 680px) {
  .doc :deep(p:has(> a:first-child > img):has(> a:nth-child(2) > img)),
  .doc :deep(p:has(> img:first-child):has(> img:nth-child(2))) {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    align-items: stretch;
    margin: 1.1rem 0;
    padding: 14px;
    border: 1px solid var(--rule);
    background: #ffffff;
  }

  .doc :deep(p:has(> a:first-child > img):has(> a:nth-child(2) > img) img),
  .doc :deep(p:has(> img:first-child):has(> img:nth-child(2)) img) {
    margin: 0;
    padding: 0;
    border: 0;
    width: 100%;
    height: 100%;
    object-fit: contain;
  }
}

.doc :deep(:not(pre) > code) {
  font-size: 0.88em;
  background: var(--code-bg);
  padding: 0.08em 0.3em;
  overflow-wrap: anywhere;
}

.doc :deep(.code) {
  position: relative;
  margin: 0.8rem 0 1.35rem;
  border: 1px solid var(--rule);
  background: var(--code-bg);
}

.doc :deep(.code-bar) {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.3rem 0.75rem;
  background: #ffffff;
  border-bottom: 1px solid var(--rule);
  font-size: 11.5px;
  color: var(--muted);
}

.doc :deep(.code-lines) {
  font-variant-numeric: tabular-nums;
}

.doc :deep(.code-bar-actions) {
  display: flex;
  align-items: center;
  gap: 0.4rem;
}

.doc :deep(.code-toggle),
.doc :deep(.copy) {
  position: static;
  padding: 0.12rem 0.5rem;
  font-family: inherit;
  font-size: 11px;
  font-weight: 400;
  color: var(--fg);
  background: #ffffff;
  border: 1px solid var(--rule);
  cursor: pointer;
  opacity: 1;
}

.doc :deep(.code-toggle:hover),
.doc :deep(.copy:hover) {
  border-color: var(--accent);
  color: var(--accent);
}

.doc :deep(.code.is-collapsed pre) {
  max-height: 320px;
  overflow-y: auto;
}

.doc :deep(.code.is-collapsed::after) {
  content: '';
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  height: 36px;
  background: linear-gradient(to bottom, rgba(245, 246, 246, 0), rgba(245, 246, 246, 0.95));
  pointer-events: none;
}

.doc :deep(pre) {
  margin: 0;
  padding: 16px 20px;
  background: var(--code-bg);
  overflow-x: auto;
  font-size: 13px;
  line-height: 1.55;
  color: #26292b;
}

@media (hover: none) {
  .doc :deep(.anchor) {
    display: none;
  }
}

.doc :deep(blockquote) {
  margin: 1rem 0;
  padding: 0.1rem 1rem;
  border-left: 3px solid var(--mark);
  color: var(--muted);
}

.doc :deep(.note) {
  margin: 0.9rem 0 1.2rem;
  padding: 0.65rem 1rem;
  border: 1px solid var(--rule);
  border-left: 3px solid var(--accent);
  background: rgba(0, 124, 85, 0.05);
}

.doc :deep(.note-title),
.doc :deep(details.note summary) {
  font-weight: 500;
  color: var(--fg);
  margin: 0.15rem 0;
}

.doc :deep(details.note summary) {
  cursor: pointer;
  user-select: none;
}

.doc :deep(details.note[open] summary) {
  margin-bottom: 0.55rem;
  padding-bottom: 0.35rem;
  border-bottom: 1px solid var(--rule);
}

.doc :deep(table) {
  width: 100%;
  border-collapse: collapse;
  border-top: 2px solid var(--fg);
  border-bottom: 1px solid var(--rule);
  margin: 1.2rem 0;
  font-size: 14px;
  display: block;
  overflow-x: auto;
}

.doc :deep(th),
.doc :deep(td) {
  text-align: left;
  padding: 10px 24px 10px 0;
  border-top: 1px solid var(--rule);
  vertical-align: top;
}

.doc :deep(tr:first-child th) {
  border-top: none;
}

.doc :deep(th) {
  font-weight: 500;
  color: var(--muted);
  white-space: nowrap;
}

@media (min-width: 680px) {
  .doc :deep(td:first-child) {
    white-space: nowrap;
  }
}

.doc :deep(table :not(pre) > code) {
  overflow-wrap: normal;
}

.doc :deep(hr) {
  display: none;
}
</style>
