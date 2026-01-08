<template>
  <div class="main-layout">
    <nav class="sidebar" :class="{ collapsed: sidebarCollapsed }">
      <button class="sidebar-toggle" @click="sidebarCollapsed = !sidebarCollapsed">
        <span class="toggle-arrow">{{ sidebarCollapsed ? '○' : '●' }}</span>
      </button>

      <div v-if="!sidebarCollapsed" class="nav-section">
        <div class="repo-icons">
          <a href="https://github.com/petrasvestartas/session" target="_blank" class="repo-link" :class="{ 'build-failed': buildStatus.session === 'failure', 'build-success': buildStatus.session === 'success', 'build-in-progress': buildStatus.session === 'in_progress' }" title="Session">
            <svg class="repo-icon" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
            </svg>
          </a>
          <a href="https://github.com/petrasvestartas/session_cpp" target="_blank" class="repo-link" :class="{ 'build-failed': buildStatus.session_cpp === 'failure', 'build-success': buildStatus.session_cpp === 'success', 'build-in-progress': buildStatus.session_cpp === 'in_progress' }" title="C++">
            <img src="/icons/session_cpp_white.png" class="repo-icon" alt="C++">
          </a>
          <a href="https://github.com/petrasvestartas/session_py" target="_blank" class="repo-link" :class="{ 'build-failed': buildStatus.session_py === 'failure', 'build-success': buildStatus.session_py === 'success', 'build-in-progress': buildStatus.session_py === 'in_progress' }" title="Python">
            <img src="/icons/session_py_white.png" class="repo-icon" alt="Python">
          </a>
          <a href="https://github.com/petrasvestartas/session_rust" target="_blank" class="repo-link" :class="{ 'build-failed': buildStatus.session_rust === 'failure', 'build-success': buildStatus.session_rust === 'success', 'build-in-progress': buildStatus.session_rust === 'in_progress' }" title="Rust">
            <img src="/icons/session_rust_white.png" class="repo-icon" alt="Rust">
          </a>
          <a href="https://github.com/petrasvestartas/session_proto" target="_blank" class="repo-link" title="Protobuf">
            <img src="/icons/session_proto_white.png" class="repo-icon" alt="Protobuf">
          </a>
          <a href="https://github.com/petrasvestartas/session_data" target="_blank" class="repo-link" title="Data">
            <img src="/icons/session_data_white.png" class="repo-icon" alt="Data">
          </a>
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
const sidebarCollapsed = ref(false);

const buildStatus = ref({
  session: null,
  session_cpp: null,
  session_py: null,
  session_rust: null
});

const fetchBuildStatus = async (repo) => {
  try {
    const response = await fetch(`https://api.github.com/repos/petrasvestartas/${repo}/actions/runs?per_page=1`);
    if (!response.ok) return null;
    const data = await response.json();
    if (data.workflow_runs && data.workflow_runs.length > 0) {
      const run = data.workflow_runs[0];
      if (run.status === 'in_progress' || run.status === 'queued') {
        return 'in_progress';
      }
      return run.conclusion;
    }
    return null;
  } catch (e) {
    return null;
  }
};

const loadBuildStatuses = async () => {
  const repos = ['session', 'session_cpp', 'session_py', 'session_rust'];
  for (const repo of repos) {
    const status = await fetchBuildStatus(repo);
    buildStatus.value[repo.replace('-', '_')] = status;
  }
};

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

let buildStatusInterval = null;

onMounted(() => {
  loadSuitesFromTestData();
  syncSelectedSuiteWithRoute();
  loadBuildStatuses();
  // Poll build statuses every 30 seconds to catch in-progress builds
  buildStatusInterval = setInterval(loadBuildStatuses, 30000);
});

watch(
  () => route.query.suite,
  () => {
    syncSelectedSuiteWithRoute();
  }
);


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
  if (buildStatusInterval) {
    clearInterval(buildStatusInterval);
  }
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
  background: #000000;
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

.repo-link.build-failed {
  color: #ff5555;
}

.repo-link.build-failed:hover {
  color: #ff8888;
}

.repo-link.build-success {
  color: #55ff55;
}

.repo-link.build-success:hover {
  color: #88ff88;
}

.repo-link.build-in-progress {
  color: #ffaa00;
}

.repo-link.build-in-progress .repo-icon {
  animation: spin 2s linear infinite;
}

.repo-link.build-in-progress:hover {
  color: #ffcc55;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
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

.repo-link.build-failed img.repo-icon {
  filter: brightness(0) saturate(100%) invert(35%) sepia(100%) saturate(2000%) hue-rotate(340deg);
}

.repo-link.build-success img.repo-icon {
  filter: brightness(0) saturate(100%) invert(60%) sepia(100%) saturate(500%) hue-rotate(80deg);
}

.repo-link.build-in-progress img.repo-icon {
  filter: brightness(0) saturate(100%) invert(70%) sepia(100%) saturate(1000%) hue-rotate(0deg);
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
