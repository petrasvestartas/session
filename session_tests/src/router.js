import { createRouter, createWebHistory } from 'vue-router';

import GeneralView from './views/GeneralView.vue';
import TestsView from './views/TestsView.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', redirect: '/viewer' },
    { path: '/viewer', component: GeneralView },
    { path: '/tests', component: TestsView }
  ]
});

export default router;
