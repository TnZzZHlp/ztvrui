<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useCurrentNetwork } from '@/composables/useCurrentNetwork'
import { Switch } from '@/components/ui/switch'
import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from '@/components/ui/number-field'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'

const { t } = useI18n()
const { networkData, networkDetailStore } = useCurrentNetwork()

const mtu = ref<number>(2800)
const multicastLimit = ref<number>(32)

watch(
  networkData,
  (newData) => {
    if (newData) {
      mtu.value = newData.mtu ?? 2800
      multicastLimit.value = newData.multicastLimit ?? 0
    }
  },
  { immediate: true },
)

const changeEnableBroadcast = async (checked: boolean) => {
  const data = networkData.value
  if (!data?.id) return

  try {
    const updated = await createOrUpdateNetwork(data.id, { enableBroadcast: checked })
    networkDetailStore.setNetworkData(updated)
    showSnackBar(t('common.updateSuccess'), 'success')
  } catch (err) {
    showSnackBar(`${t('common.updateFailed')}: ${getErrorMessage(err)}`, 'error')
  }
}

const changeMTU = async (value: number | undefined) => {
  const data = networkData.value
  if (!data?.id || typeof value !== 'number' || Number.isNaN(value)) return
  if (value === data.mtu) return

  if (value < 1280) {
    showSnackBar(t('network.mtuTooLow'), 'error')
    mtu.value = data.mtu ?? 2800
    return
  }

  mtu.value = value
  try {
    const updated = await createOrUpdateNetwork(data.id, { mtu: value })
    networkDetailStore.setNetworkData(updated)
    showSnackBar(t('common.updateSuccess'), 'success')
  } catch (err) {
    mtu.value = data.mtu ?? 2800
    showSnackBar(`${t('common.updateFailed')}: ${getErrorMessage(err)}`, 'error')
  }
}

const changeMulticastLimit = async (value: number | undefined) => {
  const data = networkData.value
  if (!data?.id || typeof value !== 'number' || Number.isNaN(value)) return
  if (value === data.multicastLimit) return

  if (value < 0) {
    showSnackBar(t('network.multicastLimitInvalid'), 'error')
    multicastLimit.value = data.multicastLimit ?? 0
    return
  }

  multicastLimit.value = value
  try {
    const updated = await createOrUpdateNetwork(data.id, { multicastLimit: value })
    networkDetailStore.setNetworkData(updated)
    showSnackBar(t('common.updateSuccess'), 'success')
  } catch (err) {
    multicastLimit.value = data.multicastLimit ?? 0
    showSnackBar(`${t('common.updateFailed')}: ${getErrorMessage(err)}`, 'error')
  }
}
</script>

<template>
  <div v-if="networkData" class="space-y-3 rounded-xl border bg-card p-5 text-card-foreground shadow-xs">
    <div>
      <p class="mb-1 text-sm font-medium text-muted-foreground">MTU</p>
      <NumberField
        v-model="mtu"
        :min="1280"
        :format-options="{ useGrouping: false }"
        @update:model-value="changeMTU"
      >
        <NumberFieldContent>
          <NumberFieldDecrement />
          <NumberFieldInput class="font-bold" />
          <NumberFieldIncrement />
        </NumberFieldContent>
      </NumberField>
    </div>

    <div>
      <p class="mb-1 text-sm font-medium text-muted-foreground">{{ t('network.multicastLimit') }}</p>
      <NumberField
        v-model="multicastLimit"
        :min="0"
        :format-options="{ useGrouping: false }"
        @update:model-value="changeMulticastLimit"
      >
        <NumberFieldContent>
          <NumberFieldDecrement />
          <NumberFieldInput class="text-sm font-bold" />
          <NumberFieldIncrement />
        </NumberFieldContent>
      </NumberField>
    </div>

    <div class="flex items-center justify-between pt-1">
      <p class="text-sm font-medium text-muted-foreground">{{ t('network.enableBroadcast') }}</p>
      <Switch
        :model-value="Boolean(networkData.enableBroadcast)"
        @update:model-value="changeEnableBroadcast"
      />
    </div>
  </div>
</template>
