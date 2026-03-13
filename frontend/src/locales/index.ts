export const messages = {
  search: {
    eyebrow: 'ProductRadar / P0',
    title: '从关键词看清产品与内容生态',
    description: '输入一个关键词，触发 YouTube 采集、达人聚合与时间线分析。',
    keywordLabel: '关键词',
    keywordPlaceholder: '例如：ninja creami',
    submitIdle: '开始分析',
    submitLoading: '分析中...',
    trendingSeeds: '热门种子词',
    currentScope: '当前能力范围',
    currentScopeDescription: 'YouTube 采集、达人聚合、时间线概览。',
    unknownSearchError: '搜索请求失败，请稍后重试。',
    unknownPollingError: '状态轮询失败，请稍后重试。',
    timeoutError: '采集任务超时，请稍后刷新重试。',
  },
  overview: {
    eyebrow: '关键词概览',
    title: '关键词概览',
    fallbackKeyword: '关键词',
    cards: {
      capturedVideos: '采集视频数',
      creatorsMapped: '关联创作者数',
      viewsTracked: '累计观看量',
    },
    loadingLabel: '采集状态',
    loadingStatus: '正在等待采集完成...',
    loadFailed: '加载失败',
    timeline: '信号时间线',
    timelineEmpty: '当前时间范围内还没有时间线数据。',
    timelineNewVideos: '新增视频',
    timelineViews: '浏览量',
    latestVideos: '最新视频',
    emptyVideosTitle: '暂时没有采集到视频',
    emptyVideosDescription: '请先发起搜索，并等待采集任务完成。',
    failedCollection: '采集任务执行失败，请稍后重试。',
    failedOverview: '关键词概览加载失败，请稍后重试。',
  },
} as const

export function useLocaleText() {
  return messages
}
