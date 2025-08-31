import type { Auth, LoginResponse, RefreshResponse } from '@/types/manage'
import { tokenManager } from '@/utils/tokenManager'

export async function login(auth: Auth): Promise<LoginResponse> {
  const response = await fetch(`/api/login`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(auth),
  })

  if (!response.ok) {
    throw new Error(`Error logging in: ${response.statusText}`)
  }

  const data: LoginResponse = await response.json()

  // Store token in localStorage
  tokenManager.setToken(data.token, data.expires_at, data.username)

  return data
}

export async function logout(): Promise<void> {
  // Clear local token first
  tokenManager.clearToken()

  try {
    const response = await fetch(`/api/logout`, {
      method: 'POST',
      headers: {
        ...tokenManager.getAuthHeader(),
      },
    })

    if (!response.ok) {
      console.warn(`Logout request failed: ${response.statusText}`)
    }
  } catch (error) {
    console.warn('Logout request failed:', error)
  }
}

export async function refreshToken(): Promise<RefreshResponse> {
  const response = await fetch(`/api/refresh`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...tokenManager.getAuthHeader(),
    },
  })

  if (!response.ok) {
    throw new Error(`Error refreshing token: ${response.statusText}`)
  }

  const data: RefreshResponse = await response.json()

  // Update stored token
  const username = tokenManager.getUsername()
  if (username) {
    tokenManager.setToken(data.token, data.expires_at, username)
  }

  return data
}

export async function editprofile(authInfo: Auth): Promise<void> {
  const response = await fetch(`/api/editprofile`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...tokenManager.getAuthHeader(),
    },
    body: JSON.stringify(authInfo),
  })

  if (!response.ok) {
    throw new Error(`Error editing profile: ${response.statusText}`)
  }
}
