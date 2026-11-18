import axios, { type AxiosInstance, type InternalAxiosRequestConfig } from 'axios'
import i18n from '@/i18n'
import router from '@/router'
import { useAuthStore } from '@/stores/auth'
import { showSnackBar } from '@/utils/showSnackBar'

interface RetryableRequestConfig extends InternalAxiosRequestConfig {
  _retry?: boolean
}

let refreshPromise: Promise<void> | null = null

const apiClient: AxiosInstance = axios.create({
  baseURL: '/',
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json',
  },
})

apiClient.interceptors.request.use((config) => {
  const authStore = useAuthStore()

  if (authStore.token) {
    config.headers.Authorization = `Bearer ${authStore.token}`
  }

  return config
})

apiClient.interceptors.response.use(undefined, async (error) => {
  const originalRequest = error.config as RetryableRequestConfig | undefined
  const url = originalRequest?.url ?? ''
  const isAuthEndpoint = url.includes('/api/login') || url.includes('/api/refresh')

  if (
    error.response?.status === 401 &&
    originalRequest &&
    !originalRequest._retry &&
    !isAuthEndpoint
  ) {
    originalRequest._retry = true
    const authStore = useAuthStore()

    if (authStore.token) {
      try {
        if (!refreshPromise) {
          refreshPromise = authStore.refresh().finally(() => {
            refreshPromise = null
          })
        }
        await refreshPromise
        originalRequest.headers.Authorization = `Bearer ${authStore.token}`
        return apiClient(originalRequest)
      } catch (refreshError) {
        authStore.clearAuth()
        showSnackBar(i18n.global.t('auth.tokenExpired'), 'error')
        if (router.currentRoute.value.name !== 'login') {
          await router.push({ name: 'login' })
        }
        return Promise.reject(refreshError)
      }
    }

    authStore.clearAuth()
    if (router.currentRoute.value.name !== 'login') {
      await router.push({ name: 'login' })
    }
  }

  return Promise.reject(error)
})

export default apiClient
