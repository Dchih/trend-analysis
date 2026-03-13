<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'

import { useLocaleText } from '../../locales'
import { useSearchStore } from '../../stores/search'

const trendingKeywords = ['ninja creami', 'cold brew maker', 'protein blender']
const keyword = ref('')
const router = useRouter()
const searchStore = useSearchStore()
const text = useLocaleText()

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
      <p class="eyebrow">{{ text.search.eyebrow }}</p>
      <h1 class="hero-title">{{ text.search.title }}</h1>
      <p class="hero-copy">{{ text.search.description }}</p>

      <form class="search-form" @submit.prevent="submitSearch">
        <label class="search-label" for="keyword">{{ text.search.keywordLabel }}</label>
        <div class="search-row">
          <input
            id="keyword"
            v-model="keyword"
            class="search-input"
            type="text"
            :placeholder="text.search.keywordPlaceholder"
          />
          <button class="search-button" type="submit" :disabled="searchStore.isSubmitting">
            {{ searchStore.isSubmitting ? text.search.submitLoading : text.search.submitIdle }}
          </button>
        </div>
        <p v-if="searchStore.lastError" class="search-error">{{ searchStore.lastError }}</p>
      </form>
    </section>

    <section class="signal-grid">
      <article class="signal-card">
        <p class="signal-label">{{ text.search.trendingSeeds }}</p>
        <ul class="keyword-list">
          <li v-for="keyword in trendingKeywords" :key="keyword">{{ keyword }}</li>
        </ul>
      </article>

      <article class="signal-card signal-card--accent">
        <p class="signal-label">{{ text.search.currentScope }}</p>
        <p class="signal-copy">{{ text.search.currentScopeDescription }}</p>
      </article>
    </section>
  </main>
</template>
