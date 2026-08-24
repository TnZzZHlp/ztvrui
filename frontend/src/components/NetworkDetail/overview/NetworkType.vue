<script setup lang="ts">
import { useCurrentNetwork } from '@/composables/useCurrentNetwork'
import { Switch } from '@/components/ui/switch'

import { useI18n } from 'vue-i18n'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'

const { t } = useI18n()
const { networkData, networkDetailStore } = useCurrentNetwork()

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
      showSnackBar(t('common.updateSuccess'), 'success')
      networkDetailStore.refreshNetworkData(data.id as string)
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
      <Switch :model-value="networkData?.private"
        @update:model-value="(checked: boolean) => changeNetworkType({ target: { checked } } as any)" />
    </div>
  </div>
</template>
