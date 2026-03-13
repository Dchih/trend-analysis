import { afterEach, describe, expect, it, vi } from 'vitest'

import {
  fetchKeywordOverview,
  fetchKeywordStatus,
  fetchKeywordTimeline,
  fetchLatestContents,
  searchKeyword,
} from './keywords'

describe('keyword api', () => {
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

  it('fetches keyword status', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({
        keyword_id: 9,
        status: 'succeeded',
        last_collected_at: '2026-03-12T10:00:00Z',
      }),
    })
    vi.stubGlobal('fetch', fetchMock)

    const result = await fetchKeywordStatus(9)

    expect(fetchMock).toHaveBeenCalledWith('/api/v1/keywords/9/status', undefined)
    expect(result.status).toBe('succeeded')
  })

  it('fetches overview, timeline, and latest contents', async () => {
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          keyword: 'ninja creami',
          total_contents: 5,
          total_creators: 4,
          total_views: 1200,
          last_collected_at: '2026-03-12T10:00:00Z',
          trend_delta: 0,
        }),
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => [{ date: '2026-03-12', new_content_count: 2, total_views: 1200, active_creator_count: 2 }],
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => [
          {
            content_id: 1,
            title: 'Ninja Creami review',
            thumbnail_url: '',
            published_at: '2026-03-12T10:00:00Z',
            view_count: 1000,
            creator: { creator_id: 1, display_name: 'Kitchen Lab' },
          },
        ],
      })
    vi.stubGlobal('fetch', fetchMock)

    const overview = await fetchKeywordOverview(7)
    const timeline = await fetchKeywordTimeline(7)
    const latest = await fetchLatestContents(7)

    expect(fetchMock).toHaveBeenNthCalledWith(1, '/api/v1/keywords/7/overview?range=30d', undefined)
    expect(fetchMock).toHaveBeenNthCalledWith(2, '/api/v1/keywords/7/timeline?range=30d', undefined)
    expect(fetchMock).toHaveBeenNthCalledWith(
      3,
      '/api/v1/keywords/7/contents/latest?range=30d&limit=10',
      undefined,
    )
    expect(overview.total_contents).toBe(5)
    expect(timeline[0].active_creator_count).toBe(2)
    expect(latest[0].creator.display_name).toBe('Kitchen Lab')
  })
})
