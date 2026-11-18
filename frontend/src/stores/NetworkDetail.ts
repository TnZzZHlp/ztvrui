import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getNetworkById } from '@/api/zerotier/controller'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'
import { eventBus } from '@/utils/eventBus'
import i18n from '@/i18n'

export const useNetworkDetailStore = defineStore('networkDetail', () => {
  const networksData = ref<ControllerNetworkInfo[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  function setNetworkData(data: ControllerNetworkInfo) {
    const index = networksData.value.findIndex((network) => network.id === data.id)
    if (index !== -1) {
      networksData.value[index] = data
    } else {
      networksData.value.push(data)
    }
  }

  async function refreshNetworkData(networkId: string): Promise<ControllerNetworkInfo | undefined> {
    loading.value = true
    error.value = null
    try {
      const data = await getNetworkById(networkId)
      setNetworkData(data)
      return data
    } catch (err) {
      const msg = getErrorMessage(err)
      error.value = msg
      showSnackBar(`${i18n.global.t('network.detail.failGet')}: ${msg}`, 'error')
      return undefined
    } finally {
      loading.value = false
    }
  }

  function getNetworkOverviewData(networkId: string) {
    if (networksData.value.some((network) => network.id === networkId)) return
    void refreshNetworkData(networkId)
  }

  function initEventListener() {
    eventBus.addEventListener('controllerNetworkInfoUpdate', (e) => {
      const networkId = (e as CustomEvent<string>).detail
      if (networkId) {
        void refreshNetworkData(networkId)
      }
    })
  }

  return {
    networksData,
    loading,
    error,
    setNetworkData,
    refreshNetworkData,
    getNetworkOverviewData,
    initEventListener,
  }
})
