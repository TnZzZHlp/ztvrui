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
import QRCode from 'qrcode'

const { t } = useI18n()
const router = useRouter()

const networks: Ref<ControllerNetworkInfo[]> = ref([])
const networkToDelete = ref<string | null>(null)
const qrCodeMap = ref<Map<string, string>>(new Map())
const showQRCodeMap = ref<Map<string, boolean>>(new Map())

// Generate QR code
const generateQRCode = async (networkId: string) => {
  try {
    const zerotierUri = `https://joinzt.com/addnetwork?nwid=${networkId}&v=1`
    const dataUrl = await QRCode.toDataURL(zerotierUri, {
      width: 200,
      margin: 2,
      color: {
        dark: '#000000',
        light: '#FFFFFF'
      }
    })
    qrCodeMap.value.set(networkId, dataUrl)
  } catch (err) {
    console.error('Failed to generate QR code:', err)
    showSnackBar(t('common.updateFailed') + err, 'error')
  }
}

// Toggle QR code display status
const toggleQRCode = async (networkId: string) => {
  const currentState = showQRCodeMap.value.get(networkId) || false
  showQRCodeMap.value.set(networkId, !currentState)

  // If it's the first time showing and the QR code hasn't been generated yet, generate it
  if (!currentState && !qrCodeMap.value.has(networkId)) {
    await generateQRCode(networkId)
  }
}

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
      <div class="flex items-center gap-2">
        <Button variant="ghost" size="sm" class="px-1 mt-1" @click="() => copyToClipboard(network.id!)">
          {{ network.id }}
        </Button>
        <Button variant="outline" size="sm" class="mt-1" @click="() => toggleQRCode(network.id!)">
          {{ showQRCodeMap.get(network.id!) ? t('network.hideQRCode') : t('network.showQRCode') }}
        </Button>
      </div>

      <!-- QR Code Display -->
      <div v-if="showQRCodeMap.get(network.id!) && qrCodeMap.get(network.id!)" class="mt-4 flex justify-center">
        <div class="p-4 bg-white border border-gray-300 rounded-lg shadow-sm">
          <img :src="qrCodeMap.get(network.id!)" :alt="'QR Code for ' + network.id" class="w-50 h-50" />
          <p class="text-center text-sm text-gray-500 mt-2">{{ t('network.scanToJoin') }}</p>
        </div>
      </div>
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
            <AlertDialogAction @click="() => { networkToDelete = network.id!; confirmDeleteNetwork(); }"
              class="bg-red-500 hover:bg-red-600">
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
