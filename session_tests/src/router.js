import { createRouter, createWebHistory } from 'vue-router';

import GeneralView from './views/GeneralView.vue';
import TestsView from './views/TestsView.vue';

// Use the same base URL as Vite (see vite.config.js: base: '/session/')
// so that routes live under /session and browser refresh works correctly.
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', redirect: '/viewer' },
    { path: '/viewer', component: GeneralView },
    { path: '/tests', component: TestsView }
  ]
});

export default router;
