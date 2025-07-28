<script setup lang="ts">
import { useRoute } from 'vue-router'
import { networksData } from '../networkDetailStorage'
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { copyToClipboard } from '@/utils/copyToClipboard'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'

const route = useRoute()
const { t } = useI18n()

const networkData = computed(() => {
  return networksData.value.find((data) => data.id === (route.params.networkId as string))
})

const changeNetworkName = (e: Event) => {
  const newName = (e.target as HTMLInputElement).value
  const data = networkData.value
  if (!data) return

  const payload: ControllerNetworkInfo = {
    ...data,
    name: newName,
  }

  createOrUpdateNetwork(data.id as string, payload)
    .then(() => {
      showSnackBar(t('common.updateSuccess'), 'success')
    })
    .catch((err) => {
      showSnackBar(t('common.updateFailed') + err, 'error')
    })
}
</script>

<template>
  <!-- Network Name and Id -->
  <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg">
    <p class="text-gray-500">{{ t('network.name') }}</p>
    <input
      class="text-3xl font-bold border-b-1 block w-full focus:outline-none"
      type="text"
      placeholder="Network Name"
      name="Network Name"
      @change="changeNetworkName"
      :value="networkData?.name"
      autocomplete="off"
    />
    <p class="text-gray-500 mt-2">{{ t('network.id') }}</p>
    <button
      class="hover:bg-gray-100 active:bg-gray-200 px-2 py-1 rounded text-gray-700 transition-all"
      @click="copyToClipboard(networkData?.id as string)"
    >
      {{ networkData?.id }}
    </button>
  </div>
</template>
