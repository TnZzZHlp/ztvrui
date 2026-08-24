import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getNetworkById } from '@/api/zerotier/controller'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'
import { eventBus } from '@/utils/eventBus'
import i18n from '@/i18n'

export const useNetworkDetailStore = defineStore('networkDetail', () => {
  // State
  const networksData = ref<ControllerNetworkInfo[]>([])

  // Actions
  function refreshNetworkData(networkId: string) {
    getNetworkById(networkId)
      .then((data) => {
        const index = networksData.value.findIndex((network) => network.id === data.id)
        if (index !== -1) {
          // Update existing network data
          networksData.value[index] = data
        } else {
          // Add new network data
          networksData.value.push(data)
        }
      })
      .catch((error) => {
        showSnackBar(i18n.global.t('failGet') + error, 'error')
      })
  }

  function getNetworkOverviewData(networkId: string) {
    if (networksData.value.some((network) => network.id === networkId)) return
    refreshNetworkData(networkId)
  }

  // Initialize event listener
  function initEventListener() {
    eventBus.addEventListener('controllerNetworkInfoUpdate', (e) => {
      const networkId = e as CustomEvent<string>
      refreshNetworkData(networkId.detail)
    })
  }

  return {
    // State
    networksData,
    // Actions
    refreshNetworkData,
    getNetworkOverviewData,
    initEventListener,
  }
})
