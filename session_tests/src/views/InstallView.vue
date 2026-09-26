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
.install-root { height: 100%; display: flex; flex-direction: column; background: #ffffff; color: var(--fg); }

.section-content { flex: 1 1 auto; min-height: 0; overflow-y: auto; padding: 24px 0; max-width: 820px; }

.markdown { line-height: 1.6; }
.markdown :deep(h1) { font-size: 24px; font-weight: 600; margin: 0 0 14px; }
.markdown :deep(h2) { font-size: 18px; font-weight: 600; margin: 24px 0 8px; }
.markdown :deep(blockquote) { border-left: 3px solid var(--rule); margin: 10px 0; padding: 4px 14px; color: var(--muted); }
.markdown :deep(p) { margin: 9px 0; }
/* Pill only for INLINE code; code blocks keep Shiki's single background. */
.markdown :deep(:not(pre) > code) { background: var(--code-bg); padding: 1px 5px; border-radius: 3px; font-size: 13px; }
.markdown :deep(pre) { border-radius: 4px; padding: 14px 16px; overflow-x: auto; font-size: 13px; }
.markdown :deep(pre code) { background: transparent; padding: 0; }
.markdown :deep(a) { color: var(--fg); text-underline-offset: 2px; text-decoration-color: #b8b8b8; }
</style>
