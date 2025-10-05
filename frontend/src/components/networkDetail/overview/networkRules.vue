<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useNetworkDetailStore } from '@/stores/networkDetail'
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import ModifyRulesDialog from './ModifyRulesDialog.vue'

const networkDetailStore = useNetworkDetailStore()
const { t } = useI18n()

const route = useRoute()
const networkData = computed(() => {
  return networkDetailStore.networksData.find((data) => data.id === (route.params.networkId as string))
})

// Dialog state
const showDialog = ref(false)
const selectedNetworkId = ref<string | null>(null)

const openDialog = () => {
  selectedNetworkId.value = networkData.value?.id || null
  showDialog.value = true
}
</script>

<template>
  <!-- Network Rules -->
  <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg">
    <div class="flex justify-between mb-4">
      <span>{{ t('network.rules.default') }}</span>
      <button class="transition-all rounded hover:bg-gray-200 active:bg-gray-400" @click="openDialog">
        <img src="@/assets/icons/setting.svg" alt="Setting" class="h-6 object-contain" />
      </button>
    </div>

    <!-- Dialog -->
    <ModifyRulesDialog :open="showDialog" :network-id="selectedNetworkId" @update:open="showDialog = $event" />
  </div>
</template>
