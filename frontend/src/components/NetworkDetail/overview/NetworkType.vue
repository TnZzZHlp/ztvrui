<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useCurrentNetwork } from '@/composables/useCurrentNetwork'
import { Switch } from '@/components/ui/switch'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'

const { t } = useI18n()
const { networkData, networkDetailStore } = useCurrentNetwork()

const changeNetworkType = async (checked: boolean) => {
  const data = networkData.value
  if (!data?.id) return

  try {
    const updated = await createOrUpdateNetwork(data.id, { private: checked })
    networkDetailStore.setNetworkData(updated)
    showSnackBar(t('common.updateSuccess'), 'success')
  } catch (err) {
    showSnackBar(`${t('common.updateFailed')}: ${getErrorMessage(err)}`, 'error')
  }
}
</script>

<template>
  <div v-if="networkData" class="rounded-xl border bg-card p-5 text-card-foreground shadow-xs">
    <p class="text-sm font-medium text-muted-foreground">{{ t('network.type.default') }}</p>
    <div class="mt-2 flex items-center justify-between gap-2">
      <span class="text-2xl font-bold tracking-tight">
        {{ networkData.private ? t('network.type.private') : t('network.type.public') }}
      </span>
      <Switch :model-value="Boolean(networkData.private)" @update:model-value="changeNetworkType" />
    </div>
  </div>
</template>
