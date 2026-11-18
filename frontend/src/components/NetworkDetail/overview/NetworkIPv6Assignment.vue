<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useCurrentNetwork } from '@/composables/useCurrentNetwork'
import { Switch } from '@/components/ui/switch'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import type { V6AssignMode } from '@/types/zerotier/controller'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'

const { t } = useI18n()
const { networkData, networkDetailStore } = useCurrentNetwork()

const ipv6Modes = ['6plane', 'rfc4193', 'zt'] as const
type IPv6ModeKey = (typeof ipv6Modes)[number]

const changeIPv6Assignment = async (checked: boolean, mode: IPv6ModeKey) => {
  const data = networkData.value
  if (!data?.id) return

  const current: V6AssignMode = data.v6AssignMode ?? {
    '6plane': false,
    rfc4193: false,
    zt: false,
  }

  try {
    const updated = await createOrUpdateNetwork(data.id, {
      v6AssignMode: {
        '6plane': Boolean(current['6plane']),
        rfc4193: Boolean(current.rfc4193),
        zt: Boolean(current.zt),
        [mode]: checked,
      },
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
    <p class="text-sm font-medium text-muted-foreground">{{ t('network.ipv6Assignment.default') }}</p>
    <div class="mt-3 space-y-2.5">
      <div v-for="mode in ipv6Modes" :key="mode" class="flex items-center justify-between gap-2">
        <span class="text-base font-semibold">
          {{ t(`network.ipv6Assignment.${mode}`) }}
        </span>
        <Switch
          :model-value="Boolean(networkData.v6AssignMode?.[mode])"
          @update:model-value="(checked: boolean) => changeIPv6Assignment(checked, mode)"
        />
      </div>
    </div>
  </div>
</template>
