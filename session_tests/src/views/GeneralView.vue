<template>
  <div class="viewer-root">
    <!-- Section is chosen in the left sidebar (MainLayout) via ?section=. Live = full-bleed viewer;
         a section = notes/code on the left, live viewer on the right. -->
    <div v-if="active === 'live'" class="live-full">
      <iframe class="viewer-frame" :src="viewerUrl" title="session_viewer"></iframe>
    </div>
    <div v-else class="section-split">
      <article ref="contentEl" class="section-content markdown" v-html="renderedHtml"></article>
      <iframe class="viewer-frame side" :src="viewerUrl" title="session_viewer"></iframe>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { marked } from 'marked';
import { getHighlighter, LANGS, THEME } from '../highlighter';
import { sections } from '../viewerSections';

const route = useRoute();
const contentEl = ref<HTMLElement | null>(null);

// Active section comes from the URL (the sidebar sets ?section=); default to the live viewer.
const active = computed(() => {
  const q = route.query.section;
  return typeof q === 'string' && sections.some((s) => s.id === q) ? q : 'live';
});

const renderedHtml = computed(() => {
  const s = sections.find((x) => x.id === active.value);
  return s ? (marked.parse(s.raw) as string) : '';
});

// Highlight fenced code blocks with Shiki (VS Code quality) after the HTML lands in the DOM.
const highlightAll = async () => {
  const el = contentEl.value;
  if (!el) return;
  const hl = await getHighlighter();
  const blocks = Array.from(el.querySelectorAll<HTMLElement>('pre code'));
  for (const code of blocks) {
    const lang = (code.className.match(/language-(\w+)/) || [])[1] || 'text';
    if (!LANGS.includes(lang)) continue; // unknown language → leave the plain block
    const html = hl.codeToHtml(code.textContent || '', { lang, theme: THEME });
    code.parentElement!.outerHTML = html; // replace <pre> with Shiki's highlighted <pre>
  }
};
watch(renderedHtml, () => nextTick(highlightAll));
onMounted(() => nextTick(highlightAll));

// In dev, embed the live `trunk serve` (:8770) so editing the viewer's Rust auto-recompiles and
// reloads inside these pages — no build_viewer.sh needed. In prod, use the static build copied into
// public/viewer/ by bash/build_viewer.sh.
const viewerUrl = import.meta.env.DEV
  ? 'http://localhost:8770/'
  : (import.meta.env.BASE_URL || '/') + 'viewer/index.html';
</script>

<style scoped>
.viewer-root { height: 100%; display: flex; flex-direction: column; background: #0d0f12; color: #d7dae0; }
.live-full { flex: 1 1 auto; min-height: 0; }
.viewer-frame { width: 100%; height: 100%; border: none; display: block; background: #000; }

.section-split { flex: 1 1 auto; min-height: 0; display: flex; }
.section-content { flex: 1 1 auto; overflow-y: auto; padding: 24px 32px; }
.viewer-frame.side { flex: 0 0 42%; border-left: 1px solid #20242b; }

.markdown { line-height: 1.6; max-width: 860px; }
.markdown :deep(h1) { font-size: 24px; margin: 0 0 14px; }
.markdown :deep(h2) { font-size: 18px; margin: 24px 0 8px; color: #cdd3db; }
.markdown :deep(blockquote) { border-left: 3px solid #2e3a46; margin: 10px 0; padding: 4px 14px; color: #9aa3af; background: #11161c; border-radius: 0 6px 6px 0; }
.markdown :deep(p) { margin: 9px 0; }
.markdown :deep(code) { background: #161a20; padding: 1px 5px; border-radius: 4px; font-size: 13px; }
.markdown :deep(pre) { border-radius: 8px; padding: 14px 16px; overflow-x: auto; font-size: 13px; }
.markdown :deep(a) { color: #6fb3ff; }
</style>
