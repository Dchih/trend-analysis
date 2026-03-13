import { defineStore } from 'pinia'

import type { KeywordSearchResponse, KeywordStatusResponse } from '../api/keywords'
import { fetchKeywordStatus, searchKeyword } from '../api/keywords'
import { messages } from '../locales'

type SearchClient = (keyword: string) => Promise<KeywordSearchResponse>
type StatusClient = (keywordId: number) => Promise<KeywordStatusResponse>

function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

export const useSearchStore = defineStore('search', {
  state: () => ({
    isSubmitting: false,
    isPolling: false,
    lastResult: null as KeywordSearchResponse | null,
    lastStatus: null as KeywordStatusResponse | null,
    lastError: '' as string,
  }),
  actions: {
    async submitKeyword(keyword: string, client: SearchClient = searchKeyword) {
      this.isSubmitting = true
      this.lastError = ''

      try {
        const result = await client(keyword)
        this.lastResult = result
        return result
      } catch (error) {
        this.lastError =
          error instanceof Error ? error.message : messages.search.unknownSearchError
        throw error
      } finally {
        this.isSubmitting = false
      }
    },

    async pollKeywordStatus(
      keywordId: number,
      client: StatusClient = fetchKeywordStatus,
      options: { attempts?: number; intervalMs?: number } = {},
    ) {
      const attempts = options.attempts ?? 12
      const intervalMs = options.intervalMs ?? 1000
      this.isPolling = true
      this.lastError = ''

      try {
        for (let attempt = 0; attempt < attempts; attempt += 1) {
          const status = await client(keywordId)
          this.lastStatus = status

          if (status.status === 'succeeded' || status.status === 'failed') {
            return status
          }

          if (attempt < attempts - 1) {
            await delay(intervalMs)
          }
        }

        throw new Error(messages.search.timeoutError)
      } catch (error) {
        this.lastError =
          error instanceof Error ? error.message : messages.search.unknownPollingError
        throw error
      } finally {
        this.isPolling = false
      }
    },
  },
})
