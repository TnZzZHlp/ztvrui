<script setup lang="ts">
import { onBeforeMount, ref } from 'vue'
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
const networkData = ref()

const close = ref(false)
const { t } = useI18n()

const handleClick = () => {
  // Validate IP range with regex
  const dns = networkData.value?.dns

  // Validate CIDR network target and IP next hop
  const ipRegex = /^(25[0-5]|2[0-4]\d|[01]?\d?\d)(\.(25[0-5]|2[0-4]\d|[01]?\d?\d)){3}$/

  const isValid = dns.servers.every((dns: string) => {
    return ipRegex.test(dns)
  })

  if (!isValid) {
    showSnackBar(t('network.ipAssignmentPools.invalidedIP'), 'error')
    return
  }

  // Submit changes
  createOrUpdateNetwork(networkData.value?.id as string, {
    dns: networkData.value?.dns,
    ...networkData.value,
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

onBeforeMount(() => {
  // Find the network data by ID
  networkData.value = _.cloneDeep(networksData.value.find((data) => data.id === networkId))

  if (!networkData.value) {
    showSnackBar(t('network.notFound'), 'error')
    closePopupPanel()
  }

  if (!networkData.value?.dns) {
    networkData.value.dns = {
      domain: '',
      servers: [],
    }
  }

  if (Array.isArray(networkData.value?.dns)) {
    networkData.value.dns = {
      domain: '',
      servers: [],
    }
  }
})
</script>

<template>
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 popup-panel"
    :class="{ 'popup-panel-leave-active': close }"
  >
    <div class="bg-white rounded p-5 flex flex-col justify-between lg:w-1/3">
      <!-- DNS -->
      <div class="flex flex-col mb-1">
        <span class="text-gray-500">{{ t('network.dns.domain') }}</span>
        <input
          type="text"
          class="border-b-1 focus:outline-none"
          v-model="networkData.dns!.domain"
        />
      </div>

      <span class="text-gray-500">{{ t('network.dns.servers') }}</span>
      <div class="flex w-full" v-for="(_, index) in networkData.dns!.servers" :key="index">
        <input
          type="text"
          class="border-b-1 focus:outline-none w-full"
          v-model="networkData.dns!.servers[index]"
          autocomplete="off"
        />
        <img
          src="@/assets/icons/delete.svg"
          alt="delete DNS server"
          class="w-4 h-4 cursor-pointer ml-2"
          @click="networkData?.dns!.servers.splice(index, 1)"
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
          @click="networkData?.dns!.servers.push('')"
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
