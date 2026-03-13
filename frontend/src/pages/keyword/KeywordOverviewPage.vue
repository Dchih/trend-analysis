<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'

import type {
  KeywordOverviewResponse,
  LatestContentItem,
  TimelinePoint,
} from '../../api/keywords'
import { fetchKeywordOverview, fetchKeywordTimeline, fetchLatestContents } from '../../api/keywords'
import { useLocaleText } from '../../locales'
import { useSearchStore } from '../../stores/search'

const route = useRoute()
const searchStore = useSearchStore()
const text = useLocaleText()
const range = ref<'7d' | '30d' | '90d'>('30d')
const isLoading = ref(true)
const overview = ref<KeywordOverviewResponse | null>(null)
const timeline = ref<TimelinePoint[]>([])
const latestVideos = ref<LatestContentItem[]>([])
const errorMessage = ref('')

const keywordId = computed(() => Number(route.params.id))
const cards = computed(() => {
  if (!overview.value) {
    return []
  }

  return [
    { label: text.overview.cards.capturedVideos, value: overview.value.total_contents.toLocaleString() },
    { label: text.overview.cards.creatorsMapped, value: overview.value.total_creators.toLocaleString() },
    { label: text.overview.cards.viewsTracked, value: overview.value.total_views.toLocaleString() },
  ]
})

const timelineSummary = computed(() => {
  if (timeline.value.length === 0) {
    return text.overview.timelineEmpty
  }

  const latestPoint = timeline.value[timeline.value.length - 1]
  return `${latestPoint.date} / ${latestPoint.new_content_count} ${text.overview.timelineNewVideos} / ${latestPoint.total_views.toLocaleString()} ${text.overview.timelineViews}`
})

async function loadOverviewData() {
  const [overviewResponse, timelineResponse, latestContentResponse] = await Promise.all([
    fetchKeywordOverview(keywordId.value, range.value),
    fetchKeywordTimeline(keywordId.value, range.value),
    fetchLatestContents(keywordId.value, range.value, 8),
  ])

  overview.value = overviewResponse
  timeline.value = timelineResponse
  latestVideos.value = latestContentResponse
}

async function refresh() {
  isLoading.value = true
  errorMessage.value = ''

  try {
    const status = await searchStore.pollKeywordStatus(keywordId.value, undefined, {
      attempts: 6,
      intervalMs: 1000,
    })

    if (status.status === 'failed') {
      throw new Error(text.overview.failedCollection)
    }

    await loadOverviewData()
  } catch (error) {
    errorMessage.value =
      error instanceof Error ? error.message : text.overview.failedOverview
  } finally {
    isLoading.value = false
  }
}

function setRange(nextRange: '7d' | '30d' | '90d') {
  range.value = nextRange
  void refresh()
}

onMounted(() => {
  void refresh()
})
</script>

<template>
  <main class="page-shell page-shell--overview">
    <section class="overview-header">
      <div>
        <p class="eyebrow">{{ text.overview.eyebrow }}</p>
        <h1 class="overview-title">{{ overview?.keyword ?? `${text.overview.fallbackKeyword} #${keywordId}` }}</h1>
      </div>
      <div class="range-pills">
        <button
          v-for="option in ['7d', '30d', '90d']"
          :key="option"
          class="range-pill"
          :class="{ 'range-pill--active': range === option }"
          type="button"
          @click="setRange(option as '7d' | '30d' | '90d')"
        >
          {{ option }}
        </button>
      </div>
    </section>

    <section v-if="isLoading" class="content-grid">
      <article class="chart-panel">
        <p class="panel-label">{{ text.overview.loadingLabel }}</p>
        <div class="panel-placeholder">{{ text.overview.loadingStatus }}</div>
      </article>
    </section>

    <section v-else-if="errorMessage" class="content-grid">
      <article class="chart-panel">
        <p class="panel-label">{{ text.overview.loadFailed }}</p>
        <div class="panel-placeholder">{{ errorMessage }}</div>
      </article>
    </section>

    <template v-else>
      <section class="stats-grid">
        <article v-for="card in cards" :key="card.label" class="stat-card">
          <p class="stat-label">{{ card.label }}</p>
          <p class="stat-value">{{ card.value }}</p>
        </article>
      </section>

      <section class="content-grid">
        <article class="chart-panel">
          <p class="panel-label">{{ text.overview.timeline }}</p>
          <div class="panel-placeholder">{{ timelineSummary }}</div>
        </article>

        <article class="list-panel">
          <p class="panel-label">{{ text.overview.latestVideos }}</p>
          <div v-if="latestVideos.length === 0" class="video-item">
            <strong>{{ text.overview.emptyVideosTitle }}</strong>
            <span>{{ text.overview.emptyVideosDescription }}</span>
          </div>
          <div v-for="item in latestVideos" :key="item.content_id" class="video-item">
            <strong>{{ item.title }}</strong>
            <span>{{ item.creator.display_name }} / {{ item.view_count.toLocaleString() }} views</span>
          </div>
        </article>
      </section>
    </template>
  </main>
</template>
