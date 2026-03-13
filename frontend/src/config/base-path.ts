function ensureLeadingSlash(value: string) {
  return value.startsWith('/') ? value : `/${value}`
}

function ensureTrailingSlash(value: string) {
  return value.endsWith('/') ? value : `${value}/`
}

export function normalizeBasePath(value: string) {
  if (!value || value === '/') {
    return '/'
  }

  return ensureTrailingSlash(ensureLeadingSlash(value))
}

export function buildApiPath(path: string, basePath = import.meta.env.BASE_URL) {
  const normalizedBasePath = normalizeBasePath(basePath)
  const normalizedPath = path.startsWith('/') ? path.slice(1) : path

  return `${normalizedBasePath}${normalizedPath}`
}
