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

const changeIPv4Assignment = (e: Event) => {
  const data = networkData.value
  if (!data) return

  const checked = (e.target as HTMLInputElement).checked

  const payload: ControllerNetworkInfo = {
    ...data,
    v4AssignMode: {
      zt: checked,
    },
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
  <!-- Network IPv4 Assignment -->
  <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg">
    <p class="text-gray-500">{{ t('network.ipv4Assignment.default') }}</p>
    <div class="flex items-center justify-between">
      <span class="text-xl font-bold">
        {{ t('network.ipv4Assignment.zt') }}
      </span>
      <input
        type="checkbox"
        @change="changeIPv4Assignment"
        :checked="networkData?.v4AssignMode?.zt"
      />
    </div>
  </div>
</template>
