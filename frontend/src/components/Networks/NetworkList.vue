<script setup lang="ts">
import { computed, onBeforeMount, onBeforeUnmount, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import QRCode from 'qrcode'
import {
  ArrowRight,
  Calendar,
  Copy,
  Globe,
  Loader2,
  Lock,
  Plus,
  QrCode,
  RefreshCw,
  Search,
} from 'lucide-vue-next'
import { getNetworkById, listNetworkIds } from '@/api/zerotier/controller'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'
import { eventBus } from '@/utils/eventBus'
import { copyToClipboard } from '@/utils/copyToClipboard'
import DeleteNetworkDialog from '@/components/Networks/DeleteNetworkDialog.vue'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'

const emit = defineEmits<{ add: [] }>()

const { t } = useI18n()
const router = useRouter()

const networks = ref<ControllerNetworkInfo[]>([])
const loading = ref(false)
const searchKeyword = ref('')
const qrCodeMap = ref<Map<string, string>>(new Map())
const showQRCodeMap = ref<Map<string, boolean>>(new Map())

const filteredNetworks = computed(() => {
  const keyword = searchKeyword.value.trim().toLowerCase()
  if (!keyword) return networks.value
  return networks.value.filter(
    (network) =>
      (network.name || '').toLowerCase().includes(keyword) ||
      (network.id || '').toLowerCase().includes(keyword),
  )
})

const generateQRCode = async (networkId: string) => {
  try {
    const zerotierUri = `https://joinzt.com/addnetwork?nwid=${networkId}&v=1`
    const dataUrl = await QRCode.toDataURL(zerotierUri, {
      width: 200,
      margin: 2,
      color: {
        dark: '#000000',
        light: '#FFFFFF',
      },
    })
    qrCodeMap.value.set(networkId, dataUrl)
  } catch (err) {
    showSnackBar(`${t('common.updateFailed')}: ${getErrorMessage(err)}`, 'error')
  }
}

const toggleQRCode = async (networkId: string) => {
  const currentState = showQRCodeMap.value.get(networkId) || false
  showQRCodeMap.value.set(networkId, !currentState)

  if (!currentState && !qrCodeMap.value.has(networkId)) {
    await generateQRCode(networkId)
  }
}

const refreshNetworks = async () => {
  loading.value = true
  try {
    const networkIds = await listNetworkIds()
    const results = await Promise.allSettled(networkIds.map((id) => getNetworkById(id)))
    const fetched: ControllerNetworkInfo[] = []
    let failedCount = 0

    for (const res of results) {
      if (res.status === 'fulfilled') {
        fetched.push(res.value)
      } else {
        failedCount++
      }
    }

    networks.value = fetched
    if (failedCount > 0) {
      showSnackBar(t('network.detail.failGet'), 'error')
    }
  } catch (error) {
    showSnackBar(`${t('network.detail.failGet')}: ${getErrorMessage(error)}`, 'error')
  } finally {
    loading.value = false
  }
}

onBeforeMount(() => {
  eventBus.addEventListener('networkListChanged', refreshNetworks)
  void refreshNetworks()
})

onBeforeUnmount(() => {
  eventBus.removeEventListener('networkListChanged', refreshNetworks)
})
</script>

<template>
  <div class="space-y-4">
    <div class="flex flex-wrap items-center gap-2">
      <div class="relative min-w-[220px] flex-1">
        <Search class="pointer-events-none absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <Input v-model="searchKeyword" :placeholder="t('common.search')" class="pl-9" />
      </div>
      <span class="rounded-md border bg-card px-3 py-1.5 text-sm text-muted-foreground">
        {{ networks.length }} {{ t('network.default') }}
      </span>
      <Button type="button" variant="outline" size="sm" :disabled="loading" @click="refreshNetworks">
        <RefreshCw class="h-4 w-4" :class="{ 'animate-spin': loading }" />
        <span>{{ t('common.refresh') }}</span>
      </Button>
    </div>

    <div
      v-if="loading && networks.length === 0"
      class="flex items-center justify-center gap-2 rounded-xl border bg-card py-16 text-muted-foreground"
    >
      <Loader2 class="h-5 w-5 animate-spin" />
      <span>{{ t('common.loading') }}</span>
    </div>

    <div
      v-else-if="networks.length === 0"
      class="flex flex-col items-center justify-center gap-3 rounded-xl border bg-card px-6 py-16 text-center"
    >
      <p class="text-sm text-muted-foreground">{{ t('network.empty') }}</p>
      <Button type="button" size="sm" @click="emit('add')">
        <Plus class="h-4 w-4" />
        <span>{{ t('network.add.default') }}</span>
      </Button>
    </div>

    <div
      v-else-if="filteredNetworks.length === 0"
      class="rounded-xl border bg-card px-6 py-12 text-center text-sm text-muted-foreground"
    >
      {{ t('common.noData') }}
    </div>

    <div v-else class="grid gap-4">
      <div
        v-for="network in filteredNetworks"
        :key="network.id"
        class="flex flex-col justify-between gap-4 rounded-xl border bg-card p-5 text-card-foreground shadow-xs transition-shadow hover:shadow-md"
      >
        <div class="space-y-3">
          <div class="flex flex-wrap items-start justify-between gap-2">
            <button
              type="button"
              class="text-left text-xl font-bold tracking-tight hover:text-primary hover:underline"
              @click="router.push(`/network/${network.id}/overview`)"
            >
              {{ network.name || t('network.unnamed') }}
            </button>
            <span
              class="inline-flex items-center gap-1 rounded-full border px-2.5 py-0.5 text-xs font-medium"
              :class="
                network.private
                  ? 'border-amber-200 bg-amber-50 text-amber-800'
                  : 'border-emerald-200 bg-emerald-50 text-emerald-800'
              "
            >
              <Lock v-if="network.private" class="h-3 w-3" />
              <Globe v-else class="h-3 w-3" />
              {{ network.private ? t('network.type.private') : t('network.type.public') }}
            </span>
          </div>

          <div class="flex flex-wrap items-center gap-2 text-sm">
            <Button
              v-if="network.id"
              variant="secondary"
              size="sm"
              class="font-mono text-xs"
              @click="copyToClipboard(network.id)"
            >
              <span>{{ network.id }}</span>
              <Copy class="h-3.5 w-3.5" />
            </Button>
            <Button
              v-if="network.id"
              variant="outline"
              size="sm"
              @click="toggleQRCode(network.id)"
            >
              <QrCode class="h-3.5 w-3.5" />
              <span>
                {{ showQRCodeMap.get(network.id) ? t('network.hideQRCode') : t('network.showQRCode') }}
              </span>
            </Button>
            <span
              v-if="network.creationTime"
              class="inline-flex items-center gap-1 text-xs text-muted-foreground"
            >
              <Calendar class="h-3.5 w-3.5" />
              {{ new Date(network.creationTime).toLocaleDateString() }}
            </span>
          </div>

          <div v-if="network.routes && network.routes.length > 0" class="flex flex-wrap gap-1.5">
            <span
              v-for="(route, idx) in network.routes"
              :key="idx"
              class="rounded bg-muted px-2 py-0.5 font-mono text-xs text-muted-foreground"
            >
              {{ route.target }}{{ route.via ? ` via ${route.via}` : '' }}
            </span>
          </div>

          <div
            v-if="network.id && showQRCodeMap.get(network.id) && qrCodeMap.get(network.id)"
            class="flex justify-center pt-2"
          >
            <div class="rounded-lg border bg-white p-4 shadow-xs">
              <img
                :src="qrCodeMap.get(network.id)"
                :alt="'QR Code for ' + network.id"
                class="h-44 w-44"
              />
              <p class="mt-2 text-center text-xs text-muted-foreground">
                {{ t('network.scanToJoin') }}
              </p>
            </div>
          </div>
        </div>

        <div class="flex flex-wrap items-center justify-end gap-2 border-t pt-3">
          <DeleteNetworkDialog
            v-if="network.id"
            :network-id="network.id"
            @deleted="refreshNetworks"
          />
          <Button
            type="button"
            @click="router.push(`/network/${network.id}/overview`)"
          >
            <span>{{ t('network.enter') }}</span>
            <ArrowRight class="h-4 w-4" />
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>
