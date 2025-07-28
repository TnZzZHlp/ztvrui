import { createApp } from 'vue'
import modifyInfoPopupPanel from '@/components/networks/modifyInfoPopupPanel.vue'
import i18n from '@/i18n'
export function showModifyInfoPanel() {
  const container = document.createElement('div')
  const app = createApp(modifyInfoPopupPanel, { container })
  document.body.appendChild(container)
  app.use(i18n)
  app.mount(container)
}
