import type { Auth, LoginResponse, RefreshResponse } from '@/types/manage'
import { useAuthStore } from '@/stores/auth'
import apiClient from '@/utils/axios'

export async function login(auth: Auth): Promise<LoginResponse> {
  const { data } = await apiClient.post<LoginResponse>('/api/login', auth)

  // Store token in auth store
  const authStore = useAuthStore()
  authStore.setAuth(data.token, data.expires_at, data.username)

  return data
}

export async function logout(): Promise<void> {
  const authStore = useAuthStore()

  try {
    await apiClient.post('/api/logout')
  } catch (error) {
    console.warn('Logout request failed:', error)
  } finally {
    // Always clear local auth
    authStore.clearAuth()
  }
}

export async function refreshToken(): Promise<RefreshResponse> {
  const authStore = useAuthStore()
  const { data } = await apiClient.post<RefreshResponse>('/api/refresh')

  // Update stored token
  if (authStore.username) {
    authStore.setAuth(data.token, data.expires_at, authStore.username)
  }

  return data
}

export async function editprofile(authInfo: Auth): Promise<void> {
  await apiClient.post('/api/editprofile', authInfo)
}
