import type { AuthCheckResponse } from '@/types/manage'
import { useAuthStore } from '@/stores/auth'
import apiClient from '@/utils/axios'

export async function check(): Promise<AuthCheckResponse> {
  const authStore = useAuthStore()

  // First check if we have a valid token locally
  if (authStore.isTokenExpired) {
    authStore.clearAuth()
    throw new Error('Token expired')
  }

  try {
    const { data } = await apiClient.get<AuthCheckResponse>('/api/check')

    // Update token data if response includes new info
    if (authStore.username && data.expires_at) {
      const token = authStore.token
      if (token) {
        authStore.setAuth(token, data.expires_at, data.username)
      }
    }

    return data
  } catch (error) {
    // Clear token if server says it's invalid
    authStore.clearAuth()
    throw error
  }
}
