import { createPinia, setActivePinia } from 'pinia'
import { describe, expect, it, vi } from 'vitest'
import { createMemoryHistory } from 'vue-router'

import { createAppRouter } from '../router'
import { useSearchStore } from './search'

describe('search flow', () => {
  it('submits a keyword, polls status, and navigates to overview', async () => {
    setActivePinia(createPinia())

    const store = useSearchStore()
    const router = createAppRouter(createMemoryHistory())
    await router.push('/')
    await router.isReady()

    const searchClient = vi.fn().mockResolvedValue({
      id: 15,
      keyword: 'ninja creami',
      task_status: 'pending',
    })
    const statusClient = vi.fn().mockResolvedValue({
      keyword_id: 15,
      status: 'succeeded',
      last_collected_at: '2026-03-12T10:00:00Z',
    })

    const result = await store.submitKeyword('ninja creami', searchClient)
    const status = await store.pollKeywordStatus(result.id, statusClient, {
      attempts: 1,
      intervalMs: 0,
    })
    await router.push(`/keywords/${result.id}`)

    expect(searchClient).toHaveBeenCalledWith('ninja creami')
    expect(statusClient).toHaveBeenCalledWith(15)
    expect(status.status).toBe('succeeded')
    expect(router.currentRoute.value.fullPath).toBe('/keywords/15')
  })
})
