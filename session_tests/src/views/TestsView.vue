<template>
  <div class="tests-view">
    <TestViewer 
      :tests="tests"
      :active-suite="activeSuite"
      @update:active-suite="activeSuite = $event">
    </TestViewer>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { useRoute } from 'vue-router';
import TestViewer from '../components/TestViewer.vue';

const route = useRoute();

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

  if (all.length > 0) {
    console.log(`✅ Loaded ${all.length} tests across ${suites.value.length} suites`);
  }
};

onMounted(() => {
  setTimeout(loadTests, 100);
});

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
  padding: 1.5rem 0;
  height: 100%;
  box-sizing: border-box;
  background: #000000;
}
</style>
