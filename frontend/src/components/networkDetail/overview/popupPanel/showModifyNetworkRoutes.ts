import { createApp } from 'vue'
import modifyNetworkRoutesPanel from '@/components/networkDetail/overview/popupPanel/modifyNetworkRoutesPanel.vue'
import i18n from '@/i18n'
export function showModifyNetworkRoutes(networkId: string) {
  const container = document.createElement('div')
  const app = createApp(modifyNetworkRoutesPanel, { container, networkId })
  document.body.appendChild(container)
  app.use(i18n)
  app.mount(container)
}
