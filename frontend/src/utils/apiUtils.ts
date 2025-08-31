import { tokenManager } from './tokenManager'
import { refreshToken } from '@/api/manage/auth'

// Create a fetch wrapper that automatically includes auth headers and handles token refresh
export async function authenticatedFetch(
  url: string,
  options: RequestInit = {},
): Promise<Response> {
  // Check if token is expired and try to refresh
  if (tokenManager.isTokenExpired()) {
    try {
      await refreshToken()
    } catch {
      // If refresh fails, clear token and throw error
      tokenManager.clearToken()
      throw new Error('Authentication failed')
    }
  }

  // Add auth headers to the request
  const authHeaders = tokenManager.getAuthHeader()
  const headers = {
    'Content-Type': 'application/json',
    ...authHeaders,
    ...options.headers,
  }

  // Make the request
  const response = await fetch(url, {
    ...options,
    headers,
  })

  // If we get a 401 and we have a token, try to refresh once
  if (response.status === 401 && tokenManager.getToken()) {
    try {
      await refreshToken()

      // Retry the original request with new token
      const newHeaders = {
        'Content-Type': 'application/json',
        ...tokenManager.getAuthHeader(),
        ...options.headers,
      }

      return fetch(url, {
        ...options,
        headers: newHeaders,
      })
    } catch {
      // If refresh fails, clear token
      tokenManager.clearToken()
      throw new Error('Authentication failed')
    }
  }

  return response
}

// Helper function for making authenticated API calls
export async function apiCall<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
  const response = await authenticatedFetch(endpoint, options)

  if (!response.ok) {
    throw new Error(`API call failed: ${response.statusText}`)
  }

  return response.json()
}
