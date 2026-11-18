<script setup lang="ts">
import { ref, toRaw, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Loader2, Trash2 } from 'lucide-vue-next'
import { useNetworkDetailStore } from '@/stores/NetworkDetail'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'
import { isValidIPPoolRange } from '@/utils/ipValidation'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import type { IPAssignmentPool } from '@/types/zerotier/controller'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'

const { t } = useI18n()
const networkDetailStore = useNetworkDetailStore()

interface Props {
  open: boolean
  networkId: string | null
}

const props = defineProps<Props>()
const emit = defineEmits<{
  'update:open': [value: boolean]
}>()

const pools = ref<IPAssignmentPool[]>([])
const submitting = ref(false)

watch(
  [() => props.open, () => props.networkId],
  ([isOpen, networkId]) => {
    if (!isOpen || !networkId) return
    const networkData = networkDetailStore.networksData.find((data) => data.id === networkId)
    if (!networkData) return
    pools.value = networkData.ipAssignmentPools
      ? structuredClone(toRaw(networkData.ipAssignmentPools))
      : []
  },
  { immediate: true },
)

const handleSubmit = async () => {
  if (submitting.value || !props.networkId) return

  const cleanedPools = pools.value
    .map((pool) => ({
      ipRangeStart: pool.ipRangeStart.trim(),
      ipRangeEnd: pool.ipRangeEnd.trim(),
    }))
    .filter((pool) => pool.ipRangeStart.length > 0 || pool.ipRangeEnd.length > 0)

  if (!cleanedPools.every((pool) => isValidIPPoolRange(pool.ipRangeStart, pool.ipRangeEnd))) {
    showSnackBar(t('network.ipAssignmentPools.invalidRange'), 'error')
    return
  }

  submitting.value = true
  try {
    const updated = await createOrUpdateNetwork(props.networkId, {
      ipAssignmentPools: cleanedPools,
    })
    networkDetailStore.setNetworkData(updated)
    showSnackBar(t('common.updateSuccess'), 'success')
    emit('update:open', false)
  } catch (err) {
    showSnackBar(`${t('common.updateFailed')}: ${getErrorMessage(err)}`, 'error')
  } finally {
    submitting.value = false
  }
}

const handleOpenChange = (newOpen: boolean) => {
  if (submitting.value) return
  emit('update:open', newOpen)
}

const addIPPool = () => {
  pools.value.push({
    ipRangeStart: '',
    ipRangeEnd: '',
  })
}

const removeIPPool = (index: number) => {
  pools.value.splice(index, 1)
}
</script>

<template>
  <Dialog :open="open" @update:open="handleOpenChange">
    <DialogContent class="sm:max-w-[600px]">
      <DialogHeader>
        <DialogTitle>{{ t('network.ipAssignmentPools.default') }}</DialogTitle>
        <DialogDescription>
          {{ t('network.ipAssignmentPools.ipRangeStart') }} / {{ t('network.ipAssignmentPools.ipRangeEnd') }}
        </DialogDescription>
      </DialogHeader>
      <div class="max-h-[360px] space-y-3 overflow-y-auto py-4 pr-1">
        <div v-for="(pool, index) in pools" :key="index" class="flex items-center gap-2">
          <Input
            v-model="pool.ipRangeStart"
            type="text"
            :placeholder="t('network.ipAssignmentPools.ipRangeStart')"
            class="flex-1 font-mono"
            :disabled="submitting"
            autocomplete="off"
          />
          <Input
            v-model="pool.ipRangeEnd"
            type="text"
            :placeholder="t('network.ipAssignmentPools.ipRangeEnd')"
            class="flex-1 font-mono"
            :disabled="submitting"
            autocomplete="off"
          />
          <Button
            type="button"
            variant="destructive"
            size="icon"
            :disabled="submitting"
            @click="removeIPPool(index)"
          >
            <Trash2 class="h-4 w-4" />
          </Button>
        </div>
        <div v-if="pools.length === 0" class="py-4 text-center text-sm text-muted-foreground">
          {{ t('common.noData') }}
        </div>
      </div>
      <DialogFooter class="gap-2">
        <Button type="button" variant="outline" :disabled="submitting" @click="handleOpenChange(false)">
          {{ t('common.cancel') }}
        </Button>
        <Button type="button" variant="secondary" :disabled="submitting" @click="addIPPool">
          {{ t('common.add') }}
        </Button>
        <Button type="button" :disabled="submitting" @click="handleSubmit">
          <Loader2 v-if="submitting" class="h-4 w-4 animate-spin" />
          <span>{{ t('common.confirm') }}</span>
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
