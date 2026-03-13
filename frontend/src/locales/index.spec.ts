import { describe, expect, it } from 'vitest'

import { messages } from './index'

describe('zh-CN locale messages', () => {
  it('provides chinese labels for the search page', () => {
    expect(messages.search.title).toBe('从关键词看清产品与内容生态')
    expect(messages.search.submitIdle).toBe('开始分析')
    expect(messages.search.trendingSeeds).toBe('热门种子词')
  })

  it('provides chinese labels for the overview page', () => {
    expect(messages.overview.title).toBe('关键词概览')
    expect(messages.overview.cards.capturedVideos).toBe('采集视频数')
    expect(messages.overview.loadingStatus).toBe('正在等待采集完成...')
  })
})
