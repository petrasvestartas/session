<template>
  <div class="classes-view">
    <aside class="class-list">
      <div class="search-row">
        <input
          v-model="filter"
          type="text"
          placeholder="filter…"
          class="search-input"
        />
      </div>
      <button
        v-for="cls in filteredClasses"
        :key="cls"
        class="class-item"
        :class="{ active: cls === selected }"
        @click="selected = cls"
      >
        {{ cls }}
      </button>
    </aside>

    <section class="class-detail" v-if="selected">
      <header class="detail-header">
        <h2>{{ selected }}</h2>
        <p v-if="summary" class="summary">{{ summary }}</p>
        <div class="legend">
          <span><span class="line solid"></span>composition</span>
          <span><span class="line dashed"></span>factory or reference</span>
        </div>
      </header>

      <div v-if="error" class="error">{{ error }}</div>
      <div ref="diagramRef" class="diagram"></div>

      <div class="sections">
        <section v-if="contains.length" class="rel-section">
          <h3>Contains <span class="count">{{ contains.length }}</span></h3>
          <p class="hint">Fields of <code>{{ selected }}</code> that are other classes.</p>
          <div class="chips">
            <button v-for="c in contains" :key="c" class="chip comp" @click="selected = c">{{ c }}</button>
          </div>
        </section>

        <section v-if="containedBy.length" class="rel-section">
          <h3>Contained by <span class="count">{{ containedBy.length }}</span></h3>
          <p class="hint">Other classes that have <code>{{ selected }}</code> as a field.</p>
          <div class="chips">
            <button v-for="c in containedBy" :key="c" class="chip comp" @click="selected = c">{{ c }}</button>
          </div>
        </section>

        <section v-if="factoriesOut.length" class="rel-section">
          <h3>Can become <span class="count">{{ factoriesOut.length }}</span></h3>
          <p class="hint"><code>{{ selected }}</code> can be converted into these via <code>from_*</code> factories.</p>
          <div class="chips">
            <button v-for="c in factoriesOut" :key="c" class="chip fact" @click="selected = c">{{ c }}</button>
          </div>
        </section>

        <section v-if="factoriesIn.length" class="rel-section">
          <h3>Created from <span class="count">{{ factoriesIn.length }}</span></h3>
          <p class="hint">These classes can be converted into <code>{{ selected }}</code>.</p>
          <div class="chips">
            <button v-for="c in factoriesIn" :key="c" class="chip fact" @click="selected = c">{{ c }}</button>
          </div>
        </section>

        <section v-if="usesOut.length" class="rel-section">
          <h3>References <span class="count">{{ usesOut.length }}</span></h3>
          <p class="hint">Mentioned in method signatures of <code>{{ selected }}</code>.</p>
          <div class="chips">
            <button v-for="c in usesOut" :key="c" class="chip uses" @click="selected = c">{{ c }}</button>
          </div>
        </section>

        <section v-if="usesIn.length" class="rel-section">
          <h3>Referenced by <span class="count">{{ usesIn.length }}</span></h3>
          <p class="hint">These classes mention <code>{{ selected }}</code> in method signatures.</p>
          <div class="chips">
            <button v-for="c in usesIn" :key="c" class="chip uses" @click="selected = c">{{ c }}</button>
          </div>
        </section>

        <p v-if="!hasAnyRelation" class="hint empty">No tracked relationships for this class.</p>
      </div>
    </section>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch, nextTick } from 'vue';
import mermaid from 'mermaid';

const graph = ref({});
const selected = ref('');
const filter = ref('');
const error = ref(null);
const diagramRef = ref(null);

const allClasses = computed(() => Object.keys(graph.value).sort());

const filteredClasses = computed(() => {
  const f = filter.value.trim().toLowerCase();
  if (!f) return allClasses.value;
  return allClasses.value.filter((c) => c.toLowerCase().includes(f));
});

const info = computed(() => graph.value[selected.value] || {});
const summary = computed(() => info.value.summary || '');
const contains = computed(() => info.value.composition || []);
const factoriesOut = computed(() => info.value.factories || []);
const usesOut = computed(() => info.value.uses || []);

function inverseEdges(field) {
  const out = [];
  for (const [cls, data] of Object.entries(graph.value)) {
    if (cls === selected.value) continue;
    if ((data[field] || []).includes(selected.value)) out.push(cls);
  }
  return out.sort();
}

const containedBy = computed(() => inverseEdges('composition'));
const factoriesIn = computed(() => inverseEdges('factories'));
const usesIn = computed(() => inverseEdges('uses'));

const hasAnyRelation = computed(
  () =>
    contains.value.length ||
    containedBy.value.length ||
    factoriesOut.value.length ||
    factoriesIn.value.length ||
    usesOut.value.length ||
    usesIn.value.length
);

function buildMermaid(cls) {
  if (!cls) return '';
  const lines = ['classDiagram'];
  const seen = new Set([cls]);
  const addClass = (c) => {
    if (!seen.has(c)) {
      lines.push(`  class ${c}`);
      seen.add(c);
    }
  };
  lines.push(`  class ${cls}`);
  // Solid line (no arrowhead) = composition, both directions deduped
  const compEdges = new Set();
  for (const c of contains.value) compEdges.add(c);
  for (const c of containedBy.value) compEdges.add(c);
  for (const c of compEdges) {
    addClass(c);
    lines.push(`  ${cls} -- ${c}`);
  }
  // Dashed line (no arrowhead) = factory or uses, deduped against composition
  const dashedEdges = new Set();
  for (const c of factoriesOut.value) dashedEdges.add(c);
  for (const c of factoriesIn.value) dashedEdges.add(c);
  for (const c of usesOut.value) dashedEdges.add(c);
  for (const c of usesIn.value) dashedEdges.add(c);
  for (const c of compEdges) dashedEdges.delete(c);
  for (const c of dashedEdges) {
    addClass(c);
    lines.push(`  ${cls} .. ${c}`);
  }
  return lines.join('\n');
}

let renderId = 0;
async function renderDiagram() {
  if (!selected.value || !diagramRef.value) return;
  try {
    const src = buildMermaid(selected.value);
    if (!src) {
      diagramRef.value.innerHTML = '';
      return;
    }
    const id = 'classGraph' + ++renderId;
    const { svg } = await mermaid.render(id, src);
    diagramRef.value.innerHTML = svg;
    // style the focused class node
    const nodes = diagramRef.value.querySelectorAll('g.node');
    nodes.forEach((n) => {
      const txt = n.querySelector('text');
      if (txt && txt.textContent.trim() === selected.value) {
        n.classList.add('focused-node');
      }
    });
    error.value = null;
  } catch (e) {
    error.value = e.message || String(e);
  }
}

onMounted(async () => {
  mermaid.initialize({ startOnLoad: false, theme: 'dark', maxTextSize: 200000 });
  graph.value = window.API_INDEX?.class_graph || {};
  if (!Object.keys(graph.value).length) {
    error.value = 'No class_graph in window.API_INDEX. Run: python -m session_mcp.generate_browser_index';
    return;
  }
  selected.value = allClasses.value[0] || '';
  await nextTick();
  renderDiagram();
});

watch(selected, async () => {
  await nextTick();
  renderDiagram();
});
</script>

<style scoped>
.classes-view {
  display: flex;
  height: 100%;
  color: #ddd;
  box-sizing: border-box;
}

.class-list {
  width: 200px;
  flex-shrink: 0;
  border-right: 1px solid #1a1a1a;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  padding: 0.75rem 0.5rem;
  scrollbar-width: thin;
}

.search-row {
  padding: 0 0.25rem 0.5rem 0.25rem;
  position: sticky;
  top: 0;
  background: #000;
  z-index: 1;
}

.search-input {
  width: 100%;
  background: #0a0a0a;
  border: 1px solid #222;
  color: #ddd;
  padding: 0.35rem 0.5rem;
  font-family: inherit;
  font-size: 13px;
  box-sizing: border-box;
}

.search-input:focus {
  outline: none;
  border-color: #444;
}

.class-item {
  background: transparent;
  border: none;
  color: #888;
  font-family: inherit;
  font-size: 14px;
  font-weight: 300;
  text-align: left;
  padding: 0.3rem 0.5rem;
  cursor: pointer;
  transition: all 0.15s;
}

.class-item:hover {
  color: #fff;
}

.class-item.active {
  background: #1a1a1a;
  color: #fff;
  font-weight: 600;
}

.class-detail {
  flex: 1;
  min-width: 0;
  overflow-y: auto;
  padding: 1rem 1.5rem;
}

.detail-header {
  margin-bottom: 1rem;
  border-bottom: 1px solid #1a1a1a;
  padding-bottom: 0.75rem;
}

.detail-header h2 {
  margin: 0 0 0.25rem 0;
  font-weight: 300;
  font-size: 28px;
}

.summary {
  margin: 0;
  color: #888;
  font-size: 13px;
}

.legend {
  display: flex;
  gap: 1.25rem;
  font-size: 12px;
  color: #888;
  margin-top: 0.5rem;
}

.legend .line {
  display: inline-block;
  width: 28px;
  vertical-align: middle;
  margin-right: 6px;
  border-top-width: 2px;
}

.legend .line.solid {
  border-top: 2px solid #ddd;
}

.legend .line.dashed {
  border-top: 2px dashed #ddd;
}

.diagram {
  width: 100%;
  margin: 1rem 0;
  overflow-x: auto;
  background: #0a0a0a;
  border: 1px solid #1a1a1a;
  padding: 0.5rem;
  min-height: 120px;
}

.diagram :deep(svg) {
  max-width: 100%;
  height: auto;
}

.diagram :deep(.focused-node rect),
.diagram :deep(.focused-node polygon) {
  stroke: #50fa7b !important;
  stroke-width: 3px !important;
}

.sections {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin-top: 1rem;
}

.rel-section h3 {
  margin: 0 0 0.25rem 0;
  font-weight: 400;
  font-size: 15px;
  color: #eee;
}

.count {
  color: #666;
  font-size: 12px;
  font-weight: 300;
  margin-left: 0.35rem;
}

.hint {
  margin: 0 0 0.5rem 0;
  font-size: 12px;
  color: #666;
}

.hint code {
  color: #aaa;
  background: #1a1a1a;
  padding: 0 4px;
  border-radius: 2px;
}

.chips {
  display: flex;
  flex-wrap: wrap;
  gap: 0.4rem;
}

.chip {
  background: #1a1a1a;
  border: 1px solid #2a2a2a;
  color: #ddd;
  font-family: inherit;
  font-size: 13px;
  padding: 0.25rem 0.6rem;
  cursor: pointer;
  border-radius: 3px;
  transition: all 0.15s;
}

.chip:hover {
  background: #2a2a2a;
  border-color: #444;
}

.chip.comp { border-left: 3px solid #50fa7b; }
.chip.fact { border-left: 3px solid #8be9fd; }
.chip.uses { border-left: 3px solid #6272a4; }

.empty {
  color: #555;
  font-style: italic;
}

.error {
  color: #ff5555;
  padding: 0.75rem;
  border: 1px solid #ff5555;
  border-radius: 4px;
  margin: 0.5rem 0;
}
</style>
