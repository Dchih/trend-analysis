import { afterEach, describe, expect, it, vi } from 'vitest'

import { searchKeyword } from './keywords'

describe('searchKeyword', () => {
  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('posts the keyword to the search endpoint', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({
        id: 9,
        keyword: 'ninja creami',
        task_status: 'pending',
      }),
    })
    vi.stubGlobal('fetch', fetchMock)

    const result = await searchKeyword('ninja creami')

    expect(fetchMock).toHaveBeenCalledWith(
      '/api/v1/keywords/search',
      expect.objectContaining({
        method: 'POST',
      }),
    )
    expect(result.keyword).toBe('ninja creami')
  })
})
