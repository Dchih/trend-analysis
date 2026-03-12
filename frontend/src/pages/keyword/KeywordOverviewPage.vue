<script setup lang="ts">
const cards = [
  { label: 'Captured videos', value: '24' },
  { label: 'Creators mapped', value: '9' },
  { label: 'Views tracked', value: '215K' },
]
</script>

<template>
  <main class="page-shell page-shell--overview">
    <section class="overview-header">
      <div>
        <p class="eyebrow">Keyword Overview</p>
        <h1 class="overview-title">ninja creami</h1>
      </div>
      <div class="range-pills">
        <span class="range-pill range-pill--active">30d</span>
        <span class="range-pill">7d</span>
        <span class="range-pill">90d</span>
      </div>
    </section>

    <section class="stats-grid">
      <article v-for="card in cards" :key="card.label" class="stat-card">
        <p class="stat-label">{{ card.label }}</p>
        <p class="stat-value">{{ card.value }}</p>
      </article>
    </section>

    <section class="content-grid">
      <article class="chart-panel">
        <p class="panel-label">Signal timeline</p>
        <div class="panel-placeholder">Timeline chart placeholder</div>
      </article>

      <article class="list-panel">
        <p class="panel-label">Latest videos</p>
        <div class="video-item">
          <strong>Ninja Creami recipes worth trying</strong>
          <span>Kitchen Lab / 32K views</span>
        </div>
        <div class="video-item">
          <strong>Hausgemachtes Softeis? Ninja CREAMi im Test</strong>
          <span>German review / live ingestion target</span>
        </div>
      </article>
    </section>
  </main>
</template>
