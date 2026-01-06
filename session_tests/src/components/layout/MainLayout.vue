<template>
  <div class="main-layout">
    <nav class="sidebar" :class="{ collapsed: sidebarCollapsed }">
      <button class="sidebar-toggle" @click="sidebarCollapsed = !sidebarCollapsed">
        <span class="toggle-arrow">{{ sidebarCollapsed ? '○' : '●' }}</span>
      </button>

      <div v-if="!sidebarCollapsed" class="nav-section">
        <a
          href="https://github.com/petrasvestartas/session"
          target="_blank"
          class="nav-button github-link"
          title="GitHub">
          <svg class="github-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
          </svg>
        </a>

        <div
          class="nav-button"
          :class="{ active: currentRoute === 'tests' }"
          @click="openTestsMenu">
          Tests
        </div>

        <div
          v-if="testsMenuOpen && testsSuites.length"
          class="suites-section">
          <button
            v-for="s in testsSuites"
            :key="s"
            type="button"
            class="suite-button"
            :class="{ active: s === selectedSuite }"
            @click="selectSuite(s)">
            {{ suiteLabel(s) }}
          </button>
        </div>

        <router-link
          :to="'/install'"
          class="nav-button"
          active-class="active">
          Install
        </router-link>

        <router-link
          :to="'/viewer'"
          class="nav-button"
          active-class="active">
          Viewer
        </router-link>
      </div>
    </nav>

    <div class="main-content" :class="{ 'cli-is-expanded': cliExpanded }">
      <div class="content-area">
        <router-view></router-view>
      </div>

      <div
        class="cli-resizer"
        :class="{ expanded: cliExpanded }"
        @mousedown="startResize"
        @click="toggleCliExpand">
        <span class="resizer-arrow">{{ cliExpanded ? '●' : '○' }}</span>
      </div>

      <CliInterface
        :active-tab="currentRoute"
        :class="{ 'cli-expanded': cliExpanded }"
        :style="cliExpanded ? {} : { flex: '0 0 ' + cliHeight + 'px' }"
      ></CliInterface>
    </div>
  </div>
</template>

<script setup>
import { computed, ref, onMounted, watch, onUnmounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import CliInterface from '../CliInterface.vue';

const route = useRoute();
const router = useRouter();

const currentRoute = computed(() => {
  const path = route.path;
  if (path.includes('/viewer')) return 'viewer';
  if (path.includes('/tests')) return 'tests';
  if (path.includes('/install')) return 'install';
  return 'viewer';
});

const testsSuites = ref([]);
const selectedSuite = ref('');
const testsMenuOpen = ref(false);
const sidebarCollapsed = ref(false);

// CLI height in pixels (resizable)
const cliHeight = ref(200);
const minCliHeight = 50;
const maxCliHeight = 10000;
const cliExpanded = ref(false);
let previousCliHeight = 200;
let startY = 0;
let startHeight = 0;
let didDrag = false;

const loadSuitesFromTestData = () => {
  if (typeof window === 'undefined' || typeof window.TEST_DATA === 'undefined') return;

  const set = new Set();
  const data = window.TEST_DATA;
  for (const [key, testArray] of Object.entries(data)) {
    if (!Array.isArray(testArray)) continue;
    const parts = key.split('_');
    parts.pop(); // remove language
    const suite = parts.join('_');
    if (suite) set.add(suite);
  }
  testsSuites.value = Array.from(set.values());
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
  if (!testsSuites.value.length) {
    loadSuitesFromTestData();
    syncSelectedSuiteWithRoute();
  }
  testsMenuOpen.value = true;
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

const suiteLabel = (suite) => {
  return suite.replace(/_test$/i, '');
};

const selectSuite = (suite) => {
  selectedSuite.value = suite;
  router.push({ path: '/tests', query: { suite } });
};

onMounted(() => {
  loadSuitesFromTestData();
  syncSelectedSuiteWithRoute();
});

watch(
  () => route.query.suite,
  () => {
    syncSelectedSuiteWithRoute();
  }
);

watch(currentRoute, (val) => {
  if (val === 'tests' && testsSuites.value.length) {
    testsMenuOpen.value = true;
  } else {
    testsMenuOpen.value = false;
  }
});

const onMouseMove = (event) => {
  didDrag = true;
  const delta = startY - event.clientY;
  let next = startHeight + delta;
  if (next < minCliHeight) next = minCliHeight;
  if (next > maxCliHeight) next = maxCliHeight;
  cliHeight.value = next;
  cliExpanded.value = false;
};

const stopResize = () => {
  window.removeEventListener('mousemove', onMouseMove);
  window.removeEventListener('mouseup', stopResize);
};

const startResize = (event) => {
  event.preventDefault();
  didDrag = false;
  startY = event.clientY;
  startHeight = cliHeight.value;
  window.addEventListener('mousemove', onMouseMove);
  window.addEventListener('mouseup', stopResize);
};

const toggleCliExpand = () => {
  if (didDrag) return;
  if (cliExpanded.value) {
    cliHeight.value = previousCliHeight;
    cliExpanded.value = false;
  } else {
    previousCliHeight = cliHeight.value;
    cliHeight.value = window.innerHeight - 32;
    cliExpanded.value = true;
  }
};

onUnmounted(() => {
  stopResize();
});
</script>

<style scoped>
.main-layout {
  height: 100vh;
  display: flex;
  flex-direction: row;
  position: relative;
}

.sidebar {
  width: 180px;
  background: linear-gradient(to right, #0a0a0a, #000000);
  display: flex;
  flex-direction: column;
  padding: 0;
  transition: width 0.2s;
  position: relative;
  z-index: 100;
}

.sidebar.collapsed {
  width: 0;
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
}

.nav-button {
  padding: 0.5rem 0.75rem;
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 21px;
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

.github-link {
  color: #ffffff;
  display: flex;
  align-items: center;
}

.github-link:hover {
  color: #aaaaaa;
}

.github-icon {
  width: 21px;
  height: 21px;
}

.suites-section {
  display: flex;
  flex-direction: column;
  padding-left: 1.5rem;
}

.suite-button {
  padding: 0.25rem 0.75rem;
  background: transparent;
  border: none;
  color: #ffffff;
  font-family: inherit;
  font-size: 20px;
  font-weight: 300;
  text-transform: capitalize;
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: left;
}

.suite-button:hover {
  color: #aaaaaa;
}

.suite-button.active {
  background: #1a1a1a;
  color: #ffffff;
  font-weight: 600;
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

.cli-resizer {
  flex: 0 0 20px;
  cursor: row-resize;
  background: transparent;
  display: flex;
  align-items: center;
  justify-content: center;
}

.cli-resizer:hover .resizer-arrow {
  color: #ffffff;
}

.resizer-arrow {
  color: #444444;
  font-size: 21px;
  transition: color 0.2s;
  pointer-events: none;
}

.cli-expanded {
  flex: 1 !important;
}

.cli-is-expanded .content-area {
  flex: 0 0 0 !important;
  min-height: 0 !important;
  padding: 0 !important;
  overflow: hidden !important;
}
</style>
