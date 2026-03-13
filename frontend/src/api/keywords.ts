import { buildApiPath } from '../config/base-path'

export type KeywordSearchResponse = {
  id: number
  keyword: string
  task_status: string
}

export type KeywordStatusResponse = {
  keyword_id: number
  status: string
  last_collected_at: string | null
}

export type KeywordOverviewResponse = {
  keyword: string
  total_contents: number
  total_creators: number
  total_views: number
  last_collected_at: string | null
  trend_delta: number
}

export type TimelinePoint = {
  date: string
  new_content_count: number
  total_views: number
  active_creator_count: number
}

export type LatestContentItem = {
  content_id: number
  title: string
  thumbnail_url: string
  published_at: string
  view_count: number
  creator: {
    creator_id: number
    display_name: string
  }
}

async function fetchJson<T>(url: string, init?: RequestInit): Promise<T> {
  const response = await fetch(url, init)

  if (!response.ok) {
    throw new Error(`Request failed with status ${response.status}`)
  }

  return response.json() as Promise<T>
}

export function searchKeyword(keyword: string): Promise<KeywordSearchResponse> {
  return fetchJson<KeywordSearchResponse>(buildApiPath('/api/v1/keywords/search'), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ keyword }),
  })
}

export function fetchKeywordStatus(keywordId: number): Promise<KeywordStatusResponse> {
  return fetchJson<KeywordStatusResponse>(buildApiPath(`/api/v1/keywords/${keywordId}/status`))
}

export function fetchKeywordOverview(
  keywordId: number,
  range = '30d',
): Promise<KeywordOverviewResponse> {
  return fetchJson<KeywordOverviewResponse>(
    buildApiPath(`/api/v1/keywords/${keywordId}/overview?range=${range}`),
  )
}

export function fetchKeywordTimeline(keywordId: number, range = '30d'): Promise<TimelinePoint[]> {
  return fetchJson<TimelinePoint[]>(buildApiPath(`/api/v1/keywords/${keywordId}/timeline?range=${range}`))
}

export function fetchLatestContents(
  keywordId: number,
  range = '30d',
  limit = 10,
): Promise<LatestContentItem[]> {
  return fetchJson<LatestContentItem[]>(
    buildApiPath(`/api/v1/keywords/${keywordId}/contents/latest?range=${range}&limit=${limit}`),
  )
}
