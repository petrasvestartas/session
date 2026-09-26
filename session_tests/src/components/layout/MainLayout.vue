<template>
  <div class="main-layout">
    <nav class="sidebar" :class="{ collapsed: sidebarCollapsed }" aria-label="Site">
      <button class="sidebar-toggle" @click="sidebarCollapsed = !sidebarCollapsed" :aria-label="sidebarCollapsed ? 'Show menu' : 'Hide menu'">
        <span class="toggle-arrow">{{ sidebarCollapsed ? '○' : '●' }}</span>
      </button>

      <div v-if="!sidebarCollapsed" class="nav-section">
        <div class="repo-icons">
          <a href="https://github.com/petrasvestartas/session" target="_blank" rel="noopener" class="repo-link" title="Session" aria-label="Session on GitHub">
            <svg class="repo-icon" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
              <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
            </svg>
          </a>
          <a v-for="r in repos" :key="r.name" :href="'https://github.com/petrasvestartas/' + r.name" target="_blank" rel="noopener" class="repo-link" :title="r.title">
            <img :src="base + 'icons/' + r.name + '_black.png'" class="repo-icon" :alt="r.title + ' kernel on GitHub'">
          </a>
        </div>

        <button type="button" class="search-open" @click="searchOpen = true">Search <kbd>/</kbd></button>

        <a
          href="#/tests"
          class="nav-button"
          :class="{ active: currentRoute === 'tests' }"
          @click.prevent="openTestsMenu">
          Kernel API
        </a>

        <div
          v-if="currentRoute === 'tests' && testsSuites.length"
          class="suites-section">
          <template v-for="s in testsSuites" :key="s">
            <button
              type="button"
              class="suite-button"
              :class="{ active: s === selectedSuite }"
              @click="selectSuite(s)">
              <span class="suite-dot" :class="{ fail: suitePassedMap.get(s) === false }">●</span>
              {{ suiteLabel(s) }}
            </button>
            <div v-if="s === selectedSuite && suiteFunctions.length" class="functions-section">
              <button
                v-for="fn in suiteFunctions" :key="fn.name"
                type="button"
                class="fn-button"
                @click="scrollToTest(fn.name)">
                <span class="fn-dot" :class="{ fail: !fn.passed }">●</span>
                {{ fn.name }}
              </button>
            </div>
          </template>
        </div>

        <router-link
          :to="'/install'"
          class="nav-button"
          active-class="active">
          Install
        </router-link>

        <div v-if="currentRoute === 'install'" class="suites-section">
          <button
            v-for="s in installSections" :key="s.id"
            type="button"
            class="suite-button"
            :class="{ active: activeInstall === s.id }"
            @click="selectInstall(s.id)">
            {{ s.title }}
          </button>
        </div>

        <router-link to="/course" class="nav-button" :class="{ active: currentRoute === 'course' }">Viewer course</router-link>

        <div v-if="currentRoute === 'course'" class="suites-section course-section">
          <template v-for="g in courseGroups" :key="g.title">
            <div class="group-title">{{ g.title }}</div>
            <router-link
              v-for="s in g.slugs" :key="s"
              :to="'/course/' + s"
              class="suite-button course-link"
              :class="{ active: s === currentSlug }">
              {{ coursePages[s]?.title }}
            </router-link>
          </template>
        </div>
      </div>
    </nav>

    <div class="main-content">
      <main class="content-area" id="content">
        <router-view></router-view>
      </main>
    </div>

    <SearchBox v-if="searchOpen" @close="searchOpen = false" />
    <a class="viewer-corner" :href="viewerHref" title="Back to the viewer" aria-label="Back to the viewer"></a>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onBeforeUnmount, watch, defineAsyncComponent } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { groups as courseGroups, pages as coursePages } from 'virtual:course';
import { ensureTestData } from '../../dataLoader';
import { sections as installSections } from '../../installSections';

const route = useRoute();
const router = useRouter();

const SearchBox = defineAsyncComponent(() => import('../SearchBox.vue'));
const base = import.meta.env.BASE_URL;
const viewerHref = new URL('../', window.location.origin + base).href;
const repos = [
  { name: 'session_cpp', title: 'C++' },
  { name: 'session_py', title: 'Python' },
  { name: 'session_rust', title: 'Rust' },
  { name: 'session_proto', title: 'Protobuf' },
  { name: 'session_data', title: 'Data' },
];

// The search box loads its index only when opened: the Search button or the / key.
const searchOpen = ref(false);
const onKey = (e: KeyboardEvent) => {
  const t = e.target as HTMLElement;
  if (e.key !== '/' || searchOpen.value || /^(INPUT|TEXTAREA)$/.test(t.tagName) || t.isContentEditable) return;
  e.preventDefault();
  searchOpen.value = true;
};
onMounted(() => window.addEventListener('keydown', onKey));
onBeforeUnmount(() => window.removeEventListener('keydown', onKey));

const currentSlug = computed(() => (route.path.startsWith('/course/') ? route.path.slice('/course/'.length) : ''));

// Install page sub-sections — each install_sections/*.md is its own page via ?section=.
const activeInstall = computed(() => {
  const q = route.query.section;
  const first = installSections[0]?.id ?? '';
  return typeof q === 'string' && installSections.some((s) => s.id === q) ? q : first;
});
const selectInstall = (id: string) => {
  const first = installSections[0]?.id ?? '';
  router.push({ path: '/install', query: id === first ? undefined : { section: id } });
};

const currentRoute = computed(() => {
  const path = route.path;
  if (path.startsWith('/install')) return 'install';
  if (path.startsWith('/course')) return 'course';
  if (path.startsWith('/tests')) return 'tests';
  return 'home';
});

const testsSuites = ref<string[]>([]);
const selectedSuite = ref('');
// Phones start with the menu folded away and fold it again after each navigation.
const narrow = window.matchMedia('(max-width: 760px)');
const sidebarCollapsed = ref(narrow.matches);
watch(() => route.fullPath, () => {
  if (narrow.matches) sidebarCollapsed.value = true;
});

const loadSuitesFromTestData = () => {
  if (typeof window === 'undefined' || typeof (window as any).TEST_DATA === 'undefined') return;

  const set = new Set<string>();
  const data = (window as any).TEST_DATA;
  for (const [key, testArray] of Object.entries(data)) {
    if (!Array.isArray(testArray)) continue;
    const parts = key.split('_');
    parts.pop(); // remove language
    const suite = parts.join('_');
    if (suite) set.add(suite);
  }
  testsSuites.value = Array.from(set.values())
    .sort((a, b) => suiteLabel(a).localeCompare(suiteLabel(b), undefined, { sensitivity: 'base' }));
};

const syncSelectedSuiteWithRoute = () => {
  const q = route.query.suite;
  const qSuite = typeof q === 'string' ? q : '';
  if (qSuite && testsSuites.value.includes(qSuite)) {
    selectedSuite.value = qSuite;
  } else if (testsSuites.value.length && !selectedSuite.value) {
    selectedSuite.value = testsSuites.value[0];
  }
};

const openTestsMenu = () => {
  if (currentRoute.value !== 'tests') {
    const suiteToShow =
      selectedSuite.value ||
      (testsSuites.value.length ? testsSuites.value[0] : '');
    router.push({
      path: '/tests',
      query: suiteToShow ? { suite: suiteToShow } : undefined,
    });
  }
};

const suiteLabel = (suite: string): string => {
  // Normalize an identifier for matching: lowercase, strip non-alnum.
  const norm = (s: string) => s.toLowerCase().replace(/[^a-z0-9]/g, '');
  const suiteKey = norm(suite.replace(/_test$/i, ''));
  if (typeof (window as any).TEST_DATA !== 'undefined') {
    const seen = new Set<string>();
    let fallback: string | null = null;
    for (const lang of ['cpp', 'python', 'rust']) {
      const arr = (window as any).TEST_DATA[suite + '_' + lang];
      if (!Array.isArray(arr)) continue;
      for (const t of arr) {
        const g = t && t.group;
        if (!g || seen.has(g)) continue;
        seen.add(g);
        if (norm(g) === suiteKey) return g;
        if (fallback === null) fallback = g;
      }
    }
    if (fallback !== null) return fallback;
  }
  return suite.replace(/_test$/i, '')
    .split('_').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join('');
};

const selectSuite = (suite: string) => {
  selectedSuite.value = suite;
  router.push({ path: '/tests', query: { suite } });
};

const suitePassedMap = computed(() => {
  const _ = testsSuites.value; // reactive dependency
  const map = new Map<string, boolean>();
  if (typeof (window as any).TEST_DATA === 'undefined') return map;
  const data = (window as any).TEST_DATA;
  for (const [key, testArray] of Object.entries(data)) {
    if (!Array.isArray(testArray)) continue;
    const parts = key.split('_');
    parts.pop();
    const suite = parts.join('_');
    if (!suite) continue;
    if (!map.has(suite)) map.set(suite, true);
    for (const t of testArray) {
      if (!t.passed) map.set(suite, false);
    }
  }
  return map;
});

const suiteFunctions = computed(() => {
  if (!selectedSuite.value || typeof (window as any).TEST_DATA === 'undefined') return [];
  const data = (window as any).TEST_DATA;
  const names = new Map<string, boolean>();
  for (const [key, testArray] of Object.entries(data)) {
    if (!Array.isArray(testArray)) continue;
    const parts = key.split('_');
    parts.pop();
    const suite = parts.join('_');
    if (suite !== selectedSuite.value) continue;
    for (const t of testArray) {
      const n = t.test_name || '(unnamed)';
      if (!names.has(n)) names.set(n, true);
      if (!t.passed) names.set(n, false);
    }
  }
  return Array.from(names.entries()).map(([name, passed]) => ({ name, passed }));
});

const scrollToTest = (name: string) => {
  const el = document.getElementById('test-' + name);
  if (el) el.scrollIntoView({ behavior: 'smooth', block: 'start' });
  router.replace({ path: '/tests', query: { suite: selectedSuite.value, test: name } });
};

// Test data is lazy now — only pull it (and build the suites sidebar) when the Tests tab is in
// play, so Viewer/Install never load the ~3.5MB testData.js.
const refreshTestsSidebar = async () => {
  await ensureTestData();
  loadSuitesFromTestData();
  syncSelectedSuiteWithRoute();
};

onMounted(() => {
  if (currentRoute.value === 'tests') refreshTestsSidebar();
  else syncSelectedSuiteWithRoute();
});

watch(currentRoute, (r) => {
  if (r === 'tests' && !testsSuites.value.length) refreshTestsSidebar();
});

watch(
  () => route.query.suite,
  () => {
    syncSelectedSuiteWithRoute();
  }
);
</script>

<style scoped>
.main-layout {
  height: 100vh;
  height: 100dvh;
  display: flex;
  flex-direction: row;
  position: relative;
}

.sidebar {
  width: fit-content;
  min-width: 120px;
  max-width: 290px;
  background: #ffffff;
  border-right: 1px solid var(--rule);
  display: flex;
  flex-direction: column;
  padding: 0;
  position: relative;
  z-index: 100;
  overflow: hidden;
}

.sidebar.collapsed {
  width: 0;
  min-width: 0;
  border-right: none;
  overflow: hidden;
}

.sidebar-toggle {
  position: absolute;
  top: 0;
  right: 0;
  width: 20px;
  height: 100%;
  background: transparent;
  border: none;
  cursor: pointer;
  padding: 0;
  margin: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.sidebar-toggle:hover .toggle-arrow {
  color: var(--fg);
}

.toggle-arrow {
  color: #c8c8c8;
  font-size: 21px;
  transition: color 0.2s;
}

.sidebar.collapsed .sidebar-toggle {
  position: fixed;
  left: 0;
  right: auto;
  height: 100vh;
  z-index: 150;
}

.nav-section {
  display: flex;
  flex-direction: column;
  padding: 1.2rem 20px 2rem 0;
  flex: 1;
  overflow-y: auto;
  min-height: 0;
  scrollbar-width: none;
}

.nav-section::-webkit-scrollbar {
  display: none;
}

.nav-button {
  padding: 0.4rem 0.75rem;
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 14px;
  font-weight: 400;
  color: var(--fg);
  text-decoration: none;
  text-align: left;
}

.nav-button:hover {
  color: var(--muted);
}

.nav-button.active {
  background: var(--hover);
  font-weight: 600;
}

.search-open {
  margin: 0.25rem 0.75rem 0.6rem;
  padding: 0.3rem 0.5rem;
  background: #ffffff;
  border: 1px solid var(--rule);
  border-radius: 4px;
  color: var(--muted);
  text-align: left;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.search-open:hover {
  border-color: var(--faint);
}

.search-open kbd {
  font-size: 11px;
  color: var(--faint);
}

.repo-icons {
  display: flex;
  flex-direction: row;
  gap: 0.25rem;
  padding: 0.5rem 0.5rem;
  flex-wrap: nowrap;
}

.repo-link {
  color: var(--fg);
  text-decoration: none;
  width: 24px;
  height: 24px;
  display: inline-flex;
  justify-content: center;
  align-items: center;
  opacity: 0.85;
}

.repo-link:hover {
  opacity: 0.5;
}

.repo-icon {
  width: 20px;
  height: 20px;
  display: block;
}

.suites-section {
  display: flex;
  flex-direction: column;
  padding-left: 1.5rem;
}

.group-title {
  padding: 0.6rem 0.75rem 0.2rem;
  font-size: 12px;
  color: var(--faint);
}

.suite-button {
  padding: 0.2rem 0.75rem;
  background: transparent;
  border: none;
  color: var(--muted);
  font-family: inherit;
  font-size: 13px;
  font-weight: 400;
  cursor: pointer;
  text-align: left;
  text-decoration: none;
  display: flex;
  align-items: center;
  gap: 0.35rem;
}

.suite-button:hover {
  color: var(--fg);
}

.suite-button.active {
  background: var(--hover);
  color: var(--fg);
  font-weight: 600;
}

.course-link {
  display: block;
  line-height: 1.35;
}

.functions-section {
  display: flex;
  flex-direction: column;
  padding-left: 1rem;
}

.fn-button {
  padding: 0.1rem 0.5rem;
  background: transparent;
  border: none;
  color: var(--muted);
  font-family: inherit;
  font-size: 13px;
  font-weight: 400;
  cursor: pointer;
  text-align: left;
  display: flex;
  align-items: center;
  gap: 0.35rem;
}

.fn-button:hover {
  color: var(--fg);
}

.fn-dot,
.suite-dot {
  font-size: 8px;
  flex-shrink: 0;
  color: #c8c8c8;
}

.fn-dot.fail,
.suite-dot.fail {
  color: var(--fail);
}

.main-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  background: #ffffff;
  overflow: hidden;
}

.content-area {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 0.25rem 1rem 1rem 1rem;
  background: #ffffff;
}

/* The black folded corner, as in the viewer: back to the app one level up. */
.viewer-corner {
  position: fixed;
  right: 0;
  bottom: 0;
  width: 0;
  height: 0;
  border-style: solid;
  border-width: 0 0 28px 28px;
  border-color: transparent transparent #000000 transparent;
  z-index: 200;
}

@media (max-width: 760px) {
  .sidebar:not(.collapsed) {
    position: fixed;
    top: 0;
    left: 0;
    height: 100%;
    max-width: 85vw;
  }

  .content-area {
    padding: 0.25rem 16px 1rem 24px;
  }
}
</style>
