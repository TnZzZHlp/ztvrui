<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useCurrentNetwork } from '@/composables/useCurrentNetwork'
import { Switch } from '@/components/ui/switch'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'

const { t } = useI18n()
const { networkData, networkDetailStore } = useCurrentNetwork()

const changeIPv4Assignment = async (checked: boolean) => {
  const data = networkData.value
  if (!data?.id) return

  try {
    const updated = await createOrUpdateNetwork(data.id, {
      v4AssignMode: { zt: checked },
    })
    networkDetailStore.setNetworkData(updated)
    showSnackBar(t('common.updateSuccess'), 'success')
  } catch (err) {
    showSnackBar(`${t('common.updateFailed')}: ${getErrorMessage(err)}`, 'error')
  }
}
</script>

<template>
  <div v-if="networkData" class="rounded-xl border bg-card p-5 text-card-foreground shadow-xs">
    <p class="text-sm font-medium text-muted-foreground">{{ t('network.ipv4Assignment.default') }}</p>
    <div class="mt-3 flex items-center justify-between gap-2">
      <span class="text-base font-semibold">
        {{ t('network.ipv4Assignment.zt') }}
      </span>
      <Switch
        :model-value="Boolean(networkData.v4AssignMode?.zt)"
        @update:model-value="changeIPv4Assignment"
      />
    </div>
  </div>
</template>
