import { createApp } from 'vue'
import confirmPopupPanel from '../components/confirmPopupPanel.vue'
import i18n from '@/i18n'

export function showConfirmPopupPanel(
  onClick: () => void,
  message: string,
  type: 'normal' | 'warn' = 'normal',
) {
  const container = document.createElement('div')
  const app = createApp(confirmPopupPanel, { onClick, message, type, container })
  document.body.appendChild(container)
  app.use(i18n)
  app.mount(container)
}
