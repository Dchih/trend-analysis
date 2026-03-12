import { describe, expect, it } from 'vitest'
import { createMemoryHistory } from 'vue-router'

import { createAppRouter } from './index'

describe('app router', () => {
  it('includes the search page route', () => {
    const router = createAppRouter(createMemoryHistory())
    const route = router.getRoutes().find((item) => item.name === 'search')

    expect(route?.path).toBe('/')
  })

  it('includes the keyword overview route', () => {
    const router = createAppRouter(createMemoryHistory())
    const route = router.getRoutes().find((item) => item.name === 'keyword-overview')

    expect(route?.path).toBe('/keywords/:id')
  })
})
