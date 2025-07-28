import { createApp } from 'vue'
import addNetworkPopupPanel from '@/components/networks/addNetworkPopupPanel.vue'
import i18n from '@/i18n'
export function showAddNetworkPanel() {
  const container = document.createElement('div')
  const app = createApp(addNetworkPopupPanel, { container })
  document.body.appendChild(container)
  app.use(i18n)
  app.mount(container)
}
