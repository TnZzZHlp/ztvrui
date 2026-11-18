<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Loader2 } from 'lucide-vue-next'
import { createOrUpdateNetworkMember } from '@/api/zerotier/controller'
import { isValidMemberId } from '@/utils/ipValidation'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'
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
import { Switch } from '@/components/ui/switch'

interface Props {
  open: boolean
  networkId: string
}

const props = defineProps<Props>()
const emit = defineEmits<{
  'update:open': [value: boolean]
  added: []
}>()

const { t } = useI18n()

const memberId = ref('')
const memberName = ref('')
const authorized = ref(true)
const submitting = ref(false)

watch(
  () => props.open,
  (isOpen) => {
    if (isOpen) {
      memberId.value = ''
      memberName.value = ''
      authorized.value = true
    }
  },
)

const handleSubmit = async () => {
  if (submitting.value) return

  const cleanId = memberId.value.trim().toLowerCase()
  if (!isValidMemberId(cleanId)) {
    showSnackBar(t('network.member.invalidId'), 'error')
    return
  }

  submitting.value = true
  try {
    await createOrUpdateNetworkMember(props.networkId, cleanId, {
      authorized: authorized.value,
      name: memberName.value.trim(),
    })
    showSnackBar(t('network.member.addSuccess'), 'success')
    emit('added')
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
    <DialogContent class="sm:max-w-[440px]">
      <DialogHeader>
        <DialogTitle>{{ t('network.member.add') }}</DialogTitle>
        <DialogDescription>{{ t('network.member.addDescription') }}</DialogDescription>
      </DialogHeader>
      <form class="space-y-4 py-2" @submit.prevent="handleSubmit">
        <div class="space-y-2">
          <label for="add-member-id" class="text-sm font-medium">
            {{ t('network.member.id') }}
          </label>
          <Input
            id="add-member-id"
            v-model="memberId"
            placeholder="8056c2e21c"
            class="font-mono"
            maxlength="10"
            :disabled="submitting"
            autocomplete="off"
            required
          />
        </div>

        <div class="space-y-2">
          <label for="add-member-name" class="text-sm font-medium">
            {{ t('network.member.name') }}
          </label>
          <Input
            id="add-member-name"
            v-model="memberName"
            :placeholder="t('network.member.unnamed')"
            :disabled="submitting"
            autocomplete="off"
          />
        </div>

        <div class="flex items-center justify-between pt-1">
          <span class="text-sm font-medium">{{ t('network.member.authorized') }}</span>
          <Switch v-model="authorized" :disabled="submitting" />
        </div>

        <DialogFooter class="gap-2 pt-2">
          <Button type="button" variant="outline" :disabled="submitting" @click="handleOpenChange(false)">
            {{ t('common.cancel') }}
          </Button>
          <Button type="submit" :disabled="submitting">
            <Loader2 v-if="submitting" class="h-4 w-4 animate-spin" />
            <span>{{ t('common.confirm') }}</span>
          </Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>
