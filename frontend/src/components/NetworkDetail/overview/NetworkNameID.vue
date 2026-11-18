<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import QRCode from 'qrcode'
import { Copy, QrCode } from 'lucide-vue-next'
import { useCurrentNetwork } from '@/composables/useCurrentNetwork'
import { copyToClipboard } from '@/utils/copyToClipboard'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'
import { Button } from '@/components/ui/button'

const { t } = useI18n()
const { networkData, networkDetailStore } = useCurrentNetwork()

const nameInput = ref('')
const qrCodeDataUrl = ref('')
const showQRCode = ref(false)

watch(
  () => networkData.value?.name,
  (newName) => {
    nameInput.value = newName ?? ''
  },
  { immediate: true },
)

watch(
  () => networkData.value?.id,
  () => {
    showQRCode.value = false
    qrCodeDataUrl.value = ''
  },
)

const toggleQRCode = async () => {
  const networkId = networkData.value?.id
  if (!networkId) return

  if (showQRCode.value) {
    showQRCode.value = false
    return
  }

  if (!qrCodeDataUrl.value) {
    try {
      const zerotierUri = `https://joinzt.com/addnetwork?nwid=${networkId}&v=1`
      qrCodeDataUrl.value = await QRCode.toDataURL(zerotierUri, {
        width: 200,
        margin: 2,
        color: {
          dark: '#000000',
          light: '#FFFFFF',
        },
      })
    } catch (err) {
      showSnackBar(`${t('common.updateFailed')}: ${getErrorMessage(err)}`, 'error')
      return
    }
  }

  showQRCode.value = true
}

const changeNetworkName = async () => {
  const data = networkData.value
  if (!data?.id) return

  const trimmed = nameInput.value.trim()
  if (!trimmed) {
    showSnackBar(t('network.needName'), 'error')
    nameInput.value = data.name ?? ''
    return
  }

  if (trimmed === data.name) {
    nameInput.value = trimmed
    return
  }

  try {
    const updated = await createOrUpdateNetwork(data.id, { name: trimmed })
    networkDetailStore.setNetworkData(updated)
    showSnackBar(t('common.updateSuccess'), 'success')
  } catch (err) {
    nameInput.value = data.name ?? ''
    showSnackBar(`${t('common.updateFailed')}: ${getErrorMessage(err)}`, 'error')
  }
}
</script>

<template>
  <div v-if="networkData" class="rounded-xl border bg-card p-5 text-card-foreground shadow-xs">
    <p class="text-sm font-medium text-muted-foreground">{{ t('network.name') }}</p>
    <input
      v-model="nameInput"
      class="mt-1 block w-full border-b border-border bg-transparent pb-1 text-2xl font-bold tracking-tight focus:border-primary focus:outline-none"
      type="text"
      :placeholder="t('network.name')"
      name="Network Name"
      autocomplete="off"
      @change="changeNetworkName"
    />

    <p class="mt-4 text-sm font-medium text-muted-foreground">{{ t('network.id') }}</p>
    <div class="mt-1.5 flex flex-wrap items-center gap-2">
      <Button
        v-if="networkData.id"
        type="button"
        variant="secondary"
        size="sm"
        class="font-mono text-xs"
        @click="copyToClipboard(networkData.id)"
      >
        <span>{{ networkData.id }}</span>
        <Copy class="h-3.5 w-3.5" />
      </Button>
      <Button type="button" variant="outline" size="sm" @click="toggleQRCode">
        <QrCode class="h-3.5 w-3.5" />
        <span>{{ showQRCode ? t('network.hideQRCode') : t('network.showQRCode') }}</span>
      </Button>
    </div>

    <div v-if="showQRCode && qrCodeDataUrl" class="mt-4 flex justify-center">
      <div class="rounded-lg border bg-white p-4 shadow-xs">
        <img :src="qrCodeDataUrl" :alt="'QR Code for ' + networkData.id" class="h-44 w-44" />
        <p class="mt-2 text-center text-xs text-muted-foreground">{{ t('network.scanToJoin') }}</p>
      </div>
    </div>
  </div>
</template>
