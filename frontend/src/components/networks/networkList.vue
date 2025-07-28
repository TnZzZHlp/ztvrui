<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { onBeforeMount, ref, type Ref } from 'vue'
import { deleteNetwork, getNetworkById, listNetworkIds } from '@/api/zerotier/controller'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'
import { showConfirmPopupPanel } from '@/utils/showConfirmPopupPanel'
import { eventBus } from '@/utils/eventBus'
import { useRouter } from 'vue-router'
import { copyToClipboard } from '@/utils/copyToClipboard'

const { t } = useI18n()
const router = useRouter()

const networks: Ref<ControllerNetworkInfo[]> = ref([])

const handleDeleteNetwork = (networkId: string) => {
  showConfirmPopupPanel(
    () => {
      // Call the API to delete the network
      deleteNetwork(networkId)
        .then(() => {
          showSnackBar(t('network.delete.success'), 'success')
          // Refresh the list of networks
          refreshNetworks()
        })
        .catch((error) => {
          showSnackBar(t('network.delete.failed') + error, 'error')
        })
    },
    t('network.delete.notice'),
    'warn',
  )
}

const refreshNetworks = () => {
  listNetworkIds()
    .then((data) => {
      networks.value = []
      data.forEach((networkId) => {
        getNetworkById(networkId)
          .then((networkInfo) => {
            networks.value.push(networkInfo)
          })
          .catch((error) => {
            showSnackBar('Error fetching network info:', error)
          })
      })
    })
    .catch((error) => {
      showSnackBar(error, 'error')
    })
}

onBeforeMount(() => {
  eventBus.addEventListener('networkListChanged', refreshNetworks)
  refreshNetworks()
})
</script>

<template>
  <div
    class="shadow w-full rounded min-h-[150px] p-4 flex flex-col justify-between my-2"
    v-for="network in networks"
    :key="network.id"
  >
    <div>
      <p class="font-bold text-xl">{{ network.name }}</p>
      <button
        class="cursor-pointer hover:bg-gray-200 transition-all px-1 mt-1 rounded"
        @click="() => copyToClipboard(network.id!)"
      >
        {{ network.id }}
      </button>
    </div>

    <div class="flex items-center justify-end">
      <button
        class="mx-2 px-4 py-2 shadow-sm/20 rounded bg-red-500 text-white hover:bg-red-600 transition-all active:bg-red-700 font-bold"
        type="button"
        @click="handleDeleteNetwork(network.id!)"
      >
        {{ t('network.delete.default') }}
      </button>
      <button
        class="mx-2 px-4 py-2 shadow-sm/20 rounded hover:bg-gray-200 transition-all active:bg-gray-300 font-bold"
        type="button"
        @click="router.push(`/network/${network.id}/overview`)"
      >
        {{ t('network.enter') }}
      </button>
    </div>
  </div>
</template>
