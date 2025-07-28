<script setup lang="ts">
import { useRoute } from 'vue-router'
import { networksData } from '../networkDetailStorage'
import { useI18n } from 'vue-i18n'
import { showModifyNetworkIPAssignmentPools } from './popupPanel/showModifyNetworkIPAssignmentPools'
import { computed } from 'vue'

const route = useRoute()
const { t } = useI18n()

const networkData = computed(() => {
  return networksData.value.find((network) => network.id === (route.params.networkId as string))
})
</script>

<template>
  <!-- Network IP AssignmentPools -->
  <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg lg:col-span-2">
    <div class="flex justify-between">
      <span class="text-gray-500">{{ t('network.ipAssignmentPools.default') }}</span>
      <button
        @click="showModifyNetworkIPAssignmentPools(networkData?.id as string)"
        class="transition-all rounded hover:bg-gray-200 active:bg-gray-400"
      >
        <img src="@/assets/icons/setting.svg" alt="Setting" class="h-6 object-contain" />
      </button>
    </div>
    <ol class="divide-y divide-gray-200">
      <li class="text-gray-700">
        <div class="flex justify-between">
          <p class="text-sm text-gray-500 w-full">
            {{ t('network.ipAssignmentPools.ipRangeStart') }}
          </p>
          <p class="text-sm text-gray-500 w-full">
            {{ t('network.ipAssignmentPools.ipRangeEnd') }}
          </p>
        </div>
      </li>
      <li class="text-gray-700" v-for="(ip, index) in networkData?.ipAssignmentPools" :key="index">
        <div class="flex justify-between">
          <p class="w-full">
            {{ ip.ipRangeStart }}
          </p>
          <p class="w-full">
            {{ ip.ipRangeEnd }}
          </p>
        </div>
      </li>
    </ol>
  </div>
</template>
