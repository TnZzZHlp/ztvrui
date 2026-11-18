import type {
  Auth,
  LoginResponse,
  RefreshResponse,
  UpdateProfilePayload,
  UpdateProfileResponse,
} from '@/types/manage'
import { useAuthStore } from '@/stores/auth'
import apiClient from '@/utils/axios'

export async function login(auth: Auth): Promise<LoginResponse> {
  const { data } = await apiClient.post<LoginResponse>('/api/login', auth)

  const authStore = useAuthStore()
  authStore.setAuth(data.token, data.expires_at, data.username)

  return data
}

export async function refreshToken(): Promise<RefreshResponse> {
  const authStore = useAuthStore()
  const { data } = await apiClient.post<RefreshResponse>('/api/refresh')

  if (authStore.username) {
    authStore.setAuth(data.token, data.expires_at, authStore.username)
  }

  return data
}

export async function editProfile(authInfo: UpdateProfilePayload): Promise<UpdateProfileResponse> {
  const { data } = await apiClient.post<UpdateProfileResponse>('/api/editprofile', authInfo)

  const authStore = useAuthStore()
  authStore.setAuth(data.token, data.expires_at, data.username)

  return data
}
