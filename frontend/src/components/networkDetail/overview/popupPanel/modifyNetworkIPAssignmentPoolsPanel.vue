<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { networksData } from '@/components/networkDetail/networkDetailStorage'
import _ from 'lodash'
import { showSnackBar } from '@/utils/showSnackBar'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import { eventBus } from '@/utils/eventBus'

const { container, networkId } = defineProps({
  container: HTMLDivElement,
  networkId: String,
})
const networkData = ref(
  _.cloneDeep(networksData.value.find((data) => data.id === (networkId as string))),
)

const close = ref(false)
const { t } = useI18n()

const handleClick = () => {
  // Validate IP range with regex
  const ipAssignmentPools = networkData.value?.ipAssignmentPools || []

  const ipRegex = /^(25[0-5]|2[0-4]\d|[01]?\d?\d)(\.(25[0-5]|2[0-4]\d|[01]?\d?\d)){3}$/

  const isValid = ipAssignmentPools.every((pool) => {
    return ipRegex.test(pool.ipRangeStart) && ipRegex.test(pool.ipRangeEnd)
  })

  if (!isValid) {
    showSnackBar(t('invalidedIP'), 'error')
    return
  }

  // Submit changes
  createOrUpdateNetwork(networkData.value?.id as string, {
    name: networkData.value?.name as string,
    ...networkData.value,
    ipAssignmentPools: networkData.value?.ipAssignmentPools,
  })
    .then(() => {
      // Update the network data in the storage
      const event = new CustomEvent('controllerNetworkInfoUpdate', {
        detail: networkData.value?.id,
      })
      eventBus.dispatchEvent(event)
      showSnackBar(t('common.updateSuccess'), 'success')
    })
    .catch((err) => {
      showSnackBar(t('common.updateFailed') + err, 'error')
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
      <!-- Routes -->
      <div class="flex mb-2">
        <span class="w-full">{{ t('network.ipAssignmentPools.ipRangeStart') }}</span>
        <span class="w-full">{{ t('network.ipAssignmentPools.ipRangeEnd') }}</span>
        <span class="w-4"></span>
      </div>
      <div
        class="flex mb-2"
        v-for="(ipAssignment, index) in networkData?.ipAssignmentPools"
        :key="index"
      >
        <input
          type="text"
          :placeholder="t('network.ipAssignmentPools.ipRangeStart')"
          class="focus:outline-none border-b w-full mr-1"
          name="ipRangeStart"
          v-model="ipAssignment.ipRangeStart"
          autocomplete="off"
        />
        <input
          type="text"
          :placeholder="t('network.ipAssignmentPools.ipRangeEnd')"
          class="focus:outline-none border-b w-full"
          name="ipRangeEnd"
          v-model="ipAssignment.ipRangeEnd"
          autocomplete="off"
        />
        <img
          src="@/assets/icons/delete.svg"
          alt="delete IP assignment pool"
          class="w-4 cursor-pointer"
          @click="networkData?.ipAssignmentPools!.splice(index, 1)"
        />
      </div>

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
          @click="
            networkData?.ipAssignmentPools?.push({
              ipRangeStart: '',
              ipRangeEnd: '',
            })
          "
          type="button"
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
