import { createRouter, createWebHashHistory } from 'vue-router';
import { ensureTestData } from './dataLoader';

// The launch URL historically carried the suite as a pre-hash query (?suite=X). With hash
// routing that param is invisible to the app and goes stale as the user navigates, leaving two
// disagreeing suite params in the address bar. Fold it into the hash route once at startup:
// hash suite wins when both exist, otherwise the legacy param is honored and removed.
const legacyParams = new URLSearchParams(window.location.search);
const legacySuite = legacyParams.get('suite');
if (legacySuite) {
  legacyParams.delete('suite');
  const rest = legacyParams.toString();
  const base = window.location.pathname + (rest ? `?${rest}` : '');
  const hash = window.location.hash.includes('suite=')
    ? window.location.hash
    : `#/tests?suite=${encodeURIComponent(legacySuite)}`;
  window.history.replaceState(null, '', base + hash);
}

// Hash mode for GitHub Pages. Views are lazy (dynamic import) so each route only loads when that
// tab is opened, not in the main bundle. A heading anchor rides after the route:
// #/course/12-picking#step-1; the content pane scrolls to it (CourseView), not the window.
const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/tests' },
    { path: '/course', component: () => import('./views/CourseIndex.vue') },
    { path: '/course/:slug(.*)', component: () => import('./views/CourseView.vue') },
    { path: '/tests', component: () => import('./views/TestsView.vue'),
      beforeEnter: async () => { await ensureTestData(); } },
    { path: '/install', component: () => import('./views/InstallView.vue') },
    { path: '/:rest(.*)', redirect: '/tests' },
  ],
});

export default router;
