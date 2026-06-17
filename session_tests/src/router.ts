import { createRouter, createWebHashHistory } from 'vue-router';
import { ensureTestData } from './dataLoader';

// Hash mode for GitHub Pages. Views are lazy (dynamic import) so each route only loads when that
// tab is opened, not in the main bundle.
const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/tests' },
    { path: '/tests', component: () => import('./views/TestsView.vue'),
      beforeEnter: async () => { await ensureTestData(); } },
    { path: '/install', component: () => import('./views/InstallView.vue') },
  ],
});

export default router;
