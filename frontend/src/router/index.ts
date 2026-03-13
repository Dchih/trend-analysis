import { createMemoryHistory, createRouter, createWebHistory, type RouterHistory } from 'vue-router'

import KeywordOverviewPage from '../pages/keyword/KeywordOverviewPage.vue'
import SearchPage from '../pages/search/SearchPage.vue'

export function createAppRouter(
  history: RouterHistory = createWebHistory(import.meta.env.BASE_URL),
) {
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
