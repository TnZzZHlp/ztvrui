<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useNetworkDetailStore } from '@/stores/NetworkDetail'
import { useI18n } from 'vue-i18n'
import { computed, ref } from 'vue'
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from '@/components/ui/table'
import ModifyIPAssignmentPoolsDialog from './ModifyIPAssignmentPoolsDialog.vue'

const networkDetailStore = useNetworkDetailStore()
const route = useRoute()
const { t } = useI18n()

const networkData = computed(() => {
    return networkDetailStore.networksData.find((network) => network.id === (route.params.networkId as string))
})

// Dialog state
const showDialog = ref(false)
const selectedNetworkId = ref<string | null>(null)

const openDialog = () => {
    selectedNetworkId.value = networkData.value?.id || null
    showDialog.value = true
}
</script>

<template>
    <!-- Network IP AssignmentPools -->
    <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg lg:col-span-2">
        <div class="flex justify-between mb-4">
            <span class="text-gray-500">{{ t('network.ipAssignmentPools.default') }}</span>
            <button @click="openDialog" class="transition-all rounded hover:bg-gray-200 active:bg-gray-400">
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

        <!-- Dialog -->
        <ModifyIPAssignmentPoolsDialog :open="showDialog" :network-id="selectedNetworkId"
            @update:open="showDialog = $event" />
    </div>
</template>
