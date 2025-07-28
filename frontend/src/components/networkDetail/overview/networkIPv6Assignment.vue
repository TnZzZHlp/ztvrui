<script setup lang="ts">
import { useRoute } from 'vue-router'
import { networksData } from '../networkDetailStorage'
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import type { ControllerNetworkInfo, V6AssignMode } from '@/types/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'

const route = useRoute()
const { t } = useI18n()

const networkData = computed(() => {
  return networksData.value.find((data) => data.id === (route.params.networkId as string))
})

const changeIPv6Assignment = (e: Event, type: string) => {
  const data = networkData
  if (!data.value) return

  const checked = (e.target as HTMLInputElement).checked

  const V6AssignMode: V6AssignMode = data.value.v6AssignMode || {
    '6plane': false,
    rfc4193: false,
    zt: false,
  }

  const payload: ControllerNetworkInfo = {
    ...data.value,
    v6AssignMode: {
      ...V6AssignMode,
      [type]: checked,
    },
  }

  createOrUpdateNetwork(data.value.id as string, payload)
    .then(() => {
      showSnackBar(t('common.updateSuccess'), 'success')
    })
    .catch((err) => {
      showSnackBar(t('common.updateFailed') + err, 'error')
    })
}
</script>

<template>
  <!-- Network IPv6 Assignment -->
  <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg">
    <p class="text-gray-500">{{ t('network.ipv6Assignment.default') }}</p>
    <div
      class="flex items-center justify-between"
      v-for="(value, item) in networkData?.v6AssignMode"
      :key="item"
    >
      <span class="text-xl font-bold">
        {{ t(`network.ipv6Assignment.${item}`) }}
      </span>
      <input type="checkbox" @change="(e) => changeIPv6Assignment(e, item)" :checked="value" />
    </div>
  </div>
</template>
