import axios, { type AxiosInstance, type InternalAxiosRequestConfig } from 'axios'
import { useAuthStore } from '@/stores/auth'

// Create an Axios instance
const apiClient: AxiosInstance = axios.create({
  baseURL: '/',
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json',
  },
})

// Request Interceptor
apiClient.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    const authStore = useAuthStore()

    // Add authentication token
    if (authStore.token) {
      config.headers.Authorization = `Bearer ${authStore.token}`
    }

    return config
  },
  (error) => {
    return Promise.reject(error)
  },
)

// Response Interceptor
apiClient.interceptors.response.use(
  (response) => {
    return response
  },
  async (error) => {
    const originalRequest = error.config

    // If 401 error and not retried yet
    if (error.response?.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true

      const authStore = useAuthStore()

      // Try refreshing the token
      if (authStore.token) {
        try {
          await authStore.refresh()
          // Retry the original request with a new token.
          originalRequest.headers.Authorization = `Bearer ${authStore.token}`
          return apiClient(originalRequest)
        } catch (refreshError) {
          // Refresh failed, clear token and redirect to login page.
          authStore.clearAuth()
          return Promise.reject(refreshError)
        }
      }
    }

    return Promise.reject(error)
  },
)

export default apiClient
