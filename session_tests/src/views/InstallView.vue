<template>
  <div class="install-root">
    <!-- Section is chosen in the left sidebar (MainLayout) via ?section=. Each install_sections/*.md
         is one page; default to the first section. -->
    <article ref="contentEl" class="section-content markdown" v-html="renderedHtml"></article>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { marked } from 'marked';
import { getHighlighter, LANGS, THEME } from '../highlighter';
import { sections } from '../installSections';

const route = useRoute();
const contentEl = ref<HTMLElement | null>(null);

// Active section comes from the URL (the sidebar sets ?section=); default to the first section.
const active = computed(() => {
  const q = route.query.section;
  return typeof q === 'string' && sections.some((s) => s.id === q)
    ? q
    : (sections[0]?.id ?? '');
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
</script>

<style scoped>
.install-root { height: 100%; display: flex; flex-direction: column; background: #0d0f12; color: #d7dae0; }

.section-content { flex: 1 1 auto; min-height: 0; overflow-y: auto; padding: 24px 32px; }

.markdown { line-height: 1.6; }
.markdown :deep(h1) { font-size: 24px; margin: 0 0 14px; }
.markdown :deep(h2) { font-size: 18px; margin: 24px 0 8px; color: #cdd3db; }
.markdown :deep(blockquote) { border-left: 3px solid #2e3a46; margin: 10px 0; padding: 4px 14px; color: #9aa3af; background: #11161c; border-radius: 0 6px 6px 0; }
.markdown :deep(p) { margin: 9px 0; }
/* Pill only for INLINE code; code blocks keep Shiki's single background. */
.markdown :deep(:not(pre) > code) { background: #161a20; padding: 1px 5px; border-radius: 4px; font-size: 13px; }
.markdown :deep(pre) { border-radius: 8px; padding: 14px 16px; overflow-x: auto; font-size: 13px; }
.markdown :deep(pre code) { background: transparent; padding: 0; }
.markdown :deep(a) { color: #6fb3ff; }
</style>
