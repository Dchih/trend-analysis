import { describe, expect, it } from 'vitest'

import { buildApiPath, normalizeBasePath } from './base-path'

describe('base path helpers', () => {
  it('normalizes a prefixed deployment path', () => {
    expect(normalizeBasePath('/product-radar')).toBe('/product-radar/')
    expect(normalizeBasePath('product-radar')).toBe('/product-radar/')
  })

  it('builds api paths under a configured base path', () => {
    expect(buildApiPath('/api/v1/keywords/search', '/product-radar/')).toBe(
      '/product-radar/api/v1/keywords/search',
    )
  })
})
