import { createRouter, createWebHashHistory } from 'vue-router';

import GeneralView from './views/GeneralView.vue';
import TestsView from './views/TestsView.vue';

// Use hash mode for GitHub Pages compatibility
// URLs: /session/#/viewer, /session/#/tests
// This allows F5 refresh to work since the server always serves index.html
const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/viewer' },
    { path: '/viewer', component: GeneralView },
    { path: '/tests', component: TestsView }
  ]
});

export default router;
