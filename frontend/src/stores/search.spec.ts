import { createPinia, setActivePinia } from 'pinia'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { useSearchStore } from './search'

describe('search store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('submits the keyword and stores the latest response', async () => {
    const store = useSearchStore()
    const client = vi.fn().mockResolvedValue({
      id: 11,
      keyword: 'ninja creami',
      task_status: 'pending',
    })

    await store.submitKeyword('ninja creami', client)

    expect(client).toHaveBeenCalledWith('ninja creami')
    expect(store.lastResult?.id).toBe(11)
    expect(store.isSubmitting).toBe(false)
  })

  it('polls until the collector succeeds', async () => {
    const store = useSearchStore()
    const statusClient = vi
      .fn()
      .mockResolvedValueOnce({
        keyword_id: 11,
        status: 'pending',
        last_collected_at: null,
      })
      .mockResolvedValueOnce({
        keyword_id: 11,
        status: 'running',
        last_collected_at: null,
      })
      .mockResolvedValueOnce({
        keyword_id: 11,
        status: 'succeeded',
        last_collected_at: '2026-03-12T10:00:00Z',
      })

    const status = await store.pollKeywordStatus(11, statusClient, {
      attempts: 3,
      intervalMs: 0,
    })

    expect(statusClient).toHaveBeenCalledTimes(3)
    expect(status.status).toBe('succeeded')
    expect(store.lastStatus?.last_collected_at).toBe('2026-03-12T10:00:00Z')
    expect(store.isPolling).toBe(false)
  })
})
