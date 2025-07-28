<script setup lang="ts">
import { useRoute } from 'vue-router'
import { networksData } from '../networkDetailStorage'
import { computed } from 'vue'
import { showModifyDNS } from './popupPanel/showModifyDNS'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const route = useRoute()
const networkData = computed(() => {
  return networksData.value.find((data) => data.id === (route.params.networkId as string))
})
</script>

<template>
  <!-- Network DNS -->
  <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg">
    <div class="flex justify-between">
      <span class="text-gray-500">DNS</span>
      <button
        @click="showModifyDNS(networkData?.id as string)"
        class="transition-all rounded hover:bg-gray-200 active:bg-gray-400"
      >
        <img src="@/assets/icons/setting.svg" alt="Setting" class="h-6 object-contain" />
      </button>
    </div>
    <span class="text-gray-500">{{ t('network.dns.domain') }}</span>
    <h1 class="text-xl font-bold overflow-hidden">{{ networkData?.dns?.domain }}</h1>
    <span class="text-gray-500">{{ t('network.dns.servers') }}</span>
    <ol>
      <li v-for="(server, index) in networkData?.dns?.servers" :key="index">{{ server }}</li>
    </ol>
  </div>
</template>
