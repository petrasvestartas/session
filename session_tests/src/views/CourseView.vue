<template>
  <div class="course-page">
    <p v-if="missing" class="missing">
      There is no course page named "{{ slug }}". <router-link to="/course">All lessons</router-link>
    </p>
    <template v-else-if="page">
      <details v-if="page.toc.length" class="toc-inline">
        <summary>On this page</summary>
        <a v-for="t in page.toc" :key="t.id" :href="'#/course/' + slug + '#' + t.id" :class="'l' + t.level" @click.prevent="go(t.id)">{{ t.text }}</a>
      </details>
      <div class="columns">
        <article class="doc" v-html="page.html" @click="onClick"></article>
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
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { loaders, pages, type CoursePage } from 'virtual:course';

const route = useRoute();
const router = useRouter();
const page = ref<CoursePage | null>(null);
const missing = ref(false);
const slug = computed(() => String(route.params.slug || ''));
const meta = computed(() => pages[slug.value]);

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
    const load = loaders[s];
    missing.value = !load;
    if (!load) {
      page.value = null;
      return;
    }
    const mod = await load();
    if (s !== slug.value) return;
    page.value = mod.default;
    document.title = `${mod.default.title} · Session docs`;
    await nextTick();
    scrollToHash();
  },
  { immediate: true },
);

watch(() => route.hash, () => nextTick(scrollToHash));

// Delegated clicks inside the rendered page: the copy buttons and in-app links.
const onClick = (e: MouseEvent) => {
  const target = e.target as HTMLElement;
  const copy = target.closest<HTMLButtonElement>('button.copy');
  if (copy) {
    const text = copy.parentElement?.querySelector('pre')?.textContent ?? '';
    navigator.clipboard?.writeText(text).then(() => {
      copy.textContent = 'Copied';
      setTimeout(() => (copy.textContent = 'Copy'), 1200);
    });
    return;
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
  padding: 1.5rem 0 3rem;
}

.missing {
  color: var(--muted);
}

.columns {
  display: flex;
  gap: 2.5rem;
  align-items: flex-start;
}

.doc {
  flex: 1 1 auto;
  min-width: 0;
  max-width: 820px;
  line-height: 1.6;
  font-size: 15.5px;
}

.toc {
  position: sticky;
  top: 1rem;
  flex: 0 0 230px;
  max-height: calc(100vh - 2rem);
  overflow-y: auto;
  font-size: 13px;
  line-height: 1.35;
  scrollbar-width: thin;
}

.toc-title {
  margin: 0 0 0.5rem;
  color: var(--faint);
  font-size: 12px;
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
  color: var(--fg);
}

.toc a.l3,
.toc-inline a.l3 {
  padding-left: 0.9rem;
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
  color: var(--muted);
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
  max-width: 820px;
  margin-top: 3rem;
  padding-top: 1rem;
  border-top: 1px solid var(--rule);
}

.pager a {
  display: flex;
  flex-direction: column;
  color: var(--fg);
  text-decoration: none;
  max-width: 48%;
}

.pager a span {
  font-size: 12px;
  color: var(--faint);
}

.pager a:hover {
  text-decoration: underline;
}

.pager .next {
  margin-left: auto;
  text-align: right;
}

/* Rendered markdown (v-html): reach in with :deep. */
.doc :deep(h1) {
  font-size: 26px;
  font-weight: 600;
  line-height: 1.25;
  margin: 0 0 1rem;
}

.doc :deep(h2) {
  font-size: 19px;
  font-weight: 600;
  margin: 2.4rem 0 0.6rem;
  padding-bottom: 0.3rem;
  border-bottom: 1px solid var(--rule);
}

.doc :deep(h3) {
  font-size: 16px;
  font-weight: 600;
  margin: 1.8rem 0 0.4rem;
}

.doc :deep(h4) {
  font-size: 15px;
  font-weight: 600;
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
  color: #c8c8c8;
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
  margin: 0.7rem 0;
}

.doc :deep(li) {
  margin: 0.2rem 0;
}

.doc :deep(a) {
  color: var(--fg);
  text-decoration: underline;
  text-decoration-color: #b8b8b8;
  text-underline-offset: 2px;
}

.doc :deep(a:hover) {
  text-decoration-color: var(--fg);
}

.doc :deep(img) {
  max-width: 100%;
  height: auto;
  display: block;
  margin: 1rem 0;
}

.doc :deep(:not(pre) > code) {
  font-size: 0.88em;
  background: var(--code-bg);
  padding: 0.08em 0.3em;
  border-radius: 3px;
  overflow-wrap: anywhere;
}

.doc :deep(.code) {
  position: relative;
  margin: 0.8rem 0 1.1rem;
}

.doc :deep(pre) {
  margin: 0;
  padding: 0.8rem 1rem;
  background: var(--code-bg);
  border-radius: 4px;
  overflow-x: auto;
  font-size: 13px;
  line-height: 1.5;
  color: #1a1a1a;
}

.doc :deep(.copy) {
  position: absolute;
  top: 0.35rem;
  right: 0.35rem;
  padding: 0.1rem 0.45rem;
  font-size: 11px;
  color: var(--muted);
  background: #ffffff;
  border: 1px solid var(--rule);
  border-radius: 3px;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.15s;
}

.doc :deep(.code:hover .copy),
.doc :deep(.copy:focus) {
  opacity: 1;
}

@media (hover: none) {
  .doc :deep(.copy) {
    opacity: 1;
  }

  .doc :deep(.anchor) {
    display: none;
  }
}

.doc :deep(blockquote) {
  margin: 1rem 0;
  padding: 0.1rem 1rem;
  border-left: 3px solid var(--rule);
  color: var(--muted);
}

.doc :deep(.note) {
  margin: 1rem 0;
  padding: 0.2rem 1rem;
  border-left: 3px solid var(--rule);
}

.doc :deep(.note-title),
.doc :deep(details.note summary) {
  font-weight: 600;
  margin: 0.4rem 0;
}

.doc :deep(details.note summary) {
  cursor: pointer;
}

.doc :deep(table) {
  border-collapse: collapse;
  margin: 1rem 0;
  font-size: 14px;
  display: block;
  overflow-x: auto;
}

.doc :deep(th),
.doc :deep(td) {
  text-align: left;
  padding: 0.3rem 0.8rem 0.3rem 0;
  border-bottom: 1px solid var(--rule);
  vertical-align: top;
}

.doc :deep(th) {
  font-weight: 600;
}

.doc :deep(hr) {
  border: none;
  border-top: 1px solid var(--rule);
  margin: 2rem 0;
}
</style>
