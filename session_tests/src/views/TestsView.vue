<template>
  <div class="tests-view">
    <p v-if="about" class="class-about"><strong>{{ about.label }}</strong> {{ about.description }}</p>
    <div class="tests-body">
      <TestViewer
        :tests="tests"
        :active-suite="activeSuite"
        @update:active-suite="activeSuite = $event">
      </TestViewer>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue';
import { useRoute } from 'vue-router';
import kernel from 'virtual:kernel';
import TestViewer from '../components/TestViewer.vue';

const route = useRoute();

// One line about the class, from its C++ docstring (plugins/kernel.ts).
const about = computed(() => kernel[activeSuite.value]);

// Deep link #/tests?suite=<class>&test=<name>: scroll that test's row into view once it renders,
// and again after the highlighter has settled the row heights.
const scrollToTest = () => {
  const t = route.query.test;
  if (typeof t !== 'string') return;
  const go = () => document.getElementById('test-' + t)?.scrollIntoView({ block: 'start' });
  nextTick(go);
  setTimeout(go, 400);
};

const activeSuite = ref('point_test');
const tests = ref<any[]>([]);
const suites = ref<string[]>([]);

const loadTests = () => {
  if (typeof (window as any).TEST_DATA === 'undefined') {
    console.warn('TEST_DATA not found. Run: bash minitest.sh');
    (window as any).TEST_DATA = {};
    return;
  }

  const all: any[] = [];
  const data = (window as any).TEST_DATA;

  for (const [key, testArray] of Object.entries(data)) {
    if (!Array.isArray(testArray)) continue;

    const parts = key.split('_');
    const language = parts.pop();
    const suite = parts.join('_');

    testArray.forEach((t: any, idx: number) => {
      all.push({
        id: `${key}:${t.test_name}:${idx}`,
        suite,
        language,
        ...t
      });
    });
  }

  tests.value = all;

  const set = new Set<string>();
  for (const t of all) {
    if (t.suite) set.add(t.suite);
  }
  suites.value = Array.from(set.values());
  
  // Initialize activeSuite from route query or default to first suite
  const q = route.query.suite;
  const qSuite = typeof q === 'string' ? q : '';
  if (qSuite && suites.value.includes(qSuite)) {
    activeSuite.value = qSuite;
  } else if (suites.value.length) {
    activeSuite.value = suites.value[0];
  }

  scrollToTest();
};

onMounted(() => {
  setTimeout(loadTests, 100);
});

watch(() => route.query.test, scrollToTest);

watch(
  () => route.query.suite,
  (newSuite) => {
    const s = typeof newSuite === 'string' ? newSuite : '';
    if (s && suites.value.includes(s)) {
      activeSuite.value = s;
    }
  }
);
</script>

<style scoped>
.tests-view {
  padding: 1rem 0 1.5rem;
  height: 100%;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  background: #ffffff;
}

.class-about {
  margin: 0 0 0.75rem;
  padding: 0 0.75rem;
  color: var(--muted);
  font-size: 14px;
}

.class-about strong {
  color: var(--fg);
  font-weight: 600;
  margin-right: 0.4rem;
}

.tests-body {
  flex: 1;
  min-height: 0;
}
</style>
