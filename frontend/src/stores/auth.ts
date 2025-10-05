import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

interface TokenData {
  token: string
  expiresAt: number
  username: string
}

export const useAuthStore = defineStore('auth', () => {
  // State
  const token = ref<string | null>(null)
  const expiresAt = ref<number | null>(null)
  const username = ref<string | null>(null)

  // Computed
  const isAuthenticated = computed(() => !!token.value && !isTokenExpired.value)

  const isTokenExpired = computed(() => {
    if (!expiresAt.value) return true
    // Add a day (86400 seconds) buffer before expiration
    const now = Math.floor(Date.now() / 1000)
    return now >= expiresAt.value - 86400
  })

  // Actions
  function setAuth(tokenValue: string, expiresAtValue: number, usernameValue: string) {
    token.value = tokenValue
    expiresAt.value = expiresAtValue
    username.value = usernameValue

    // Persist to localStorage
    localStorage.setItem('jwt_token', tokenValue)
    localStorage.setItem('jwt_expires', expiresAtValue.toString())
    localStorage.setItem('jwt_username', usernameValue)
  }

  function clearAuth() {
    token.value = null
    expiresAt.value = null
    username.value = null

    // Clear localStorage
    localStorage.removeItem('jwt_token')
    localStorage.removeItem('jwt_expires')
    localStorage.removeItem('jwt_username')
  }

  function loadFromStorage() {
    const storedToken = localStorage.getItem('jwt_token')
    const storedExpires = localStorage.getItem('jwt_expires')
    const storedUsername = localStorage.getItem('jwt_username')

    if (storedToken && storedExpires && storedUsername) {
      token.value = storedToken
      expiresAt.value = parseInt(storedExpires)
      username.value = storedUsername
    }
  }

  function getAuthHeader(): Record<string, string> {
    return token.value ? { Authorization: `Bearer ${token.value}` } : {}
  }

  async function refresh() {
    const { refreshToken } = await import('@/api/manage/auth')
    await refreshToken()
  }

  loadFromStorage()

  return {
    // State
    token,
    expiresAt,
    username,
    // Computed
    isAuthenticated,
    isTokenExpired,
    // Actions
    setAuth,
    clearAuth,
    loadFromStorage,
    getAuthHeader,
    refresh,
  }
})
