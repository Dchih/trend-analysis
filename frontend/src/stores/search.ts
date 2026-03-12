import { defineStore } from 'pinia'

import type { KeywordSearchResponse } from '../api/keywords'
import { searchKeyword } from '../api/keywords'

type SearchClient = (keyword: string) => Promise<KeywordSearchResponse>

export const useSearchStore = defineStore('search', {
  state: () => ({
    isSubmitting: false,
    lastResult: null as KeywordSearchResponse | null,
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
        this.lastError = error instanceof Error ? error.message : 'Unknown search error'
        throw error
      } finally {
        this.isSubmitting = false
      }
    },
  },
})
