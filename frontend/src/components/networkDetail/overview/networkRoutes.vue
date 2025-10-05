<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useNetworkDetailStore } from '@/stores/networkDetail'
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { showModifyNetworkRoutes } from './popupPanel/showModifyNetworkRoutes'
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
const networkDetailStore = useNetworkDetailStore()

const networkData = computed(() => {
  return networkDetailStore.networksData.find((data) => data.id === (route.params.networkId as string))
})
</script>

<template>
  <!-- Network Routes -->
  <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg lg:col-span-2">
    <div class="flex justify-between mb-4">
      <span class="text-gray-500">{{ t('network.routes') }}</span>
      <button @click="showModifyNetworkRoutes(networkData?.id as string)"
        class="transition-all rounded hover:bg-gray-200 active:bg-gray-400">
        <img src="@/assets/icons/setting.svg" alt="Setting" class="h-6 object-contain" />
      </button>
    </div>
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>{{ t('network.default') }}</TableHead>
          <TableHead>{{ t('network.via.default') }}</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-for="(route, index) in networkData?.routes" :key="index">
          <TableCell>{{ route.target }}</TableCell>
          <TableCell>{{ route.via ?? 'LAN' }}</TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>
</template>
