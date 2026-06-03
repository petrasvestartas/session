<template>
  <div class="main-layout">
    <nav class="sidebar" :class="{ collapsed: sidebarCollapsed }">
      <button class="sidebar-toggle" @click="sidebarCollapsed = !sidebarCollapsed">
        <span class="toggle-arrow">{{ sidebarCollapsed ? '○' : '●' }}</span>
      </button>

      <div v-if="!sidebarCollapsed" class="nav-section">
        <div class="repo-icons">
          <a href="https://github.com/petrasvestartas/session" target="_blank" class="repo-link" title="Session">
            <svg class="repo-icon" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
            </svg>
          </a>
          <a href="https://github.com/petrasvestartas/session_cpp" target="_blank" class="repo-link" title="C++">
            <img src="/icons/session_cpp_white.png" class="repo-icon" alt="C++">
          </a>
          <a href="https://github.com/petrasvestartas/session_py" target="_blank" class="repo-link" title="Python">
            <img src="/icons/session_py_white.png" class="repo-icon" alt="Python">
          </a>
          <a href="https://github.com/petrasvestartas/session_rust" target="_blank" class="repo-link" title="Rust">
            <img src="/icons/session_rust_white.png" class="repo-icon" alt="Rust">
          </a>
          <a href="https://github.com/petrasvestartas/session_proto" target="_blank" class="repo-link" title="Protobuf">
            <img src="/icons/session_proto_white.png" class="repo-icon" alt="Protobuf">
          </a>
          <a href="https://github.com/petrasvestartas/session_data" target="_blank" class="repo-link" title="Data">
            <img src="/icons/session_data_white.png" class="repo-icon" alt="Data">
          </a>
        </div>

        <router-link
          :to="'/viewer'"
          class="nav-button"
          active-class="active">
          Viewer
        </router-link>

        <div v-if="currentRoute === 'viewer'" class="suites-section">
          <button
            type="button"
            class="suite-button"
            :class="{ active: activeSection === 'live' }"
            @click="selectSection('live')">
            Live
          </button>
          <button
            v-for="s in viewerSections" :key="s.id"
            type="button"
            class="suite-button"
            :class="{ active: activeSection === s.id }"
            @click="selectSection(s.id)">
            {{ s.title }}
          </button>
        </div>

        <div
          class="nav-button"
          :class="{ active: currentRoute === 'tests' }"
          @click="openTestsMenu">
          Tests
        </div>

        <div
          v-if="testsSuites.length"
          class="suites-section">
          <template v-for="s in testsSuites" :key="s">
            <button
              type="button"
              class="suite-button"
              :class="{ active: s === selectedSuite }"
              @click="selectSuite(s)">
              <span class="suite-dot" :style="{ color: suitePassedMap.get(s) === false ? '#ff5555' : '#50fa7b' }">●</span>
              {{ suiteLabel(s) }}
            </button>
            <div v-if="s === selectedSuite && suiteFunctions.length" class="functions-section">
              <button
                v-for="fn in suiteFunctions" :key="fn.name"
                type="button"
                class="fn-button"
                @click="scrollToTest(fn.name)">
                <span class="fn-dot" :style="{ color: fn.passed ? '#50fa7b' : '#ff5555' }">●</span>
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
      </div>
    </nav>

    <div class="main-content" :class="{ 'viewer-mode': viewerMode }">
      <div class="content-area">
        <router-view></router-view>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ensureTestData } from '../../dataLoader';
import { sections as viewerSections } from '../../viewerSections';

const route = useRoute();
const router = useRouter();

// Viewer sub-sections (Live + each viewer_sections/*.md), chosen here in the sidebar via ?section=.
const activeSection = computed(() => {
  const q = route.query.section;
  return typeof q === 'string' && viewerSections.some((s) => s.id === q) ? q : 'live';
});
const selectSection = (id: string) => {
  router.push({ path: '/viewer', query: id === 'live' ? undefined : { section: id } });
};

const currentRoute = computed(() => {
  const path = route.path;
  if (path.includes('/viewer')) return 'viewer';
  if (path.includes('/tests')) return 'tests';
  if (path.includes('/install')) return 'install';
  return 'viewer';
});

// Viewer is full-bleed (no padding around the iframe).
const viewerMode = computed(() => currentRoute.value === 'viewer');

const testsSuites = ref<string[]>([]);
const selectedSuite = ref('');
const sidebarCollapsed = ref(false);

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
  display: flex;
  flex-direction: row;
  position: relative;
}

.sidebar {
  width: fit-content;
  min-width: 120px;
  background: #000000;
  display: flex;
  flex-direction: column;
  padding: 0;
  transition: width 0.2s;
  position: relative;
  z-index: 100;
  overflow: hidden;
}

.sidebar.collapsed {
  width: 0;
  min-width: 0;
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
  color: #ffffff;
}

.toggle-arrow {
  color: #444444;
  font-size: 21px;
  transition: color 0.2s;
}

.sidebar.collapsed .sidebar-toggle {
  position: fixed;
  left: 0;
  right: auto;
  height: 100vh;
}

.nav-section {
  display: flex;
  flex-direction: column;
  padding-top: 1.8rem;
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
  font-size: 13px;
  font-weight: 300;
  color: #ffffff;
  transition: all 0.2s;
  text-decoration: none;
  text-align: left;
}

.nav-button:hover {
  color: #aaaaaa;
}

.nav-button.active {
  background: #1a1a1a;
  color: #ffffff;
  font-weight: 600;
}

.repo-icons {
  display: flex;
  flex-direction: row;
  gap: 0.25rem;
  padding: 0.5rem 0.5rem;
  flex-wrap: nowrap;
}

.repo-link {
  color: #ffffff;
  text-decoration: none;
  transition: color 0.2s;
  width: 24px;
  height: 24px;
  display: inline-flex;
  justify-content: center;
  align-items: center;
}

.repo-link:hover {
  color: #aaaaaa;
}


.repo-icon {
  width: 20px;
  height: 20px;
  font-size: 20px;
  line-height: 1;
  display: block;
}

svg.repo-icon {
  width: 20px;
  height: 20px;
}

i.repo-icon {
  width: 20px;
  height: 20px;
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
}

img.repo-icon {
  width: 20px;
  height: 20px;
}


.suites-section {
  display: flex;
  flex-direction: column;
  padding-left: 1.5rem;
}

.suite-button {
  padding: 0.2rem 0.75rem;
  background: transparent;
  border: none;
  color: #888888;
  font-family: inherit;
  font-size: 13px;
  font-weight: 300;
  text-transform: capitalize;
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: left;
  display: flex;
  align-items: center;
  gap: 0.35rem;
}

.suite-button:hover {
  color: #ffffff;
}

.suite-button.active {
  background: #1a1a1a;
  color: #ffffff;
  font-weight: 600;
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
  color: #888888;
  font-family: inherit;
  font-size: 13px;
  font-weight: 300;
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: left;
  display: flex;
  align-items: center;
  gap: 0.35rem;
}

.fn-button:hover {
  color: #ffffff;
}

.fn-dot {
  font-size: 8px;
}

.suite-dot {
  font-size: 8px;
  flex-shrink: 0;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: #000000;
  overflow: hidden;
}

.content-area {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 0.25rem 1rem 1rem 1rem;
  box-sizing: border-box;
  background: #000000;
}

/* Viewer: full-bleed, no padding around the iframe. */
.viewer-mode .content-area {
  padding: 0;
  overflow: hidden;
}
</style>
