export type KeywordSearchResponse = {
  id: number
  keyword: string
  task_status: string
}

export async function searchKeyword(keyword: string): Promise<KeywordSearchResponse> {
  const response = await fetch('/api/v1/keywords/search', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ keyword }),
  })

  if (!response.ok) {
    throw new Error(`Search request failed with status ${response.status}`)
  }

  return response.json() as Promise<KeywordSearchResponse>
}
