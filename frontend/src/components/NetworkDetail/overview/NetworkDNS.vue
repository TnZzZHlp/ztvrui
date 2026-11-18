<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { Settings } from 'lucide-vue-next'
import { useCurrentNetwork } from '@/composables/useCurrentNetwork'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import ModifyDNSDialog from './ModifyDNSDialog.vue'

const { t } = useI18n()
const { networkData } = useCurrentNetwork()

const showDialog = ref(false)
</script>

<template>
  <div v-if="networkData" class="space-y-4 rounded-xl border bg-card p-5 text-card-foreground shadow-xs">
    <div class="flex items-center justify-between">
      <h2 class="text-base font-semibold">DNS</h2>
      <Button type="button" variant="ghost" size="icon" aria-label="DNS Settings" @click="showDialog = true">
        <Settings class="h-4 w-4" />
      </Button>
    </div>

    <Separator />

    <div class="space-y-3">
      <div>
        <p class="mb-1 text-sm font-medium text-muted-foreground">
          {{ t('network.dns.domain') }}
        </p>
        <p class="text-base font-medium break-all">
          {{ networkData.dns?.domain || t('common.noData') }}
        </p>
      </div>

      <Separator />

      <div>
        <p class="mb-2 text-sm font-medium text-muted-foreground">
          {{ t('network.dns.servers') }}
        </p>
        <ol v-if="networkData.dns?.servers && networkData.dns.servers.length > 0" class="space-y-1">
          <li
            v-for="(server, index) in networkData.dns.servers"
            :key="index"
            class="relative pl-4 font-mono text-sm text-foreground before:absolute before:left-0 before:content-['•']"
          >
            {{ server }}
          </li>
        </ol>
        <p v-else class="text-sm text-muted-foreground">
          {{ t('common.noData') }}
        </p>
      </div>
    </div>

    <ModifyDNSDialog
      :open="showDialog"
      :network-id="networkData.id ?? null"
      @update:open="showDialog = $event"
    />
  </div>
</template>
