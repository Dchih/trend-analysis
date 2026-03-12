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
})
