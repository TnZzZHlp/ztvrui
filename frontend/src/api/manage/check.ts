import type { AuthCheckResponse } from '@/types/manage'
import { tokenManager } from '@/utils/tokenManager'

export async function check(): Promise<AuthCheckResponse> {
  // First check if we have a valid token locally
  if (tokenManager.isTokenExpired()) {
    tokenManager.clearToken()
    throw new Error('Token expired')
  }

  const response = await fetch(`/api/check`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
      ...tokenManager.getAuthHeader(),
    },
  })

  if (!response.ok) {
    // Clear token if server says it's invalid
    tokenManager.clearToken()
    throw new Error(`Error checking auth status: ${response.statusText}`)
  }

  const data: AuthCheckResponse = await response.json()

  // Update token data if response includes new info
  const username = tokenManager.getUsername()
  if (username && data.expires_at) {
    const token = tokenManager.getToken()
    if (token) {
      tokenManager.setToken(token, data.expires_at, data.username)
    }
  }

  return data
}
