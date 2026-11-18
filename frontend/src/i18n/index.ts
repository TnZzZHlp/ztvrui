import { createI18n } from 'vue-i18n'
import zh_cn from './zh_cn'
import en from './en'

export type SupportedLocale = 'zh-CN' | 'en'

const LOCALE_STORAGE_KEY = 'ztvrui_locale'

function resolveInitialLocale(): SupportedLocale {
  const stored = localStorage.getItem(LOCALE_STORAGE_KEY)
  if (stored === 'zh-CN' || stored === 'en') {
    return stored
  }
  return navigator.language.toLowerCase().startsWith('zh') ? 'zh-CN' : 'en'
}

const initialLocale = resolveInitialLocale()
document.documentElement.lang = initialLocale

const i18n = createI18n({
  locale: initialLocale,
  legacy: false,
  globalInjection: true,
  fallbackLocale: 'en',
  messages: {
    'zh-CN': zh_cn,
    en,
  },
})

export function setLocale(locale: SupportedLocale) {
  i18n.global.locale.value = locale
  localStorage.setItem(LOCALE_STORAGE_KEY, locale)
  document.documentElement.lang = locale
}

export default i18n
