<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Loader2, Trash2 } from 'lucide-vue-next'
import { useNetworkDetailStore } from '@/stores/NetworkDetail'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'
import { isValidCIDR, isValidIP } from '@/utils/ipValidation'
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

interface RouteDraft {
  target: string
  via: string
}

interface Props {
  open: boolean
  networkId: string | null
}

const props = defineProps<Props>()
const emit = defineEmits<{
  'update:open': [value: boolean]
}>()

const routes = ref<RouteDraft[]>([])
const submitting = ref(false)

watch(
  [() => props.open, () => props.networkId],
  ([isOpen, networkId]) => {
    if (!isOpen || !networkId) return
    const networkData = networkDetailStore.networksData.find((data) => data.id === networkId)
    if (!networkData) return
    routes.value = (networkData.routes ?? []).map((route) => ({
      target: route.target ?? '',
      via: route.via ?? '',
    }))
  },
  { immediate: true },
)

const handleSubmit = async () => {
  if (submitting.value || !props.networkId) return

  const nonEmptyRoutes = routes.value
    .map((route) => ({
      target: route.target.trim(),
      via: route.via.trim(),
    }))
    .filter((route) => route.target.length > 0 || route.via.length > 0)

  for (const route of nonEmptyRoutes) {
    if (!isValidCIDR(route.target)) {
      showSnackBar(t('network.routesInvalidTarget'), 'error')
      return
    }
    if (route.via.length > 0 && !isValidIP(route.via)) {
      showSnackBar(t('network.routesInvalidVia'), 'error')
      return
    }
  }

  const cleanedRoutes = nonEmptyRoutes.map((route) => ({
    target: route.target,
    via: route.via.length > 0 ? route.via : null,
  }))

  submitting.value = true
  try {
    const updated = await createOrUpdateNetwork(props.networkId, {
      routes: cleanedRoutes,
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

const addRoute = () => {
  routes.value.push({
    target: '',
    via: '',
  })
}

const removeRoute = (index: number) => {
  routes.value.splice(index, 1)
}
</script>

<template>
  <Dialog :open="open" @update:open="handleOpenChange">
    <DialogContent class="max-h-[90vh] max-w-[95vw] overflow-y-auto sm:max-w-[600px]">
      <DialogHeader>
        <DialogTitle>{{ t('network.routes') }}</DialogTitle>
        <DialogDescription>
          {{ t('network.default') }} / {{ t('network.via.default') }}
        </DialogDescription>
      </DialogHeader>
      <div class="max-h-[360px] space-y-3 overflow-y-auto py-4 pr-1">
        <div v-for="(route, index) in routes" :key="index" class="flex items-center gap-2">
          <Input
            v-model="route.target"
            type="text"
            placeholder="10.147.17.0/24"
            class="min-w-0 flex-1 font-mono"
            :disabled="submitting"
            autocomplete="off"
          />
          <Input
            v-model="route.via"
            type="text"
            :placeholder="t('network.via.notice')"
            class="min-w-0 flex-1 font-mono"
            :disabled="submitting"
            autocomplete="off"
          />
          <Button
            type="button"
            variant="destructive"
            size="icon"
            :disabled="submitting"
            @click="removeRoute(index)"
          >
            <Trash2 class="h-4 w-4" />
          </Button>
        </div>
        <div v-if="routes.length === 0" class="py-4 text-center text-sm text-muted-foreground">
          {{ t('common.noData') }}
        </div>
      </div>
      <DialogFooter class="gap-2">
        <Button type="button" variant="outline" :disabled="submitting" @click="handleOpenChange(false)">
          {{ t('common.cancel') }}
        </Button>
        <Button type="button" variant="secondary" :disabled="submitting" @click="addRoute">
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
