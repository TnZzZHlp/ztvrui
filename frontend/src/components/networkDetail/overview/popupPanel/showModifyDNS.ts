import { createApp } from 'vue'
import modifyNetworkDNSPanel from '@/components/networkDetail/overview/popupPanel/modifyNetworkDNSPanel.vue'
import i18n from '@/i18n'
export function showModifyDNS(networkId: string) {
  const container = document.createElement('div')
  const app = createApp(modifyNetworkDNSPanel, { container, networkId })
  document.body.appendChild(container)
  app.use(i18n)
  app.mount(container)
}
