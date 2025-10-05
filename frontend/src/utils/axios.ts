import axios, { type AxiosInstance, type InternalAxiosRequestConfig } from 'axios'
import { useAuthStore } from '@/stores/auth'

// 创建 axios 实例
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

      // 尝试刷新 token
      if (authStore.token) {
        try {
          await authStore.refresh()
          // 使用新 token 重试原始请求
          originalRequest.headers.Authorization = `Bearer ${authStore.token}`
          return apiClient(originalRequest)
        } catch (refreshError) {
          // 刷新失败，清除 token 并跳转到登录页
          authStore.clearAuth()
          return Promise.reject(refreshError)
        }
      }
    }

    return Promise.reject(error)
  },
)

export default apiClient
