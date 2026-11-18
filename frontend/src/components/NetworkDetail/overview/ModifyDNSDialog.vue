<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Loader2, Trash2 } from 'lucide-vue-next'
import { useNetworkDetailStore } from '@/stores/NetworkDetail'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'
import { isValidIP } from '@/utils/ipValidation'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
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

const domain = ref('')
const servers = ref<string[]>([])
const submitting = ref(false)

watch(
  [() => props.open, () => props.networkId],
  ([isOpen, networkId]) => {
    if (!isOpen || !networkId) return
    const networkData = networkDetailStore.networksData.find((data) => data.id === networkId)
    if (!networkData) return

    const dns = networkData.dns && !Array.isArray(networkData.dns) ? networkData.dns : null
    domain.value = dns?.domain ?? ''
    servers.value = dns?.servers ? [...dns.servers] : []
  },
  { immediate: true },
)

const handleSubmit = async () => {
  if (submitting.value || !props.networkId) return

  const trimmedDomain = domain.value.trim()
  const cleanedServers = servers.value.map((s) => s.trim()).filter((s) => s.length > 0)

  if (!cleanedServers.every((server) => isValidIP(server))) {
    showSnackBar(t('network.ipAssignmentPools.invalidedIP'), 'error')
    return
  }

  submitting.value = true
  try {
    const updated = await createOrUpdateNetwork(props.networkId, {
      dns: {
        domain: trimmedDomain,
        servers: cleanedServers,
      },
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

const addServer = () => {
  servers.value.push('')
}

const removeServer = (index: number) => {
  servers.value.splice(index, 1)
}
</script>

<template>
  <Dialog :open="open" @update:open="handleOpenChange">
    <DialogContent class="sm:max-w-[600px]">
      <DialogHeader>
        <DialogTitle>DNS</DialogTitle>
        <DialogDescription>
          {{ t('network.dns.domain') }} / {{ t('network.dns.servers') }}
        </DialogDescription>
      </DialogHeader>
      <div class="space-y-4 py-4">
        <div class="space-y-2">
          <label class="text-sm font-medium">{{ t('network.dns.domain') }}</label>
          <Input
            v-model="domain"
            :placeholder="t('network.dns.domain')"
            type="text"
            :disabled="submitting"
          />
        </div>

        <div class="space-y-2">
          <label class="text-sm font-medium">{{ t('network.dns.servers') }}</label>
          <div class="max-h-[300px] space-y-2 overflow-y-auto pr-1">
            <div v-for="(_, index) in servers" :key="index" class="flex items-center gap-2">
              <Input
                v-model="servers[index]"
                :placeholder="t('network.dns.servers')"
                type="text"
                class="flex-1 font-mono"
                :disabled="submitting"
              />
              <Button
                type="button"
                variant="destructive"
                size="icon"
                :disabled="submitting"
                @click="removeServer(index)"
              >
                <Trash2 class="h-4 w-4" />
              </Button>
            </div>
            <div v-if="servers.length === 0" class="py-4 text-center text-sm text-muted-foreground">
              {{ t('common.noData') }}
            </div>
          </div>
        </div>
      </div>
      <DialogFooter class="gap-2">
        <Button type="button" variant="outline" :disabled="submitting" @click="handleOpenChange(false)">
          {{ t('common.cancel') }}
        </Button>
        <Button type="button" variant="secondary" :disabled="submitting" @click="addServer">
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
