import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useNetworkDetailStore } from '@/stores/NetworkDetail'

export function useCurrentNetwork() {
  const route = useRoute()
  const networkDetailStore = useNetworkDetailStore()
  const networkData = computed(() =>
    networkDetailStore.networksData.find((data) => data.id === (route.params.networkId as string)),
  )

  return { networkData, networkDetailStore }
}
