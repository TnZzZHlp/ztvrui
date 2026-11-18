<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Loader2 } from 'lucide-vue-next'
import { useNetworkDetailStore } from '@/stores/NetworkDetail'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import type { Capability, Rule, Tag } from '@/types/zerotier/controller'
import compile from './RulesCompile'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'

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

const originalRules = ref('')
const submitting = ref(false)

watch(
  [() => props.open, () => props.networkId],
  ([isOpen, networkId]) => {
    if (!isOpen || !networkId) return
    const savedRules = localStorage.getItem(`networkRules_${networkId}`)
    if (savedRules !== null) {
      originalRules.value = savedRules
      return
    }
    const networkData = networkDetailStore.networksData.find((data) => data.id === networkId)
    const rules = networkData?.rules
    if (!rules || rules.length === 0 || (rules.length === 1 && rules[0].type === 'ACTION_ACCEPT')) {
      originalRules.value = 'accept;\n'
    } else {
      originalRules.value = ''
    }
  },
  { immediate: true },
)

const handleSubmit = async () => {
  if (submitting.value || !props.networkId) return

  const rules: Rule[] = []
  const tags: Record<string, { id: number; default: number | null }> = {}
  const caps: Record<string, Capability> = {}

  const compileError = compile(originalRules.value, rules, caps, tags)
  if (compileError) {
    const [row, col, error] = compileError
    showSnackBar(`row:${row} col:${col} ${error}`, 'error')
    return
  }

  const capsArray: Capability[] = Object.values(caps)
  const tagsArray: Tag[] = Object.values(tags).map((tagItem) => ({
    id: tagItem.id,
    default: tagItem.default || tagItem.default === 0 ? tagItem.default : null,
  }))

  submitting.value = true
  try {
    const updated = await createOrUpdateNetwork(props.networkId, {
      rules,
      tags: tagsArray,
      capabilities: capsArray,
    })
    networkDetailStore.setNetworkData(updated)
    localStorage.setItem(`networkRules_${props.networkId}`, originalRules.value)
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
</script>

<template>
  <Dialog :open="open" @update:open="handleOpenChange">
    <DialogContent class="max-h-[85vh] sm:max-w-[800px]">
      <DialogHeader>
        <DialogTitle>{{ t('network.rules.default') }}</DialogTitle>
        <DialogDescription class="font-medium text-amber-600">
          {{ t('network.rules.warn') }}
        </DialogDescription>
      </DialogHeader>
      <div class="py-2">
        <Textarea
          v-model="originalRules"
          :placeholder="t('network.rules.default')"
          class="h-[360px] max-h-[55vh] font-mono text-sm"
          :disabled="submitting"
        />
      </div>
      <DialogFooter class="gap-2">
        <Button type="button" variant="outline" :disabled="submitting" @click="handleOpenChange(false)">
          {{ t('common.cancel') }}
        </Button>
        <Button type="button" :disabled="submitting" @click="handleSubmit">
          <Loader2 v-if="submitting" class="h-4 w-4 animate-spin" />
          <span>{{ t('common.confirm') }}</span>
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
