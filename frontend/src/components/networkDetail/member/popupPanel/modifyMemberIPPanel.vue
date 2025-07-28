<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import _ from 'lodash'
import type { ControllerNetworkMemberInfo } from '@/types/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'
import { eventBus } from '@/utils/eventBus'
import { createOrUpdateNetworkMember } from '@/api/zerotier/controller'

const { container, memberInfo } = defineProps<{
  container: HTMLDivElement
  memberInfo: ControllerNetworkMemberInfo
}>()

const memberInfoClone = ref<ControllerNetworkMemberInfo>(_.cloneDeep(memberInfo))

const close = ref(false)
const { t } = useI18n()

const handleClick = () => {
  const ipv4Regex =
    /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/

  const ipv6Regex =
    /^([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$|^([0-9a-fA-F]{1,4}:){1,7}:$|^([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}$|^([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}$|^([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}$|^([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}$|^([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}$|^[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){6}|:)$|^:((:[0-9a-fA-F]{1,4}){7}|:)$|^:((:[0-9a-fA-F]{1,4}){6}|:)$/

  const ipAssignments = memberInfoClone.value.ipAssignments

  // Validate IP addresses
  for (const ip of ipAssignments) {
    if (ip && !ipv4Regex.test(ip) && !ipv6Regex.test(ip)) {
      showSnackBar(t('network.ipAssignmentPools.invalidedIP'), 'error')
      return
    }
  }

  const controllerNetworkMemberSettings = {
    ...memberInfoClone.value,
    ipAssignments: ipAssignments,
  }

  // Update member info
  createOrUpdateNetworkMember(
    memberInfoClone.value.nwid,
    memberInfoClone.value.id,
    controllerNetworkMemberSettings,
  )
    .then(() => {
      eventBus.dispatchEvent(new Event('networkMemberChanged'))
      showSnackBar(t('common.updateSuccess'), 'success')
    })
    .catch((error) => {
      showSnackBar(error.message, 'error')
    })
  closePopupPanel()
}

const closePopupPanel = () => {
  close.value = true
  setTimeout(() => {
    container?.remove()
  }, 300)
}
</script>

<template>
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 popup-panel"
    :class="{ 'popup-panel-leave-active': close }"
  >
    <div class="bg-white rounded p-5 flex flex-col justify-between lg:w-1/3">
      <!-- IPs -->
      <div></div>
      <h4 class="font-semibold mb-2">{{ t('network.ip') }}</h4>
      <ul>
        <li
          v-for="(_, index) in memberInfoClone.ipAssignments"
          :key="index"
          class="flex items-center mb-2"
        >
          <div class="flex items-center w-full">
            <input
              type="text"
              v-model="memberInfoClone.ipAssignments[index]"
              class="focus:outline-none border-b-1 border-gray-300 w-full mr-2"
            />
            <button @click="memberInfoClone.ipAssignments.splice(index, 1)" type="button">
              <img src="@/assets/icons/delete.svg" alt="deleteMemberIP" class="w-4 h-4" />
            </button>
          </div>
        </li>
      </ul>

      <!-- Actions -->
      <div class="flex justify-between mt-4">
        <button
          class="mx-2 px-4 py-2 shadow-sm/20 rounded hover:bg-gray-200 transition-all active:bg-gray-300"
          @click="closePopupPanel"
          type="button"
        >
          <b>{{ t('common.cancel') }}</b>
        </button>
        <button
          class="mx-2 px-4 py-2 shadow-sm/20 rounded hover:bg-gray-200 transition-all active:bg-gray-300"
          type="button"
          @click="memberInfoClone.ipAssignments.push('')"
        >
          <b>{{ t('common.add') }}</b>
        </button>
        <button
          class="mx-2 px-4 py-2 shadow-sm/20 rounded hover:bg-gray-200 transition-all active:bg-gray-300"
          @click="handleClick"
          type="submit"
        >
          <b>{{ t('common.confirm') }}</b>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.popup-panel {
  animation: fade-in 0.3s forwards;
}

.popup-panel-leave-active {
  animation: fade-out 0.3s forwards;
}

@keyframes fade-out {
  from {
    opacity: 1;
  }

  to {
    opacity: 0;
  }
}

@keyframes fade-in {
  from {
    opacity: 0;
  }

  to {
    opacity: 1;
  }
}
</style>
