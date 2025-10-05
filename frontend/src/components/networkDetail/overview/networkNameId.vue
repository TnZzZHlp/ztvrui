<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useNetworkDetailStore } from '@/stores/networkDetail'

const networkDetailStore = useNetworkDetailStore()
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { copyToClipboard } from '@/utils/copyToClipboard'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'
import QRCode from 'qrcode'

const route = useRoute()
const { t } = useI18n()

const networkData = computed(() => {
  return networkDetailStore.networksData.find((data) => data.id === (route.params.networkId as string))
})

const qrCodeDataUrl = ref<string>('')
const showQRCode = ref(false)

// 生成二维码
const generateQRCode = async () => {
  if (!networkData.value?.id) return

  try {
    const dataUrl = await QRCode.toDataURL(networkData.value.id, {
      width: 200,
      margin: 2,
      color: {
        dark: '#000000',
        light: '#FFFFFF'
      }
    })
    qrCodeDataUrl.value = dataUrl
    showQRCode.value = true
  } catch (err) {
    console.error('Failed to generate QR code:', err)
    showSnackBar(t('common.updateFailed') + err, 'error')
  }
}

// 监听 networkData 变化，自动生成二维码
watch(
  () => networkData.value?.id,
  (newId) => {
    if (newId) {
      generateQRCode()
    }
  },
  { immediate: true }
)

const changeNetworkName = (e: Event) => {
  const newName = (e.target as HTMLInputElement).value
  const data = networkData.value
  if (!data) return

  const payload: ControllerNetworkInfo = {
    ...data,
    name: newName,
  }

  createOrUpdateNetwork(data.id as string, payload)
    .then(() => {
      showSnackBar(t('common.updateSuccess'), 'success')
    })
    .catch((err) => {
      showSnackBar(t('common.updateFailed') + err, 'error')
    })
}
</script>

<template>
  <!-- Network Name and Id -->
  <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg">
    <p class="text-gray-500">{{ t('network.name') }}</p>
    <input class="text-3xl font-bold border-b-1 block w-full focus:outline-none" type="text" placeholder="Network Name"
      name="Network Name" @change="changeNetworkName" :value="networkData?.name" autocomplete="off" />
    <p class="text-gray-500 mt-2">{{ t('network.id') }}</p>
    <div class="flex items-center gap-4">
      <button class="hover:bg-gray-100 active:bg-gray-200 px-2 py-1 rounded text-gray-700 transition-all"
        @click="copyToClipboard(networkData?.id as string)">
        {{ networkData?.id }}
      </button>
      <button
        class="hover:bg-gray-100 active:bg-gray-200 px-3 py-1 rounded text-gray-700 transition-all border border-gray-300"
        @click="showQRCode = !showQRCode">
        {{ showQRCode ? t('network.hideQRCode') : t('network.showQRCode') }}
      </button>
    </div>
    <!-- QR Code Display -->
    <div v-if="showQRCode && qrCodeDataUrl" class="mt-4 flex justify-center">
      <div class="p-4 bg-white border border-gray-300 rounded-lg shadow-sm">
        <img :src="qrCodeDataUrl" :alt="'QR Code for ' + networkData?.id" class="w-50 h-50" />
        <p class="text-center text-sm text-gray-500 mt-2">{{ t('network.scanToJoin') }}</p>
      </div>
    </div>
  </div>
</template>
