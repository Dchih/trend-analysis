<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'

import { useSearchStore } from '../../stores/search'

const trendingKeywords = ['ninja creami', 'cold brew maker', 'protein blender']
const keyword = ref('')
const router = useRouter()
const searchStore = useSearchStore()

async function submitSearch() {
  if (!keyword.value.trim()) {
    return
  }

  const result = await searchStore.submitKeyword(keyword.value.trim())
  await router.push(`/keywords/${result.id}`)
}
</script>

<template>
  <main class="page-shell page-shell--search">
    <section class="hero-panel">
      <p class="eyebrow">ProductRadar / P0</p>
      <h1 class="hero-title">Trace a product from keyword to creator gravity.</h1>
      <p class="hero-copy">
        Search starts the ingestion flow. This first cut is wired for YouTube-led
        product intelligence.
      </p>

      <form class="search-form" @submit.prevent="submitSearch">
        <label class="search-label" for="keyword">Keyword</label>
        <div class="search-row">
          <input
            id="keyword"
            v-model="keyword"
            class="search-input"
            type="text"
            placeholder="ninja creami"
          />
          <button class="search-button" type="submit" :disabled="searchStore.isSubmitting">
            {{ searchStore.isSubmitting ? 'Scanning...' : 'Start scan' }}
          </button>
        </div>
        <p v-if="searchStore.lastError" class="search-error">{{ searchStore.lastError }}</p>
      </form>
    </section>

    <section class="signal-grid">
      <article class="signal-card">
        <p class="signal-label">Trending seeds</p>
        <ul class="keyword-list">
          <li v-for="keyword in trendingKeywords" :key="keyword">{{ keyword }}</li>
        </ul>
      </article>

      <article class="signal-card signal-card--accent">
        <p class="signal-label">Current scope</p>
        <p class="signal-copy">YouTube ingestion, creator aggregation, timeline overview.</p>
      </article>
    </section>
  </main>
</template>
