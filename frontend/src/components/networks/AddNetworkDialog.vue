<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { generateRandomNetworkId } from '@/api/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'
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

const { t } = useI18n()

const open = defineModel<boolean>('open', { required: true })
const networkName = ref('')

const handleSubmit = () => {
  const name = networkName.value.trim()

  if (!name) {
    showSnackBar(t('network.needName'), 'error')
    return
  }

  generateRandomNetworkId({ name })
    .then(() => {
      showSnackBar(t('network.add.success'), 'success')
      // Dispatch an event to notify that the network list has changed
      eventBus.dispatchEvent(new Event('networkListChanged'))
      open.value = false
      networkName.value = ''
    })
    .catch((error) => {
      showSnackBar(t('network.add.error') + `: ${error.message}`, 'error')
    })
}

const handleOpenChange = (newOpen: boolean) => {
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
        <DialogDescription>
          {{ t('network.name') }}
        </DialogDescription>
      </DialogHeader>
      <form @submit.prevent="handleSubmit">
        <div class="grid gap-4 py-4">
          <input v-model="networkName" type="text" :placeholder="t('network.name')" class="border p-3 rounded"
            autocomplete="off" />
        </div>
        <DialogFooter>
          <Button type="button" variant="outline" @click="open = false">
            {{ t('common.cancel') }}
          </Button>
          <Button type="submit">
            {{ t('common.confirm') }}
          </Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>
