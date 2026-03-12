import { createMemoryHistory, createRouter, createWebHistory, type RouterHistory } from 'vue-router'

import KeywordOverviewPage from '../pages/keyword/KeywordOverviewPage.vue'
import SearchPage from '../pages/search/SearchPage.vue'

export function createAppRouter(history: RouterHistory = createWebHistory()) {
  return createRouter({
    history,
    routes: [
      {
        path: '/',
        name: 'search',
        component: SearchPage,
      },
      {
        path: '/keywords/:id',
        name: 'keyword-overview',
        component: KeywordOverviewPage,
        props: true,
      },
    ],
  })
}
