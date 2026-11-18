<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { Settings } from 'lucide-vue-next'
import { useCurrentNetwork } from '@/composables/useCurrentNetwork'
import { Button } from '@/components/ui/button'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import ModifyIPAssignmentPoolsDialog from './ModifyIPAssignmentPoolsDialog.vue'

const { t } = useI18n()
const { networkData } = useCurrentNetwork()

const showDialog = ref(false)
</script>

<template>
  <div v-if="networkData" class="rounded-xl border bg-card p-5 text-card-foreground shadow-xs lg:col-span-2">
    <div class="mb-3 flex items-center justify-between">
      <h2 class="text-base font-semibold">{{ t('network.ipAssignmentPools.default') }}</h2>
      <Button
        type="button"
        variant="ghost"
        size="icon"
        :aria-label="t('network.ipAssignmentPools.default')"
        @click="showDialog = true"
      >
        <Settings class="h-4 w-4" />
      </Button>
    </div>

    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>{{ t('network.ipAssignmentPools.ipRangeStart') }}</TableHead>
          <TableHead>{{ t('network.ipAssignmentPools.ipRangeEnd') }}</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-if="!networkData.ipAssignmentPools || networkData.ipAssignmentPools.length === 0">
          <TableCell :colspan="2" class="text-center text-muted-foreground">
            {{ t('common.noData') }}
          </TableCell>
        </TableRow>
        <TableRow v-for="(pool, index) in networkData.ipAssignmentPools" :key="index">
          <TableCell class="font-mono text-sm">{{ pool.ipRangeStart }}</TableCell>
          <TableCell class="font-mono text-sm">{{ pool.ipRangeEnd }}</TableCell>
        </TableRow>
      </TableBody>
    </Table>

    <ModifyIPAssignmentPoolsDialog
      :open="showDialog"
      :network-id="networkData.id ?? null"
      @update:open="showDialog = $event"
    />
  </div>
</template>
