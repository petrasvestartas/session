<template>
  <div class="search-backdrop" @mousedown.self="emit('close')">
    <div class="search-panel" role="dialog" aria-label="Search">
      <input
        ref="input"
        v-model="query"
        type="search"
        placeholder="Search the course and the kernel API"
        aria-label="Search"
        @keydown.esc="emit('close')"
        @keydown.down.prevent="move(1)"
        @keydown.up.prevent="move(-1)"
        @keydown.enter.prevent="open(results[active])"
      />
      <p v-if="!index" class="status">Loading the index</p>
      <p v-else-if="query.trim() && !results.length" class="status">No results</p>
      <ul v-else class="results">
        <li v-for="(r, i) in results" :key="r.id">
          <a :href="'#' + r.route" :class="{ active: i === active }" @click.prevent="open(r)" @mouseenter="active = i">
            <span class="title">{{ r.title }}</span>
            <span class="page">{{ r.page }}</span>
          </a>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, shallowRef, watch } from 'vue';
import { useRouter } from 'vue-router';
import type MiniSearch from 'minisearch';
import { SEARCH_OPTIONS } from '../searchOptions';

interface Hit {
  id: string;
  title: string;
  page: string;
  route: string;
}

const emit = defineEmits<{ close: [] }>();
const router = useRouter();
const input = ref<HTMLInputElement | null>(null);
const query = ref('');
const active = ref(0);
const index = shallowRef<MiniSearch | null>(null);

// The library and the index load together, only when the box opens.
onMounted(async () => {
  input.value?.focus();
  const [{ default: MiniSearchLib }, { default: json }] = await Promise.all([import('minisearch'), import('virtual:search-index')]);
  index.value = MiniSearchLib.loadJSON(json, SEARCH_OPTIONS);
});

const results = computed<Hit[]>(() => {
  const q = query.value.trim();
  if (!index.value || !q) return [];
  return index.value
    .search(q, { prefix: true, fuzzy: 0.15, combineWith: 'AND', boost: { title: 3 } })
    .slice(0, 30) as unknown as Hit[];
});

watch(query, () => (active.value = 0));

const move = (d: number) => {
  const n = results.value.length;
  if (n) active.value = (active.value + d + n) % n;
};

const open = (r: Hit | undefined) => {
  if (!r) return;
  emit('close');
  router.push(r.route);
};
</script>

<style scoped>
.search-backdrop {
  position: fixed;
  inset: 0;
  z-index: 300;
  background: rgba(255, 255, 255, 0.7);
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding: 10vh 16px 0;
}

.search-panel {
  width: 100%;
  max-width: 620px;
  max-height: 75vh;
  display: flex;
  flex-direction: column;
  background: #ffffff;
  border: 1px solid var(--rule);
  border-radius: 6px;
  box-shadow: 0 6px 24px rgba(0, 0, 0, 0.08);
  overflow: hidden;
}

input {
  width: 100%;
  padding: 0.8rem 1rem;
  border: none;
  border-bottom: 1px solid var(--rule);
  outline: none;
  font-size: 16px;
  color: var(--fg);
}

.status {
  margin: 0;
  padding: 0.8rem 1rem;
  color: var(--faint);
  font-size: 14px;
}

.results {
  list-style: none;
  margin: 0;
  padding: 0.3rem 0;
  overflow-y: auto;
}

.results a {
  display: flex;
  flex-direction: column;
  padding: 0.45rem 1rem;
  color: var(--fg);
  text-decoration: none;
}

.results a.active {
  background: var(--hover);
}

.title {
  font-size: 14px;
}

.page {
  font-size: 12px;
  color: var(--faint);
}
</style>
