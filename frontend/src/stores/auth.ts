import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { useNow } from '@vueuse/core'
import { refreshToken } from '@/api/manage/auth'

const TOKEN_REFRESH_BUFFER_SECONDS = 86400

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(null)
  const expiresAt = ref<number | null>(null)
  const username = ref<string | null>(null)

  const now = useNow({ interval: 60_000 })
  const nowSeconds = computed(() => Math.floor(now.value.getTime() / 1000))

  const isTokenExpired = computed(() => {
    if (!expiresAt.value) return true
    return nowSeconds.value >= expiresAt.value
  })

  const shouldRefreshToken = computed(() => {
    if (!token.value || !expiresAt.value || isTokenExpired.value) return false
    return nowSeconds.value >= expiresAt.value - TOKEN_REFRESH_BUFFER_SECONDS
  })

  const isAuthenticated = computed(() => !!token.value && !isTokenExpired.value)

  function setAuth(tokenValue: string, expiresAtValue: number, usernameValue: string) {
    token.value = tokenValue
    expiresAt.value = expiresAtValue
    username.value = usernameValue

    localStorage.setItem('jwt_token', tokenValue)
    localStorage.setItem('jwt_expires', expiresAtValue.toString())
    localStorage.setItem('jwt_username', usernameValue)
  }

  function clearAuth() {
    token.value = null
    expiresAt.value = null
    username.value = null

    localStorage.removeItem('jwt_token')
    localStorage.removeItem('jwt_expires')
    localStorage.removeItem('jwt_username')
  }

  function loadFromStorage() {
    const storedToken = localStorage.getItem('jwt_token')
    const storedExpires = localStorage.getItem('jwt_expires')
    const storedUsername = localStorage.getItem('jwt_username')

    if (!storedToken || !storedExpires || !storedUsername) {
      clearAuth()
      return
    }

    const parsedExpires = Number.parseInt(storedExpires, 10)
    if (Number.isNaN(parsedExpires) || Math.floor(Date.now() / 1000) >= parsedExpires) {
      clearAuth()
      return
    }

    token.value = storedToken
    expiresAt.value = parsedExpires
    username.value = storedUsername
  }

  async function refresh() {
    await refreshToken()
  }

  loadFromStorage()

  return {
    token,
    expiresAt,
    username,
    isAuthenticated,
    isTokenExpired,
    shouldRefreshToken,
    setAuth,
    clearAuth,
    loadFromStorage,
    refresh,
  }
})
