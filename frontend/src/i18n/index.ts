import { createI18n } from 'vue-i18n'
import zh_cn from './zh_cn'
import en from './en'

const i18n = createI18n({
  locale: navigator.language,
  legacy: false,
  globalInjection: true,
  fallbackLocale: 'en',
  messages: {
    'zh-CN': zh_cn,
    en,
  },
})

export default i18n
