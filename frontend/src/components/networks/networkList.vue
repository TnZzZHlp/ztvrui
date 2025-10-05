<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { onBeforeMount, ref, type Ref } from 'vue'
import { deleteNetwork, getNetworkById, listNetworkIds } from '@/api/zerotier/controller'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'
import { eventBus } from '@/utils/eventBus'
import { useRouter } from 'vue-router'
import { copyToClipboard } from '@/utils/copyToClipboard'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog'
import { Button } from '@/components/ui/button'

const { t } = useI18n()
const router = useRouter()

const networks: Ref<ControllerNetworkInfo[]> = ref([])
const networkToDelete = ref<string | null>(null)

const confirmDeleteNetwork = () => {
  if (!networkToDelete.value) return

  // Call the API to delete the network
  deleteNetwork(networkToDelete.value)
    .then(() => {
      showSnackBar(t('network.delete.success'), 'success')
      // Refresh the list of networks
      refreshNetworks()
    })
    .catch((error) => {
      showSnackBar(t('network.delete.failed') + error, 'error')
    })
    .finally(() => {
      networkToDelete.value = null
    })
}

const refreshNetworks = () => {
  listNetworkIds()
    .then((data) => {
      networks.value = []
      data.forEach((networkId) => {
        getNetworkById(networkId)
          .then((networkInfo) => {
            networks.value.push(networkInfo)
          })
          .catch((error) => {
            showSnackBar('Error fetching network info:', error)
          })
      })
    })
    .catch((error) => {
      showSnackBar(error, 'error')
    })
}

onBeforeMount(() => {
  eventBus.addEventListener('networkListChanged', refreshNetworks)
  refreshNetworks()
})
</script>

<template>
  <div class="shadow w-full rounded min-h-[150px] p-4 flex flex-col justify-between my-2" v-for="network in networks"
    :key="network.id">
    <div>
      <p class="font-bold text-xl">{{ network.name }}</p>
      <Button variant="ghost" size="sm" class="px-1 mt-1" @click="() => copyToClipboard(network.id!)">
        {{ network.id }}
      </Button>
    </div>

    <div class="flex items-center justify-end">
      <AlertDialog>
        <AlertDialogTrigger as-child>
          <Button variant="destructive" type="button" class="mx-2">
            {{ t('network.delete.default') }}
          </Button>
        </AlertDialogTrigger>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{{ t('common.confirm') }}</AlertDialogTitle>
            <AlertDialogDescription>
              {{ t('network.delete.notice') }}
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{{ t('common.cancel') }}</AlertDialogCancel>
            <AlertDialogAction variant="destructive" @click="
              () => {
                networkToDelete = network.id!
                confirmDeleteNetwork()
              }
            ">
              {{ t('common.confirm') }}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <Button variant="outline" type="button" class="mx-2" @click="router.push(`/network/${network.id}/overview`)">
        {{ t('network.enter') }}
      </Button>
    </div>
  </div>
</template>
