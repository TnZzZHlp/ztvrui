import { createApp } from 'vue'
import membermodifyMemberIPPanel from '@/components/networkDetail/member/popupPanel/modifyMemberIPPanel.vue'
import i18n from '@/i18n'
import type { ControllerNetworkMemberInfo } from '@/types/zerotier/controller'
export function showModifyMemberIP(memberInfo: ControllerNetworkMemberInfo) {
  const container = document.createElement('div')
  const app = createApp(membermodifyMemberIPPanel, { container, memberInfo })
  document.body.appendChild(container)
  app.use(i18n)
  app.mount(container)
}
