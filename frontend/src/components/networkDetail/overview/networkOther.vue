<script setup lang="ts">
import { useRoute } from 'vue-router'
import { networksData } from '../networkDetailStorage'
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'

const route = useRoute()
const { t } = useI18n()

const networkData = computed(() => {
  return networksData.value.find((data) => data.id === (route.params.networkId as string))
})

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

const changeMTU = (e: Event) => {
  const data = networkData.value
  if (!data) return

  const mtu = parseInt((e.target as HTMLInputElement).value, 10)

  if (mtu < 1280) {
    showSnackBar(t('network.mtuTooLow'), 'error')
    return
  }

  const payload: ControllerNetworkInfo = {
    ...data,
    mtu,
  }

  createOrUpdateNetwork(data.id as string, payload)
    .then(() => {
      showSnackBar(t('common.updateSuccess'), 'success')
    })
    .catch((err) => {
      showSnackBar(t('common.updateFailed') + err, 'error')
    })
}

const changeMulticastLimit = (e: Event) => {
  const data = networkData.value
  if (!data) return

  const multicastLimit = parseInt((e.target as HTMLInputElement).value, 10)

  if (multicastLimit < 0) {
    showSnackBar(t('network.multicastLimitInvalid'), 'error')
    return
  }

  const payload: ControllerNetworkInfo = {
    ...data,
    multicastLimit,
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
    <input
      class="text-3xl font-bold focus:outline-none border-b-1 w-full"
      type="number"
      min="1280"
      v-model="networkData!.mtu"
      autocomplete="off"
      @change="changeMTU"
    />

    <p class="text-gray-500 mt-2">{{ t('network.multicastLimit') }}</p>
    <input
      class="text-ms font-bold focus:outline-none border-b-1 w-full"
      type="number"
      min="0"
      v-model="networkData!.multicastLimit"
      autocomplete="off"
      @change="changeMulticastLimit"
    />

    <p class="text-gray-500 mt-2">{{ t('network.enableBroadcast') }}</p>
    <input
      type="checkbox"
      @change="changeEnableBroadcast"
      :checked="networkData?.enableBroadcast"
    />
  </div>
</template>
