<template>
  <div class="main-layout">
    <nav class="tab-nav">
      <router-link 
        :to="'/viewer'"
        class="tab-button"
        active-class="active">
        Viewer
      </router-link>

      <div 
        class="tab-button tab-button-tests" 
        :class="{ active: currentRoute === 'tests' }"
        @click="openTestsMenu">
        <span>Tests</span>
        <span class="tests-tab-caret">▾</span>
      </div>

      <router-link 
        :to="'/install'"
        class="tab-button"
        active-class="active">
        Install
      </router-link>
    </nav>

    <div
      v-if="testsMenuOpen && testsSuites.length"
      class="tests-suite-bar"
      @click.stop>
      <button
        v-for="s in testsSuites"
        :key="s"
        type="button"
        class="tests-suite-button tab-button"
        :class="{ active: s === selectedSuite }"
        @click="selectSuite(s)">
        {{ suiteLabel(s) }}
      </button>
    </div>

    <div class="tab-content">
      <!-- CLI Interface (top panel for all tabs, resizable) -->
      <CliInterface
        :active-tab="currentRoute"
        :style="{ flex: '0 0 ' + cliHeight + 'px' }"
      ></CliInterface>

      <!-- Drag handle between CLI and page content -->
      <div class="cli-resizer" @mousedown="startResize"></div>

      <!-- Main page content below -->
      <div class="content-area">
        <router-view></router-view>
      </div>
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

// CLI height in pixels (resizable)
const cliHeight = ref(325); // start slightly taller so answers area is higher
const minCliHeight = 160;
const maxCliHeight = 500;
let startY = 0;
let startHeight = 0;

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
  const delta = event.clientY - startY;
  let next = startHeight + delta;
  if (next < minCliHeight) next = minCliHeight;
  if (next > maxCliHeight) next = maxCliHeight;
  cliHeight.value = next;
};

const stopResize = () => {
  window.removeEventListener('mousemove', onMouseMove);
  window.removeEventListener('mouseup', stopResize);
};

const startResize = (event) => {
  event.preventDefault();
  startY = event.clientY;
  startHeight = cliHeight.value;
  window.addEventListener('mousemove', onMouseMove);
  window.addEventListener('mouseup', stopResize);
};

onUnmounted(() => {
  stopResize();
});
</script>

<style scoped>
.main-layout {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.tab-nav {
  display: flex;
  background: #2563eb;
  border-bottom: none;
  padding: 0 1rem;
}

.tab-button {
  padding: 1rem 1.5rem;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.8);
  transition: all 0.2s;
  text-decoration: none;
}

.tab-button-tests {
  position: relative;
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.tab-button:hover {
  color: #ffffff;
  background: rgba(255, 255, 255, 0.08);
}

.tab-button.active {
  color: #ffffff;
  background: rgba(255, 255, 255, 0.12); /* lighter, like submenu */
}

.tests-tab-caret {
  font-size: 10px;
}

.tests-suite-bar {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-start;
  gap: 0;
  padding: 0;
  background: #2563eb;
  border-bottom: 1px solid rgba(255, 255, 255, 0.12);
}

.tests-suite-button {
  flex: 0 1 160px;
  height: 44px;
  margin: 0;
  padding: 0;
  background: transparent;
  border: none;
  border-radius: 0;
  color: rgba(255, 255, 255, 0.85);
  font: inherit;
  font-weight: 600;
  text-transform: capitalize;
  cursor: pointer;
  transition: background 0.2s ease, color 0.2s ease;
}

.tests-suite-button:hover {
  color: #ffffff;
  background: rgba(255, 255, 255, 0.08);
}

.tests-suite-button.active {
  color: #ffffff;
  background: rgba(255, 255, 255, 0.15);
}

.tab-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: #fff;
  overflow: hidden;
}

.content-area {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
  box-sizing: border-box;
}

.cli-resizer {
  flex: 0 0 4px;           /* 4px hit area */
  cursor: row-resize;
  background: linear-gradient(
    to bottom,
    #f9fafb 0%,   /* match chat background on top half */
    #f9fafb 50%,
    #ffffff 50%,  /* match page background on bottom half */
    #ffffff 100%
  );
  position: relative;
}

.cli-resizer:hover {
  background: linear-gradient(
    to bottom,
    #f9fafb 0%,
    #f9fafb 50%,
    #ffffff 50%,
    #ffffff 100%
  );
}

.cli-resizer::before {
  content: '';
  position: absolute;
  left: 0;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  height: 1px;                   /* 1px visible line */
  background: #e5e7eb;           /* same grey as other borders */
}
</style>
