<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { Loader2 } from 'lucide-vue-next'
import { generateRandomNetworkId } from '@/api/zerotier/controller'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'
import { eventBus } from '@/utils/eventBus'
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

const open = defineModel<boolean>('open', { required: true })
const networkName = ref('')
const submitting = ref(false)

const handleSubmit = async () => {
  if (submitting.value) return
  const name = networkName.value.trim()

  if (!name) {
    showSnackBar(t('network.needName'), 'error')
    return
  }

  submitting.value = true
  try {
    await generateRandomNetworkId({ name })
    showSnackBar(t('network.add.success'), 'success')
    eventBus.dispatchEvent(new Event('networkListChanged'))
    open.value = false
    networkName.value = ''
  } catch (error) {
    showSnackBar(`${t('network.add.error')}: ${getErrorMessage(error)}`, 'error')
  } finally {
    submitting.value = false
  }
}

const handleOpenChange = (newOpen: boolean) => {
  if (submitting.value) return
  open.value = newOpen
  if (!newOpen) {
    networkName.value = ''
  }
}
</script>

<template>
  <Dialog :open="open" @update:open="handleOpenChange">
    <DialogContent>
      <DialogHeader>
        <DialogTitle>{{ t('network.add.default') }}</DialogTitle>
        <DialogDescription>{{ t('network.add.description') }}</DialogDescription>
      </DialogHeader>
      <form @submit.prevent="handleSubmit">
        <div class="grid gap-2 py-4">
          <label for="networkName" class="text-sm font-medium">
            {{ t('network.name') }}
          </label>
          <Input
            id="networkName"
            v-model="networkName"
            type="text"
            :placeholder="t('network.name')"
            :disabled="submitting"
            autocomplete="off"
          />
        </div>
        <DialogFooter>
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
