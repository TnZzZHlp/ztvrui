<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useNetworkDetailStore } from '@/stores/networkDetail'
import { Switch } from '@/components/ui/switch'
import { NumberField, NumberFieldContent, NumberFieldInput, NumberFieldIncrement, NumberFieldDecrement } from '@/components/ui/number-field'

const networkDetailStore = useNetworkDetailStore()
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'

const route = useRoute()
const { t } = useI18n()

const networkData = computed(() => {
  return networkDetailStore.networksData.find((data) => data.id === (route.params.networkId as string))
})

const mtu = ref<number>(1280)
const multicastLimit = ref<number>(0)

// 监听 networkData 变化，同步到本地状态
watch(networkData, (newData) => {
  if (newData) {
    mtu.value = newData.mtu || 1280
    multicastLimit.value = newData.multicastLimit || 0
  }
}, { immediate: true })

const changeEnableBroadcast = (e: Event) => {
  const data = networkData.value
  if (!data) return

  const checked = (e.target as HTMLInputElement).checked

  const payload: ControllerNetworkInfo = {
    ...data,
    enableBroadcast: checked,
  }

  createOrUpdateNetwork(data.id as string, payload)
    .then(() => {
      showSnackBar(t('common.updateSuccess'), 'success')
    })
    .catch((err) => {
      showSnackBar(t('common.updateFailed') + err, 'error')
    })
}

const changeMTU = (value: number) => {
  const data = networkData.value
  if (!data) return

  mtu.value = value

  if (value < 1280) {
    showSnackBar(t('network.mtuTooLow'), 'error')
    return
  }

  const payload: ControllerNetworkInfo = {
    ...data,
    mtu: value,
  }

  createOrUpdateNetwork(data.id as string, payload)
    .then(() => {
      showSnackBar(t('common.updateSuccess'), 'success')
    })
    .catch((err) => {
      showSnackBar(t('common.updateFailed') + err, 'error')
    })
}

const changeMulticastLimit = (value: number) => {
  const data = networkData.value
  if (!data) return

  multicastLimit.value = value

  if (value < 0) {
    showSnackBar(t('network.multicastLimitInvalid'), 'error')
    return
  }

  const payload: ControllerNetworkInfo = {
    ...data,
    multicastLimit: value,
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
  <!-- Network MTU MulticastLimit enableBroadcast -->
  <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg">
    <p class="text-gray-500">MTU</p>
    <NumberField v-model="mtu" :min="1280" :format-options="{ useGrouping: false }" @update:model-value="changeMTU">
      <NumberFieldContent>
        <NumberFieldDecrement />
        <NumberFieldInput class="font-bold" />
        <NumberFieldIncrement />
      </NumberFieldContent>
    </NumberField>

    <p class="text-gray-500 mt-2">{{ t('network.multicastLimit') }}</p>
    <NumberField v-model="multicastLimit" :min="0" :format-options="{ useGrouping: false }"
      @update:model-value="changeMulticastLimit">
      <NumberFieldContent>
        <NumberFieldDecrement />
        <NumberFieldInput class="text-ms font-bold" />
        <NumberFieldIncrement />
      </NumberFieldContent>
    </NumberField>

    <p class="text-gray-500 mt-2">{{ t('network.enableBroadcast') }}</p>
    <Switch :model-value="networkData?.enableBroadcast"
      @update:model-value="(checked: boolean) => changeEnableBroadcast({ target: { checked } } as any)" />
  </div>
</template>
