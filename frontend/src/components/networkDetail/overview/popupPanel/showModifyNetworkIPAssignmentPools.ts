import { createApp } from 'vue'
import modifyNetworkIPAssignmentPoolsPanel from '@/components/networkDetail/overview/popupPanel/modifyNetworkIPAssignmentPoolsPanel.vue'
import i18n from '@/i18n'
export function showModifyNetworkIPAssignmentPools(networkId: string) {
  const container = document.createElement('div')
  const app = createApp(modifyNetworkIPAssignmentPoolsPanel, { container, networkId })
  document.body.appendChild(container)
  app.use(i18n)
  app.mount(container)
}
