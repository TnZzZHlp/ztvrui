<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useNetworkDetailStore } from '@/stores/networkDetail'

const networkDetailStore = useNetworkDetailStore()
import { useI18n } from 'vue-i18n'
import { showModifyNetworkIPAssignmentPools } from './popupPanel/showModifyNetworkIPAssignmentPools'
import { computed } from 'vue'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

const route = useRoute()
const { t } = useI18n()

const networkData = computed(() => {
  return networkDetailStore.networksData.find((network) => network.id === (route.params.networkId as string))
})
</script>

<template>
  <!-- Network IP AssignmentPools -->
  <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg lg:col-span-2">
    <div class="flex justify-between mb-4">
      <span class="text-gray-500">{{ t('network.ipAssignmentPools.default') }}</span>
      <button @click="showModifyNetworkIPAssignmentPools(networkData?.id as string)"
        class="transition-all rounded hover:bg-gray-200 active:bg-gray-400">
        <img src="@/assets/icons/setting.svg" alt="Setting" class="h-6 object-contain" />
      </button>
    </div>
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>{{ t('network.ipAssignmentPools.ipRangeStart') }}</TableHead>
          <TableHead>{{ t('network.ipAssignmentPools.ipRangeEnd') }}</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-for="(ip, index) in networkData?.ipAssignmentPools" :key="index">
          <TableCell>{{ ip.ipRangeStart }}</TableCell>
          <TableCell>{{ ip.ipRangeEnd }}</TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>
</template>
