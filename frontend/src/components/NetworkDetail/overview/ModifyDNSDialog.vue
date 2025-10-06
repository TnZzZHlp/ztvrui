<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import _ from 'lodash'
import { useNetworkDetailStore } from '@/stores/NetworkDetail'
import { showSnackBar } from '@/utils/showSnackBar'
import { eventBus } from '@/utils/eventBus'
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
import deleteIcon from '@/assets/icons/delete.svg'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'

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

const networkDataClone = ref<ControllerNetworkInfo | null>(null)

watch(
  () => props.networkId,
  (newValue) => {
    if (newValue) {
      const networkData = networkDetailStore.networksData.find((data) => data.id === newValue)
      if (networkData) {
        networkDataClone.value = _.cloneDeep(networkData)

        // Initialize DNS if not exists
        if (!networkDataClone.value.dns) {
          networkDataClone.value.dns = {
            domain: '',
            servers: [],
          }
        }

        // Fix if dns is array instead of object
        if (Array.isArray(networkDataClone.value.dns)) {
          networkDataClone.value.dns = {
            domain: '',
            servers: [],
          }
        }
      }
    }
  },
  { immediate: true }
)

const handleSubmit = () => {
  if (!networkDataClone.value) return

  // Validate IP range with regex
  const dns = networkDataClone.value.dns

  if (dns && dns.servers && dns.servers.length > 0) {
    const ipRegex = /^(25[0-5]|2[0-4]\d|[01]?\d?\d)(\.(25[0-5]|2[0-4]\d|[01]?\d?\d)){3}$/

    const isValid = dns.servers.every((server: string) => {
      return !server || ipRegex.test(server)
    })

    if (!isValid) {
      showSnackBar(t('network.ipAssignmentPools.invalidedIP'), 'error')
      return
    }
  }

  // Submit changes
  createOrUpdateNetwork(networkDataClone.value.id as string, {
    ...networkDataClone.value,
    dns: networkDataClone.value.dns,
  })
    .then(() => {
      // Update the network data in the storage
      const event = new CustomEvent('controllerNetworkInfoUpdate', {
        detail: networkDataClone.value?.id,
      })
      eventBus.dispatchEvent(event)
      showSnackBar(t('common.updateSuccess'), 'success')
      emit('update:open', false)
    })
    .catch((err) => {
      showSnackBar(t('common.updateFailed') + err, 'error')
    })
}

const handleOpenChange = (newOpen: boolean) => {
  emit('update:open', newOpen)
}

const addServer = () => {
  if (networkDataClone.value && networkDataClone.value.dns) {
    if (!networkDataClone.value.dns.servers) {
      networkDataClone.value.dns.servers = []
    }
    networkDataClone.value.dns.servers.push('')
  }
}

const removeServer = (index: number) => {
  if (networkDataClone.value && networkDataClone.value.dns && networkDataClone.value.dns.servers) {
    networkDataClone.value.dns.servers.splice(index, 1)
  }
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
      <div class="py-4 space-y-4" v-if="networkDataClone && networkDataClone.dns">
        <!-- Domain -->
        <div class="space-y-2">
          <label class="text-sm font-medium">{{ t('network.dns.domain') }}</label>
          <Input v-model="networkDataClone.dns.domain" :placeholder="t('network.dns.domain')" type="text" />
        </div>

        <!-- Servers -->
        <div class="space-y-2">
          <label class="text-sm font-medium">{{ t('network.dns.servers') }}</label>
          <div class="space-y-3">
            <div v-for="(server, index) in networkDataClone.dns.servers" :key="index" class="flex items-center gap-2">
              <Input v-model="networkDataClone.dns.servers[index]" :placeholder="t('network.dns.servers')" type="text"
                class="flex-1" />
              <Button type="button" variant="destructive" size="icon" @click="removeServer(index)">
                <img :src="deleteIcon" alt="delete" class="w-4 h-4 filter brightness-0 invert" />
              </Button>
            </div>
            <div v-if="!networkDataClone.dns.servers || networkDataClone.dns.servers.length === 0"
              class="text-center text-gray-500 py-4">
              {{ t('common.noData') }}
            </div>
          </div>
        </div>
      </div>
      <DialogFooter class="gap-2">
        <Button type="button" variant="outline" @click="emit('update:open', false)">
          {{ t('common.cancel') }}
        </Button>
        <Button type="button" variant="secondary" @click="addServer">
          {{ t('common.add') }}
        </Button>
        <Button type="button" @click="handleSubmit">
          {{ t('common.confirm') }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
