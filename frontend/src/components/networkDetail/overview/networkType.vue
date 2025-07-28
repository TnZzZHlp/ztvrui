<script setup lang="ts">
import { useRoute } from 'vue-router'
import { networksData } from '../networkDetailStorage'
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'

const route = useRoute()
const { t } = useI18n()

const networkData = computed(() => {
  return networksData.value.find((data) => data.id === (route.params.networkId as string))
})

const changeNetworkType = (e: Event) => {
  const checked = (e.target as HTMLInputElement).checked
  const data = networkData.value
  if (!data) return

  const payload: ControllerNetworkInfo = {
    ...data,
    private: checked,
  }

  createOrUpdateNetwork(data.id as string, payload)
    .then(() => {
      const data = networkData.value
      if (!data) return

      data.private = checked
      showSnackBar(t('common.updateSuccess'), 'success')
    })
    .catch((err) => {
      showSnackBar(t('common.updateFailed') + err, 'error')
    })
}
</script>

<template>
  <!-- Network Type -->
  <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg">
    <p class="text-gray-500">{{ t('network.type.default') }}</p>
    <div class="flex justify-between items-center">
      <span class="text-3xl font-bold">
        {{ networkData?.private ? t('network.type.private') : t('network.type.public') }}
      </span>
      <input type="checkbox" @change="changeNetworkType" :checked="networkData?.private" />
    </div>
  </div>
</template>
