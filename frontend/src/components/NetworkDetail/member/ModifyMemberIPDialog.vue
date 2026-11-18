<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Loader2, Trash2 } from 'lucide-vue-next'
import type { ControllerNetworkMemberInfo } from '@/types/zerotier/controller'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'
import { isValidIP } from '@/utils/ipValidation'
import { eventBus } from '@/utils/eventBus'
import { createOrUpdateNetworkMember } from '@/api/zerotier/controller'
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

interface Props {
  open: boolean
  memberInfo: ControllerNetworkMemberInfo | null
}

const props = defineProps<Props>()
const emit = defineEmits<{
  'update:open': [value: boolean]
}>()

const ipList = ref<string[]>([])
const submitting = ref(false)

watch(
  [() => props.open, () => props.memberInfo],
  ([isOpen, newValue]) => {
    if (isOpen && newValue) {
      ipList.value = [...(newValue.ipAssignments ?? [])]
    }
  },
  { immediate: true },
)

const handleSubmit = async () => {
  if (submitting.value || !props.memberInfo) return

  const cleanedIps = ipList.value.map((ip) => ip.trim()).filter((ip) => ip.length > 0)
  if (!cleanedIps.every((ip) => isValidIP(ip))) {
    showSnackBar(t('network.ipAssignmentPools.invalidedIP'), 'error')
    return
  }

  const uniqueIps = [...new Set(cleanedIps)]

  submitting.value = true
  try {
    await createOrUpdateNetworkMember(props.memberInfo.nwid, props.memberInfo.id, {
      ipAssignments: uniqueIps,
    })
    eventBus.dispatchEvent(new Event('networkMemberChanged'))
    showSnackBar(t('common.updateSuccess'), 'success')
    emit('update:open', false)
  } catch (error) {
    showSnackBar(`${t('common.updateFailed')}: ${getErrorMessage(error)}`, 'error')
  } finally {
    submitting.value = false
  }
}

const handleOpenChange = (newOpen: boolean) => {
  if (submitting.value) return
  emit('update:open', newOpen)
}

const addIP = () => {
  ipList.value.push('')
}

const removeIP = (index: number) => {
  ipList.value.splice(index, 1)
}
</script>

<template>
  <Dialog :open="open" @update:open="handleOpenChange">
    <DialogContent class="sm:max-w-[500px]">
      <DialogHeader>
        <DialogTitle>{{ t('network.member.modifyIP') }}</DialogTitle>
        <DialogDescription>
          {{ memberInfo?.name || memberInfo?.id }}
        </DialogDescription>
      </DialogHeader>
      <div class="py-4" v-if="memberInfo">
        <h4 class="mb-3 text-sm font-semibold">{{ t('network.ip') }}</h4>
        <ul class="max-h-[300px] space-y-2 overflow-y-auto pr-1">
          <li v-for="(_, index) in ipList" :key="index" class="flex items-center gap-2">
            <Input
              v-model="ipList[index]"
              type="text"
              class="flex-1 font-mono"
              :placeholder="t('network.ip')"
              :disabled="submitting"
            />
            <Button
              type="button"
              variant="destructive"
              size="icon"
              :disabled="submitting"
              @click="removeIP(index)"
            >
              <Trash2 class="h-4 w-4" />
            </Button>
          </li>
        </ul>
        <div v-if="ipList.length === 0" class="py-4 text-center text-sm text-muted-foreground">
          {{ t('common.noData') }}
        </div>
      </div>
      <DialogFooter class="gap-2">
        <Button type="button" variant="outline" :disabled="submitting" @click="handleOpenChange(false)">
          {{ t('common.cancel') }}
        </Button>
        <Button type="button" variant="secondary" :disabled="submitting" @click="addIP">
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
