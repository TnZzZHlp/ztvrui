<script setup lang="ts">
import { useRoute } from 'vue-router'
import { networksData } from '../networkDetailStorage'
import { computed } from 'vue'
import { showModifyNetworkRules } from './popupPanel/showModifyNetworkRules'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const route = useRoute()
const networkData = computed(() => {
  return networksData.value.find((data) => data.id === (route.params.networkId as string))
})
</script>

<template>
  <!-- Network Rules -->
  <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg">
    <div class="flex justify-between mb-4">
      <span>{{ t('network.rules.default') }}</span>
      <button
        class="transition-all rounded hover:bg-gray-200 active:bg-gray-400"
        @click="showModifyNetworkRules(networkData.id as string)"
      >
        <img src="@/assets/icons/setting.svg" alt="Setting" class="h-6 object-contain" />
      </button>
    </div>
  </div>
</template>
