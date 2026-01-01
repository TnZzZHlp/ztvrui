import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export type Theme = 'light' | 'dark'

export const useThemeStore = defineStore('theme', () => {
  const theme = ref<Theme>('light')

  function loadTheme() {
    const stored = localStorage.getItem('theme') as Theme | null
    if (stored) {
      theme.value = stored
    } else if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      theme.value = 'dark'
    }
    applyTheme()
  }

  function setTheme(newTheme: Theme) {
    theme.value = newTheme
    localStorage.setItem('theme', newTheme)
    applyTheme()
  }

  function toggleTheme() {
    setTheme(theme.value === 'light' ? 'dark' : 'light')
  }

  function applyTheme() {
    const root = document.documentElement
    if (theme.value === 'dark') {
      root.classList.add('dark')
    } else {
      root.classList.remove('dark')
    }
  }

  // Watch for system preference changes when no stored preference
  function handleSystemChange(e: MediaQueryListEvent) {
    if (!localStorage.getItem('theme')) {
      theme.value = e.matches ? 'dark' : 'light'
      applyTheme()
    }
  }

  // Listen for system preference changes
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  mediaQuery.addEventListener('change', handleSystemChange)

  loadTheme()

  return {
    theme,
    setTheme,
    toggleTheme,
    loadTheme,
  }
})
