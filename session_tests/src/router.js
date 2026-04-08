import { createRouter, createWebHashHistory } from 'vue-router';

import GeneralView from './views/GeneralView.vue';
import TestsView from './views/TestsView.vue';
import InstallView from './views/InstallView.vue';
import ClassesView from './views/ClassesView.vue';

// Use hash mode for GitHub Pages compatibility
// URLs: /session/#/viewer, /session/#/tests, /session/#/install, /session/#/classes
// This allows F5 refresh to work since the server always serves index.html
const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/viewer' },
    { path: '/viewer', component: GeneralView },
    { path: '/tests', component: TestsView },
    { path: '/classes', component: ClassesView },
    { path: '/install', component: InstallView }
  ]
});

export default router;
